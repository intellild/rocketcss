//! Generated visitor callback bitset.
/// Identifies the callbacks implemented by a visitor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisitorMethods {
    words: [u64; 7usize],
}
impl VisitorMethods {
    #[doc(hidden)]
    pub const ALL: Self = Self {
        words: [
            18446744073709551615u64,
            18446744073709551615u64,
            18446744073709551615u64,
            18446744073709551615u64,
            18446744073709551615u64,
            18446744073709551615u64,
            65535u64,
        ],
    };
    #[doc(hidden)]
    pub const fn from_words(words: [u64; 7usize]) -> Self {
        Self { words }
    }
    #[doc(hidden)]
    #[inline]
    pub const fn contains(&self, method: usize) -> bool {
        self.words[method / u64::BITS as usize] & (1 << (method % u64::BITS as usize)) != 0
    }
    #[doc(hidden)]
    pub const ENTER_NODE: usize = 0usize;
    #[doc(hidden)]
    pub const LEAVE_NODE: usize = 1usize;
    #[doc(hidden)]
    pub const VISIT_STR: usize = 2usize;
    #[doc(hidden)]
    pub const VISIT_CSS_COLOR: usize = 3usize;
    #[doc(hidden)]
    pub const VISIT_RGBA: usize = 4usize;
    #[doc(hidden)]
    pub const VISIT_LAB_COLOR: usize = 5usize;
    #[doc(hidden)]
    pub const VISIT_PREDEFINED_COLOR: usize = 6usize;
    #[doc(hidden)]
    pub const VISIT_FLOAT_COLOR: usize = 7usize;
    #[doc(hidden)]
    pub const VISIT_LIGHT_DARK: usize = 8usize;
    #[doc(hidden)]
    pub const VISIT_SYSTEM_COLOR: usize = 9usize;
    #[doc(hidden)]
    pub const VISIT_UNRESOLVED_COLOR: usize = 10usize;
    #[doc(hidden)]
    pub const VISIT_CSS_RULE: usize = 11usize;
    #[doc(hidden)]
    pub const VISIT_LENGTH: usize = 12usize;
    #[doc(hidden)]
    pub const VISIT_LENGTH_UNIT: usize = 13usize;
    #[doc(hidden)]
    pub const VISIT_CALC: usize = 14usize;
    #[doc(hidden)]
    pub const VISIT_MATH_FUNCTION: usize = 15usize;
    #[doc(hidden)]
    pub const VISIT_ROUNDING_STRATEGY: usize = 16usize;
    #[doc(hidden)]
    pub const VISIT_RESOLUTION: usize = 17usize;
    #[doc(hidden)]
    pub const VISIT_RATIO: usize = 18usize;
    #[doc(hidden)]
    pub const VISIT_ANGLE: usize = 19usize;
    #[doc(hidden)]
    pub const VISIT_TIME: usize = 20usize;
    #[doc(hidden)]
    pub const VISIT_MEDIA_CONDITION: usize = 21usize;
    #[doc(hidden)]
    pub const VISIT_QUERY_FEATURE: usize = 22usize;
    #[doc(hidden)]
    pub const VISIT_MEDIA_FEATURE_NAME: usize = 23usize;
    #[doc(hidden)]
    pub const VISIT_MEDIA_FEATURE_ID: usize = 24usize;
    #[doc(hidden)]
    pub const VISIT_MEDIA_FEATURE_VALUE: usize = 25usize;
    #[doc(hidden)]
    pub const VISIT_MEDIA_FEATURE_COMPARISON: usize = 26usize;
    #[doc(hidden)]
    pub const VISIT_OPERATOR: usize = 27usize;
    #[doc(hidden)]
    pub const VISIT_MEDIA_TYPE: usize = 28usize;
    #[doc(hidden)]
    pub const VISIT_QUALIFIER: usize = 29usize;
    #[doc(hidden)]
    pub const VISIT_SUPPORTS_CONDITION: usize = 30usize;
    #[doc(hidden)]
    pub const VISIT_BLEND_MODE: usize = 31usize;
    #[doc(hidden)]
    pub const VISIT_TRANSITION: usize = 32usize;
    #[doc(hidden)]
    pub const VISIT_SCROLL_TIMELINE: usize = 33usize;
    #[doc(hidden)]
    pub const VISIT_VIEW_TIMELINE: usize = 34usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION_RANGE: usize = 35usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION: usize = 36usize;
    #[doc(hidden)]
    pub const VISIT_SUPPORTS_RULE: usize = 37usize;
    #[doc(hidden)]
    pub const VISIT_COUNTER_STYLE_RULE: usize = 38usize;
    #[doc(hidden)]
    pub const VISIT_NAMESPACE_RULE: usize = 39usize;
    #[doc(hidden)]
    pub const VISIT_MOZ_DOCUMENT_RULE: usize = 40usize;
    #[doc(hidden)]
    pub const VISIT_NESTING_RULE: usize = 41usize;
    #[doc(hidden)]
    pub const VISIT_NESTED_DECLARATIONS_RULE: usize = 42usize;
    #[doc(hidden)]
    pub const VISIT_VIEWPORT_RULE: usize = 43usize;
    #[doc(hidden)]
    pub const VISIT_CUSTOM_MEDIA_RULE: usize = 44usize;
    #[doc(hidden)]
    pub const VISIT_LAYER_STATEMENT_RULE: usize = 45usize;
    #[doc(hidden)]
    pub const VISIT_LAYER_BLOCK_RULE: usize = 46usize;
    #[doc(hidden)]
    pub const VISIT_SCOPE_RULE: usize = 47usize;
    #[doc(hidden)]
    pub const VISIT_STARTING_STYLE_RULE: usize = 48usize;
    #[doc(hidden)]
    pub const VISIT_POSITION_TRY_RULE: usize = 49usize;
    #[doc(hidden)]
    pub const VISIT_UNKNOWN_AT_RULE: usize = 50usize;
    #[doc(hidden)]
    pub const VISIT_POSITION: usize = 51usize;
    #[doc(hidden)]
    pub const VISIT_WEB_KIT_GRADIENT_POINT: usize = 52usize;
    #[doc(hidden)]
    pub const VISIT_WEB_KIT_COLOR_STOP: usize = 53usize;
    #[doc(hidden)]
    pub const VISIT_IMAGE_SET: usize = 54usize;
    #[doc(hidden)]
    pub const VISIT_IMAGE_SET_OPTION: usize = 55usize;
    #[doc(hidden)]
    pub const VISIT_BACKGROUND_POSITION: usize = 56usize;
    #[doc(hidden)]
    pub const VISIT_BACKGROUND_REPEAT: usize = 57usize;
    #[doc(hidden)]
    pub const VISIT_BACKGROUND: usize = 58usize;
    #[doc(hidden)]
    pub const VISIT_BOX_SHADOW: usize = 59usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_RADIUS: usize = 60usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_IMAGE_REPEAT: usize = 61usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_IMAGE_SLICE: usize = 62usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_IMAGE: usize = 63usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_COLOR: usize = 64usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_STYLE: usize = 65usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_WIDTH: usize = 66usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_BLOCK_COLOR: usize = 67usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_BLOCK_STYLE: usize = 68usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_BLOCK_WIDTH: usize = 69usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_INLINE_COLOR: usize = 70usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_INLINE_STYLE: usize = 71usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_INLINE_WIDTH: usize = 72usize;
    #[doc(hidden)]
    pub const VISIT_GENERIC_BORDER: usize = 73usize;
    #[doc(hidden)]
    pub const VISIT_CONTAINER_CONDITION: usize = 74usize;
    #[doc(hidden)]
    pub const VISIT_CONTAINER_SIZE_FEATURE_ID: usize = 75usize;
    #[doc(hidden)]
    pub const VISIT_STYLE_QUERY: usize = 76usize;
    #[doc(hidden)]
    pub const VISIT_SCROLL_STATE_QUERY: usize = 77usize;
    #[doc(hidden)]
    pub const VISIT_SCROLL_STATE_FEATURE_ID: usize = 78usize;
    #[doc(hidden)]
    pub const VISIT_CONTAINER: usize = 79usize;
    #[doc(hidden)]
    pub const VISIT_CONTAINER_RULE: usize = 80usize;
    #[doc(hidden)]
    pub const VISIT_FONT_FACE_PROPERTY: usize = 81usize;
    #[doc(hidden)]
    pub const VISIT_SOURCE: usize = 82usize;
    #[doc(hidden)]
    pub const VISIT_FONT_FORMAT: usize = 83usize;
    #[doc(hidden)]
    pub const VISIT_FONT_TECHNOLOGY: usize = 84usize;
    #[doc(hidden)]
    pub const VISIT_FONT_FACE_STYLE: usize = 85usize;
    #[doc(hidden)]
    pub const VISIT_FONT_PALETTE_VALUES_PROPERTY: usize = 86usize;
    #[doc(hidden)]
    pub const VISIT_BASE_PALETTE: usize = 87usize;
    #[doc(hidden)]
    pub const VISIT_FONT_FEATURE_SUBRULE_TYPE: usize = 88usize;
    #[doc(hidden)]
    pub const VISIT_FONT: usize = 89usize;
    #[doc(hidden)]
    pub const VISIT_FONT_FACE_RULE: usize = 90usize;
    #[doc(hidden)]
    pub const VISIT_URL_SOURCE: usize = 91usize;
    #[doc(hidden)]
    pub const VISIT_UNICODE_RANGE: usize = 92usize;
    #[doc(hidden)]
    pub const VISIT_FONT_PALETTE_VALUES_RULE: usize = 93usize;
    #[doc(hidden)]
    pub const VISIT_OVERRIDE_COLORS: usize = 94usize;
    #[doc(hidden)]
    pub const VISIT_FONT_FEATURE_VALUES_RULE: usize = 95usize;
    #[doc(hidden)]
    pub const VISIT_FONT_FEATURE_SUBRULE: usize = 96usize;
    #[doc(hidden)]
    pub const VISIT_FONT_FEATURE_DECLARATION: usize = 97usize;
    #[doc(hidden)]
    pub const VISIT_FAMILY_NAME: usize = 98usize;
    #[doc(hidden)]
    pub const VISIT_KEYFRAME_SELECTOR: usize = 99usize;
    #[doc(hidden)]
    pub const VISIT_KEYFRAMES_NAME: usize = 100usize;
    #[doc(hidden)]
    pub const VISIT_KEYFRAMES_RULE: usize = 101usize;
    #[doc(hidden)]
    pub const VISIT_KEYFRAME: usize = 102usize;
    #[doc(hidden)]
    pub const VISIT_TIMELINE_RANGE_PERCENTAGE: usize = 103usize;
    #[doc(hidden)]
    pub const VISIT_ASPECT_RATIO: usize = 104usize;
    #[doc(hidden)]
    pub const VISIT_OVERFLOW: usize = 105usize;
    #[doc(hidden)]
    pub const VISIT_INSET_BLOCK: usize = 106usize;
    #[doc(hidden)]
    pub const VISIT_INSET_INLINE: usize = 107usize;
    #[doc(hidden)]
    pub const VISIT_INSET: usize = 108usize;
    #[doc(hidden)]
    pub const VISIT_FLEX_FLOW: usize = 109usize;
    #[doc(hidden)]
    pub const VISIT_FLEX: usize = 110usize;
    #[doc(hidden)]
    pub const VISIT_PLACE_CONTENT: usize = 111usize;
    #[doc(hidden)]
    pub const VISIT_PLACE_SELF: usize = 112usize;
    #[doc(hidden)]
    pub const VISIT_PLACE_ITEMS: usize = 113usize;
    #[doc(hidden)]
    pub const VISIT_GAP: usize = 114usize;
    #[doc(hidden)]
    pub const VISIT_TRACK_REPEAT: usize = 115usize;
    #[doc(hidden)]
    pub const VISIT_GRID_AUTO_FLOW: usize = 116usize;
    #[doc(hidden)]
    pub const VISIT_GRID_TEMPLATE: usize = 117usize;
    #[doc(hidden)]
    pub const VISIT_GRID: usize = 118usize;
    #[doc(hidden)]
    pub const VISIT_GRID_ROW: usize = 119usize;
    #[doc(hidden)]
    pub const VISIT_GRID_COLUMN: usize = 120usize;
    #[doc(hidden)]
    pub const VISIT_GRID_AREA: usize = 121usize;
    #[doc(hidden)]
    pub const VISIT_MARGIN_BLOCK: usize = 122usize;
    #[doc(hidden)]
    pub const VISIT_MARGIN_INLINE: usize = 123usize;
    #[doc(hidden)]
    pub const VISIT_MARGIN: usize = 124usize;
    #[doc(hidden)]
    pub const VISIT_PADDING_BLOCK: usize = 125usize;
    #[doc(hidden)]
    pub const VISIT_PADDING_INLINE: usize = 126usize;
    #[doc(hidden)]
    pub const VISIT_PADDING: usize = 127usize;
    #[doc(hidden)]
    pub const VISIT_SCROLL_MARGIN_BLOCK: usize = 128usize;
    #[doc(hidden)]
    pub const VISIT_SCROLL_MARGIN_INLINE: usize = 129usize;
    #[doc(hidden)]
    pub const VISIT_SCROLL_MARGIN: usize = 130usize;
    #[doc(hidden)]
    pub const VISIT_SCROLL_PADDING_BLOCK: usize = 131usize;
    #[doc(hidden)]
    pub const VISIT_SCROLL_PADDING_INLINE: usize = 132usize;
    #[doc(hidden)]
    pub const VISIT_SCROLL_PADDING: usize = 133usize;
    #[doc(hidden)]
    pub const VISIT_PAGE_MARGIN_BOX: usize = 134usize;
    #[doc(hidden)]
    pub const VISIT_PAGE_PSEUDO_CLASS: usize = 135usize;
    #[doc(hidden)]
    pub const VISIT_PAGE_RULE: usize = 136usize;
    #[doc(hidden)]
    pub const VISIT_PAGE_MARGIN_RULE: usize = 137usize;
    #[doc(hidden)]
    pub const VISIT_PAGE_SELECTOR: usize = 138usize;
    #[doc(hidden)]
    pub const VISIT_PARSED_COMPONENT: usize = 139usize;
    #[doc(hidden)]
    pub const VISIT_MULTIPLIER: usize = 140usize;
    #[doc(hidden)]
    pub const VISIT_SYNTAX_STRING: usize = 141usize;
    #[doc(hidden)]
    pub const VISIT_SYNTAX_COMPONENT_KIND: usize = 142usize;
    #[doc(hidden)]
    pub const VISIT_UNPARSED_PROPERTY: usize = 143usize;
    #[doc(hidden)]
    pub const VISIT_CUSTOM_PROPERTY: usize = 144usize;
    #[doc(hidden)]
    pub const VISIT_PROPERTY_RULE: usize = 145usize;
    #[doc(hidden)]
    pub const VISIT_SYNTAX_COMPONENT: usize = 146usize;
    #[doc(hidden)]
    pub const VISIT_INSET_RECT: usize = 147usize;
    #[doc(hidden)]
    pub const VISIT_CIRCLE_SHAPE: usize = 148usize;
    #[doc(hidden)]
    pub const VISIT_ELLIPSE_SHAPE: usize = 149usize;
    #[doc(hidden)]
    pub const VISIT_POLYGON: usize = 150usize;
    #[doc(hidden)]
    pub const VISIT_POINT: usize = 151usize;
    #[doc(hidden)]
    pub const VISIT_MASK: usize = 152usize;
    #[doc(hidden)]
    pub const VISIT_MASK_BORDER: usize = 153usize;
    #[doc(hidden)]
    pub const VISIT_DROP_SHADOW: usize = 154usize;
    #[doc(hidden)]
    pub const VISIT_DEFAULT_AT_RULE: usize = 155usize;
    #[doc(hidden)]
    pub const VISIT_STYLE_SHEET: usize = 156usize;
    #[doc(hidden)]
    pub const VISIT_MEDIA_RULE: usize = 157usize;
    #[doc(hidden)]
    pub const VISIT_MEDIA_LIST: usize = 158usize;
    #[doc(hidden)]
    pub const VISIT_MEDIA_QUERY: usize = 159usize;
    #[doc(hidden)]
    pub const VISIT_LENGTH_VALUE: usize = 160usize;
    #[doc(hidden)]
    pub const VISIT_ENVIRONMENT_VARIABLE: usize = 161usize;
    #[doc(hidden)]
    pub const VISIT_URL: usize = 162usize;
    #[doc(hidden)]
    pub const VISIT_VARIABLE: usize = 163usize;
    #[doc(hidden)]
    pub const VISIT_DASHED_IDENT_REFERENCE: usize = 164usize;
    #[doc(hidden)]
    pub const VISIT_FUNCTION: usize = 165usize;
    #[doc(hidden)]
    pub const VISIT_FUNCTION_REPLACEMENT: usize = 166usize;
    #[doc(hidden)]
    pub const VISIT_IMPORT_RULE: usize = 167usize;
    #[doc(hidden)]
    pub const VISIT_STYLE_RULE: usize = 168usize;
    #[doc(hidden)]
    pub const VISIT_DECLARATION_BLOCK: usize = 169usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_TRANSFORM: usize = 170usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_INDENT: usize = 171usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_DECORATION: usize = 172usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_EMPHASIS: usize = 173usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_EMPHASIS_POSITION: usize = 174usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_SHADOW: usize = 175usize;
    #[doc(hidden)]
    pub const VISIT_MATRIX_FOR_FLOAT: usize = 176usize;
    #[doc(hidden)]
    pub const VISIT_MATRIX_3_D_FOR_FLOAT: usize = 177usize;
    #[doc(hidden)]
    pub const VISIT_ROTATE: usize = 178usize;
    #[doc(hidden)]
    pub const VISIT_CURSOR: usize = 179usize;
    #[doc(hidden)]
    pub const VISIT_CURSOR_IMAGE: usize = 180usize;
    #[doc(hidden)]
    pub const VISIT_CARET: usize = 181usize;
    #[doc(hidden)]
    pub const VISIT_LIST_STYLE: usize = 182usize;
    #[doc(hidden)]
    pub const VISIT_COMPOSES: usize = 183usize;
    #[doc(hidden)]
    pub const VISIT_COLOR_SCHEME: usize = 184usize;
    #[doc(hidden)]
    pub const VISIT_VIEW_TRANSITION_PROPERTY: usize = 185usize;
    #[doc(hidden)]
    pub const VISIT_NAVIGATION: usize = 186usize;
    #[doc(hidden)]
    pub const VISIT_VIEW_TRANSITION_PART_SELECTOR: usize = 187usize;
    #[doc(hidden)]
    pub const VISIT_VIEW_TRANSITION_RULE: usize = 188usize;
    #[doc(hidden)]
    pub const VISIT_SELECTOR_COMPONENT: usize = 189usize;
    #[doc(hidden)]
    pub const VISIT_COMBINATOR: usize = 190usize;
    #[doc(hidden)]
    pub const VISIT_ATTR_SELECTOR: usize = 191usize;
    #[doc(hidden)]
    pub const VISIT_NAMESPACE_CONSTRAINT: usize = 192usize;
    #[doc(hidden)]
    pub const VISIT_ATTR_OPERATION: usize = 193usize;
    #[doc(hidden)]
    pub const VISIT_PARSED_CASE_SENSITIVITY: usize = 194usize;
    #[doc(hidden)]
    pub const VISIT_ATTR_SELECTOR_OPERATOR: usize = 195usize;
    #[doc(hidden)]
    pub const VISIT_NTH_TYPE: usize = 196usize;
    #[doc(hidden)]
    pub const VISIT_NTH_SELECTOR_DATA: usize = 197usize;
    #[doc(hidden)]
    pub const VISIT_DIRECTION: usize = 198usize;
    #[doc(hidden)]
    pub const VISIT_PSEUDO_CLASS: usize = 199usize;
    #[doc(hidden)]
    pub const VISIT_WEB_KIT_SCROLLBAR_PSEUDO_CLASS: usize = 200usize;
    #[doc(hidden)]
    pub const VISIT_PSEUDO_ELEMENT: usize = 201usize;
    #[doc(hidden)]
    pub const VISIT_WEB_KIT_SCROLLBAR_PSEUDO_ELEMENT: usize = 202usize;
    #[doc(hidden)]
    pub const VISIT_VIEW_TRANSITION_PART_NAME: usize = 203usize;
    #[doc(hidden)]
    pub const VISIT_SPAN: usize = 204usize;
    #[doc(hidden)]
    pub const VISIT_TOKEN_OR_VALUE: usize = 205usize;
    #[doc(hidden)]
    pub const VISIT_UNIT: usize = 206usize;
    #[doc(hidden)]
    pub const VISIT_TOKEN: usize = 207usize;
    #[doc(hidden)]
    pub const VISIT_SPECIFIER: usize = 208usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION_NAME: usize = 209usize;
    #[doc(hidden)]
    pub const VISIT_ENVIRONMENT_VARIABLE_NAME: usize = 210usize;
    #[doc(hidden)]
    pub const VISIT_UA_ENVIRONMENT_VARIABLE: usize = 211usize;
    #[doc(hidden)]
    pub const VISIT_ALIGN_CONTENT: usize = 212usize;
    #[doc(hidden)]
    pub const VISIT_BASELINE_POSITION: usize = 213usize;
    #[doc(hidden)]
    pub const VISIT_CONTENT_DISTRIBUTION: usize = 214usize;
    #[doc(hidden)]
    pub const VISIT_OVERFLOW_POSITION: usize = 215usize;
    #[doc(hidden)]
    pub const VISIT_CONTENT_POSITION: usize = 216usize;
    #[doc(hidden)]
    pub const VISIT_JUSTIFY_CONTENT: usize = 217usize;
    #[doc(hidden)]
    pub const VISIT_ALIGN_SELF: usize = 218usize;
    #[doc(hidden)]
    pub const VISIT_SELF_POSITION: usize = 219usize;
    #[doc(hidden)]
    pub const VISIT_JUSTIFY_SELF: usize = 220usize;
    #[doc(hidden)]
    pub const VISIT_ALIGN_ITEMS: usize = 221usize;
    #[doc(hidden)]
    pub const VISIT_JUSTIFY_ITEMS: usize = 222usize;
    #[doc(hidden)]
    pub const VISIT_LEGACY_JUSTIFY: usize = 223usize;
    #[doc(hidden)]
    pub const VISIT_GAP_VALUE: usize = 224usize;
    #[doc(hidden)]
    pub const VISIT_EASING_FUNCTION: usize = 225usize;
    #[doc(hidden)]
    pub const VISIT_STEP_POSITION: usize = 226usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION_ITERATION_COUNT: usize = 227usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION_DIRECTION: usize = 228usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION_PLAY_STATE: usize = 229usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION_FILL_MODE: usize = 230usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION_COMPOSITION: usize = 231usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION_TIMELINE: usize = 232usize;
    #[doc(hidden)]
    pub const VISIT_SCROLL_AXIS: usize = 233usize;
    #[doc(hidden)]
    pub const VISIT_SCROLLER: usize = 234usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION_ATTACHMENT_RANGE: usize = 235usize;
    #[doc(hidden)]
    pub const VISIT_TIMELINE_RANGE_NAME: usize = 236usize;
    #[doc(hidden)]
    pub const VISIT_LINE_STYLE: usize = 237usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_SIDE_WIDTH: usize = 238usize;
    #[doc(hidden)]
    pub const VISIT_LENGTH_OR_NUMBER: usize = 239usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_IMAGE_REPEAT_KEYWORD: usize = 240usize;
    #[doc(hidden)]
    pub const VISIT_BORDER_IMAGE_SIDE_WIDTH: usize = 241usize;
    #[doc(hidden)]
    pub const VISIT_OUTLINE_STYLE: usize = 242usize;
    #[doc(hidden)]
    pub const VISIT_DISPLAY: usize = 243usize;
    #[doc(hidden)]
    pub const VISIT_DISPLAY_KEYWORD: usize = 244usize;
    #[doc(hidden)]
    pub const VISIT_DISPLAY_INSIDE: usize = 245usize;
    #[doc(hidden)]
    pub const VISIT_DISPLAY_OUTSIDE: usize = 246usize;
    #[doc(hidden)]
    pub const VISIT_VISIBILITY: usize = 247usize;
    #[doc(hidden)]
    pub const VISIT_SIZE: usize = 248usize;
    #[doc(hidden)]
    pub const VISIT_MAX_SIZE: usize = 249usize;
    #[doc(hidden)]
    pub const VISIT_BOX_SIZING: usize = 250usize;
    #[doc(hidden)]
    pub const VISIT_OVERFLOW_KEYWORD: usize = 251usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_OVERFLOW: usize = 252usize;
    #[doc(hidden)]
    pub const VISIT_POSITION_PROPERTY: usize = 253usize;
    #[doc(hidden)]
    pub const VISIT_SIZE_2_D: usize = 254usize;
    #[doc(hidden)]
    pub const VISIT_RECT: usize = 255usize;
    #[doc(hidden)]
    pub const VISIT_BOX_DECORATION_BREAK: usize = 256usize;
    #[doc(hidden)]
    pub const VISIT_Z_INDEX: usize = 257usize;
    #[doc(hidden)]
    pub const VISIT_CONTAINER_TYPE: usize = 258usize;
    #[doc(hidden)]
    pub const VISIT_CONTAINER_NAME_LIST: usize = 259usize;
    #[doc(hidden)]
    pub const VISIT_FILTER_LIST: usize = 260usize;
    #[doc(hidden)]
    pub const VISIT_FILTER: usize = 261usize;
    #[doc(hidden)]
    pub const VISIT_FLEX_DIRECTION: usize = 262usize;
    #[doc(hidden)]
    pub const VISIT_FLEX_WRAP: usize = 263usize;
    #[doc(hidden)]
    pub const VISIT_BOX_ORIENT: usize = 264usize;
    #[doc(hidden)]
    pub const VISIT_BOX_DIRECTION: usize = 265usize;
    #[doc(hidden)]
    pub const VISIT_BOX_ALIGN: usize = 266usize;
    #[doc(hidden)]
    pub const VISIT_BOX_PACK: usize = 267usize;
    #[doc(hidden)]
    pub const VISIT_BOX_LINES: usize = 268usize;
    #[doc(hidden)]
    pub const VISIT_FLEX_PACK: usize = 269usize;
    #[doc(hidden)]
    pub const VISIT_FLEX_ITEM_ALIGN: usize = 270usize;
    #[doc(hidden)]
    pub const VISIT_FLEX_LINE_PACK: usize = 271usize;
    #[doc(hidden)]
    pub const VISIT_FONT_WEIGHT: usize = 272usize;
    #[doc(hidden)]
    pub const VISIT_ABSOLUTE_FONT_WEIGHT: usize = 273usize;
    #[doc(hidden)]
    pub const VISIT_FONT_SIZE: usize = 274usize;
    #[doc(hidden)]
    pub const VISIT_ABSOLUTE_FONT_SIZE: usize = 275usize;
    #[doc(hidden)]
    pub const VISIT_RELATIVE_FONT_SIZE: usize = 276usize;
    #[doc(hidden)]
    pub const VISIT_FONT_STRETCH: usize = 277usize;
    #[doc(hidden)]
    pub const VISIT_FONT_STRETCH_KEYWORD: usize = 278usize;
    #[doc(hidden)]
    pub const VISIT_FONT_FAMILY: usize = 279usize;
    #[doc(hidden)]
    pub const VISIT_GENERIC_FONT_FAMILY: usize = 280usize;
    #[doc(hidden)]
    pub const VISIT_FONT_STYLE: usize = 281usize;
    #[doc(hidden)]
    pub const VISIT_FONT_VARIANT_CAPS: usize = 282usize;
    #[doc(hidden)]
    pub const VISIT_LINE_HEIGHT: usize = 283usize;
    #[doc(hidden)]
    pub const VISIT_VERTICAL_ALIGN: usize = 284usize;
    #[doc(hidden)]
    pub const VISIT_VERTICAL_ALIGN_KEYWORD: usize = 285usize;
    #[doc(hidden)]
    pub const VISIT_TRACK_SIZING: usize = 286usize;
    #[doc(hidden)]
    pub const VISIT_TRACK_LIST_ITEM: usize = 287usize;
    #[doc(hidden)]
    pub const VISIT_TRACK_SIZE: usize = 288usize;
    #[doc(hidden)]
    pub const VISIT_TRACK_BREADTH: usize = 289usize;
    #[doc(hidden)]
    pub const VISIT_REPEAT_COUNT: usize = 290usize;
    #[doc(hidden)]
    pub const VISIT_AUTO_FLOW_DIRECTION: usize = 291usize;
    #[doc(hidden)]
    pub const VISIT_GRID_TEMPLATE_AREAS: usize = 292usize;
    #[doc(hidden)]
    pub const VISIT_GRID_LINE: usize = 293usize;
    #[doc(hidden)]
    pub const VISIT_IMAGE: usize = 294usize;
    #[doc(hidden)]
    pub const VISIT_GRADIENT: usize = 295usize;
    #[doc(hidden)]
    pub const VISIT_WEB_KIT_GRADIENT: usize = 296usize;
    #[doc(hidden)]
    pub const VISIT_LINE_DIRECTION: usize = 297usize;
    #[doc(hidden)]
    pub const VISIT_HORIZONTAL_POSITION_KEYWORD: usize = 298usize;
    #[doc(hidden)]
    pub const VISIT_VERTICAL_POSITION_KEYWORD: usize = 299usize;
    #[doc(hidden)]
    pub const VISIT_GRADIENT_ITEM: usize = 300usize;
    #[doc(hidden)]
    pub const VISIT_DIMENSION_PERCENTAGE: usize = 301usize;
    #[doc(hidden)]
    pub const VISIT_POSITION_COMPONENT: usize = 302usize;
    #[doc(hidden)]
    pub const VISIT_ENDING_SHAPE: usize = 303usize;
    #[doc(hidden)]
    pub const VISIT_ELLIPSE: usize = 304usize;
    #[doc(hidden)]
    pub const VISIT_SHAPE_EXTENT: usize = 305usize;
    #[doc(hidden)]
    pub const VISIT_CIRCLE: usize = 306usize;
    #[doc(hidden)]
    pub const VISIT_WEB_KIT_GRADIENT_POINT_COMPONENT: usize = 307usize;
    #[doc(hidden)]
    pub const VISIT_NUMBER_OR_PERCENTAGE: usize = 308usize;
    #[doc(hidden)]
    pub const VISIT_BACKGROUND_SIZE: usize = 309usize;
    #[doc(hidden)]
    pub const VISIT_LENGTH_PERCENTAGE_OR_AUTO: usize = 310usize;
    #[doc(hidden)]
    pub const VISIT_BACKGROUND_REPEAT_KEYWORD: usize = 311usize;
    #[doc(hidden)]
    pub const VISIT_BACKGROUND_ATTACHMENT: usize = 312usize;
    #[doc(hidden)]
    pub const VISIT_BACKGROUND_CLIP: usize = 313usize;
    #[doc(hidden)]
    pub const VISIT_BACKGROUND_ORIGIN: usize = 314usize;
    #[doc(hidden)]
    pub const VISIT_LIST_STYLE_TYPE: usize = 315usize;
    #[doc(hidden)]
    pub const VISIT_COUNTER_STYLE: usize = 316usize;
    #[doc(hidden)]
    pub const VISIT_SYMBOLS_TYPE: usize = 317usize;
    #[doc(hidden)]
    pub const VISIT_PREDEFINED_COUNTER_STYLE: usize = 318usize;
    #[doc(hidden)]
    pub const VISIT_SYMBOL: usize = 319usize;
    #[doc(hidden)]
    pub const VISIT_LIST_STYLE_POSITION: usize = 320usize;
    #[doc(hidden)]
    pub const VISIT_MARKER_SIDE: usize = 321usize;
    #[doc(hidden)]
    pub const VISIT_MASK_MODE: usize = 322usize;
    #[doc(hidden)]
    pub const VISIT_MASK_CLIP: usize = 323usize;
    #[doc(hidden)]
    pub const VISIT_MASK_COMPOSITE: usize = 324usize;
    #[doc(hidden)]
    pub const VISIT_MASK_TYPE: usize = 325usize;
    #[doc(hidden)]
    pub const VISIT_MASK_BORDER_MODE: usize = 326usize;
    #[doc(hidden)]
    pub const VISIT_WEB_KIT_MASK_COMPOSITE: usize = 327usize;
    #[doc(hidden)]
    pub const VISIT_WEB_KIT_MASK_SOURCE_TYPE: usize = 328usize;
    #[doc(hidden)]
    pub const VISIT_CSS_WIDE_KEYWORD: usize = 329usize;
    #[doc(hidden)]
    pub const VISIT_CUSTOM_PROPERTY_NAME: usize = 330usize;
    #[doc(hidden)]
    pub const VISIT_CLIP_PATH: usize = 331usize;
    #[doc(hidden)]
    pub const VISIT_GEOMETRY_BOX: usize = 332usize;
    #[doc(hidden)]
    pub const VISIT_BASIC_SHAPE: usize = 333usize;
    #[doc(hidden)]
    pub const VISIT_SHAPE_RADIUS: usize = 334usize;
    #[doc(hidden)]
    pub const VISIT_SVG_PAINT: usize = 335usize;
    #[doc(hidden)]
    pub const VISIT_SVG_PAINT_FALLBACK: usize = 336usize;
    #[doc(hidden)]
    pub const VISIT_FILL_RULE: usize = 337usize;
    #[doc(hidden)]
    pub const VISIT_STROKE_LINECAP: usize = 338usize;
    #[doc(hidden)]
    pub const VISIT_STROKE_LINEJOIN: usize = 339usize;
    #[doc(hidden)]
    pub const VISIT_STROKE_DASHARRAY: usize = 340usize;
    #[doc(hidden)]
    pub const VISIT_MARKER: usize = 341usize;
    #[doc(hidden)]
    pub const VISIT_COLOR_INTERPOLATION: usize = 342usize;
    #[doc(hidden)]
    pub const VISIT_COLOR_RENDERING: usize = 343usize;
    #[doc(hidden)]
    pub const VISIT_SHAPE_RENDERING: usize = 344usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_RENDERING: usize = 345usize;
    #[doc(hidden)]
    pub const VISIT_IMAGE_RENDERING: usize = 346usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_TRANSFORM_CASE: usize = 347usize;
    #[doc(hidden)]
    pub const VISIT_WHITE_SPACE: usize = 348usize;
    #[doc(hidden)]
    pub const VISIT_WORD_BREAK: usize = 349usize;
    #[doc(hidden)]
    pub const VISIT_LINE_BREAK: usize = 350usize;
    #[doc(hidden)]
    pub const VISIT_HYPHENS: usize = 351usize;
    #[doc(hidden)]
    pub const VISIT_OVERFLOW_WRAP: usize = 352usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_ALIGN: usize = 353usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_ALIGN_LAST: usize = 354usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_JUSTIFY: usize = 355usize;
    #[doc(hidden)]
    pub const VISIT_SPACING: usize = 356usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_DECORATION_LINE: usize = 357usize;
    #[doc(hidden)]
    pub const VISIT_EXCLUSIVE_TEXT_DECORATION_LINE: usize = 358usize;
    #[doc(hidden)]
    pub const VISIT_OTHER_TEXT_DECORATION_LINE: usize = 359usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_DECORATION_STYLE: usize = 360usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_DECORATION_THICKNESS: usize = 361usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_DECORATION_SKIP_INK: usize = 362usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_EMPHASIS_STYLE: usize = 363usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_EMPHASIS_FILL_MODE: usize = 364usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_EMPHASIS_SHAPE: usize = 365usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_EMPHASIS_POSITION_HORIZONTAL: usize = 366usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_EMPHASIS_POSITION_VERTICAL: usize = 367usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_SIZE_ADJUST: usize = 368usize;
    #[doc(hidden)]
    pub const VISIT_TEXT_DIRECTION: usize = 369usize;
    #[doc(hidden)]
    pub const VISIT_UNICODE_BIDI: usize = 370usize;
    #[doc(hidden)]
    pub const VISIT_TRANSFORM: usize = 371usize;
    #[doc(hidden)]
    pub const VISIT_TRANSFORM_STYLE: usize = 372usize;
    #[doc(hidden)]
    pub const VISIT_TRANSFORM_BOX: usize = 373usize;
    #[doc(hidden)]
    pub const VISIT_BACKFACE_VISIBILITY: usize = 374usize;
    #[doc(hidden)]
    pub const VISIT_PERSPECTIVE: usize = 375usize;
    #[doc(hidden)]
    pub const VISIT_TRANSLATE: usize = 376usize;
    #[doc(hidden)]
    pub const VISIT_SCALE: usize = 377usize;
    #[doc(hidden)]
    pub const VISIT_RESIZE: usize = 378usize;
    #[doc(hidden)]
    pub const VISIT_CURSOR_KEYWORD: usize = 379usize;
    #[doc(hidden)]
    pub const VISIT_COLOR_OR_AUTO: usize = 380usize;
    #[doc(hidden)]
    pub const VISIT_CARET_SHAPE: usize = 381usize;
    #[doc(hidden)]
    pub const VISIT_USER_SELECT: usize = 382usize;
    #[doc(hidden)]
    pub const VISIT_APPEARANCE: usize = 383usize;
    #[doc(hidden)]
    pub const VISIT_PRINT_COLOR_ADJUST: usize = 384usize;
    #[doc(hidden)]
    pub const VISIT_VIEW_TRANSITION_NAME: usize = 385usize;
    #[doc(hidden)]
    pub const VISIT_NONE_OR_CUSTOM_IDENT_LIST: usize = 386usize;
    #[doc(hidden)]
    pub const VISIT_VIEW_TRANSITION_GROUP: usize = 387usize;
    #[doc(hidden)]
    pub const VISIT_MEDIA_FEATURE: usize = 388usize;
    #[doc(hidden)]
    pub const VISIT_CONTAINER_SIZE_FEATURE: usize = 389usize;
    #[doc(hidden)]
    pub const VISIT_SCROLL_STATE_FEATURE: usize = 390usize;
    #[doc(hidden)]
    pub const VISIT_SELECTOR_LIST: usize = 391usize;
    #[doc(hidden)]
    pub const VISIT_SELECTOR: usize = 392usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION_RANGE_START: usize = 393usize;
    #[doc(hidden)]
    pub const VISIT_ANIMATION_RANGE_END: usize = 394usize;
    #[doc(hidden)]
    pub const VISIT_LENGTH_PERCENTAGE: usize = 395usize;
    #[doc(hidden)]
    pub const VISIT_ANGLE_PERCENTAGE: usize = 396usize;
    #[doc(hidden)]
    pub const VISIT_DECLARATION: usize = 397usize;
    #[doc(hidden)]
    pub const VISIT_PROPERTY_ID: usize = 398usize;
    #[doc(hidden)]
    pub const VISIT_VENDOR_PREFIX: usize = 399usize;
}
