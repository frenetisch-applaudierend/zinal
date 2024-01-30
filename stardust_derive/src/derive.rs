use std::path::PathBuf;

use proc_macro2::{Span, TokenStream};

use syn::{spanned::Spanned, Error, FieldsNamed, Ident, ItemStruct};

use crate::{
    opts::TemplateOptions,
    parser::{self, Item}, generics::TemplateGenerics,
};

pub(crate) fn derive(input: ItemStruct) -> Result<TokenStream, Error> {
    let options = TemplateOptions::from_struct(&input)?;
    let generics = TemplateGenerics::from_template(&input);

    let template_impl = derive_template_impl(&input, &options, &generics)?;
    let values = derive_values(&input, &generics)?;
    let properties = derive_properties(&input)?;
    let builder = derive_builder(&input, &generics)?;

    Ok(quote! {
        #template_impl
        #values
        #properties
        #builder
    })
}

fn derive_template_impl(
    input: &ItemStruct,
    options: &TemplateOptions,
    generics: &TemplateGenerics
) -> Result<TokenStream, Error> {
    let content = read_content(options)?;

    let items = parser::parse(&content)?;
    let items = Item::emit_all(items)?;

    let name = &input.ident;

    let builder_ty = generated_ident(input, "Builder");
    let builder_generic_args = generics.builder_args(parse_quote!(()));

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut expanded = TokenStream::new();

    expanded.extend(quote! {

        #[automatically_derived]
        impl #impl_generics ::stardust::Template for #name #ty_generics #where_clause {
            type Builder = #builder_ty #builder_generic_args; 
            
            fn render(&self, __stardust_context: &mut ::stardust::RenderContext) -> ::std::result::Result<(), ::std::fmt::Error> {
                #(#items)*

                Ok(())
            }

            fn builder() -> Self::Builder {
                #builder_ty::new(())
            }
        }
    });

    #[cfg(feature = "axum")]
    expanded.extend(quote! {

        #[automatically_derived]
        impl #impl_generics ::core::convert::Into<::axum::body::Body> for #name #ty_generics #where_clause {
            fn into(self) -> ::axum::body::Body {
                self.render_to_string().expect("Could not render template to string").into()
            }
        }
    });

    // Hand the output tokens back to the compiler
    Ok(expanded)
}

fn derive_values(input: &ItemStruct, generics: &TemplateGenerics) -> Result<TokenStream, Error> {
    let name = generated_ident(input, "Values");
    let ty_generics = generics.values_generics();
    let fields = derive_fields(get_named_fields(input)?);
    
    return Ok(quote! {

        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        struct #name #ty_generics {
            #(#fields),*
        }
    });

    fn derive_fields(fields: Option<&FieldsNamed>) -> Vec<TokenStream> {
        let Some(fields) = fields else {
            return vec![TokenStream::new()];
        };

        fields
            .named
            .iter()
            .map(|f| {
                let name = f.ident.clone();
                let ty = f.ty.clone();
                quote! { #name: ::stardust::derive::Property<#ty> }
            })
            .collect::<Vec<_>>()
    }
}

fn derive_properties(input: &ItemStruct) -> Result<TokenStream, Error> {
    // No need for props module if there are no fields
    let Some(fields) = get_named_fields(input)? else {
        return Ok(TokenStream::new());
    };

    let mod_name = generated_ident(input, "Properties");
    let property_tys = fields
        .named
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref().expect("Must be a named field");
            quote! {
                #[allow(non_camel_case_types)]
                pub struct #ident;
            }
        })
        .collect::<Vec<_>>();


    Ok(quote! {

        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub mod #mod_name {
            #(#property_tys)*
        }
    })
}

fn derive_builder(input: &ItemStruct, generics: &TemplateGenerics) -> Result<TokenStream, Error> {
    let builder_ident = generated_ident(input, "Builder");
    let values_ident = generated_ident(input, "Values");

    let decl_generics = generics.builder_generics();
    let (impl_generics, ty_generics, where_clause) = generics.builder_generics().split_for_impl();
    let values_args = generics.values_args();

    Ok(quote! {

        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        struct #builder_ident #decl_generics {
            values: #values_ident #values_args,
            token: __stardust_Token
        }

        impl #impl_generics #builder_ident #ty_generics #where_clause {
            pub fn new(token: __stardust_Token) -> Self { todo!() }
        }
    })
}

fn generated_ident(input: &ItemStruct, name: &str) -> Ident {
    Ident::new(&format!("__stardust_generated_{}_{}", input.ident, name), input.ident.span())
}

fn get_named_fields(input: &ItemStruct) -> Result<Option<&FieldsNamed>, Error> {
    let fields = match &input.fields {
        syn::Fields::Named(f) => f,
        syn::Fields::Unnamed(_) => {
            return Err(Error::new(
                input.fields.span(),
                "Cannot derive Template for tuple structs, please use a struct with named fields instead",
            ))
        }
        syn::Fields::Unit => return Ok(None),
    };

    Ok(Some(fields))
}

fn read_content(options: &TemplateOptions) -> Result<String, Error> {
    if options.path.is_some() {
        read_file_content(options)
    } else {
        read_inline_content(options)
    }
}

fn read_file_content(options: &TemplateOptions) -> Result<String, Error> {
    let file_name = PathBuf::from(options.path.as_ref().expect("Should have been verified"));

    let mut full_path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("Internal error: environmental variable `CARGO_MANIFEST_DIR` is not set."),
    );
    full_path.push("templates");
    full_path.push(file_name);

    let content =
        std::fs::read_to_string(full_path).map_err(|e| Error::new(Span::call_site(), e))?;

    Ok(content)
}

fn read_inline_content(options: &TemplateOptions) -> Result<String, Error> {
    Ok(options.content.clone().expect("Should have been verified"))
}
