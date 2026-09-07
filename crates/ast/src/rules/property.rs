use crate::*;

use crate::{
    AstNodeClone, AstNodeStorage, ExtraData, ExtraDataClone, ExtraDataCompact, NodeKind,
    NodePayload,
};

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum ParsedComponent<'a> {
    Length(NodeId<'a, Length<'a>>),
    Number(f32),
    Percentage(f32),
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
    String(AstStr<'a>),
    Color(NodeId<'a, CssColor<'a>>),
    Image(NodeId<'a, Image<'a>>),
    Url(NodeId<'a, Url<'a>>),
    Integer(i32),
    Angle(Angle),
    Time(Time),
    Resolution(Resolution),
    TransformFunction(NodeId<'a, Transform<'a>>),
    TransformList(Vec<'a, NodeId<'a, Transform<'a>>>),
    CustomIdent(AstStr<'a>),
    Literal(AstStr<'a>),
    Repeated {
        components: Vec<'a, NodeId<'a, ParsedComponent<'a>>>,
        multiplier: Multiplier,
    },
    TokenList(Vec<'a, TokenOrValue<'a>>),
}

unsafe impl<'ast> AstNodeStorage<'ast> for ParsedComponent<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0017_0005);
    #[inline]
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    #[inline]
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    #[inline]
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::String(left), Self::String(right))
            | (Self::CustomIdent(left), Self::CustomIdent(right))
            | (Self::Literal(left), Self::Literal(right)) => {
                left == right || context.str(*left) == context.str(*right)
            }
            _ => self == other,
        }
    }
}

impl<'ast> AstNodeClone<'ast> for ParsedComponent<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            Self::Number(value) => Self::Number(value),
            Self::Percentage(value) => Self::Percentage(value),
            Self::LengthPercentage(value) => {
                Self::LengthPercentage(context.clone_encoded_node(value))
            }
            Self::String(value) => Self::String(value),
            Self::Color(value) => Self::Color(context.clone_encoded_node(value)),
            Self::Image(value) => Self::Image(context.clone_encoded_node(value)),
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
            Self::Integer(value) => Self::Integer(value),
            Self::Angle(value) => Self::Angle(value),
            Self::Time(value) => Self::Time(value),
            Self::Resolution(value) => Self::Resolution(value),
            Self::TransformFunction(value) => {
                Self::TransformFunction(context.clone_encoded_node(value))
            }
            Self::TransformList(values) => Self::TransformList(context.clone_encoded_vec(values)),
            Self::CustomIdent(value) => Self::CustomIdent(value),
            Self::Literal(value) => Self::Literal(value),
            Self::Repeated {
                components,
                multiplier,
            } => Self::Repeated {
                components: context.clone_encoded_vec(components),
                multiplier,
            },
            Self::TokenList(values) => Self::TokenList(context.clone_encoded_vec(values)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum Multiplier {
    None,
    Space,
    Comma,
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum SyntaxString<'a> {
    Components(Vec<'a, SyntaxComponent<'a>>),
    Universal,
}

impl_inline_node!(SyntaxString<'ast>, 0x0017_0001);

impl<'ast> AstNodeClone<'ast> for SyntaxString<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Components(values) => Self::Components(context.clone_encoded_vec(values)),
            Self::Universal => Self::Universal,
        }
    }
}

#[derive(Clone, Copy, CssKeyword, Debug, PartialEq, Visit)]
pub enum SyntaxComponentKind<'a> {
    Length,
    Number,
    Percentage,
    LengthPercentage,
    String,
    Color,
    Image,
    Url,
    Integer,
    Angle,
    Time,
    Resolution,
    TransformFunction,
    TransformList,
    CustomIdent,
    Literal(AstStr<'a>),
}

// SAFETY: this KIND identifies native SyntaxComponentKind values.
unsafe impl<'ast> AstNodeStorage<'ast> for SyntaxComponentKind<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0017_0002);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::Literal(left), Self::Literal(right)) => {
                left == right || context.str(*left) == context.str(*right)
            }
            _ => self == other,
        }
    }

    #[inline]
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        // SAFETY: the typed node owner validated KIND.
        unsafe { payload.read_value() }
    }
    #[inline]
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    #[inline]
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
}

impl<'ast> AstNodeClone<'ast> for SyntaxComponentKind<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub struct UnparsedProperty<'a> {
    pub property_id: NodeId<'a, PropertyId<'a>>,
    #[visit(skip)]
    pub reason: UnparsedPropertyReason,
    /// The authored value after removing declaration-level whitespace and
    /// `!important`. This keeps fallback serialization independent from the
    /// lossy numeric and function normalization used by typed tokens.
    #[visit(skip)]
    pub raw_value: Option<AstStr<'a>>,
    pub value: Vec<'a, TokenOrValue<'a>>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct UnparsedPropertyHeader<'a> {
    raw_value: Option<AstStr<'a>>,
    extra: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct UnparsedPropertyMetadata<'a> {
    property_id: NodeId<'a, PropertyId<'a>>,
    reason: UnparsedPropertyReason,
}

pub use access::UnparsedPropertyRef;

// Access views are storage infrastructure, not persistent visitor nodes.
mod access {
    use super::*;

    /// Read-only fields of a stored fallback declaration. The context borrow keeps
    /// the header and its overflow stable while fields are accessed.
    pub struct UnparsedPropertyRef<'context, 'storage, 'ast> {
        context: &'context AstContext<'storage>,
        header: UnparsedPropertyHeader<'ast>,
    }

    impl<'ast> AstContext<'ast> {
        #[inline]
        pub fn unparsed_property<'id>(
            &self,
            id: NodeId<'_, UnparsedProperty<'id>>,
        ) -> UnparsedPropertyRef<'_, 'ast, 'id> {
            // SAFETY: the checked node kind stores this native header. Lifetimes
            // only brand its ranges; the ID's lifetime is retained on returned ranges.
            let header = unsafe { self.node_payload(id).read_value() };
            UnparsedPropertyRef {
                context: self,
                header,
            }
        }
    }

    impl<'ast> UnparsedPropertyRef<'_, '_, 'ast> {
        #[inline]
        pub fn raw_value(&self) -> Option<AstStr<'ast>> {
            self.header.raw_value
        }

        #[inline]
        pub fn value(&self) -> Vec<'ast, TokenOrValue<'ast>> {
            // SAFETY: the first overflow slot is written as this token-list range.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            }
        }

        #[inline]
        pub fn property_id(&self) -> NodeId<'ast, PropertyId<'ast>> {
            // SAFETY: the second overflow slot is written as native metadata.
            let metadata: UnparsedPropertyMetadata<'ast> = unsafe {
                self.context
                    .extra_slot(self.header.extra as usize + 1)
                    .read_value()
            };
            metadata.property_id
        }
    }
}

unsafe impl<'ast> AstNodeStorage<'ast> for UnparsedProperty<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0017_0003);
    #[inline]
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: UnparsedPropertyHeader<'ast> = unsafe { payload.read_value() };
        let extra = header.extra as usize;
        let metadata: UnparsedPropertyMetadata<'ast> =
            unsafe { context.extra_slot(extra + 1).read_value() };
        Self {
            raw_value: header.raw_value,
            value: unsafe { context.extra_slot(extra).read_value() },
            property_id: metadata.property_id,
            reason: metadata.reason,
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_unparsed_property(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: UnparsedPropertyHeader<'ast> = unsafe { current.read_value() };
        encode_unparsed_property(self, Some(header.extra as usize), context)
    }
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        self.property_id == other.property_id
            && self.reason == other.reason
            && self.value == other.value
            && self.raw_value.map(|value| context.str(value))
                == other.raw_value.map(|value| context.str(value))
    }
}

impl<'ast> AstNodeClone<'ast> for UnparsedProperty<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            property_id: context.clone_encoded_node(self.property_id),
            reason: self.reason,
            raw_value: self.raw_value,
            value: context.clone_encoded_vec(self.value),
        }
    }
}

fn encode_unparsed_property<'ast>(
    value: UnparsedProperty<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let low = ExtraData::from_value(value.value);
    let high = ExtraData::from_value(UnparsedPropertyMetadata {
        property_id: value.property_id,
        reason: value.reason,
    });
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, low);
            context.set_extra_slot(extra + 1, high);
            extra
        }
        None => context.alloc_extra_slots([low, high]),
    };
    NodePayload::from_value(UnparsedPropertyHeader {
        raw_value: value.raw_value,
        extra: u32::try_from(extra).expect("UnparsedProperty overflow index exceeds u32"),
    })
}

/// Why a declaration could not be represented by its typed value AST.
///
/// Keeping this decision in the parsed tree lets transforms distinguish
/// unsupported grammar from values whose syntax or semantics are opaque.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Visit)]
pub enum UnparsedPropertyReason {
    /// RocketCSS recognizes the property, but does not implement its grammar yet.
    UnsupportedGrammar,
    /// The property name is unknown, so its value grammar is also unknown.
    UnknownProperty,
    /// A supported grammar contains a function or comment that cannot be
    /// validated without preserving its original token representation.
    OpaqueValue,
    /// The implemented grammar rejected an otherwise tokenizable value.
    InvalidValue,
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub struct CustomProperty<'a> {
    pub name: NodeId<'a, CustomPropertyName<'a>>,
    pub value: Vec<'a, TokenOrValue<'a>>,
}

impl_inline_node!(CustomProperty<'ast>, 0x0017_0004);

impl<'ast> AstNodeClone<'ast> for CustomProperty<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            name: context.clone_encoded_node(self.name),
            value: context.clone_encoded_vec(self.value),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub struct SyntaxComponent<'a> {
    pub kind: NodeId<'a, SyntaxComponentKind<'a>>,
    pub multiplier: Multiplier,
}

unsafe impl<'ast> ExtraDataCompact<'ast> for SyntaxComponent<'ast> {
    #[inline]
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(self)
    }
    #[inline]
    unsafe fn decode_extra(data: ExtraData) -> Self {
        unsafe { data.read_value() }
    }
}

impl<'ast> ExtraDataClone<'ast> for SyntaxComponent<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            kind: context.clone_encoded_node(self.kind),
            multiplier: self.multiplier,
        }
    }
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, AstStr, CustomProperty, CustomPropertyName, DUMMY_SP, KeyframesName,
        NoneOrCustomIdentList, ParsedComponent, PropertyId, SyntaxComponent, SyntaxComponentKind,
        SyntaxString, TokenOrValue, UnparsedProperty, UnparsedPropertyReason,
    };

    use super::Multiplier;

    #[test]
    fn property_metadata_codecs_round_trip_and_deep_clone_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let literal_text = context.add_str("|");
        let literal =
            context.alloc_encoded_node(SyntaxComponentKind::Literal(literal_text), DUMMY_SP);
        let components = context.alloc_encoded_vec(
            [SyntaxComponent {
                kind: literal,
                multiplier: Multiplier::Comma,
            }]
            .into_iter(),
        );
        let syntax = context.alloc_encoded_node(SyntaxString::Components(components), DUMMY_SP);
        let cloned = context.clone_encoded_node(syntax);
        let SyntaxString::Components(cloned_components) = context.encoded_node(cloned) else {
            panic!("expected component syntax")
        };
        assert_ne!(components, cloned_components);
        let component = context
            .encoded_vec_get(cloned_components, 0)
            .expect("cloned syntax component");
        assert_ne!(component.kind, literal);
        assert_eq!(
            context.encoded_node(component.kind),
            SyntaxComponentKind::Literal(literal_text)
        );
    }

    #[test]
    fn parsed_component_codec_deep_clones_recursive_component_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let text = context.add_str("/");
        let literal = context.alloc_encoded_node(ParsedComponent::Literal(text), DUMMY_SP);
        let components = context.alloc_encoded_vec([literal].into_iter());
        let repeated = context.alloc_encoded_node(
            ParsedComponent::Repeated {
                components,
                multiplier: Multiplier::Space,
            },
            DUMMY_SP,
        );

        let cloned = context.clone_encoded_node(repeated);
        let ParsedComponent::Repeated {
            components: cloned_components,
            multiplier,
        } = context.encoded_node(cloned)
        else {
            panic!("expected repeated component")
        };
        assert_eq!(multiplier, Multiplier::Space);
        assert_ne!(cloned_components, components);
        let cloned_literal = context
            .encoded_vec_get(cloned_components, 0)
            .expect("cloned parsed component");
        assert_ne!(cloned_literal, literal);
        assert_eq!(
            context.encoded_node(cloned_literal),
            ParsedComponent::Literal(text)
        );
    }

    #[test]
    fn unparsed_property_reuses_its_fixed_overflow_slot() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let property_name = context.add_str("future-prop");
        let property_id = context.alloc_encoded_node(PropertyId::Custom(property_name), DUMMY_SP);
        let ident = context.add_str("--fallback");
        let ident = context.alloc_encoded_node(crate::DashedIdent { value: ident }, DUMMY_SP);
        let value = context.alloc_encoded_vec([TokenOrValue::DashedIdent(ident)].into_iter());
        let raw_value = context.add_str(" var(--fallback) ");
        let before = context.encoded_extra_len();
        let property = context.alloc_encoded_node(
            UnparsedProperty {
                property_id,
                reason: UnparsedPropertyReason::UnknownProperty,
                raw_value: Some(raw_value),
                value,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), before + 2);
        assert_eq!(context.encoded_node(property).value, value);

        context.mutate_encoded_node(property, |value, _| {
            value.reason = UnparsedPropertyReason::OpaqueValue;
            value.raw_value = None;
        });
        assert_eq!(context.encoded_extra_len(), before + 2);
        let decoded = context.encoded_node(property);
        assert_eq!(decoded.reason, UnparsedPropertyReason::OpaqueValue);
        assert_eq!(decoded.raw_value, None);
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for reason in [
            UnparsedPropertyReason::UnsupportedGrammar,
            UnparsedPropertyReason::UnknownProperty,
            UnparsedPropertyReason::OpaqueValue,
            UnparsedPropertyReason::InvalidValue,
        ] {
            for raw in [Some(raw_value), Some(AstStr::EMPTY), None] {
                context.mutate_encoded_node(property, |node, _| {
                    node.reason = reason;
                    node.raw_value = raw;
                });
                let actual = context.encoded_node(property);
                assert_eq!(actual.reason, reason);
                assert_eq!(actual.raw_value, raw);
                assert_eq!(actual.property_id, property_id);
                assert_eq!(actual.value, value);
                let view = context.unparsed_property(property);
                assert_eq!(view.raw_value(), raw);
                assert_eq!(view.property_id(), property_id);
                assert_eq!(view.value(), value);
            }
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);

        let cloned = context.clone_encoded_node(property);
        let cloned = context.encoded_node(cloned);
        assert_ne!(cloned.property_id, property_id);
        assert_ne!(cloned.value, value);
        let Some(TokenOrValue::DashedIdent(cloned_ident)) =
            context.encoded_vec_get(cloned.value, 0)
        else {
            panic!("expected cloned dashed identifier");
        };
        assert_ne!(cloned_ident, ident);
        assert_eq!(
            context.str(context.encoded_node(cloned_ident).value),
            "--fallback"
        );
    }

    #[test]
    fn compact_name_and_custom_property_codecs_keep_typed_ids() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let text = context.add_str("--theme");
        let name = context.alloc_encoded_node(CustomPropertyName::Custom(text), DUMMY_SP);
        let ident = context.add_str("--base");
        let ident = context.alloc_encoded_node(crate::DashedIdent { value: ident }, DUMMY_SP);
        let value = context.alloc_encoded_vec([TokenOrValue::DashedIdent(ident)].into_iter());
        let property = context.alloc_encoded_node(CustomProperty { name, value }, DUMMY_SP);
        let cloned_property = context.clone_encoded_node(property);
        let cloned = context.encoded_node(cloned_property);
        assert_ne!(cloned.name, name);
        assert_ne!(cloned.value, value);
        assert_eq!(
            context.encoded_node(cloned.name),
            CustomPropertyName::Custom(text)
        );

        let motion = context.add_str("--motion");
        let keyframes = context.alloc_encoded_node(KeyframesName::Custom(motion), DUMMY_SP);
        assert_eq!(
            context.encoded_node(keyframes),
            KeyframesName::Custom(motion)
        );
        let names = [context.add_str("one"), context.add_str("two")];
        let idents = context.alloc_encoded_vec(names.into_iter());
        let names = context.alloc_encoded_node(NoneOrCustomIdentList::Idents(idents), DUMMY_SP);
        let cloned_names = context.clone_encoded_node(names);
        let NoneOrCustomIdentList::Idents(cloned_idents) = context.encoded_node(cloned_names)
        else {
            panic!("expected custom identifier list")
        };
        assert_ne!(cloned_idents, idents);
        assert_eq!(
            context.str(context.encoded_vec_get(cloned_idents, 1).unwrap()),
            "two"
        );
    }
}
