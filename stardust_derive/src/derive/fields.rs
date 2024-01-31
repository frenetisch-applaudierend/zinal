use proc_macro2::Ident;
use syn::{spanned::Spanned, Error, Expr, ItemStruct, Type};

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
        let fields = match &template.fields {
        syn::Fields::Named(f) => f,
        syn::Fields::Unnamed(_) => {
            return Err(Error::new(
                template.fields.span(),
                "Cannot derive Template for tuple structs, please use a struct with named fields instead",
            ))
        }
        syn::Fields::Unit => return Ok(Self(Vec::new())),
    };

        Ok(Self(
            fields
                .named
                .iter()
                .map(|f| {
                    let is_optional = f.attrs.iter().any(|a| a.path().is_ident("optional"));

                    TemplateField {
                        ident: f.ident.clone().expect("Retrieved from FieldsNamed"),
                        ty: f.ty.clone(),
                        optionality: if is_optional {
                            Optionality::Optional(parse_quote!(::std::default::Default::default()))
                        } else {
                            Optionality::Required
                        },
                    }
                })
                .collect(),
        ))
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
