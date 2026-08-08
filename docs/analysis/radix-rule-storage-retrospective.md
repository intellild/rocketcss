# RadixRange 用于 CssRule 存储的失败复盘

## 结论

这次失败并不是 `RadixArena` 作为通用容器完全没有价值，而是它不适合承担
RocketCSS 中 `CssRuleList` 的主存储和遍历拓扑。

我们为了少量后置插入，引入了 Radix ID、sibling sidecar、range cursor、
`descendants` 和多段迭代器；但 CSS 规则的主要负载是线性遍历、已知 ID 的随机访问，
以及在已知位置附近插入。它并不需要按 `start_id + offset` 查找第 N 条规则。
最终，罕见插入的成本被转移到了每一次常规遍历上，而且原有 topology 并没有真正被
range 取代，AST 同时维护了两套结构。

对 CssRule 应恢复为：连续的稳定 ID 存储负责身份和局部性，紧凑的 rule-list topology
负责语义顺序；插入集中在 reification 阶段，通过 append 或复用安全墓碑完成。

## 最初目标

Radix 方案试图同时满足：

- Parser 按源码顺序线性构建 AST；
- `RuleId` 在 transform 期间保持稳定；
- Nano 完成主要分析后允许少量插入；
- 插入时不复制或重建整个 AST；
- `RuleList` 和子树可以用一个 range 表示；
- 当前层遍历能够根据 `descendants` 跳过深层节点。

预期执行流是：

```text
Parser
  按源码 preorder 分配 CssRule
       ↓
AST
  RuleList = RadixRange<RuleId>
       ↓
Nano / transforms
  少量 sibling 插入
       ↓
Codegen
  range 遍历并按 descendants 跳过子树
```

这个设计把“稳定身份”“物理存储顺序”“语义遍历顺序”和“插入空间”绑定在了同一种
ID 编码和容器结构上。后来所有复杂性都来自这个绑定。

## 对访问模式的误判

CssRule 的真实访问模式主要是：

- 按语义顺序遍历整个列表；
- 用已经明确持有的 `RuleId` 随机访问；
- 遍历某条规则的直接子规则；
- transform 保留 previous/current/next 上下文；
- 在已知 entry 前后插入、删除或替换；
- codegen 按最终 topology 输出。

典型场景包括前缀生成、nesting lowering、相邻规则合并、`@media`/`@layer`
处理、source map 和最终序列化。它们需要“下一个语义节点”，但没有按规则序号访问
`start_id + offset` 的需求。CSS 的 `:nth-child()` 描述 DOM，也与 AST 中第 N 条规则
的随机访问无关。

这与 declaration block 不同：声明列表确实可能存在 block 内位置访问，因此可以使用
简单的连续 `DenseRange<I> { start, len }`。这不意味着 CssRuleList 也应使用同一种
range 抽象。

## 被忽略的更简单方案

我们把“在 `Vec` 中间插入会移动全部元素”当成核心约束，却没有充分利用另一种结构：

- `DenseStore` 只 append，保持 `RuleId` 稳定；
- 语义顺序由 list 中的 `next_sibling` 表示；
- 新规则 append 到 store，随后只 splice topology；
- Parser 产生的绝大多数 next ID 天然连续，仍有良好的局部性；
- transform 已知插入位置，不需要从头寻找 sibling；
- Nano 的主要分析在插入前完成一次线性读取。

更符合实际 pipeline 的执行流是：

```text
Parser
  DenseStore push，源码 preorder、稳定 RuleId
       ↓
Persistent Scheduler
  插入前对 DenseStore 做一次线性读取
       ↓
ReificationPlan
  汇总所有结构修改
       ↓
一次性更新 AST
  append / 复用已证明安全的墓碑，并 splice RuleList topology
       ↓
Codegen
  按 RuleList 迭代器输出最终语义顺序
```

这里不要求插入后物理存储仍保持完整的语义顺序。只要稳定 ID 和最终 topology 正确，
就无需为罕见插入改变常规遍历的数据结构。

## 实现为何越来越复杂

### Range 没有真正替代 topology

当前演进过程中，rule record 同时出现或保留了这些信息：

- `parent` / `parent_list`；
- `previous_sibling` / `next_sibling`；
- `previous_in_source` / `next_in_source`；
- `child_list`；
- `descendants`；
- range 的 `start` / `len` 或 endpoint 状态。

range 主要用于 `rules_in_list` 和一致性校验，而 mutation 仍依赖 topology。结果不是把
链式结构换成 range，而是在链式结构旁边增加了第二套需要同步维护的结构。

### RadixRange 也不能提供通用 O(1) offset

一旦 range 中出现 sibling radix 节点，语义上的第 N 个元素就不能再由简单整数加法
得到，仍需要 cursor 按段推进。只有纯 primary 段才具备直接索引能力。因此它没有为
CssRule 提供真实需要的随机访问能力，却为不需要的 offset 语义付出了额外成本。

### 业务状态泄漏到容器边界

`live`/tombstone 是 AST 的业务状态，不应成为 RadixArena 的遍历语义。当前实现虽然由
上层 `RuleListIter.remaining_live` 处理 live 数量，但 range 的物理跨度仍包含退休节点，
导致上层必须理解容器范围、墓碑和语义长度之间的差异。

### 容量与重平衡问题

每个 primary 的 sibling radix 容量有限。超容之后需要更高层结构负责重平衡、重映射
或拆分 range。这会继续扩大 mutation API 和失败处理，而实际 CSS pipeline 的插入数量
很少，DenseStore append + topology splice 不存在这一类容量上限。

## 基准数据

新增的纯容器 benchmark 排除了 StyleSheet、业务 live 过滤和 transform，只比较相同
`RadixArena` 上的 direct range traversal 与 topology link traversal。每个节点均为 16
字节，构建过程不计时，规模提高到 16K–262K 以减少纳秒级噪音。

| 场景 | Range 中位数 | Topology 中位数 | 结果 |
| --- | ---: | ---: | ---: |
| flat 65,536 | 120.0 µs | 46.26 µs | Range 慢 2.59× |
| flat 262,144 | 480.1 µs | 184.5 µs | Range 慢 2.60× |
| sparse sibling 65,536 | 122.6 µs | 108.1 µs | Range 慢 13.4% |
| sparse sibling 262,144 | 486.5 µs | 434.7 µs | Range 慢 11.9% |
| deep 16,384 | 74.70 µs | 11.47 µs | Range 慢 6.51× |
| deep 65,536 | 301.4 µs | 47.41 µs | Range 慢 6.36× |

命令：

```sh
cargo bench -p rocketcss_benchmark --bench radix_range -- \
  --timer tsc --sample-count 30 --sample-size 100
```

稀疏 sibling 场景缩小到约 12% 差距，说明 lazy sibling cursor 和 primary-only segment
确实改善了原实现；但最常见的 flat traversal 仍慢约 2.6 倍，短 range/deep tree 场景
慢约 6.4 倍。为了罕见插入让所有普通遍历承担这个成本，不符合负载比例。

需要注意，这里的 topology benchmark 仍通过 `RadixArena::get` 读取节点。真正的
`DenseStore` + topology 预计会更简单，至少不会比这个基线引入 Radix 解码成本。

## 火焰图和汇编揭示的成本

分段迭代器并没有被编译成预期的“纯 primary slice hot loop”：

- `RadixDirectRangeIter::next_entry` 仍是约 1.6 KiB 的独立函数，每个元素调用一次；
- 它包含 primary/sibling segment 状态机、分支和函数栈开销；
- `from_range` 和 `advance_segment` 也保留为独立函数；
- iterator 状态约 80 字节，构造时需要初始化/复制；
- deep tree 中每个 range 只读取一个 direct child，固定构造成本无法摊薄；
- topology loop 被内联后只是读取当前 record 的 `next` 并访问下一项。

这里的“链表”不是堆上随机指针链表：它使用稳定整数 ID，Parser 产生的 ID 基本连续，
record 又连续存放。`next` 通常与 payload 位于同一 cache line，所以经典链表的缓存
劣势在该负载下并不成立。

元数据也没有形成优势：direct range traversal 需要 `descendants: u32`，单向 topology
只需要 `next_sibling: Option<RuleId>`，两者通常都是 4 字节。

## 结构性成本

即使继续优化迭代器，Radix 方案仍保留以下固定成本：

- 复杂的 ID 编码和解码；
- sibling sidecar/radix tree；
- primary sibling 容量限制；
- 超容后的重平衡和 ID/remap 语义；
- range endpoint、semantic len 与 tombstone 的同步；
- primary/sibling 双段 cursor 和更大的迭代器状态；
- 为通用容器暴露容易被 AST 误用的 advance/insert API；
- AST mutation 与容器内部结构耦合。

继续针对某一个 iterator hot path 做微优化，无法消除这些结构性成本。

## 推荐的数据结构

```text
StyleSheet
  rules: DenseStore<RuleId, RuleRecord>

RuleList
  first: Option<RuleId>
  last: Option<RuleId>
  len: u32

RuleRecord
  rule: CssRule
  parent_list: RuleListId
  next_sibling: Option<RuleId>
  child_list: Option<RuleListId>
  declaration_block: Option<DeclarationBlockId>
  revision/live state as required
```

约束如下：

- Parser 只按源码 preorder push；
- 顺序遍历只能通过带状态的 iterator；
- 随机访问只接受明确已知的 `RuleId`；
- iterator 自身保留父 list 和 neighbor context，禁止从 list 头重复扫描；
- mutation 通过已解析的 entry 执行，避免重复寻找和重复判断；
- ReificationPlan 汇总插入后一次性应用；
- 新节点优先复用已证明不会再被引用的墓碑，否则 append；
- `last` 由 `RuleList` 明确保存，不能通过 ID 算术猜测；
- AST 细节封装在 stylesheet/AST 模块，不泄漏给 compiler 和 transforms。

`previous_sibling`、source-order links 等额外字段是否保留，应由实际调用点和 benchmark
单独证明。Persistent Scheduler 与 ReificationPlan 已经可以携带上下文时，不应为了
方便查询再增加反向链或重复索引。

## 保留的正确方向

本次失败不否定以下设计：

- Parser 按源码顺序线性分配；
- 使用 typed stable ID；
- DeclarationBlock 与 EffectiveKey 一起保存在 AST；
- Persistent Scheduler 一次线性读取；
- ReificationPlan 集中应用结构修改；
- 明确墓碑的所有权和安全复用时机；
- 通过 AST iterator 封装遍历细节；
- 用纯容器 benchmark 和火焰图验证抽象成本。

这些方向应在更简单的 DenseStore + compact topology 上实现。

## 后续决策

1. 停止以 CssRule 场景为理由继续优化 Radix direct range iterator。
2. 用同一组 flat/sparse/deep 数据构建 `DenseStore + next_sibling` benchmark。
3. benchmark 必须区分 parse-time linear scan、transform neighbor traversal、插入和
   codegen traversal，不能只看合成的随机访问。
4. 若 DenseStore topology 结果符合预期，再渐进迁移 CssRule 存储；每一步保持 AST
   行为和序列化结果不变。
5. Declaration range 使用独立、简单的 dense range，不继承 Radix 的 sibling 语义。
6. `RadixArena` 可以暂时作为 common 中的实验容器保留，但其去留应由独立使用场景
   决定，不能再反向塑造 CSS AST。

最终原则是：容器只解决存储问题，AST topology 表达 CSS 语义，scheduler/reification
决定修改时机。不要再让一种 ID 编码同时承担这三层职责。
