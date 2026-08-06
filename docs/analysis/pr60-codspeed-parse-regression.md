# PR #60 CodSpeed 结果分析：parse 回归与优化空间

日期：2026-08-04（基准：本地 benchmark 复测 + CodSpeed PR 数据）
分支：`codex/bplus-sequence-flat-ast-design`（PR #60，HEAD `893dd19`，BASE = master `648fce1`）

## 1. CodSpeed 原始结果（PR check）

| Benchmark          | BASE     | HEAD     | 变化          |
| ------------------ | -------- | -------- | ------------- |
| parse[bootstrap]   | 178.7 ms | 217.6 ms | **-17.9% ❌** |
| parse[tailwind]    | 334.1 ms | 378.6 ms | **-11.8% ❌** |
| minify[bootstrap]  | 84.7 ms  | 54.1 ms  | +56.6% ⚡     |
| minify[tailwind]   | 215.1 ms | 158.7 ms | +35.5% ⚡     |
| codegen[bootstrap] | 24.0 ms  | 19.3 ms  | +23.9% ⚡     |
| codegen[tailwind]  | 58.6 ms  | 49.6 ms  | +18.2% ⚡     |

> 注意：check 报告了 "Different runtime environments detected" 警告（BASE 与 HEAD 跑在不同 runner 上），绝对百分比可能含环境噪声。本地复测确认回归真实存在，见下。

## 2. 本地复测（同一台机器，mimalloc，release）

`tasks/benchmark/benches/pipeline.rs` 的 `parse` bench：每轮 `Allocator::new()` + `parse`（bootstrap ×10 轮、tailwind ×1 轮），median of 100 samples。

| 版本                       | parse[bootstrap] | parse[tailwind]  |
| -------------------------- | ---------------- | ---------------- |
| master `648fce1`（跑两次） | 51.65 / 51.81 ms | 85.91 / 84.54 ms |
| HEAD `893dd19`（跑两次）   | 64.34 / 65.09 ms | 101.8 / 104.7 ms |
| **回归**                   | **约 +24%**      | **约 +19%**      |

同机复测与 CodSpeed 方向一致（CodSpeed 的 -17.9%/-11.8% 可能是跨 runner 的绝对值差被低估，或环境不同）。

## 3. 回归来源定位（bisect，全部同机顺序运行）

用 worktree 逐 commit 复测 parse bench（同一 target dir 内两两配对比较，排除构建缓存差异）：

| commit                                                        | bootstrap | tailwind     | 说明                   |
| ------------------------------------------------------------- | --------- | ------------ | ---------------------- |
| master `648fce1`                                              | 51.7 ms   | 84.5–85.9 ms | 基线                   |
| `0c05487` refactor(ast): migrate CSS pipeline to radix stores | 55.6 ms   | 95.6 ms      | **回归起点**           |
| `61a59a1` Complete typed property AST coverage                | 69.1 ms   | 112.6 ms     | 继续恶化               |
| `6d185ce` refactor(parser): replay decoded tokens…            | 66.2 ms   | 106.9 ms     | 相对父提交**回稳** ~4% |

结论：**回归主要由 radix 存储迁移（`0c05487` 起）引入**；后续 replay 提交反而把 parse 拉回了一点。尾部 commit 累计的 typed 覆盖扩大也加剧了（typed 尝试失败→fallback 重读，见 §5）。

## 4. 容量预分配：bootstrap 低估、tailwind 高估

`stylesheet_capacity`（crates/parser/src/parser/radix_ast/mod.rs:88）按 `source.len()/160`、`/80`、`/192`、`/1024` 估计。用真实 parse 统计（临时 example 计数）：

| 输入                   | rules 实际 | 估计 `/160`       | declarations 实际 | 估计 `/80`        | blocks 实际 |
| ---------------------- | ---------- | ----------------- | ----------------- | ----------------- | ----------- |
| bootstrap.css (281 KB) | **2677**   | 1756（低估 35%）  | **5542**          | 3513（低估 37%）  | 2562        |
| tailwind.css (5.7 MB)  | 31190      | 35821（高估 13%） | 61659             | 71643（高估 14%） | 28822       |

- bootstrap 规则/声明密度高（utility CSS，短声明），`/160`、`/80` 明显低估 → 触发一次几何扩容，是 bootstrap 回归的一个放大因素。
- tailwind 每条规则/声明的体积更大，估计反而偏高；但 `declarations` 容量直接以 `with_capacity` 给了 `DenseStore`，高估只是多占内存，不影响时间。
- `selectors: /192` 对 tailwind：`intern_selector_value` 只在 bucket 未命中时 push，`root_selector_paths` 按此预分配——高估无害。

## 5. 根因：parse 路径上的哈希互操作 + 每规则固定开销变高

对比 master 与 radix 版的 parse 热路径，新增/变贵的固定成本集中在：

1. **选择器去重（每 style rule 一次，tailwind 的 2.6 万+ 规则全走）**
   - `selector_value_fingerprint`（effective_key.rs:1111）：每次对整棵 `SelectorList` 做 `FxHasher` 全遍历哈希——master 没有这一步。
   - bucket 内线性扫描 + `value.selectors == selectors` 深度相等比较（可能重新哈希 selector 内部字段）。
   - `intern_selector_path`（effective_key.rs:971）每次都要查 `FxHashMap`，并重新计算父子 fingerprint 组合哈希。
   - `context_value`/`layer_context` 同理，每个 wrapper 规则多一次哈希/查询。
   - 这些去重对 minify 阶段有巨大收益（minify +56%/+35% 正是共享 selector/context 的结果），但 parse 阶段纯属净成本。

2. **`enter_selector_context` 每规则一次**（effective_key.rs:591）：每进一个 style rule 就 `intern_selector_path`。

3. **append 路径的簿记**
   - `append_rule`（stylesheet/mod.rs）：除 push 外，还要维护 `previous_in_source/next_in_source` 双向链 + `previous_sibling/next_sibling` 双向链，每个规则 2 次 `get_mut` 指针写。
   - `append_declaration`（stylesheet/mod.rs）：每次 `try_next_id` + range 连续性校验 + 记录 push，比 master 的 `DeclarationBlock.push` 多一次 ID 记账。

4. **容量低估**（§4）：bootstrap 触发几何扩容。

## 6. 可执行的优化方向（按性价比排序）

### A. 消除 parse 阶段纯净成本的去重（预期收益最大）

- `intern_selector_value` 的 fingerprint 遍历 + bucket 线性扫描是 parse 的最大新增成本。可：
  - 指纹改用廉价结构哈希（如只哈希 selector 顶层 `SelectorComponent` 类型序列 / 原子 ID 数组），而不是深度全遍历；碰撞时再用精确相等兜底。
  - 或 parse 阶段只做**去重禁用**（`selectors` 直接 push 一条记录、fingerprint 标记为惰性），把去重推迟到 minify 之前一次成批完成——minify 已有 `refresh_*`/transform 通道，批量重建去重索引比逐条在线互操作便宜。
- `enter_selector_context` 的 `intern_selector_path`：root 级 path 可缓存 `root_selector_paths[value.index()]` 直读（已有），确认走的是快速路径；非 root 再查 map。

### B. 修容量启发式（已完成）

- 新除数（calibrated，crates/parser/src/parser/radix_ast/mod.rs:88）：`rules: /96`、`rule_lists: /512`、`declarations: /44`、`selectors: /100`、`contexts: /512`。
- 实测裕量（bootstrap / tailwind）：
  | store        | bootstrap 实际 | 估计 | tailwind 实际 | 估计   |
  | ------------ | -------------- | ---- | ------------- | ------ |
  | rules        | 2677           | 2927 | 31190         | 59703  |
  | declarations | 5542           | 6387 | 61659         | 130261 |
  | selectors    | 2498           | 2810 | 25012         | 57314  |
  | contexts     | 18             | 549  | 82            | 11194  |
- 新增保护测试 `capacity_estimates_cover_benchmark_corpora`（crates/parser/src/parser/radix_ast/tests.rs）：解析两个 corpus 后断言每个 store 的实际数量 ≤ 容量估计，防止未来 denser 语法回退。
- 实测 parse 耗时：bootstrap 64.1–64.7ms、tailwind 107.4–108.2ms，与改动前（64.3/105）在 ~5% 噪声范围内——**消除了几何扩容这一变量，但扩容本来就不是主要成本**；剩余回归主要来自 §5 的哈希互操作。

### C. 砍 append 簿记（中等收益）

- `RuleRecord` 里的 `previous_in_source/next_in_source` 若只在 minify/代码生成时用，可改为**惰性重建**：parse 阶段只记 `last_rule_in_source`，minify 前一次性串链；省掉每规则 2 次 `get_mut`。
- `append_declaration` 的 `NonContiguousDeclarationRange` 校验是 debug 语义，可 `debug_assert` 化（release 下已是纯加法，收益小）。

### D. 验证方法

- 建议把 parse 拆成两个 CodSpeed job（BASE/HEAD 同一 runner 重跑）消除 "Different runtime environments" 噪声后再定版；或直接在本地 CI 加 `parse` 门禁。
- 逐项改动用 `pipeline` bench 的 `parse` 单独跑，和 master 基线做括号式 A/B（master 51.7/84.5 → HEAD 65/105）。

### E. 不要动的部分

- `DeclarationTokenReplay` 与 `collect_tokens_impl` 不是回归源（§3），replay 反而回稳 ~4%，保持现状。
- minify/codegen 的收益（+56%/+35%/+24%/+18%）来自同一套 radix 去重存储，A 项若把去重惰性化，必须保持 minify 侧收益不回退。
