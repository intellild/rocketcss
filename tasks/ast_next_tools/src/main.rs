//! Generates the flattened RocketCSS AST from declaration files.
//!
//! Declarations live under `crates/ast_next/declarations`. Every item marked
//! `#[ast]` becomes a four-byte node handle plus a copyable data record; other
//! public items are emitted as plain values. Generated output is checked in.

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span as MacroSpan, TokenStream};
use quote::{format_ident, quote};
use std::sync::OnceLock;
use syn::{
    Attribute, Fields, GenericArgument, GenericParam, Generics, Item, ItemEnum, ItemStruct,
    ItemType, PathArguments, Type, Visibility,
};

static NODE_NAMES: OnceLock<HashSet<String>> = OnceLock::new();

fn is_node(name: &str) -> bool {
    NODE_NAMES.get().is_some_and(|names| names.contains(name))
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = root.join("crates/ast_next/declarations");
    let output = root.join("crates/ast_next/src/generated");
    fs::create_dir_all(&output).unwrap();

    let mut files = collect_files(&declarations);
    files.sort();

    let mut node_names = HashSet::new();
    let mut parsed = Vec::new();
    for path in &files {
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        if path.file_name().is_some_and(|name| name == "mod.rs") {
            continue;
        }
        let source = fs::read_to_string(path).unwrap();
        let file = syn::parse_file(&source)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));
        for item in &file.items {
            match item {
                Item::Struct(item) if has_ast_attribute(&item.attrs) => {
                    node_names.insert(item.ident.to_string());
                }
                Item::Enum(item) if has_ast_attribute(&item.attrs) => {
                    node_names.insert(item.ident.to_string());
                }
                _ => {}
            }
        }
        parsed.push((path.clone(), file.items));
    }

    let mut modules = Vec::new();
    let mut kinds = Vec::new();
    NODE_NAMES.set(node_names).ok();
    for (path, items) in parsed {
        let module = path
            .strip_prefix(&declarations)
            .unwrap()
            .with_extension("")
            .to_string_lossy()
            .replace(['/', '\\'], "__");
        let generated = generate_module(&items, &mut kinds);
        modules.push(module.clone());
        write_rust(&output.join(format!("{module}.rs")), generated);
    }

    let kind_tokens = generate_kinds(&kinds);
    write_rust(&output.join("kind.rs"), kind_tokens);

    let module_exports = modules.iter().map(|module| {
        let ident = format_ident!("{module}");
        quote!(mod #ident; pub use #ident::*;)
    });
    write_rust(
        &output.join("mod.rs"),
        quote! {
            //! Generated from `crates/ast_next/declarations`.

            mod kind;
            pub(crate) use kind::*;
            #(#module_exports)*
        },
    );
}

fn collect_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut pending = vec![dir.to_path_buf()];
    while let Some(dir) = pending.pop() {
        for entry in fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files
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

fn generate_kinds(kinds: &[Ident]) -> TokenStream {
    let constants = kinds.iter().enumerate().map(|(index, ident)| {
        let value = index as u16 + 1;
        let constant = kind_constant(ident);
        quote!(pub(crate) const #constant: NodeKind = NodeKind::new(#value);)
    });
    quote! {
        //! Node kind discriminants shared by storage and typed handles.

        use crate::NodeKind;

        #(#constants)*
    }
}

fn kind_constant(name: &Ident) -> Ident {
    format_ident!("KIND_{}", name.to_string().to_case(Case::UpperSnake))
}

fn has_ast_attribute(attributes: &[Attribute]) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute.path().is_ident("ast"))
}

fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

fn strip_sort_attribute(attributes: &[Attribute]) -> Vec<Attribute> {
    attributes
        .iter()
        .filter(|attribute| {
            !attribute.path().is_ident("ast") && !attribute.path().is_ident("visit")
        })
        .cloned()
        .collect()
}

fn has_derive_copy(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        if !attribute.path().is_ident("derive") {
            return false;
        }
        let mut copy = false;
        let _ = attribute.parse_nested_meta(|meta| {
            copy |= meta.path.is_ident("Copy");
            Ok(())
        });
        copy
    })
}

fn generate_module(items: &[Item], kinds: &mut Vec<Ident>) -> TokenStream {
    let mut body = TokenStream::new();

    for item in items {
        match item {
            Item::Struct(item) if is_public(&item.vis) => {
                if has_ast_attribute(&item.attrs) {
                    kinds.push(item.ident.clone());
                    body.extend(generate_struct(item));
                } else {
                    body.extend(generate_plain_struct(item));
                }
            }
            Item::Enum(item) if is_public(&item.vis) => {
                let data_enum = item.variants.iter().any(|variant| {
                    !matches!(variant.fields, Fields::Unit)
                });
                if has_ast_attribute(&item.attrs) && data_enum {
                    kinds.push(item.ident.clone());
                    body.extend(generate_enum(item));
                } else {
                    body.extend(generate_plain_enum(item));
                }
            }
            Item::Type(item) if is_public(&item.vis) => body.extend(generate_alias(item)),
            _ => {}
        }
    }

    body
}

fn ast_lifetime(generics: &Generics) -> Ident {
    let _ = generics;
    format_ident!("'ast")
}

fn generic_params(generics: &Generics) -> TokenStream {
    let params = generics.params.iter().map(|param| match param {
        GenericParam::Lifetime(param) => {
            let lifetime = syn::Lifetime::new("'ast", MacroSpan::call_site());
            let bounds = &param.bounds;
            if bounds.is_empty() {
                quote!(#lifetime)
            } else {
                quote!(#lifetime: #bounds)
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
        quote!()
    } else {
        quote!(<#(#params),*>)
    }
}

fn generic_args(generics: &Generics) -> TokenStream {
    let args = generics.params.iter().map(|param| match param {
        GenericParam::Lifetime(_) => {
            let lifetime = syn::Lifetime::new("'ast", MacroSpan::call_site());
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
        quote!()
    } else {
        quote!(<#(#args),*>)
    }
}

fn transform_type(ty: &Type) -> Type {
    match ty {
        Type::Path(path) if path.qself.is_none() => {
            let segment = path.path.segments.last().unwrap();
            let name = segment.ident.to_string();
            match name.as_str() {
                "Box" => {
                    let inner = first_type_argument(&segment.arguments).unwrap();
                    transform_type(inner)
                }
                "Vec" => {
                    let inner = first_type_argument(&segment.arguments).unwrap();
                    let inner = transform_type(inner);
                    syn::parse_quote!(&'ast [#inner])
                }
                "Option" => {
                    let inner = first_type_argument(&segment.arguments).unwrap();
                    let inner = transform_type(inner);
                    syn::parse_quote!(Option<#inner>)
                }
                _ => {
                    let mut path = path.clone();
                    for segment in &mut path.path.segments {
                        replace_lifetimes(&mut segment.arguments);
                    }
                    if is_node(&name) {
                        let segment = path.path.segments.last_mut().unwrap();
                        if matches!(segment.arguments, PathArguments::None) {
                            let arguments: syn::AngleBracketedGenericArguments =
                                syn::parse_quote!(<'ast>);
                            segment.arguments = PathArguments::AngleBracketed(arguments);
                        }
                    }
                    Type::Path(path)
                }
            }
        }
        Type::Reference(reference) => {
            let elem = transform_type(&reference.elem);
            let mut reference = reference.clone();
            reference.lifetime = Some(syn::Lifetime::new("'ast", MacroSpan::call_site()));
            reference.elem = Box::new(elem);
            Type::Reference(reference)
        }
        Type::Tuple(tuple) => {
            let elems = tuple.elems.iter().map(transform_type);
            syn::parse_quote!((#(#elems),*))
        }
        Type::Array(array) => {
            let elem = transform_type(&array.elem);
            let len = &array.len;
            syn::parse_quote!([#elem; #len])
        }
        Type::Paren(paren) => {
            let elem = transform_type(&paren.elem);
            syn::parse_quote!((#elem))
        }
        other => other.clone(),
    }
}

fn replace_lifetimes(arguments: &mut PathArguments) {
    if let PathArguments::AngleBracketed(arguments) = arguments {
        for argument in &mut arguments.args {
            match argument {
                GenericArgument::Lifetime(lifetime) => *lifetime = syn::Lifetime::new("'ast", MacroSpan::call_site()),
                GenericArgument::Type(ty) => *ty = transform_type(ty),
                _ => {}
            }
        }
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

fn data_ident(name: &Ident) -> Ident {
    format_ident!("{name}Data")
}

fn kind_ident(name: &Ident) -> Ident {
    format_ident!("{name}Kind")
}

fn alloc_ident(name: &Ident) -> Ident {
    format_ident!("alloc_{}", name.to_string().to_case(Case::Snake))
}

fn handle_generics(generics: &Generics) -> TokenStream {
    generic_params(generics)
}

fn node_generic_params(generics: &Generics) -> TokenStream {
    let mut params = Vec::new();
    let mut has_lifetime = false;
    for param in &generics.params {
        match param {
            GenericParam::Lifetime(_) => {
                has_lifetime = true;
                let lifetime = syn::Lifetime::new("'ast", MacroSpan::call_site());
                params.push(quote!(#lifetime));
            }
            GenericParam::Type(param) => {
                let ident = &param.ident;
                params.push(quote!(#ident));
            }
            GenericParam::Const(param) => {
                let ident = &param.ident;
                params.push(quote!(#ident));
            }
        }
    }
    if !has_lifetime {
        params.insert(0, quote!('ast));
    }
    quote!(<#(#params),*>)
}

fn node_generic_args(generics: &Generics) -> TokenStream {
    let mut args = Vec::new();
    let mut has_lifetime = false;
    for param in &generics.params {
        match param {
            GenericParam::Lifetime(_) => {
                has_lifetime = true;
                let lifetime = syn::Lifetime::new("'ast", MacroSpan::call_site());
                args.push(quote!(#lifetime));
            }
            GenericParam::Type(param) => {
                let ident = &param.ident;
                args.push(quote!(#ident));
            }
            GenericParam::Const(param) => {
                let ident = &param.ident;
                args.push(quote!(#ident));
            }
        }
    }
    if !has_lifetime {
        args.insert(0, quote!('ast));
    }
    quote!(<#(#args),*>)
}

fn impl_generics(generics: &Generics) -> TokenStream {
    let mut params = Vec::new();
    let mut has_lifetime = false;
    for param in &generics.params {
        match param {
            GenericParam::Lifetime(_) => {
                has_lifetime = true;
                params.push(quote!('ast));
            }
            GenericParam::Type(param) => {
                let ident = &param.ident;
                params.push(quote!(#ident));
            }
            GenericParam::Const(param) => {
                let ident = &param.ident;
                params.push(quote!(#ident));
            }
        }
    }
    if !has_lifetime {
        params.insert(0, quote!('ast));
    }
    quote!(<#(#params),*>)
}

fn method_generics(generics: &Generics) -> TokenStream {
    let params = generics.params.iter().filter_map(|param| match param {
        GenericParam::Lifetime(_) => None,
        GenericParam::Type(param) => {
            let ident = &param.ident;
            Some(quote!(#ident))
        }
        GenericParam::Const(param) => {
            let ident = &param.ident;
            Some(quote!(#ident))
        }
    });
    let params = params.collect::<Vec<_>>();
    if params.is_empty() {
        quote!()
    } else {
        quote!(<#(#params),*>)
    }
}

fn copy_bounds(generics: &Generics) -> TokenStream {
    let names = generics
        .type_params()
        .map(|param| &param.ident)
        .collect::<Vec<_>>();
    if names.is_empty() {
        quote!()
    } else {
        quote!(where #(#names: Copy),*)
    }
}

fn type_params_only(generics: &Generics) -> TokenStream {
    let params = generics.params.iter().filter_map(|param| match param {
        GenericParam::Lifetime(_) => None,
        GenericParam::Type(param) => {
            let ident = &param.ident;
            Some(quote!(#ident))
        }
        GenericParam::Const(param) => {
            let ident = &param.ident;
            Some(quote!(#ident))
        }
    });
    let params = params.collect::<Vec<_>>();
    if params.is_empty() {
        quote!()
    } else {
        quote!(<#(#params),*>)
    }
}

fn type_args_only(generics: &Generics) -> TokenStream {
    type_params_only(generics)
}

fn data_generics(use_lifetime: bool, generics: &Generics) -> TokenStream {
    if use_lifetime {
        node_generic_params(generics)
    } else {
        type_params_only(generics)
    }
}

fn data_args(use_lifetime: bool, generics: &Generics) -> TokenStream {
    if use_lifetime {
        node_generic_args(generics)
    } else {
        type_args_only(generics)
    }
}

fn generate_struct(item: &ItemStruct) -> TokenStream {
    let name = &item.ident;
    let data = data_ident(name);
    let generics = &item.generics;
    let params = node_generic_params(generics);
    let args = node_generic_args(generics);
    let impl_params = impl_generics(generics);
    let method_params = method_generics(generics);
    let bounds = copy_bounds(generics);
    let alloc = alloc_ident(name);
    let kind_const = kind_constant(name);

    let mut data_fields = Vec::new();
    let mut accessors = Vec::new();
    for (index, field) in item.fields.iter().enumerate() {
        let ident = field
            .ident
            .clone()
            .unwrap_or_else(|| format_ident!("field_{index}"));
        let ty = transform_type(&field.ty);
        data_fields.push(quote!(pub #ident: #ty));
        let setter = format_ident!("set_{ident}");
        accessors.push(quote! {
            #[inline]
            pub fn #ident(self, ast: &Ast<'ast>) -> #ty {
                ast.data(self).#ident
            }
            #[inline]
            pub fn #setter(self, ast: &mut Ast<'ast>, value: #ty) {
                let mut data = ast.data(self);
                data.#ident = value;
                ast.set_data(self, data);
            }
        });
    }
    let fields_stream = quote!(#(#data_fields)*);
    let use_lifetime = fields_stream.to_string().contains("'ast");
    let data_params = data_generics(use_lifetime, generics);
    let data_args = data_args(use_lifetime, generics);

    quote! {
        use crate::*;

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct #name #params (pub(crate) NodeId<'ast, #data #data_args>);

        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct #data #data_params {
            #(#data_fields,)*
        }

        impl #impl_params NodeStorage<'ast> for #name #args #bounds {
            const KIND: NodeKind = super::kind::#kind_const;
            type Data = #data #data_args;

            #[inline]
            fn node_id(self) -> NodeId<'ast, #data #data_args> {
                self.0
            }

            #[inline]
            unsafe fn from_node_id(id: NodeId<'ast, #data #data_args>) -> Self {
                Self(id)
            }
        }

        impl #impl_params #name #args #bounds {
            #(#accessors)*
        }

        impl #impl_params GetSpan<'ast> for #name #args #bounds {
            #[inline]
            fn span(self, ast: &Ast<'ast>) -> Span {
                ast.span(self)
            }
        }

        impl #impl_params SetSpan<'ast> for #name #args #bounds {
            #[inline]
            fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
                ast.set_span(self, span);
            }
        }

        impl<'ast> Ast<'ast> {
            #[inline]
            pub fn #alloc #method_params (&mut self, data: #data #data_args, span: Span) -> #name #args #bounds {
                self.alloc(data, span)
            }
        }
    }
}

fn generate_enum(item: &ItemEnum) -> TokenStream {
    let name = &item.ident;
    let data = data_ident(name);
    let kind = kind_ident(name);
    let generics = &item.generics;
    let params = node_generic_params(generics);
    let args = node_generic_args(generics);
    let impl_params = impl_generics(generics);
    let method_params = method_generics(generics);
    let bounds = copy_bounds(generics);
    let alloc = alloc_ident(name);
    let kind_const = kind_constant(name);

    let mut data_variants = Vec::new();
    let mut kind_variants = Vec::new();
    let mut kind_arms = Vec::new();
    for variant in &item.variants {
        let variant_name = &variant.ident;
        kind_variants.push(quote!(#variant_name));
        kind_arms.push(quote!(#data::#variant_name { .. } => #kind::#variant_name));
        let fields = match &variant.fields {
            Fields::Unit => quote!(),
            Fields::Unnamed(fields) => {
                let types = fields.unnamed.iter().map(|field| transform_type(&field.ty));
                quote!((#(#types),*))
            }
            Fields::Named(fields) => {
                let fields = fields.named.iter().filter_map(|field| {
                    let ident = field.ident.as_ref()?;
                    let ty = transform_type(&field.ty);
                    Some(quote!(#ident: #ty))
                });
                quote!({ #(#fields),* })
            }
        };
        data_variants.push(quote!(#variant_name #fields));
    }
    let variants_stream = quote!(#(#data_variants)*);
    let use_lifetime = variants_stream.to_string().contains("'ast");
    let data_params = data_generics(use_lifetime, generics);
    let data_args = data_args(use_lifetime, generics);

    quote! {
        use crate::*;

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct #name #params (pub(crate) NodeId<'ast, #data #data_args>);

        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum #data #data_params {
            #(#data_variants,)*
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum #kind {
            #(#kind_variants,)*
        }

        impl #impl_params NodeStorage<'ast> for #name #args #bounds {
            const KIND: NodeKind = super::kind::#kind_const;
            type Data = #data #data_args;

            #[inline]
            fn node_id(self) -> NodeId<'ast, #data #data_args> {
                self.0
            }

            #[inline]
            unsafe fn from_node_id(id: NodeId<'ast, #data #data_args>) -> Self {
                Self(id)
            }
        }

        impl #impl_params #name #args #bounds {
            #[inline]
            pub fn data(self, ast: &Ast<'ast>) -> #data #data_args {
                ast.data(self)
            }

            #[inline]
            pub fn kind(self, ast: &Ast<'ast>) -> #kind {
                match ast.data(self) { #(#kind_arms,)* }
            }
        }

        impl #impl_params GetSpan<'ast> for #name #args #bounds {
            #[inline]
            fn span(self, ast: &Ast<'ast>) -> Span {
                ast.span(self)
            }
        }

        impl #impl_params SetSpan<'ast> for #name #args #bounds {
            #[inline]
            fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
                ast.set_span(self, span);
            }
        }

        impl<'ast> Ast<'ast> {
            #[inline]
            pub fn #alloc #method_params (&mut self, data: #data #data_args, span: Span) -> #name #args #bounds {
                self.alloc(data, span)
            }
        }
    }
}

fn clean_plain_attrs(attributes: &[Attribute]) -> Vec<Attribute> {
    let mut cleaned = Vec::new();
    for attribute in attributes {
        if attribute.path().is_ident("ast") || attribute.path().is_ident("visit") {
            continue;
        }
        if attribute.path().is_ident("derive") {
            let mut derives = Vec::new();
            let _ = attribute.parse_nested_meta(|meta| {
                if !meta.path.is_ident("Visit") {
                    derives.push(meta.path);
                }
                Ok(())
            });
            if derives.is_empty() {
                continue;
            }
            let tokens = quote!(#(#derives),*);
            let derive = syn::parse_quote!(#[derive(#tokens)]);
            cleaned.push(derive);
            continue;
        }
        cleaned.push(attribute.clone());
    }
    cleaned
}

fn generate_plain_struct(item: &ItemStruct) -> TokenStream {
    let name = &item.ident;
    let attrs = clean_plain_attrs(&item.attrs);
    let generics = &item.generics;
    let params = handle_generics(generics);
    let fields = item
        .fields
        .iter()
        .filter_map(|field| {
            let ident = field.ident.as_ref()?;
            let ty = transform_type(&field.ty);
            Some(quote!(pub #ident: #ty))
        })
        .collect::<Vec<_>>();
    let has_derive = attrs.iter().any(|attribute| attribute.path().is_ident("derive"));
    let fallback = (!has_derive).then(|| quote!(#[derive(Clone, Copy, Debug, PartialEq)]));
    quote! {
        use crate::*;
        #(#attrs)*
        #fallback
        pub struct #name #params { #(#fields,)* }
    }
}

fn generate_plain_enum(item: &ItemEnum) -> TokenStream {
    let name = &item.ident;
    let attrs = clean_plain_attrs(&item.attrs);
    let generics = &item.generics;
    let params = handle_generics(generics);
    let variants = item.variants.iter().map(|variant| {
        let name = &variant.ident;
        let fields = match &variant.fields {
            Fields::Unit => quote!(),
            Fields::Unnamed(fields) => {
                let types = fields.unnamed.iter().map(|field| transform_type(&field.ty));
                quote!((#(#types),*))
            }
            Fields::Named(fields) => {
                let fields = fields.named.iter().filter_map(|field| {
                    let ident = field.ident.as_ref()?;
                    let ty = transform_type(&field.ty);
                    let attrs = &field.attrs;
                    Some(quote!(#(#attrs)* #ident: #ty))
                });
                quote!({ #(#fields),* })
            }
        };
        let variant_attrs = &variant.attrs;
        quote!(#(#variant_attrs)* #name #fields)
    });
    let has_derive = attrs.iter().any(|attribute| attribute.path().is_ident("derive"));
    let fallback = (!has_derive).then(|| quote!(#[derive(Clone, Copy, Debug, PartialEq)]));
    quote! {
        use crate::*;
        #(#attrs)*
        #fallback
        pub enum #name #params { #(#variants,)* }
    }
}

fn generate_alias(item: &ItemType) -> TokenStream {
    let name = &item.ident;
    let generics = &item.generics;
    let params = handle_generics(generics);
    let ty = transform_type(&item.ty);
    quote! {
        pub type #name #params = #ty;
    }
}
