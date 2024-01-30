use proc_macro2::{Ident, Span};
use syn::{
    punctuated::Punctuated, token::Comma, AngleBracketedGenericArguments, GenericArgument,
    GenericParam, Generics, Type, WhereClause,
};

use crate::derive::generated_ident;

pub(crate) struct TemplateGenerics {
    builder_gen: Generics,
    values_gen: Generics,
    args_template: Punctuated<GenericArgument, Comma>,
    build_params: Option<AngleBracketedGenericArguments>,
    build_where_clause: Option<WhereClause>,
}

impl TemplateGenerics {
    pub(crate) fn from_template(input: &syn::ItemStruct) -> Self {
        let mut builder_gen = input.generics.clone();
        builder_gen
            .params
            .push(GenericParam::Type(parse_quote!(__stardust_Token)));

        let values_gen = input.generics.clone();

        let mut args_template = Punctuated::new();
        for param in input.generics.params.iter() {
            match param {
                GenericParam::Lifetime(l) => {
                    args_template.push(GenericArgument::Lifetime(l.lifetime.clone()));
                }
                GenericParam::Type(t) => {
                    let ident = &t.ident;
                    args_template.push(GenericArgument::Type(parse_quote!(#ident)));
                }
                GenericParam::Const(c) => {
                    let ident = &c.ident;
                    args_template.push(GenericArgument::Const(parse_quote!(#ident)));
                }
            }
        }

        let mut args = Punctuated::new();
        let mut predicates = Punctuated::new();

        for field in input.fields.iter() {
            let prop_mod_ident = generated_ident(input, "Properties");
            let prop_ident = field.ident.as_ref().expect("Should be a named field");
            let tail_ident = Ident::new(&format!("Tail_{}", prop_ident), Span::mixed_site());
            let prop_type: Type = parse_quote!(#prop_mod_ident::#prop_ident);

            args.push(parse_quote!(#tail_ident));
            predicates.push(
                parse_quote!(__stardust_Token: ::stardust::builder::HasProperty<#prop_type, #tail_ident>),
            );
        }

        let build_params = if !args.is_empty() {
            Some(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: <Token![<]>::default(),
                args,
                gt_token: <Token![>]>::default(),
            })
        } else {
            None
        };
        let build_where_clause = if !predicates.is_empty() {
            Some(WhereClause {
                where_token: <Token![where]>::default(),
                predicates,
            })
        } else {
            None
        };

        Self {
            builder_gen,
            values_gen,
            args_template,
            build_params,
            build_where_clause,
        }
    }

    pub(crate) fn builder_generics(&self) -> &Generics {
        &self.builder_gen
    }

    pub(crate) fn builder_args(&self, token_ty: Type) -> AngleBracketedGenericArguments {
        let mut args = self.args_template.clone();
        args.push(GenericArgument::Type(token_ty));

        AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: parse_quote!(<),
            args,
            gt_token: parse_quote!(>),
        }
    }

    pub(crate) fn builder_build_params(&self) -> Option<&AngleBracketedGenericArguments> {
        self.build_params.as_ref()
    }

    pub(crate) fn builder_build_where_clause(&self) -> Option<&WhereClause> {
        self.build_where_clause.as_ref()
    }

    pub(crate) fn values_generics(&self) -> &Generics {
        &self.values_gen
    }

    pub(crate) fn values_args(&self) -> AngleBracketedGenericArguments {
        AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: parse_quote!(<),
            args: self.args_template.clone(),
            gt_token: parse_quote!(>),
        }
    }
}
