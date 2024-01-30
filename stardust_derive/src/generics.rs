use syn::{
    punctuated::Punctuated, token::Comma, AngleBracketedGenericArguments, GenericArgument,
    GenericParam, Generics, Type,
};

pub(crate) struct TemplateGenerics {
    builder_gen: Generics,
    values_gen: Generics,
    args_template: Punctuated<GenericArgument, Comma>,
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

        Self {
            builder_gen,
            values_gen,
            args_template,
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
