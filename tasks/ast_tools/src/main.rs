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
    Fields, GenericArgument, GenericParam, Generics, Item, ItemEnum, ItemStruct, LitStr,
    PathArguments, Token, Type, Visibility, parenthesized, parse::Parse,
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
enum NodeData {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

#[derive(Clone)]
struct Node {
    module: Ident,
    ident: Ident,
    generics: Generics,
    data: NodeData,
}

#[derive(Clone)]
struct Alias {
    module: Ident,
    ident: Ident,
    generics: Generics,
    ty: Type,
}

struct Property {
    ident: Ident,
    vendor_prefix: Option<Type>,
}

struct PropertyList(Vec<Property>);

impl Parse for PropertyList {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut properties = Vec::new();
        while !input.is_empty() {
            let _: LitStr = input.parse()?;
            let _: Token![:] = input.parse()?;
            let ident = input.parse()?;
            let content;
            parenthesized!(content in input);
            let _: Type = content.parse()?;
            let vendor_prefix = if content.is_empty() {
                None
            } else {
                let _: Token![,] = content.parse()?;
                Some(content.parse()?)
            };
            let _: Token![,] = input.parse()?;
            properties.push(Property {
                ident,
                vendor_prefix,
            });
        }
        Ok(Self(properties))
    }
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
                "Visit"
            } else {
                "VisitMut"
            }
        )
    }

    fn node_trait(self) -> Ident {
        format_ident!(
            "{}",
            if matches!(self, Self::Read) {
                "VisitNode"
            } else {
                "VisitMutNode"
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
    let visitor_src = root.join("crates/visitor/src/generated");

    let mut nodes = Vec::new();
    let mut aliases = Vec::new();
    let mut properties = None;
    for name in AST_FILES {
        let module = format_ident!("{name}");
        let source = fs::read_to_string(ast_src.join(format!("{name}.rs"))).unwrap();
        let file = syn::parse_file(&source).unwrap();
        for item in file.items {
            match item {
                Item::Struct(item) if is_public(&item.vis) => nodes.push(Node {
                    module: module.clone(),
                    ident: item.ident.clone(),
                    generics: item.generics.clone(),
                    data: NodeData::Struct(item),
                }),
                Item::Enum(item) if is_public(&item.vis) => nodes.push(Node {
                    module: module.clone(),
                    ident: item.ident.clone(),
                    generics: item.generics.clone(),
                    data: NodeData::Enum(item),
                }),
                Item::Type(item) if is_public(&item.vis) => aliases.push(Alias {
                    module: module.clone(),
                    ident: item.ident.clone(),
                    generics: item.generics.clone(),
                    ty: *item.ty,
                }),
                Item::Macro(item)
                    if item
                        .ident
                        .as_ref()
                        .is_some_and(|ident| ident == "for_each_property") =>
                {
                    properties = find_property_list(item.mac.tokens);
                }
                _ => {}
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
    let properties = properties.expect("could not find for_each_property! property definitions");

    fs::create_dir_all(&visitor_src).unwrap();
    write_rust(
        &visitor_src.join("kind.rs"),
        generate_kind(&nodes, &aliases),
    );
    for mode in [Mode::Read, Mode::Mut] {
        let mode_dir = visitor_src.join(mode.module_name());
        fs::create_dir_all(&mode_dir).unwrap();
        for module in AST_FILES {
            let module_ident = format_ident!("{module}");
            let module_nodes = nodes
                .iter()
                .filter(|node| node.module == module_ident)
                .cloned()
                .collect::<Vec<_>>();
            let module_aliases = aliases
                .iter()
                .filter(|alias| alias.module == module_ident)
                .cloned()
                .collect::<Vec<_>>();
            write_rust(
                &mode_dir.join(format!("{module}.rs")),
                generate_walk_module(mode, &module_nodes, &module_aliases, &known, &aliases_set),
            );
        }
        write_rust(
            &visitor_src.join(format!("{}.rs", mode.module_name())),
            generate_visitor(mode, &nodes, &aliases, &properties),
        );
    }
}

fn find_property_list(tokens: TokenStream) -> Option<Vec<Property>> {
    for token in tokens {
        if let proc_macro2::TokenTree::Group(group) = token {
            if let Ok(properties) = syn::parse2::<PropertyList>(group.stream())
                && properties.0.len() > 100
            {
                return Some(properties.0);
            }
            if let Some(properties) = find_property_list(group.stream()) {
                return Some(properties);
            }
        }
    }
    None
}

fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
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

fn generate_visitor(
    mode: Mode,
    nodes: &[Node],
    aliases: &[Alias],
    properties: &[Property],
) -> TokenStream {
    let visitor_trait = mode.visitor_trait();
    let node_trait = mode.node_trait();
    let reference = mode.reference();
    let modules = AST_FILES
        .iter()
        .map(|name| format_ident!("{name}"))
        .collect::<Vec<_>>();

    let methods = nodes.iter().map(|node| {
        let method = visit_method(&node.ident);
        let module = &node.module;
        let walk = walk_method(&node.ident);
        let (method_generics, ty, bounds) =
            signature_parts(&node.ident, &node.generics, &node_trait, quote!(Self));
        quote! {
            #[inline]
            fn #method #method_generics (&mut self, node: #reference #ty) #bounds {
                #module::#walk(self, node);
            }
        }
    });
    let alias_methods = aliases.iter().map(|alias| {
        let method = visit_method(&alias.ident);
        let module = &alias.module;
        let walk = walk_method(&alias.ident);
        let (method_generics, ty, bounds) =
            signature_parts(&alias.ident, &alias.generics, &node_trait, quote!(Self));
        quote! {
            #[inline]
            fn #method #method_generics (&mut self, node: #reference #ty) #bounds {
                #module::#walk(self, node);
            }
        }
    });
    let node_impls = nodes.iter().map(|node| {
        let method = visit_method(&node.ident);
        let (impl_generics, ty, bounds) =
            impl_parts(&node.ident, &node.generics, &visitor_trait, &node_trait);
        if matches!(mode, Mode::Read) {
            quote! {
                impl #impl_generics #node_trait<'a, VisitorT> for #ty #bounds {
                    #[inline]
                    fn visit_node(&self, visitor: &mut VisitorT) {
                        visitor.#method(self);
                    }
                }
            }
        } else {
            quote! {
                impl #impl_generics #node_trait<'a, VisitorT> for #ty #bounds {
                    #[inline]
                    fn visit_node(&mut self, visitor: &mut VisitorT) {
                        visitor.#method(self);
                    }
                }
            }
        }
    });

    let str_method = if matches!(mode, Mode::Read) {
        quote! { fn visit_str(&mut self, _value: &&'a str) {} }
    } else {
        quote! { fn visit_str(&mut self, _value: &mut &'a str) {} }
    };
    let manual_methods = manual_methods(mode);
    let manual_impls = manual_impls(mode);
    let container_impls = container_impls(mode);
    let manual_walkers = manual_walkers(mode, properties);
    let walk_exports = modules.iter();

    quote! {
        //! Generated typed visitor API. Regenerate with `cargo run -p rocketcss_ast_tools`.

        #![allow(clippy::match_same_arms, clippy::needless_borrow, unused_imports, unused_variables)]

        use rocketcss_ast::*;
        use crate::AstType;

        #(pub mod #modules;)*

        pub trait #visitor_trait<'a> {
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

        #[doc(hidden)]
        pub trait #node_trait<'a, VisitorT: ?Sized + #visitor_trait<'a>> {
            fn visit_node(#reference self, visitor: &mut VisitorT);
        }

        #container_impls
        #(#node_impls)*
        #manual_impls
        #manual_walkers

        pub mod walk {
            #(pub use super::#walk_exports::*;)*
            pub use super::{walk_declaration, walk_property_id, walk_vendor_prefix};
        }
    }
}

fn manual_methods(mode: Mode) -> TokenStream {
    let reference = mode.reference();
    quote! {
        #[inline]
        fn visit_declaration(&mut self, node: #reference Declaration<'a>) {
            walk_declaration(self, node);
        }

        #[inline]
        fn visit_property_id(&mut self, node: #reference PropertyId<'a>) {
            walk_property_id(self, node);
        }

        #[inline]
        fn visit_vendor_prefix(&mut self, node: #reference VendorPrefix) {
            walk_vendor_prefix(self, node);
        }
    }
}

fn manual_impls(mode: Mode) -> TokenStream {
    let visitor_trait = mode.visitor_trait();
    let node_trait = mode.node_trait();
    if matches!(mode, Mode::Read) {
        quote! {
            impl<'a, VisitorT: ?Sized + #visitor_trait<'a>> #node_trait<'a, VisitorT> for Declaration<'a> {
                fn visit_node(&self, visitor: &mut VisitorT) { visitor.visit_declaration(self); }
            }
            impl<'a, VisitorT: ?Sized + #visitor_trait<'a>> #node_trait<'a, VisitorT> for PropertyId<'a> {
                fn visit_node(&self, visitor: &mut VisitorT) { visitor.visit_property_id(self); }
            }
            impl<'a, VisitorT: ?Sized + #visitor_trait<'a>> #node_trait<'a, VisitorT> for VendorPrefix {
                fn visit_node(&self, visitor: &mut VisitorT) { visitor.visit_vendor_prefix(self); }
            }
        }
    } else {
        quote! {
            impl<'a, VisitorT: ?Sized + #visitor_trait<'a>> #node_trait<'a, VisitorT> for Declaration<'a> {
                fn visit_node(&mut self, visitor: &mut VisitorT) { visitor.visit_declaration(self); }
            }
            impl<'a, VisitorT: ?Sized + #visitor_trait<'a>> #node_trait<'a, VisitorT> for PropertyId<'a> {
                fn visit_node(&mut self, visitor: &mut VisitorT) { visitor.visit_property_id(self); }
            }
            impl<'a, VisitorT: ?Sized + #visitor_trait<'a>> #node_trait<'a, VisitorT> for VendorPrefix {
                fn visit_node(&mut self, visitor: &mut VisitorT) { visitor.visit_vendor_prefix(self); }
            }
        }
    }
}

fn container_impls(mode: Mode) -> TokenStream {
    let visitor_trait = mode.visitor_trait();
    let node_trait = mode.node_trait();
    if matches!(mode, Mode::Read) {
        quote! {
            macro_rules! impl_leaf_visit_node {
                ($($ty:ty),+ $(,)?) => {$(
                    impl<'a, VisitorT: ?Sized + #visitor_trait<'a>> #node_trait<'a, VisitorT> for $ty {
                        fn visit_node(&self, _visitor: &mut VisitorT) {}
                    }
                )+};
            }
            impl_leaf_visit_node!(bool, char, f32, i32, u8, u16, u32, usize);

            impl<'a, VisitorT, T> #node_trait<'a, VisitorT> for rocketcss_allocator::boxed::Box<'a, T>
            where VisitorT: ?Sized + #visitor_trait<'a>, T: ?Sized + #node_trait<'a, VisitorT> {
                fn visit_node(&self, visitor: &mut VisitorT) { self.as_ref().visit_node(visitor); }
            }
            impl<'a, VisitorT, T> #node_trait<'a, VisitorT> for rocketcss_allocator::vec::Vec<'a, T>
            where VisitorT: ?Sized + #visitor_trait<'a>, T: #node_trait<'a, VisitorT> {
                fn visit_node(&self, visitor: &mut VisitorT) { for value in self { value.visit_node(visitor); } }
            }
            impl<'a, VisitorT, T> #node_trait<'a, VisitorT> for Option<T>
            where VisitorT: ?Sized + #visitor_trait<'a>, T: #node_trait<'a, VisitorT> {
                fn visit_node(&self, visitor: &mut VisitorT) { if let Some(value) = self { value.visit_node(visitor); } }
            }
            impl<'a, VisitorT: ?Sized + #visitor_trait<'a>> #node_trait<'a, VisitorT> for &'a str {
                fn visit_node(&self, visitor: &mut VisitorT) { visitor.visit_str(self); }
            }
        }
    } else {
        quote! {
            macro_rules! impl_leaf_visit_mut_node {
                ($($ty:ty),+ $(,)?) => {$(
                    impl<'a, VisitorT: ?Sized + #visitor_trait<'a>> #node_trait<'a, VisitorT> for $ty {
                        fn visit_node(&mut self, _visitor: &mut VisitorT) {}
                    }
                )+};
            }
            impl_leaf_visit_mut_node!(bool, char, f32, i32, u8, u16, u32, usize);

            impl<'a, VisitorT, T> #node_trait<'a, VisitorT> for rocketcss_allocator::boxed::Box<'a, T>
            where VisitorT: ?Sized + #visitor_trait<'a>, T: ?Sized + #node_trait<'a, VisitorT> {
                fn visit_node(&mut self, visitor: &mut VisitorT) { self.as_mut().visit_node(visitor); }
            }
            impl<'a, VisitorT, T> #node_trait<'a, VisitorT> for rocketcss_allocator::vec::Vec<'a, T>
            where VisitorT: ?Sized + #visitor_trait<'a>, T: #node_trait<'a, VisitorT> {
                fn visit_node(&mut self, visitor: &mut VisitorT) { for value in self { value.visit_node(visitor); } }
            }
            impl<'a, VisitorT, T> #node_trait<'a, VisitorT> for Option<T>
            where VisitorT: ?Sized + #visitor_trait<'a>, T: #node_trait<'a, VisitorT> {
                fn visit_node(&mut self, visitor: &mut VisitorT) { if let Some(value) = self { value.visit_node(visitor); } }
            }
            impl<'a, VisitorT: ?Sized + #visitor_trait<'a>> #node_trait<'a, VisitorT> for &'a str {
                fn visit_node(&mut self, visitor: &mut VisitorT) { visitor.visit_str(self); }
            }
        }
    }
}

fn manual_walkers(mode: Mode, properties: &[Property]) -> TokenStream {
    let visitor_trait = mode.visitor_trait();
    let node_trait = mode.node_trait();
    let reference = mode.reference();
    let declaration_arms = properties.iter().map(|property| {
        let ident = &property.ident;
        if property.vendor_prefix.is_some() {
            quote!(
                Declaration::#ident(value, vendor_prefix) => {
                    #node_trait::visit_node(value, visitor);
                    #node_trait::visit_node(vendor_prefix, visitor);
                }
            )
        } else {
            quote!(Declaration::#ident(value) => #node_trait::visit_node(value, visitor),)
        }
    });
    let property_id_arms = properties.iter().map(|property| {
        let ident = &property.ident;
        if property.vendor_prefix.is_some() {
            quote!(PropertyId::#ident(value) => #node_trait::visit_node(value, visitor),)
        } else {
            quote!(PropertyId::#ident => {})
        }
    });

    quote! {
        pub fn walk_declaration<'a, VisitorT: ?Sized + #visitor_trait<'a>>(
            visitor: &mut VisitorT,
            node: #reference Declaration<'a>,
        ) {
            visitor.enter_node(AstType::Declaration);
            match node {
                #(#declaration_arms)*
                Declaration::All(value) => #node_trait::visit_node(value, visitor),
                Declaration::Unparsed(value) => #node_trait::visit_node(value, visitor),
                Declaration::Custom(value) => #node_trait::visit_node(value, visitor),
            }
            visitor.leave_node(AstType::Declaration);
        }

        pub fn walk_property_id<'a, VisitorT: ?Sized + #visitor_trait<'a>>(
            visitor: &mut VisitorT,
            node: #reference PropertyId<'a>,
        ) {
            visitor.enter_node(AstType::PropertyId);
            match node {
                #(#property_id_arms)*
                PropertyId::ColumnRule
                | PropertyId::Columns
                | PropertyId::GridColumnGap
                | PropertyId::GridRowGap
                | PropertyId::All
                | PropertyId::Unparsed => {}
                PropertyId::Custom(value) => visitor.visit_str(value),
            }
            visitor.leave_node(AstType::PropertyId);
        }

        pub fn walk_vendor_prefix<'a, VisitorT: ?Sized + #visitor_trait<'a>>(
            visitor: &mut VisitorT,
            _node: #reference VendorPrefix,
        ) {
            visitor.enter_node(AstType::VendorPrefix);
            visitor.leave_node(AstType::VendorPrefix);
        }
    }
}

fn generate_walk_module(
    mode: Mode,
    nodes: &[Node],
    aliases: &[Alias],
    known: &HashSet<String>,
    alias_names: &HashSet<String>,
) -> TokenStream {
    let visitor_trait = mode.visitor_trait();
    let node_trait = mode.node_trait();
    let walkers = nodes.iter().map(|node| {
        let walk = walk_method(&node.ident);
        let variant = &node.ident;
        let (function_generics, ty, bounds) =
            function_parts(&node.ident, &node.generics, &visitor_trait, &node_trait);
        let body = walk_data(mode, node, known, alias_names);
        let reference = mode.reference();
        quote! {
            pub fn #walk #function_generics (visitor: &mut VisitorT, node: #reference #ty) #bounds {
                visitor.enter_node(AstType::#variant);
                #body
                visitor.leave_node(AstType::#variant);
            }
        }
    });
    let alias_walkers = aliases.iter().map(|alias| {
        let walk = walk_method(&alias.ident);
        let variant = &alias.ident;
        let (function_generics, ty, bounds) =
            function_parts(&alias.ident, &alias.generics, &visitor_trait, &node_trait);
        let reference = mode.reference();
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
            pub fn #walk #function_generics (visitor: &mut VisitorT, node: #reference #ty) #bounds {
                visitor.enter_node(AstType::#variant);
                #body
                visitor.leave_node(AstType::#variant);
            }
        }
    });
    quote! {
        #![allow(clippy::match_same_arms, clippy::needless_borrow, unused_imports, unused_variables)]

        use super::{#visitor_trait, #node_trait};
        use crate::AstType;
        use rocketcss_ast::*;

        #(#walkers)*
        #(#alias_walkers)*
    }
}

fn walk_data(
    mode: Mode,
    node: &Node,
    known: &HashSet<String>,
    aliases: &HashSet<String>,
) -> TokenStream {
    let generic_names = type_param_names(&node.generics);
    let mut counter = 0;
    match &node.data {
        NodeData::Struct(item) => walk_fields(
            mode,
            &item.fields,
            quote!(node),
            known,
            aliases,
            &generic_names,
            &mut counter,
        ),
        NodeData::Enum(item) => {
            let ident = &item.ident;
            let arms = item.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                match &variant.fields {
                    Fields::Unit => quote!(#ident::#variant_ident => {}),
                    Fields::Unnamed(fields) => {
                        let bindings = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(index, _)| format_ident!("field_{index}"))
                            .collect::<Vec<_>>();
                        let visits =
                            fields
                                .unnamed
                                .iter()
                                .zip(&bindings)
                                .map(|(field, binding)| {
                                    visit_type(
                                        mode,
                                        &field.ty,
                                        quote!(#binding),
                                        known,
                                        aliases,
                                        &generic_names,
                                        &mut counter,
                                    )
                                });
                        quote!(#ident::#variant_ident(#(#bindings),*) => { #(#visits)* })
                    }
                    Fields::Named(fields) => {
                        let bindings = fields
                            .named
                            .iter()
                            .map(|field| field.ident.as_ref().unwrap())
                            .collect::<Vec<_>>();
                        let visits = fields.named.iter().zip(&bindings).map(|(field, binding)| {
                            visit_type(
                                mode,
                                &field.ty,
                                quote!(#binding),
                                known,
                                aliases,
                                &generic_names,
                                &mut counter,
                            )
                        });
                        quote!(#ident::#variant_ident { #(#bindings),* } => { #(#visits)* })
                    }
                }
            });
            quote!(match node { #(#arms),* })
        }
    }
}

fn walk_fields(
    mode: Mode,
    fields: &Fields,
    base: TokenStream,
    known: &HashSet<String>,
    aliases: &HashSet<String>,
    generics: &HashSet<String>,
    counter: &mut usize,
) -> TokenStream {
    let reference = mode.reference();
    match fields {
        Fields::Unit => quote!(),
        Fields::Named(fields) => {
            let visits = fields.named.iter().map(|field| {
                let ident = field.ident.as_ref().unwrap();
                visit_type(
                    mode,
                    &field.ty,
                    quote!(#reference #base.#ident),
                    known,
                    aliases,
                    generics,
                    counter,
                )
            });
            quote!(#(#visits)*)
        }
        Fields::Unnamed(fields) => {
            let visits = fields.unnamed.iter().enumerate().map(|(index, field)| {
                let index = syn::Index::from(index);
                visit_type(
                    mode,
                    &field.ty,
                    quote!(#reference #base.#index),
                    known,
                    aliases,
                    generics,
                    counter,
                )
            });
            quote!(#(#visits)*)
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
            quote!(visitor.visit_str(#expression);)
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
            if matches!(name.as_str(), "Box" | "Option") {
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
            } else if generics.contains(&name) {
                let node_trait = mode.node_trait();
                quote!(#node_trait::visit_node(#expression, visitor);)
            } else if known.contains(&name) || aliases.contains(&name) {
                let method = visit_method(&segment.ident);
                quote!(visitor.#method(#expression);)
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

fn walk_method(ident: &Ident) -> Ident {
    format_ident!("walk_{}", ident.to_string().to_case(Case::Snake))
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
            quote!(#lifetime)
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
    visitor: TokenStream,
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
        quote!(where #(#names: #node_trait<'a, #visitor>),*)
    };
    (method_generics, ty, bounds)
}

fn function_parts(
    ident: &Ident,
    generics: &Generics,
    visitor_trait: &Ident,
    node_trait: &Ident,
) -> (TokenStream, TokenStream, TokenStream) {
    let params = non_lifetime_params(generics);
    let function_generics = if params.is_empty() {
        quote!(<'a, VisitorT>)
    } else {
        quote!(<'a, #(#params,)* VisitorT>)
    };
    let ty = type_tokens(ident, generics);
    let names = generics
        .type_params()
        .map(|param| &param.ident)
        .collect::<Vec<_>>();
    let bounds =
        quote!(where VisitorT: ?Sized + #visitor_trait<'a> #(, #names: #node_trait<'a, VisitorT>)*);
    (function_generics, ty, bounds)
}

fn impl_parts(
    ident: &Ident,
    generics: &Generics,
    visitor_trait: &Ident,
    node_trait: &Ident,
) -> (TokenStream, TokenStream, TokenStream) {
    let params = non_lifetime_params(generics);
    let impl_generics = if params.is_empty() {
        quote!(<'a, VisitorT>)
    } else {
        quote!(<'a, #(#params,)* VisitorT>)
    };
    let ty = type_tokens(ident, generics);
    let names = generics
        .type_params()
        .map(|param| &param.ident)
        .collect::<Vec<_>>();
    let bounds =
        quote!(where VisitorT: ?Sized + #visitor_trait<'a> #(, #names: #node_trait<'a, VisitorT>)*);
    (impl_generics, ty, bounds)
}
