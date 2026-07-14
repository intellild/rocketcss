use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input, spanned::Spanned};

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
    use super::to_css_case;

    #[test]
    fn converts_ast_variant_names_to_css_case() {
        assert_eq!(to_css_case("CurrentColor"), "current-color");
        assert_eq!(to_css_case("RGBColor"), "rgb-color");
        assert_eq!(to_css_case("WebKit"), "web-kit");
        assert_eq!(to_css_case("Static_"), "static");
        assert_eq!(to_css_case("Woff2"), "woff2");
    }
}
