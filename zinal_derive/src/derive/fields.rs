use proc_macro2::Ident;
use syn::{spanned::Spanned, Error, Expr, Field, ItemStruct, Meta, Type};

pub struct TemplateFields(Vec<TemplateField>);

pub struct TemplateField {
    pub ident: Ident,
    pub ty: Type,
    pub source: Source,
    pub optionality: Optionality,
    pub provides_context: bool,
}

pub enum Source {
    Argument,
    Context,
}

pub enum Optionality {
    Required,
    Optional(Expr),
}

impl TemplateFields {
    pub fn from_template(template: &ItemStruct) -> Result<Self, Error> {
        let fields_named = match &template.fields {
            syn::Fields::Named(f) => f,
            syn::Fields::Unnamed(_) => {
                return Err(Error::new(
                    template.fields.span(),
                    "Cannot derive Template for tuple structs, please use a struct with named fields instead",
                ))
            }
            syn::Fields::Unit => return Ok(Self(Vec::new())),
        };

        let mut fields = Vec::new();

        for field in fields_named.named.iter() {
            let source = parse_source(field)?;
            let optionality = parse_optionality(field)?;
            let provides_context = parse_provides_context(field)?;

            if matches!(source, Source::Context) && provides_context {
                return Err(Error::new(
                    field.span(),
                    "Field cannot be both #[from_context] and #[provide_context]",
                ));
            }

            fields.push(TemplateField {
                ident: field.ident.clone().expect("retrieved from FieldsNamed"),
                ty: field.ty.clone(),
                source,
                optionality,
                provides_context,
            });
        }

        return Ok(Self(fields));

        fn parse_optionality(field: &Field) -> Result<Optionality, Error> {
            let mut optional_expr = None;
            for attr in field.attrs.iter() {
                if !attr.path().is_ident("optional") {
                    continue;
                }

                if optional_expr.is_some() {
                    return Err(Error::new(
                        attr.span(),
                        "Only one #[optional] attribute is supported per field",
                    ));
                }

                if matches!(attr.meta, Meta::Path(_)) {
                    optional_expr = Some(parse_quote!(::std::default::Default::default()));
                } else {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("default") {
                            let value = meta.value()?;
                            let expr: Expr = value.parse()?;

                            optional_expr = Some(expr);
                            Ok(())
                        } else {
                            Err(meta.error("Unsupported attribute. Expected #[optional] or #[optional(default = ...)]"))
                        }
                    })?;
                }
            }

            match optional_expr {
                Some(expr) => Ok(Optionality::Optional(expr)),
                None => Ok(Optionality::Required),
            }
        }

        fn parse_source(field: &Field) -> Result<Source, Error> {
            let mut from_context = false;
            for attr in field.attrs.iter() {
                if !attr.path().is_ident("from_context") {
                    continue;
                }

                if from_context {
                    return Err(Error::new(
                        attr.span(),
                        "Only one #[from_context] attribute is supported per field",
                    ));
                }

                if !matches!(attr.meta, Meta::Path(_)) {
                    return Err(Error::new(
                        attr.span(),
                        "#[from_context] attribute does not support arguments",
                    ));
                }

                from_context = true;
            }

            if from_context {
                Ok(Source::Context)
            } else {
                Ok(Source::Argument)
            }
        }

        fn parse_provides_context(field: &Field) -> Result<bool, Error> {
            let mut provides_context = false;
            for attr in field.attrs.iter() {
                if !attr.path().is_ident("provide_context") {
                    continue;
                }

                if provides_context {
                    return Err(Error::new(
                        attr.span(),
                        "Only one #[provide_context] attribute is supported per field",
                    ));
                }

                if !matches!(attr.meta, Meta::Path(_)) {
                    return Err(Error::new(
                        attr.span(),
                        "#[provide_context] attribute does not support arguments",
                    ));
                }

                provides_context = true;
            }

            Ok(provides_context)
        }
    }

    pub fn all(&self) -> impl Iterator<Item = &TemplateField> {
        self.0.iter()
    }

    pub fn args(&self) -> impl Iterator<Item = &TemplateField> {
        self.0
            .iter()
            .filter(|f| matches!(f.source, Source::Argument))
    }

    pub fn ctx(&self) -> impl Iterator<Item = &TemplateField> {
        self.0
            .iter()
            .filter(|f| matches!(f.source, Source::Context))
    }
}

impl Optionality {
    pub fn is_optional(&self) -> bool {
        match self {
            Optionality::Required => false,
            Optionality::Optional(_) => true,
        }
    }
}
