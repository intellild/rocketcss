use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DeriveInput, Field, Fields, GenericArgument, GenericParam, Generics, Lifetime,
    LitStr, Path, PathArguments, Type, parse_macro_input, parse_quote, spanned::Spanned,
};

mod generated_visit_aliases;

/// Generates immutable and mutable typed traversal for an AST node.
///
/// Fields are traversed through their node traits by default. `#[visit(skip)]`
/// excludes a field from both modes, while `#[visit(skip_mut)]` only excludes
/// mutable traversal. `#[visit(with = path)]` supplies a custom field visitor,
/// and `#[visit(with_mut = path)]` overrides it for mutable traversal.
/// `#[visit(pinned)]` on a struct preserves pinning during mutable traversal.
#[proc_macro_derive(Visit, attributes(visit))]
pub fn derive_visit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_visit(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[derive(Clone, Copy)]
enum VisitMode {
    Read,
    Mut,
}

impl VisitMode {
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

    fn reference(self) -> TokenStream2 {
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

#[derive(Default)]
struct VisitAttributes {
    pinned: bool,
    skip: bool,
    skip_mut: bool,
    with: Option<Path>,
    with_mut: Option<Path>,
}

fn expand_visit(input: DeriveInput) -> syn::Result<TokenStream2> {
    let attributes = visit_attributes(&input.attrs)?;
    if attributes.skip
        || attributes.skip_mut
        || attributes.with.is_some()
        || attributes.with_mut.is_some()
    {
        return Err(syn::Error::new(
            input.span(),
            "only #[visit(pinned)] is supported on an AST type",
        ));
    }
    if attributes.pinned && !matches!(input.data, Data::Struct(_)) {
        return Err(syn::Error::new(
            input.span(),
            "#[visit(pinned)] is only supported on structs",
        ));
    }

    let (ast_lifetime, ghost_lifetime) = visitor_lifetimes(&input.generics)?;
    let read = expand_visit_mode(
        VisitMode::Read,
        &input.ident,
        &input.generics,
        &input.data,
        &ast_lifetime,
        &ghost_lifetime,
        false,
    )?;
    let mutable = expand_visit_mode(
        VisitMode::Mut,
        &input.ident,
        &input.generics,
        &input.data,
        &ast_lifetime,
        &ghost_lifetime,
        attributes.pinned,
    )?;

    Ok(quote! {
        #read
        #mutable
    })
}

fn expand_visit_mode(
    mode: VisitMode,
    name: &Ident,
    generics: &Generics,
    data: &Data,
    ast_lifetime: &Lifetime,
    ghost_lifetime: &Lifetime,
    pinned: bool,
) -> syn::Result<TokenStream2> {
    let visitor_trait = mode.visitor_trait();
    let node_trait = mode.node_trait();
    let visit = mode.visit_method();
    let visit_children = mode.visit_children_method();
    let context = format_ident!(
        "{}",
        if matches!(mode, VisitMode::Read) {
            "VisitContext"
        } else {
            "VisitMutContext"
        }
    );
    let context_reference = if matches!(mode, VisitMode::Read) {
        quote!(&)
    } else {
        quote!(&mut)
    };
    let callback = format_ident!("visit_{}", name.to_string().to_case(Case::Snake));
    let body = visit_data(mode, data)?;
    let impl_generics = impl_generics(generics, ast_lifetime, ghost_lifetime, &node_trait);
    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();
    let (_, type_generics, _) = generics.split_for_impl();

    if matches!(mode, VisitMode::Mut) && pinned {
        Ok(quote! {
            #[allow(clippy::match_same_arms, clippy::needless_borrow, unused_variables)]
            impl #impl_generics crate::#node_trait<#ast_lifetime, #ghost_lifetime>
                for ::core::pin::Pin<&mut #name #type_generics>
                #where_clause
            {
                #[inline]
                fn #visit<VisitorT: ?Sized + crate::#visitor_trait<#ast_lifetime, #ghost_lifetime>>(
                    &mut self,
                    visitor: &mut VisitorT,
                    cx: &mut crate::VisitMutContext<'_, #ghost_lifetime>,
                ) {
                    visitor.#callback(self.as_mut(), cx);
                }

                fn #visit_children<VisitorT: ?Sized + crate::#visitor_trait<#ast_lifetime, #ghost_lifetime>>(
                    &mut self,
                    visitor: &mut VisitorT,
                    cx: &mut crate::VisitMutContext<'_, #ghost_lifetime>,
                ) {
                    visitor.enter_node(crate::AstType::#name);
                    // SAFETY: traversal mutates fields without moving the pinned node.
                    let node = unsafe { self.as_mut().get_unchecked_mut() };
                    #body
                    visitor.leave_node(crate::AstType::#name);
                }
            }
        })
    } else {
        let reference = mode.reference();
        Ok(quote! {
            #[allow(clippy::match_same_arms, clippy::needless_borrow, unused_variables)]
            impl #impl_generics crate::#node_trait<#ast_lifetime, #ghost_lifetime>
                for #name #type_generics #where_clause
            {
                #[inline]
                fn #visit<VisitorT: ?Sized + crate::#visitor_trait<#ast_lifetime, #ghost_lifetime>>(
                    #reference self,
                    visitor: &mut VisitorT,
                    cx: #context_reference crate::#context<'_, #ghost_lifetime>,
                ) {
                    visitor.#callback(self, cx);
                }

                fn #visit_children<VisitorT: ?Sized + crate::#visitor_trait<#ast_lifetime, #ghost_lifetime>>(
                    #reference self,
                    visitor: &mut VisitorT,
                    cx: #context_reference crate::#context<'_, #ghost_lifetime>,
                ) {
                    visitor.enter_node(crate::AstType::#name);
                    let node = self;
                    #body
                    visitor.leave_node(crate::AstType::#name);
                }
            }
        })
    }
}

fn visitor_lifetimes(generics: &Generics) -> syn::Result<(Lifetime, Lifetime)> {
    let lifetimes = generics.lifetimes().collect::<Vec<_>>();
    match lifetimes.as_slice() {
        [] => Ok((parse_quote!('a), parse_quote!('ghost))),
        [lifetime] => Ok((lifetime.lifetime.clone(), parse_quote!('ghost))),
        [ast, ghost] if ghost.lifetime.ident == "ghost" => {
            Ok((ast.lifetime.clone(), ghost.lifetime.clone()))
        }
        _ => Err(syn::Error::new(
            generics.span(),
            "Visit supports an AST lifetime and an optional `'ghost` lifetime",
        )),
    }
}

fn impl_generics(
    generics: &Generics,
    ast_lifetime: &Lifetime,
    ghost_lifetime: &Lifetime,
    node_trait: &Ident,
) -> Generics {
    let mut impl_generics = generics.clone();
    if generics.lifetimes().next().is_none() {
        impl_generics
            .params
            .insert(0, GenericParam::Lifetime(parse_quote!(#ast_lifetime)));
    }
    if !generics
        .lifetimes()
        .any(|lifetime| lifetime.lifetime.ident == ghost_lifetime.ident)
    {
        impl_generics.params.insert(
            usize::from(generics.lifetimes().next().is_some()),
            GenericParam::Lifetime(parse_quote!(#ghost_lifetime)),
        );
    }
    for type_parameter in generics.type_params() {
        let ident = &type_parameter.ident;
        impl_generics
            .make_where_clause()
            .predicates
            .push(parse_quote!(
                #ident: crate::#node_trait<#ast_lifetime, #ghost_lifetime>
            ));
    }
    impl_generics
}

fn visit_data(mode: VisitMode, data: &Data) -> syn::Result<TokenStream2> {
    let mut counter = 0;
    match data {
        Data::Struct(data) => visit_fields(mode, &data.fields, quote!(node), &mut counter),
        Data::Enum(data) => {
            let arms = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_attributes = visit_attributes(&variant.attrs)?;
                    if variant_attributes.pinned
                        || variant_attributes.with.is_some()
                        || variant_attributes.with_mut.is_some()
                    {
                        return Err(syn::Error::new(
                            variant.span(),
                            "visitor variants only support skip and skip_mut",
                        ));
                    }
                    let skip = variant_attributes.skip
                        || (matches!(mode, VisitMode::Mut) && variant_attributes.skip_mut);
                    let variant_name = &variant.ident;
                    match &variant.fields {
                        Fields::Unit => Ok(quote!(Self::#variant_name => {})),
                        Fields::Unnamed(fields) => {
                            let bindings = fields
                                .unnamed
                                .iter()
                                .enumerate()
                                .map(|(index, _)| format_ident!("field_{index}"))
                                .collect::<Vec<_>>();
                            let visits = if skip {
                                Vec::new()
                            } else {
                                fields
                                    .unnamed
                                    .iter()
                                    .zip(&bindings)
                                    .map(|(field, binding)| {
                                        visit_field(mode, field, quote!(#binding), &mut counter)
                                    })
                                    .collect::<syn::Result<Vec<_>>>()?
                            };
                            Ok(quote!(Self::#variant_name(#(#bindings),*) => { #(#visits)* }))
                        }
                        Fields::Named(fields) => {
                            let bindings = fields
                                .named
                                .iter()
                                .map(|field| field.ident.as_ref().unwrap())
                                .collect::<Vec<_>>();
                            let visits = if skip {
                                Vec::new()
                            } else {
                                fields
                                    .named
                                    .iter()
                                    .zip(&bindings)
                                    .map(|(field, binding)| {
                                        visit_field(mode, field, quote!(#binding), &mut counter)
                                    })
                                    .collect::<syn::Result<Vec<_>>>()?
                            };
                            Ok(quote!(Self::#variant_name { #(#bindings),* } => { #(#visits)* }))
                        }
                    }
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote!(match node { #(#arms),* }))
        }
        Data::Union(data) => Err(syn::Error::new(
            data.union_token.span,
            "Visit cannot be derived for unions",
        )),
    }
}

fn visit_fields(
    mode: VisitMode,
    fields: &Fields,
    base: TokenStream2,
    counter: &mut usize,
) -> syn::Result<TokenStream2> {
    let reference = mode.reference();
    let visits = match fields {
        Fields::Unit => Vec::new(),
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref().unwrap();
                visit_field(mode, field, quote!(#reference #base.#ident), counter)
            })
            .collect::<syn::Result<Vec<_>>>()?,
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let index = syn::Index::from(index);
                visit_field(mode, field, quote!(#reference #base.#index), counter)
            })
            .collect::<syn::Result<Vec<_>>>()?,
    };
    Ok(quote!(#(#visits)*))
}

fn visit_field(
    mode: VisitMode,
    field: &Field,
    expression: TokenStream2,
    counter: &mut usize,
) -> syn::Result<TokenStream2> {
    let attributes = visit_attributes(&field.attrs)?;
    if attributes.pinned {
        return Err(syn::Error::new(
            field.span(),
            "#[visit(pinned)] is only supported on structs",
        ));
    }
    if attributes.skip || (matches!(mode, VisitMode::Mut) && attributes.skip_mut) {
        return Ok(TokenStream2::new());
    }
    let custom = if matches!(mode, VisitMode::Mut) {
        attributes.with_mut.as_ref().or(attributes.with.as_ref())
    } else {
        attributes.with.as_ref()
    };
    if let Some(custom) = custom {
        return Ok(quote!(#custom(#expression, visitor, cx);));
    }
    visit_type(mode, &field.ty, expression, counter)
}

fn visit_type(
    mode: VisitMode,
    ty: &Type,
    expression: TokenStream2,
    counter: &mut usize,
) -> syn::Result<TokenStream2> {
    match ty {
        Type::Reference(reference) if matches!(&*reference.elem, Type::Path(path) if path.path.is_ident("str")) => {
            Ok(quote!(visitor.visit_str(#expression, cx);))
        }
        Type::Reference(reference) => visit_type(mode, &reference.elem, expression, counter),
        Type::Paren(paren) => visit_type(mode, &paren.elem, expression, counter),
        Type::Group(group) => visit_type(mode, &group.elem, expression, counter),
        Type::Tuple(tuple) => {
            let reference = mode.reference();
            let visits = tuple
                .elems
                .iter()
                .enumerate()
                .map(|(index, ty)| {
                    let index = syn::Index::from(index);
                    visit_type(mode, ty, quote!(#reference (#expression).#index), counter)
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote!(#(#visits)*))
        }
        Type::Array(array) => {
            let binding = fresh_binding(counter);
            let iterator = mode.iterator();
            let inner = visit_type(mode, &array.elem, quote!(#binding), counter)?;
            Ok(quote!(for #binding in (#expression).#iterator() { #inner }))
        }
        Type::Path(path) if path.qself.is_none() => {
            let Some(segment) = path.path.segments.last() else {
                return Err(syn::Error::new(
                    path.span(),
                    "expected a visitor field type",
                ));
            };
            let name = segment.ident.to_string();
            if name == "Pin" {
                let Some(pin_target) = first_type_argument(&segment.arguments) else {
                    return Err(syn::Error::new(segment.span(), "expected a Pin target"));
                };
                if let Type::Reference(reference) = pin_target {
                    return visit_type(
                        mode,
                        &reference.elem,
                        quote!((#expression).as_ref()),
                        counter,
                    );
                }
                let Type::Path(box_path) = pin_target else {
                    return Err(syn::Error::new(
                        pin_target.span(),
                        "Visit only supports Pin<Box<T>> and Pin<&T> fields",
                    ));
                };
                let Some(box_segment) = box_path.path.segments.last() else {
                    return Err(syn::Error::new(box_path.span(), "expected Pin<Box<T>>"));
                };
                let Some(inner_ty) = first_type_argument(&box_segment.arguments) else {
                    return Err(syn::Error::new(box_segment.span(), "expected Pin<Box<T>>"));
                };
                let pins_ghost_cell = matches!(
                    inner_ty,
                    Type::Path(path)
                        if path.path.segments.last().is_some_and(|segment| segment.ident == "GhostCell")
                );
                if pins_ghost_cell {
                    return visit_type(mode, inner_ty, quote!((#expression).as_ref()), counter);
                }
                if matches!(mode, VisitMode::Read) {
                    visit_type(
                        mode,
                        inner_ty,
                        quote!((#expression).as_ref().get_ref()),
                        counter,
                    )
                } else {
                    let binding = fresh_binding(counter);
                    let inner = visit_type(mode, inner_ty, quote!(&mut #binding), counter)?;
                    Ok(quote! {
                        let mut #binding = (#expression).as_mut();
                        #inner
                    })
                }
            } else if matches!(name.as_str(), "Box" | "Option") {
                let Some(inner_ty) = first_type_argument(&segment.arguments) else {
                    return Err(syn::Error::new(
                        segment.span(),
                        "expected a container type argument",
                    ));
                };
                let binding = fresh_binding(counter);
                let accessor = mode.option_accessor();
                if name == "Option" {
                    let inner = visit_type(mode, inner_ty, quote!(#binding), counter)?;
                    Ok(quote!(if let Some(#binding) = (#expression).#accessor() { #inner }))
                } else {
                    visit_type(mode, inner_ty, quote!((#expression).#accessor()), counter)
                }
            } else if name == "Vec" {
                let Some(inner_ty) = first_type_argument(&segment.arguments) else {
                    return Err(syn::Error::new(
                        segment.span(),
                        "expected a Vec type argument",
                    ));
                };
                let binding = fresh_binding(counter);
                let iterator = mode.iterator();
                let inner = visit_type(mode, inner_ty, quote!(#binding), counter)?;
                Ok(quote!(for #binding in (#expression).#iterator() { #inner }))
            } else if name == "GhostCell" {
                let Some(inner_ty) = first_type_argument(&segment.arguments) else {
                    return Err(syn::Error::new(
                        segment.span(),
                        "expected a GhostCell value type",
                    ));
                };
                let is_style_rule = matches!(
                    inner_ty,
                    Type::Path(path)
                        if path.path.segments.last().is_some_and(|segment| segment.ident == "StyleRule")
                );
                if matches!(mode, VisitMode::Mut) && is_style_rule {
                    return Ok(quote!(cx.visit_style_cell(#expression, visitor);));
                }
                let node_trait = mode.node_trait();
                let visit = mode.visit_method();
                let accessor = if matches!(mode, VisitMode::Read) {
                    quote!(get_ref)
                } else {
                    quote!(get_mut)
                };
                Ok(quote! {
                    cx.with_cell(#expression, |value, cx| {
                        crate::#node_trait::#visit(
                            value.#accessor(),
                            visitor,
                            cx,
                        );
                    });
                })
            } else if name == "Ref" {
                if matches!(mode, VisitMode::Read) {
                    Ok(quote! {
                        cx.with_ref(*#expression, |value, cx| {
                            crate::Visit::visit(
                                value.get_ref(),
                                visitor,
                                cx,
                            );
                        });
                    })
                } else {
                    Ok(quote! {
                        cx.visit_ref(*#expression, visitor);
                    })
                }
            } else if generated_visit_aliases::VISIT_ALIASES.contains(&name.as_str()) {
                let method = format_ident!("visit_{}", name.to_case(Case::Snake));
                Ok(quote!(visitor.#method(#expression, cx);))
            } else {
                let node_trait = mode.node_trait();
                let visit = mode.visit_method();
                Ok(quote!(crate::#node_trait::#visit(#expression, visitor, cx);))
            }
        }
        _ => Err(syn::Error::new(
            ty.span(),
            "unsupported visitor field type; use #[visit(skip)] or #[visit(with = path)]",
        )),
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

fn visit_attributes(attributes: &[Attribute]) -> syn::Result<VisitAttributes> {
    let mut result = VisitAttributes::default();
    for attribute in attributes {
        if !attribute.path().is_ident("visit") {
            continue;
        }
        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("pinned") {
                if result.pinned {
                    return Err(meta.error("duplicate pinned option"));
                }
                result.pinned = true;
                return Ok(());
            }
            if meta.path.is_ident("skip") {
                if result.skip {
                    return Err(meta.error("duplicate skip option"));
                }
                result.skip = true;
                return Ok(());
            }
            if meta.path.is_ident("skip_mut") {
                if result.skip_mut {
                    return Err(meta.error("duplicate skip_mut option"));
                }
                result.skip_mut = true;
                return Ok(());
            }
            if meta.path.is_ident("with") {
                if result.with.is_some() {
                    return Err(meta.error("duplicate with option"));
                }
                result.with = Some(meta.value()?.parse()?);
                return Ok(());
            }
            if meta.path.is_ident("with_mut") {
                if result.with_mut.is_some() {
                    return Err(meta.error("duplicate with_mut option"));
                }
                result.with_mut = Some(meta.value()?.parse()?);
                return Ok(());
            }
            Err(meta.error("unsupported visit option"))
        })?;
    }
    if result.skip && (result.skip_mut || result.with.is_some() || result.with_mut.is_some()) {
        return Err(syn::Error::new(
            attributes
                .iter()
                .find(|attribute| attribute.path().is_ident("visit"))
                .map_or(proc_macro2::Span::call_site(), Attribute::span),
            "skip cannot be combined with other visitor options",
        ));
    }
    Ok(result)
}

/// Generates a zero-allocation CSS string mapping for an AST enum.
///
/// Unit variants are converted from Rust's PascalCase naming to CSS kebab-case.
/// A variant can override the generated value with `#[css_keyword("value")]`.
/// Variants with fields return `None` unless an explicit value is provided.
#[proc_macro_derive(CssKeyword, attributes(css_keyword))]
pub fn derive_css_keyword(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_css_keyword(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand_css_keyword(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let input_span = input.span();
    let name = input.ident;
    let Data::Enum(data) = input.data else {
        return Err(syn::Error::new(
            input_span,
            "CssKeyword can only be derived for enums",
        ));
    };
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let arms = data
        .variants
        .into_iter()
        .map(|variant| {
            let ident = &variant.ident;
            let pattern = match &variant.fields {
                Fields::Unit => quote!(Self::#ident),
                Fields::Unnamed(_) => {
                    quote!(Self::#ident(..))
                }
                Fields::Named(_) => {
                    quote!(Self::#ident { .. })
                }
            };
            let value = css_keyword_value(&variant)?;
            Ok(quote!(#pattern => #value))
        })
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
        impl #impl_generics #name #type_generics #where_clause {
            #[inline]
            pub const fn as_css_str(&self) -> Option<&'static str> {
                match self {
                    #(#arms,)*
                }
            }
        }
    })
}

fn css_keyword_value(variant: &syn::Variant) -> syn::Result<proc_macro2::TokenStream> {
    let mut override_value = None;
    for attribute in &variant.attrs {
        if !attribute.path().is_ident("css_keyword") {
            continue;
        }
        if override_value.is_some() {
            return Err(syn::Error::new(
                attribute.span(),
                "duplicate css_keyword attribute",
            ));
        }
        override_value = Some(attribute.parse_args::<LitStr>()?);
    }

    if let Some(value) = override_value {
        return Ok(quote!(Some(#value)));
    }
    if !matches!(variant.fields, Fields::Unit) {
        return Ok(quote!(None));
    }

    let value = LitStr::new(
        &to_css_case(&variant.ident.to_string()),
        variant.ident.span(),
    );
    Ok(quote!(Some(#value)))
}

fn to_css_case(name: &str) -> String {
    let name = name.strip_suffix('_').unwrap_or(name);
    let characters = name.chars().collect::<Vec<_>>();
    let mut value = String::with_capacity(name.len() + 4);

    for (index, &character) in characters.iter().enumerate() {
        if character.is_ascii_uppercase()
            && index > 0
            && (characters[index - 1].is_ascii_lowercase()
                || characters[index - 1].is_ascii_digit()
                || characters
                    .get(index + 1)
                    .is_some_and(char::is_ascii_lowercase))
        {
            value.push('-');
        }
        value.push(character.to_ascii_lowercase());
    }

    value
}

#[cfg(test)]
mod tests {
    use super::{expand_visit, to_css_case};
    use syn::{DeriveInput, parse_quote};

    #[test]
    fn converts_ast_variant_names_to_css_case() {
        assert_eq!(to_css_case("CurrentColor"), "current-color");
        assert_eq!(to_css_case("RGBColor"), "rgb-color");
        assert_eq!(to_css_case("WebKit"), "web-kit");
        assert_eq!(to_css_case("Static_"), "static");
        assert_eq!(to_css_case("Woff2"), "woff2");
    }

    #[test]
    fn visit_derive_handles_pinned_and_skipped_fields() {
        let input: DeriveInput = parse_quote! {
            #[visit(pinned)]
            struct Node<'a, T> {
                child: T,
                #[visit(skip)]
                marker: &'a str,
            }
        };

        let expansion = expand_visit(input).unwrap().to_string();
        assert!(expansion.contains("crate :: Visit < 'a , 'ghost >"));
        assert!(expansion.contains("crate :: VisitMut < 'a , 'ghost >"));
        assert!(expansion.contains("VisitContext < '_ , 'ghost >"));
        assert!(expansion.contains("VisitMutContext < '_ , 'ghost >"));
        assert!(expansion.contains("get_unchecked_mut"));
        assert!(expansion.contains("node . child"));
        assert!(!expansion.contains("node . marker"));
    }
}
