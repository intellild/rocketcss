use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Length<'ast>(pub(crate) NodeId<'ast, LengthData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LengthData<'ast> {
    Value(LengthValue<'ast>),
    Calc(Calc<'ast, Length<'ast>>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LengthKind {
    Value,
    Calc,
}
impl<'ast> NodeStorage<'ast> for Length<'ast> {
    const KIND: NodeKind = super::kind::KIND_LENGTH;
    type Data = LengthData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, LengthData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, LengthData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Length<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> LengthData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> LengthKind {
        match ast.data(self) {
            LengthData::Value { .. } => LengthKind::Value,
            LengthData::Calc { .. } => LengthKind::Calc,
        }
    }
}
impl<'ast> GetSpan<'ast> for Length<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Length<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_length(&mut self, data: LengthData<'ast>, span: Span) -> Length<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LengthUnit {
    Px,
    In,
    Cm,
    Mm,
    Q,
    Pt,
    Pc,
    Em,
    Rem,
    Ex,
    Rex,
    Ch,
    Rch,
    Cap,
    Rcap,
    Ic,
    Ric,
    Lh,
    Rlh,
    Vw,
    Lvw,
    Svw,
    Dvw,
    Cqw,
    Vh,
    Lvh,
    Svh,
    Dvh,
    Cqh,
    Vi,
    Svi,
    Lvi,
    Dvi,
    Cqi,
    Vb,
    Svb,
    Lvb,
    Dvb,
    Cqb,
    Vmin,
    Svmin,
    Lvmin,
    Dvmin,
    Cqmin,
    Vmax,
    Svmax,
    Lvmax,
    Dvmax,
    Cqmax,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Calc<'ast, V>(pub(crate) NodeId<'ast, CalcData<'ast, V>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CalcData<'ast, V> {
    Value(V),
    Number(f32),
    Sum((Calc<'ast, V>, Calc<'ast, V>)),
    Product((f32, Calc<'ast, V>)),
    Function(MathFunction<'ast, V>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CalcKind {
    Value,
    Number,
    Sum,
    Product,
    Function,
}
impl<'ast, V> NodeStorage<'ast> for Calc<'ast, V>
where
    V: Copy,
{
    const KIND: NodeKind = super::kind::KIND_CALC;
    type Data = CalcData<'ast, V>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CalcData<'ast, V>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CalcData<'ast, V>>) -> Self {
        Self(id)
    }
}
impl<'ast, V> Calc<'ast, V>
where
    V: Copy,
{
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> CalcData<'ast, V> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> CalcKind {
        match ast.data(self) {
            CalcData::Value { .. } => CalcKind::Value,
            CalcData::Number { .. } => CalcKind::Number,
            CalcData::Sum { .. } => CalcKind::Sum,
            CalcData::Product { .. } => CalcKind::Product,
            CalcData::Function { .. } => CalcKind::Function,
        }
    }
}
impl<'ast, V> GetSpan<'ast> for Calc<'ast, V>
where
    V: Copy,
{
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast, V> SetSpan<'ast> for Calc<'ast, V>
where
    V: Copy,
{
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_calc<V>(&mut self, data: CalcData<'ast, V>, span: Span) -> Calc<'ast, V>
    where
        V: Copy,
    {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MathFunction<'ast, V>(pub(crate) NodeId<'ast, MathFunctionData<'ast, V>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MathFunctionData<'ast, V> {
    Calc(Calc<'ast, V>),
    Min(&'ast [Calc<'ast, V>]),
    Max(&'ast [Calc<'ast, V>]),
    Clamp((Calc<'ast, V>, Calc<'ast, V>, Calc<'ast, V>)),
    Round((RoundingStrategy, Calc<'ast, V>, Calc<'ast, V>)),
    Rem((Calc<'ast, V>, Calc<'ast, V>)),
    Mod((Calc<'ast, V>, Calc<'ast, V>)),
    Abs(Calc<'ast, V>),
    Sign(Calc<'ast, V>),
    Hypot(&'ast [Calc<'ast, V>]),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MathFunctionKind {
    Calc,
    Min,
    Max,
    Clamp,
    Round,
    Rem,
    Mod,
    Abs,
    Sign,
    Hypot,
}
impl<'ast, V> NodeStorage<'ast> for MathFunction<'ast, V>
where
    V: Copy,
{
    const KIND: NodeKind = super::kind::KIND_MATH_FUNCTION;
    type Data = MathFunctionData<'ast, V>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MathFunctionData<'ast, V>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MathFunctionData<'ast, V>>) -> Self {
        Self(id)
    }
}
impl<'ast, V> MathFunction<'ast, V>
where
    V: Copy,
{
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> MathFunctionData<'ast, V> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> MathFunctionKind {
        match ast.data(self) {
            MathFunctionData::Calc { .. } => MathFunctionKind::Calc,
            MathFunctionData::Min { .. } => MathFunctionKind::Min,
            MathFunctionData::Max { .. } => MathFunctionKind::Max,
            MathFunctionData::Clamp { .. } => MathFunctionKind::Clamp,
            MathFunctionData::Round { .. } => MathFunctionKind::Round,
            MathFunctionData::Rem { .. } => MathFunctionKind::Rem,
            MathFunctionData::Mod { .. } => MathFunctionKind::Mod,
            MathFunctionData::Abs { .. } => MathFunctionKind::Abs,
            MathFunctionData::Sign { .. } => MathFunctionKind::Sign,
            MathFunctionData::Hypot { .. } => MathFunctionKind::Hypot,
        }
    }
}
impl<'ast, V> GetSpan<'ast> for MathFunction<'ast, V>
where
    V: Copy,
{
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast, V> SetSpan<'ast> for MathFunction<'ast, V>
where
    V: Copy,
{
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_math_function<V>(
        &mut self,
        data: MathFunctionData<'ast, V>,
        span: Span,
    ) -> MathFunction<'ast, V>
    where
        V: Copy,
    {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum RoundingStrategy {
    Nearest,
    Up,
    Down,
    ToZero,
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Resolution {
    Dpi(f32),
    Dpcm(f32),
    Dppx(f32),
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ratio<'ast>(pub(crate) NodeId<'ast, RatioData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RatioData {
    pub denominator: Option<f32>,
    pub numerator: f32,
}
impl<'ast> NodeStorage<'ast> for Ratio<'ast> {
    const KIND: NodeKind = super::kind::KIND_RATIO;
    type Data = RatioData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, RatioData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, RatioData>) -> Self {
        Self(id)
    }
}
impl<'ast> Ratio<'ast> {
    #[inline]
    pub fn denominator(self, ast: &Ast<'ast>) -> Option<f32> {
        ast.data(self).denominator
    }
    #[inline]
    pub fn set_denominator(self, ast: &mut Ast<'ast>, value: Option<f32>) {
        let mut data = ast.data(self);
        data.denominator = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn numerator(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).numerator
    }
    #[inline]
    pub fn set_numerator(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.numerator = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Ratio<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Ratio<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_ratio(&mut self, data: RatioData, span: Span) -> Ratio<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Angle {
    Deg(f32),
    Rad(f32),
    Grad(f32),
    Turn(f32),
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Time {
    Seconds(f32),
    Milliseconds(f32),
}
