use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Cursor<'a> {
    pub images: Vec<'a, NodeId<'a, CursorImage<'a>>>,
    pub keyword: CursorKeyword,
}

impl_inline_node!(Cursor<'ast>, 0x00210001);

impl<'ast> AstNodeClone<'ast> for Cursor<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            images: context.clone_encoded_vec(self.images),
            keyword: self.keyword,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct CursorImage<'a> {
    pub hotspot: Option<(f32, f32)>,
    pub url: NodeId<'a, Url<'a>>,
}

impl_inline_node!(CursorImage<'ast>, 0x00210002);

impl<'ast> AstNodeClone<'ast> for CursorImage<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            hotspot: self.hotspot,
            url: context.clone_encoded_node(self.url),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Caret<'a> {
    pub color: NodeId<'a, ColorOrAuto<'a>>,
    pub shape: CaretShape,
}

impl_inline_node!(Caret<'ast>, 0x00210003);

impl<'ast> AstNodeClone<'ast> for Caret<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            color: context.clone_encoded_node(self.color),
            shape: self.shape,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct ListStyle<'a> {
    pub image: NodeId<'a, Image<'a>>,
    pub list_style_type: NodeId<'a, ListStyleType<'a>>,
    pub position: ListStylePosition,
}

impl_inline_node!(ListStyle<'ast>, 0x00210004);

impl<'ast> AstNodeClone<'ast> for ListStyle<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            image: context.clone_encoded_node(self.image),
            list_style_type: context.clone_encoded_node(self.list_style_type),
            position: self.position,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Composes<'a> {
    pub from: Option<NodeId<'a, Specifier<'a>>>,
    pub names: Vec<'a, AstStr<'a>>,
}

impl_inline_node!(Composes<'ast>, 0x0021_0005);

impl<'ast> AstNodeClone<'ast> for Composes<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            from: self.from.map(|value| context.clone_encoded_node(value)),
            names: context.clone_encoded_vec(self.names),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct ColorScheme {
    pub dark: bool,
    pub light: bool,
    pub only: bool,
}
#[cfg(test)]
mod native_tests {
    use super::*;

    #[test]
    fn cursor_hotspot_preserves_optional_float_bits_and_clones_url() {
        let allocator = rocketcss_common::Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let text = ast.add_str("cursor.svg");
        let url = ast.alloc_node(Url { url: text }, DUMMY_SP);
        let image = ast.alloc_node(CursorImage { hotspot: None, url }, DUMMY_SP);
        let images = ast.alloc_encoded_vec([image].into_iter());
        let cursor = ast.alloc_node(
            Cursor {
                images,
                keyword: CursorKeyword::Pointer,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        for bits in [0, 0x8000_0000, 0x7f80_0000, 0xff80_0000, 0x7fc0_1234] {
            ast.mutate_node(image, |value, _| {
                value.hotspot = Some((f32::from_bits(bits), -2.5))
            });
            let actual = ast.resolve_node(image);
            let (x, y) = actual.hotspot.unwrap();
            assert_eq!(x.to_bits(), bits);
            assert_eq!(y, -2.5);
            assert_eq!(actual.url, url);
            ast.mutate_node(image, |value, _| value.hotspot = None);
            assert_eq!(ast.resolve_node(image).hotspot, None);
        }
        for keyword in [
            CursorKeyword::Auto,
            CursorKeyword::None,
            CursorKeyword::Pointer,
            CursorKeyword::NeswResize,
            CursorKeyword::ZoomOut,
        ] {
            ast.mutate_node(cursor, |value, _| value.keyword = keyword);
            assert_eq!(ast.resolve_node(cursor), Cursor { images, keyword });
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        let cloned = ast.clone_node(cursor);
        let cloned_images = ast.resolve_node(cloned).images;
        let cloned_image = ast.encoded_vec_get(cloned_images, 0).unwrap();
        let cloned_url = ast.resolve_node(cloned_image).url;
        assert_ne!(cloned_url, url);
        let replacement = ast.add_str("replacement.svg");
        ast.mutate_node(cloned_url, |value, _| value.url = replacement);
        assert_eq!(ast.str(ast.resolve_node(url).url), "cursor.svg");
    }
}
