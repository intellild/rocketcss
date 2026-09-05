use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum FilterList<'a> {
    None,
    Filters(Vec<'a, NodeId<'a, Filter<'a>>>),
}

impl<'ast> AstNodeStorage<'ast> for FilterList<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0022_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Filters(
                context
                    .encoded_vec_range(read_u32(&bytes, 4) as usize, read_u32(&bytes, 8) as usize),
            ),
            _ => panic!("invalid encoded FilterList variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        if let Self::Filters(values) = self {
            bytes[0] = 1;
            write_u32(&mut bytes, 4, values.start_index());
            write_u32(&mut bytes, 8, values.end_index());
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for FilterList<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Filters(values) => Self::Filters(context.clone_encoded_vec(values)),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum Filter<'a> {
    Blur(NodeId<'a, Length<'a>>),
    Brightness(NumberOrPercentage),
    Contrast(NumberOrPercentage),
    Grayscale(NumberOrPercentage),
    HueRotate(Angle),
    Invert(NumberOrPercentage),
    Opacity(NumberOrPercentage),
    Saturate(NumberOrPercentage),
    Sepia(NumberOrPercentage),
    DropShadow(NodeId<'a, DropShadow<'a>>),
    Url(NodeId<'a, Url<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for Filter<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0022_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let data = read_u32(&bytes, 4);
        let number = || decode_number_or_percentage(bytes[1], data);
        match bytes[0] {
            0 => Self::Blur(context.encoded_node_id_at(data as usize)),
            1 => Self::Brightness(number()),
            2 => Self::Contrast(number()),
            3 => Self::Grayscale(number()),
            4 => Self::HueRotate(crate::token::decode_angle(bytes[1], f32::from_bits(data))),
            5 => Self::Invert(number()),
            6 => Self::Opacity(number()),
            7 => Self::Saturate(number()),
            8 => Self::Sepia(number()),
            9 => Self::DropShadow(context.encoded_node_id_at(data as usize)),
            10 => Self::Url(context.encoded_node_id_at(data as usize)),
            _ => panic!("invalid encoded Filter variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        let data = match self {
            Self::Blur(value) => node_variant(0, value, &mut bytes),
            Self::Brightness(value) => number_variant(1, value, &mut bytes),
            Self::Contrast(value) => number_variant(2, value, &mut bytes),
            Self::Grayscale(value) => number_variant(3, value, &mut bytes),
            Self::HueRotate(value) => {
                bytes[0] = 4;
                let (kind, value) = crate::token::encode_angle(value);
                bytes[1] = kind;
                value.to_bits()
            }
            Self::Invert(value) => number_variant(5, value, &mut bytes),
            Self::Opacity(value) => number_variant(6, value, &mut bytes),
            Self::Saturate(value) => number_variant(7, value, &mut bytes),
            Self::Sepia(value) => number_variant(8, value, &mut bytes),
            Self::DropShadow(value) => node_variant(9, value, &mut bytes),
            Self::Url(value) => node_variant(10, value, &mut bytes),
        };
        write_u32(&mut bytes, 4, data);
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for Filter<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Blur(value) => Self::Blur(context.clone_encoded_node(value)),
            Self::DropShadow(value) => Self::DropShadow(context.clone_encoded_node(value)),
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

fn node_variant<T>(tag: u8, value: NodeId<'_, T>, bytes: &mut [u8]) -> u32 {
    bytes[0] = tag;
    u32::try_from(value.index()).expect("AST node ID exceeds four bytes")
}

fn number_variant(tag: u8, value: NumberOrPercentage, bytes: &mut [u8]) -> u32 {
    bytes[0] = tag;
    let (kind, value) = match value {
        NumberOrPercentage::Number(value) => (0, value),
        NumberOrPercentage::Percentage(value) => (1, value),
    };
    bytes[1] = kind;
    value.to_bits()
}

fn decode_number_or_percentage(kind: u8, value: u32) -> NumberOrPercentage {
    match kind {
        0 => NumberOrPercentage::Number(f32::from_bits(value)),
        1 => NumberOrPercentage::Percentage(f32::from_bits(value)),
        _ => panic!("invalid encoded NumberOrPercentage variant"),
    }
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn write_u32(bytes: &mut [u8], offset: usize, value: impl TryInto<u32>) {
    bytes[offset..offset + 4].copy_from_slice(
        &value
            .try_into()
            .unwrap_or_else(|_| panic!("AST compact value exceeds four bytes"))
            .to_le_bytes(),
    );
}
