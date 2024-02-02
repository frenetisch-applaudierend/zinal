use proc_macro2::Ident;
use syn::{spanned::Spanned, Error, Expr, Field, ItemStruct, Meta, Type};

pub struct TemplateFields(Vec<TemplateField>);

pub struct TemplateField {
    pub ident: Ident,
    pub ty: Type,
    pub optionality: Optionality,
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
            let optionality = parse_optionality(field)?;

            fields.push(TemplateField {
                ident: field.ident.clone().expect("Retrieved from FieldsNamed"),
                ty: field.ty.clone(),
                optionality,
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
    }

    pub fn iter(&self) -> impl Iterator<Item = &TemplateField> {
        self.0.iter()
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
