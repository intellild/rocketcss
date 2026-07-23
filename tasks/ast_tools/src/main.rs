use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    Attribute, GenericArgument, GenericParam, Generics, Item, LitStr, Path as SynPath,
    PathArguments, Token, Type, Visibility, punctuated::Punctuated,
};

const AST_FILES: &[&str] = &[
    "color",
    "css_rule",
    "length",
    "media",
    "properties",
    "rules",
    "selector",
    "span",
    "token",
    "values",
];

#[derive(Clone)]
struct Node {
    ident: Ident,
    generics: Generics,
    pinned: bool,
}

#[derive(Clone)]
struct Alias {
    ident: Ident,
    generics: Generics,
    ty: Type,
}

#[derive(Clone, Copy)]
enum Mode {
    Read,
    Mut,
}

impl Mode {
    fn visitor_trait(self) -> Ident {
        format_ident!(
            "{}",
            if matches!(self, Self::Read) {
                "Visitor"
            } else {
                "VisitorMut"
            }
        )
    }

    fn node_trait(self) -> Ident {
        format_ident!(
            "{}",
            if matches!(self, Self::Read) {
                "Visit"
            } else {
                "VisitMut"
            }
        )
    }

    fn visit_method(self) -> Ident {
        format_ident!(
            "{}",
            if matches!(self, Self::Read) {
                "visit"
            } else {
                "visit_mut"
            }
        )
    }

    fn visit_children_method(self) -> Ident {
        format_ident!(
            "{}",
            if matches!(self, Self::Read) {
                "visit_children"
            } else {
                "visit_mut_children"
            }
        )
    }

    fn module_name(self) -> &'static str {
        if matches!(self, Self::Read) {
            "visit"
        } else {
            "visit_mut"
        }
    }

    fn reference(self) -> TokenStream {
        if matches!(self, Self::Read) {
            quote!(&)
        } else {
            quote!(&mut)
        }
    }

    fn iterator(self) -> Ident {
        format_ident!(
            "{}",
            if matches!(self, Self::Read) {
                "iter"
            } else {
                "iter_mut"
            }
        )
    }

    fn option_accessor(self) -> Ident {
        format_ident!(
            "{}",
            if matches!(self, Self::Read) {
                "as_ref"
            } else {
                "as_mut"
            }
        )
    }
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ast_src = root.join("crates/ast/src");
    let visitor_src = ast_src.join("generated");
    let macro_aliases = root.join("crates/macros/src/generated_visit_aliases.rs");

    let mut nodes = Vec::new();
    let mut aliases = Vec::new();
    for name in AST_FILES {
        for path in ast_module_files(&ast_src, name) {
            let source = fs::read_to_string(&path).unwrap();
            let file = syn::parse_file(&source).unwrap();
            for item in file.items {
                match item {
                    Item::Struct(item) if is_public(&item.vis) => {
                        assert_derives_visit(&item.attrs, &item.ident, &path);
                        nodes.push(Node {
                            ident: item.ident,
                            generics: item.generics,
                            pinned: has_visit_option(&item.attrs, "pinned"),
                        });
                    }
                    Item::Enum(item) if is_public(&item.vis) => {
                        assert_derives_visit(&item.attrs, &item.ident, &path);
                        nodes.push(Node {
                            ident: item.ident,
                            generics: item.generics,
                            pinned: false,
                        });
                    }
                    Item::Type(item) if is_public(&item.vis) => aliases.push(Alias {
                        ident: item.ident.clone(),
                        generics: item.generics.clone(),
                        ty: *item.ty,
                    }),
                    _ => {}
                }
            }
        }
    }

    let mut known = nodes
        .iter()
        .map(|node| node.ident.to_string())
        .collect::<HashSet<_>>();
    known.extend(aliases.iter().map(|alias| alias.ident.to_string()));
    known.extend(["Declaration", "PropertyId", "VendorPrefix"].map(str::to_owned));
    let aliases_set = aliases
        .iter()
        .map(|alias| alias.ident.to_string())
        .collect::<HashSet<_>>();

    fs::create_dir_all(&visitor_src).unwrap();
    write_rust(
        &visitor_src.join("kind.rs"),
        generate_kind(&nodes, &aliases),
    );
    for mode in [Mode::Read, Mode::Mut] {
        write_rust(
            &visitor_src.join(format!("{}.rs", mode.module_name())),
            generate_visitor(mode, &nodes, &aliases, &known, &aliases_set),
        );
    }
    write_rust(
        &visitor_src.join("mod.rs"),
        quote! {
            pub mod kind;
            pub mod visit;
            pub mod visit_mut;
        },
    );
    write_rust(&macro_aliases, generate_visit_aliases(&aliases));
}

fn ast_module_files(ast_src: &Path, name: &str) -> Vec<PathBuf> {
    let file = ast_src.join(format!("{name}.rs"));
    if file.is_file() {
        return vec![file];
    }

    let directory = ast_src.join(name);
    let mut files = fs::read_dir(&directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

fn assert_derives_visit(attributes: &[Attribute], ident: &Ident, path: &Path) {
    let derives_visit = attributes
        .iter()
        .filter(|attribute| attribute.path().is_ident("derive"))
        .filter_map(|attribute| {
            attribute
                .parse_args_with(Punctuated::<SynPath, Token![,]>::parse_terminated)
                .ok()
        })
        .flatten()
        .any(|derive| {
            derive
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "Visit")
        });
    assert!(
        derives_visit,
        "public AST node {ident} in {} must derive Visit",
        path.display()
    );
}

fn has_visit_option(attributes: &[Attribute], expected: &str) -> bool {
    attributes
        .iter()
        .filter(|attribute| attribute.path().is_ident("visit"))
        .any(|attribute| {
            let mut found = false;
            attribute
                .parse_nested_meta(|meta| {
                    found |= meta.path.is_ident(expected);
                    Ok(())
                })
                .unwrap();
            found
        })
}

fn write_rust(path: &Path, tokens: TokenStream) {
    let file = syn::parse2(tokens)
        .unwrap_or_else(|error| panic!("failed to parse generated {}: {error}", path.display()));
    fs::write(path, prettyplease::unparse(&file)).unwrap();
    let status = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(path)
        .status()
        .unwrap();
    assert!(status.success(), "rustfmt failed for {}", path.display());
}

fn generate_kind(nodes: &[Node], aliases: &[Alias]) -> TokenStream {
    let variants = nodes
        .iter()
        .map(|node| &node.ident)
        .chain(aliases.iter().map(|alias| &alias.ident));
    quote! {
        //! Generated node kinds shared by immutable and mutable visitors.

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub enum AstType {
            #(#variants,)*
            Declaration,
            PropertyId,
            VendorPrefix,
        }
    }
}

fn generate_visit_aliases(aliases: &[Alias]) -> TokenStream {
    let aliases = aliases
        .iter()
        .map(|alias| LitStr::new(&alias.ident.to_string(), alias.ident.span()));
    quote! {
        //! Generated visitor type aliases used by the AST traversal derive.

        pub(crate) const VISIT_ALIASES: &[&str] = &[#(#aliases),*];
    }
}

fn generate_visitor(
    mode: Mode,
    nodes: &[Node],
    aliases: &[Alias],
    known: &HashSet<String>,
    alias_names: &HashSet<String>,
) -> TokenStream {
    let visitor_trait = mode.visitor_trait();
    let node_trait = mode.node_trait();
    let pin_import =
        (matches!(mode, Mode::Mut) && nodes.iter().any(|node| node.pinned)).then(|| {
            quote!(
                use std::pin::Pin;
            )
        });
    let reference = mode.reference();
    let visit = mode.visit_method();
    let visit_children = mode.visit_children_method();
    let context = format_ident!(
        "{}",
        if matches!(mode, Mode::Read) {
            "VisitContext"
        } else {
            "VisitMutContext"
        }
    );
    let context_reference = if matches!(mode, Mode::Read) {
        quote!(&)
    } else {
        quote!(&mut)
    };
    let methods = nodes.iter().map(|node| {
        let method = visit_method(&node.ident);
        let (method_generics, ty, bounds) =
            signature_parts(&node.ident, &node.generics, &node_trait);
        if matches!(mode, Mode::Mut) && node.pinned {
            quote! {
                #[inline]
                fn #method #method_generics (
                    &mut self,
                    _node: Pin<&mut #ty>,
                    _cx: &mut VisitMutContext<'_, 'ghost>,
                ) #bounds {}
            }
        } else {
            quote! {
                #[inline]
                fn #method #method_generics (
                    &mut self,
                    node: #reference #ty,
                    cx: #context_reference #context<'_, 'ghost>,
                ) #bounds {
                    #node_trait::#visit_children(node, self, cx);
                }
            }
        }
    });
    let alias_methods = aliases.iter().map(|alias| {
        let method = visit_method(&alias.ident);
        let visit_children = format_ident!("{method}_children");
        let visit_children_doc = format!(
            "Continues traversal of [`{}`] without redispatching its visitor callback.",
            alias.ident
        );
        let variant = &alias.ident;
        let (method_generics, ty, bounds) =
            signature_parts(&alias.ident, &alias.generics, &node_trait);
        let generic_names = type_param_names(&alias.generics);
        let mut counter = 0;
        let body = visit_type(
            mode,
            &alias.ty,
            quote!(node),
            known,
            alias_names,
            &generic_names,
            &mut counter,
        );
        quote! {
            #[inline]
            fn #method #method_generics (
                &mut self,
                node: #reference #ty,
                cx: #context_reference #context<'_, 'ghost>,
            ) #bounds {
                self.#visit_children(node, cx);
            }

            #[doc = #visit_children_doc]
            fn #visit_children #method_generics (
                &mut self,
                node: #reference #ty,
                cx: #context_reference #context<'_, 'ghost>,
            ) #bounds {
                let visitor = self;
                visitor.enter_node(AstType::#variant);
                #body
                visitor.leave_node(AstType::#variant);
            }
        }
    });

    let str_method = if matches!(mode, Mode::Read) {
        quote! {
            fn visit_str(
                &mut self,
                _value: &&'a str,
                _cx: &VisitContext<'_, 'ghost>,
            ) {}
        }
    } else {
        quote! {
            fn visit_str(
                &mut self,
                _value: &mut &'a str,
                _cx: &mut VisitMutContext<'_, 'ghost>,
            ) {}
        }
    };
    let manual_methods = manual_methods(mode);
    let manual_impls = manual_impls(mode);
    let container_impls = container_impls(mode);

    quote! {
        //! Generated typed visitor API. Regenerate with `cargo run -p rocketcss_ast_tools`.

        #![allow(clippy::match_same_arms, clippy::needless_borrow, unused_imports, unused_variables)]

        use crate::*;
        #pin_import

        /// Typed callbacks invoked while traversing CSS AST nodes.
        pub trait #visitor_trait<'a, 'ghost> {
            #[inline]
            fn enter_node(&mut self, _kind: AstType) {}

            #[inline]
            fn leave_node(&mut self, _kind: AstType) {}

            #[inline]
            #str_method

            #(#methods)*
            #(#alias_methods)*
            #manual_methods
        }

        /// Traversal implemented by CSS AST nodes.
        pub trait #node_trait<'a, 'ghost> {
            /// Dispatches this node to its typed visitor callback.
            fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                #reference self,
                visitor: &mut VisitorT,
                cx: #context_reference #context<'_, 'ghost>,
            );

            /// Continues traversal without redispatching this node's visitor callback.
            #[inline]
            fn #visit_children<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                #reference self,
                _visitor: &mut VisitorT,
                _cx: #context_reference #context<'_, 'ghost>,
            ) {}
        }

        #container_impls
        #manual_impls
    }
}

fn manual_methods(mode: Mode) -> TokenStream {
    let reference = mode.reference();
    let node_trait = mode.node_trait();
    let visit_children = mode.visit_children_method();
    let context = format_ident!(
        "{}",
        if matches!(mode, Mode::Read) {
            "VisitContext"
        } else {
            "VisitMutContext"
        }
    );
    let context_reference = if matches!(mode, Mode::Read) {
        quote!(&)
    } else {
        quote!(&mut)
    };
    quote! {
        #[inline]
        fn visit_declaration(
            &mut self,
            node: #reference Declaration<'a>,
            cx: #context_reference #context<'_, 'ghost>,
        ) {
            #node_trait::#visit_children(node, self, cx);
        }

        #[inline]
        fn visit_property_id(
            &mut self,
            node: #reference PropertyId<'a>,
            cx: #context_reference #context<'_, 'ghost>,
        ) {
            #node_trait::#visit_children(node, self, cx);
        }

        #[inline]
        fn visit_vendor_prefix(
            &mut self,
            node: #reference VendorPrefix,
            cx: #context_reference #context<'_, 'ghost>,
        ) {
            #node_trait::#visit_children(node, self, cx);
        }
    }
}

fn manual_impls(mode: Mode) -> TokenStream {
    let visitor_trait = mode.visitor_trait();
    let node_trait = mode.node_trait();
    let visit = mode.visit_method();
    let visit_children = mode.visit_children_method();
    let reference = mode.reference();
    let context = format_ident!(
        "{}",
        if matches!(mode, Mode::Read) {
            "VisitContext"
        } else {
            "VisitMutContext"
        }
    );
    let context_reference = if matches!(mode, Mode::Read) {
        quote!(&)
    } else {
        quote!(&mut)
    };
    quote! {
        impl<'a, 'ghost> #node_trait<'a, 'ghost> for VendorPrefix {
            fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                #reference self,
                visitor: &mut VisitorT,
                cx: #context_reference #context<'_, 'ghost>,
            ) {
                visitor.visit_vendor_prefix(self, cx);
            }

            fn #visit_children<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                #reference self,
                visitor: &mut VisitorT,
                _cx: #context_reference #context<'_, 'ghost>,
            ) {
                visitor.enter_node(AstType::VendorPrefix);
                visitor.leave_node(AstType::VendorPrefix);
            }
        }
    }
}

fn container_impls(mode: Mode) -> TokenStream {
    let visitor_trait = mode.visitor_trait();
    let node_trait = mode.node_trait();
    let visit = mode.visit_method();
    if matches!(mode, Mode::Read) {
        quote! {
            macro_rules! impl_leaf_visit {
                ($($ty:ty),+ $(,)?) => {$(
                    impl<'a, 'ghost> #node_trait<'a, 'ghost> for $ty {
                        fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                            &self,
                            _visitor: &mut VisitorT,
                            _cx: &VisitContext<'_, 'ghost>,
                        ) {}
                    }
                )+};
            }
            impl_leaf_visit!(bool, char, f32, i32, u8, u16, u32, usize);

            impl<'a, 'ghost, T: ?Sized + #node_trait<'a, 'ghost>>
                #node_trait<'a, 'ghost>
                for rocketcss_allocator::boxed::Box<'a, T>
            {
                fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                    &self,
                    visitor: &mut VisitorT,
                    cx: &VisitContext<'_, 'ghost>,
                ) {
                    #node_trait::#visit(self.as_ref(), visitor, cx);
                }
            }
            impl<'a, 'ghost, T: #node_trait<'a, 'ghost> + Unpin>
                #node_trait<'a, 'ghost>
                for rocketcss_allocator::vec::Vec<'a, T>
            {
                fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                    &self,
                    visitor: &mut VisitorT,
                    cx: &VisitContext<'_, 'ghost>,
                ) {
                    for value in self {
                        #node_trait::#visit(value, visitor, cx);
                    }
                }
            }
            impl<'a, 'ghost, T: #node_trait<'a, 'ghost>>
                #node_trait<'a, 'ghost> for Option<T>
            {
                fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                    &self,
                    visitor: &mut VisitorT,
                    cx: &VisitContext<'_, 'ghost>,
                ) {
                    if let Some(value) = self {
                        #node_trait::#visit(value, visitor, cx);
                    }
                }
            }
            impl<'a, 'ghost> #node_trait<'a, 'ghost> for &'a str {
                fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                    &self,
                    visitor: &mut VisitorT,
                    cx: &VisitContext<'_, 'ghost>,
                ) {
                    visitor.visit_str(self, cx);
                }
            }
        }
    } else {
        quote! {
            macro_rules! impl_leaf_visit_mut {
                ($($ty:ty),+ $(,)?) => {$(
                    impl<'a, 'ghost> #node_trait<'a, 'ghost> for $ty {
                        fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                            &mut self,
                            _visitor: &mut VisitorT,
                            _cx: &mut VisitMutContext<'_, 'ghost>,
                        ) {}
                    }
                )+};
            }
            impl_leaf_visit_mut!(bool, char, f32, i32, u8, u16, u32, usize);

            impl<'a, 'ghost, T: ?Sized + #node_trait<'a, 'ghost>>
                #node_trait<'a, 'ghost>
                for rocketcss_allocator::boxed::Box<'a, T>
            {
                fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                    &mut self,
                    visitor: &mut VisitorT,
                    cx: &mut VisitMutContext<'_, 'ghost>,
                ) {
                    #node_trait::#visit(self.as_mut(), visitor, cx);
                }
            }
            impl<'a, 'ghost, T: #node_trait<'a, 'ghost> + Unpin>
                #node_trait<'a, 'ghost>
                for rocketcss_allocator::vec::Vec<'a, T>
            {
                fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                    &mut self,
                    visitor: &mut VisitorT,
                    cx: &mut VisitMutContext<'_, 'ghost>,
                ) {
                    for value in self {
                        #node_trait::#visit(value, visitor, cx);
                    }
                }
            }
            impl<'a, 'ghost, T: #node_trait<'a, 'ghost>>
                #node_trait<'a, 'ghost> for Option<T>
            {
                fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                    &mut self,
                    visitor: &mut VisitorT,
                    cx: &mut VisitMutContext<'_, 'ghost>,
                ) {
                    if let Some(value) = self {
                        #node_trait::#visit(value, visitor, cx);
                    }
                }
            }
            impl<'a, 'ghost> #node_trait<'a, 'ghost> for &'a str {
                fn #visit<VisitorT: ?Sized + #visitor_trait<'a, 'ghost>>(
                    &mut self,
                    visitor: &mut VisitorT,
                    cx: &mut VisitMutContext<'_, 'ghost>,
                ) {
                    visitor.visit_str(self, cx);
                }
            }
        }
    }
}

fn visit_type(
    mode: Mode,
    ty: &Type,
    expression: TokenStream,
    known: &HashSet<String>,
    aliases: &HashSet<String>,
    generics: &HashSet<String>,
    counter: &mut usize,
) -> TokenStream {
    match ty {
        Type::Reference(reference) if matches!(&*reference.elem, Type::Path(path) if path.path.is_ident("str")) =>
        {
            quote!(visitor.visit_str(#expression, cx);)
        }
        Type::Reference(reference) => visit_type(
            mode,
            &reference.elem,
            expression,
            known,
            aliases,
            generics,
            counter,
        ),
        Type::Tuple(tuple) => {
            let reference = mode.reference();
            let visits = tuple.elems.iter().enumerate().map(|(index, ty)| {
                let index = syn::Index::from(index);
                visit_type(
                    mode,
                    ty,
                    quote!(#reference (#expression).#index),
                    known,
                    aliases,
                    generics,
                    counter,
                )
            });
            quote!(#(#visits)*)
        }
        Type::Array(array) => {
            let binding = fresh_binding(counter);
            let iterator = mode.iterator();
            let inner = visit_type(
                mode,
                &array.elem,
                quote!(#binding),
                known,
                aliases,
                generics,
                counter,
            );
            quote!(for #binding in (#expression).#iterator() { #inner })
        }
        Type::Path(path) if path.qself.is_none() => {
            let Some(segment) = path.path.segments.last() else {
                return quote!();
            };
            let name = segment.ident.to_string();
            if name == "Pin" {
                let Some(pin_target) = first_type_argument(&segment.arguments) else {
                    return quote!();
                };
                if let Type::Reference(reference) = pin_target {
                    return visit_type(
                        mode,
                        &reference.elem,
                        quote!((#expression).as_ref()),
                        known,
                        aliases,
                        generics,
                        counter,
                    );
                }
                let Type::Path(box_path) = pin_target else {
                    return quote!();
                };
                let Some(box_segment) = box_path.path.segments.last() else {
                    return quote!();
                };
                let Some(inner_ty) = first_type_argument(&box_segment.arguments) else {
                    return quote!();
                };
                if matches!(mode, Mode::Read) {
                    visit_type(
                        mode,
                        inner_ty,
                        quote!((#expression).as_ref().get_ref()),
                        known,
                        aliases,
                        generics,
                        counter,
                    )
                } else {
                    let binding = fresh_binding(counter);
                    let inner = visit_type(
                        mode,
                        inner_ty,
                        quote!(&mut #binding),
                        known,
                        aliases,
                        generics,
                        counter,
                    );
                    quote! {
                        let mut #binding = (#expression).as_mut();
                        #inner
                    }
                }
            } else if matches!(name.as_str(), "Box" | "Option") {
                let Some(inner_ty) = first_type_argument(&segment.arguments) else {
                    return quote!();
                };
                let binding = fresh_binding(counter);
                let accessor = mode.option_accessor();
                let inner = visit_type(
                    mode,
                    inner_ty,
                    quote!(#binding),
                    known,
                    aliases,
                    generics,
                    counter,
                );
                if name == "Option" {
                    quote!(if let Some(#binding) = (#expression).#accessor() { #inner })
                } else {
                    let inner_expression = quote!((#expression).#accessor());
                    visit_type(
                        mode,
                        inner_ty,
                        inner_expression,
                        known,
                        aliases,
                        generics,
                        counter,
                    )
                }
            } else if name == "Vec" {
                let Some(inner_ty) = first_type_argument(&segment.arguments) else {
                    return quote!();
                };
                let binding = fresh_binding(counter);
                let iterator = mode.iterator();
                let inner = visit_type(
                    mode,
                    inner_ty,
                    quote!(#binding),
                    known,
                    aliases,
                    generics,
                    counter,
                );
                quote!(for #binding in (#expression).#iterator() { #inner })
            } else if name == "GhostCell" {
                if matches!(mode, Mode::Read) {
                    quote! {
                        cx.with_cell(#expression, |value, cx| {
                            Visit::visit(value, visitor, cx);
                        });
                    }
                } else {
                    quote! {
                        cx.with_cell(#expression, |value, cx| {
                            VisitMut::visit(value, visitor, cx);
                        });
                    }
                }
            } else if name == "Ref" {
                if matches!(mode, Mode::Read) {
                    quote! {
                        cx.with_ref(*#expression, |value, cx| {
                            Visit::visit(value.get_ref(), visitor, cx);
                        });
                    }
                } else {
                    quote! {
                        cx.visit_ref(*#expression, visitor);
                    }
                }
            } else if generics.contains(&name) {
                let node_trait = mode.node_trait();
                let visit = mode.visit_method();
                quote!(#node_trait::#visit(#expression, visitor, cx);)
            } else if aliases.contains(&name) {
                let method = visit_method(&segment.ident);
                quote!(visitor.#method(#expression, cx);)
            } else if known.contains(&name) {
                let node_trait = mode.node_trait();
                let visit = mode.visit_method();
                quote!(#node_trait::#visit(#expression, visitor, cx);)
            } else {
                quote!()
            }
        }
        _ => quote!(),
    }
}

fn first_type_argument(arguments: &PathArguments) -> Option<&Type> {
    let PathArguments::AngleBracketed(arguments) = arguments else {
        return None;
    };
    arguments.args.iter().find_map(|argument| match argument {
        GenericArgument::Type(ty) => Some(ty),
        _ => None,
    })
}

fn fresh_binding(counter: &mut usize) -> Ident {
    let binding = format_ident!("value_{}", *counter);
    *counter += 1;
    binding
}

fn visit_method(ident: &Ident) -> Ident {
    format_ident!("visit_{}", ident.to_string().to_case(Case::Snake))
}

fn type_param_names(generics: &Generics) -> HashSet<String> {
    generics
        .type_params()
        .map(|param| param.ident.to_string())
        .collect()
}

fn type_tokens(ident: &Ident, generics: &Generics) -> TokenStream {
    let arguments = generics.params.iter().map(|param| match param {
        GenericParam::Lifetime(param) => {
            let lifetime = &param.lifetime;
            if lifetime.ident == "ghost" {
                quote!('ghost)
            } else {
                quote!('a)
            }
        }
        GenericParam::Type(param) => {
            let ident = &param.ident;
            quote!(#ident)
        }
        GenericParam::Const(param) => {
            let ident = &param.ident;
            quote!(#ident)
        }
    });
    if generics.params.is_empty() {
        quote!(#ident)
    } else {
        quote!(#ident<#(#arguments),*>)
    }
}

fn non_lifetime_params(generics: &Generics) -> Vec<GenericParam> {
    generics
        .params
        .iter()
        .filter(|param| !matches!(param, GenericParam::Lifetime(_)))
        .cloned()
        .collect()
}

fn signature_parts(
    ident: &Ident,
    generics: &Generics,
    node_trait: &Ident,
) -> (TokenStream, TokenStream, TokenStream) {
    let params = non_lifetime_params(generics);
    let method_generics = if params.is_empty() {
        quote!()
    } else {
        quote!(<#(#params),*>)
    };
    let ty = type_tokens(ident, generics);
    let names = generics
        .type_params()
        .map(|param| &param.ident)
        .collect::<Vec<_>>();
    let bounds = if names.is_empty() {
        quote!()
    } else {
        quote!(where #(#names: #node_trait<'a, 'ghost>),*)
    };
    (method_generics, ty, bounds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_single_file_and_directory_ast_modules() {
        let ast_src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../crates/ast/src");

        assert_eq!(
            ast_module_files(&ast_src, "color"),
            vec![ast_src.join("color.rs")]
        );

        for (module, representative) in [("rules", "stylesheet.rs"), ("values", "image.rs")] {
            let files = ast_module_files(&ast_src, module);
            assert!(files.len() > 1);
            assert!(
                files
                    .iter()
                    .any(|path| path.ends_with(format!("{module}/mod.rs")))
            );
            assert!(files.iter().any(|path| path.ends_with(representative)));
            for path in files {
                let source = fs::read_to_string(path).unwrap();
                syn::parse_file(&source).unwrap();
            }
        }
    }
}
