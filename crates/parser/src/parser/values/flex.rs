use crate::prelude::*;

keyword_parse!(
    FlexDirection,
    "row" => Self::Row,
    "row-reverse" => Self::RowReverse,
    "column" => Self::Column,
    "column-reverse" => Self::ColumnReverse,
);
keyword_parse!(
    FlexWrap,
    "nowrap" => Self::Nowrap,
    "wrap" => Self::Wrap,
    "wrap-reverse" => Self::WrapReverse,
);
keyword_parse!(
    BoxOrient,
    "horizontal" => Self::Horizontal,
    "vertical" => Self::Vertical,
    "inline-axis" => Self::InlineAxis,
    "block-axis" => Self::BlockAxis,
);
keyword_parse!(BoxDirection, "normal" => Self::Normal, "reverse" => Self::Reverse,);
keyword_parse!(
    BoxAlign,
    "start" => Self::Start,
    "end" => Self::End,
    "center" => Self::Center,
    "baseline" => Self::Baseline,
    "stretch" => Self::Stretch,
);
keyword_parse!(
    BoxPack,
    "start" => Self::Start,
    "end" => Self::End,
    "center" => Self::Center,
    "justify" => Self::Justify,
);
keyword_parse!(BoxLines, "single" => Self::Single, "multiple" => Self::Multiple,);
keyword_parse!(
    FlexPack,
    "start" => Self::Start,
    "end" => Self::End,
    "center" => Self::Center,
    "justify" => Self::Justify,
    "distribute" => Self::Distribute,
);
keyword_parse!(
    FlexItemAlign,
    "auto" => Self::Auto,
    "start" => Self::Start,
    "end" => Self::End,
    "center" => Self::Center,
    "baseline" => Self::Baseline,
    "stretch" => Self::Stretch,
);
keyword_parse!(
    FlexLinePack,
    "start" => Self::Start,
    "end" => Self::End,
    "center" => Self::Center,
    "justify" => Self::Justify,
    "distribute" => Self::Distribute,
    "stretch" => Self::Stretch,
);
