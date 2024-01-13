use proc_macro2::Span;

use syn::{ext::IdentExt, parse::ParseStream, spanned::Spanned, Attribute, Ident, LitStr};

#[derive(Debug, Default)]
pub(crate) struct TemplateOptions {
    pub(crate) content: Option<String>,
    pub(crate) path: Option<String>,
}

impl TemplateOptions {
    pub(crate) fn parse_attr(input: ParseStream) -> syn::Result<Self> {
        let mut parsed = TemplateOptions::default();
        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(LitStr) {
                // template("Foo")
                // Shorthand for template(content = "Foo")
                let content = input.parse::<LitStr>().expect("Lookahead");
                parsed.set_content(content.value(), content.span())?;
            } else if lookahead.peek(Ident::peek_any) {
                // template(x = "...")
                // where x could be path, content, content_type
                // we need to parse Ident '=' LitStr

                let key = input.call(Ident::parse_any).expect("Lookahead");
                match key.to_string().as_str() {
                    "content" => {
                        input.parse::<Token![=]>()?;
                        let content = input.parse::<LitStr>()?;
                        parsed.set_content(content.value(), content.span())?;
                    }

                    "path" => {
                        input.parse::<Token![=]>()?;
                        let path = input.parse::<LitStr>()?;
                        parsed.set_path(path.value(), path.span())?;
                    }

                    _ => {
                        let unknown_option = key.to_string();
                        return Err(syn::Error::new_spanned(
                            key,
                            format!("Unknown template option `{}`", unknown_option),
                        ));
                    }
                };
            } else {
                return Err(lookahead.error());
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        parsed.validate()?;

        Ok(parsed)
    }

    pub(crate) fn merge_attr(&mut self, attr: &Attribute) -> Result<(), syn::Error> {
        let parsed = attr.parse_args_with(TemplateOptions::parse_attr)?;

        if let Some(content) = parsed.content {
            self.set_content(content, attr.span())?;
        }

        if let Some(path) = parsed.path {
            self.set_path(path, attr.span())?;
        }

        Ok(())
    }

    pub(crate) fn set_content(&mut self, content: String, span: Span) -> Result<(), syn::Error> {
        if self.content.is_none() {
            self.content.replace(content);
            Ok(())
        } else {
            Err(syn::Error::new(span, "Duplicate content declaration"))
        }
    }

    pub(crate) fn set_path(&mut self, path: String, span: Span) -> Result<(), syn::Error> {
        if self.path.is_none() {
            self.path.replace(path);
            Ok(())
        } else {
            Err(syn::Error::new(span, "Duplicate path declaration"))
        }
    }

    pub(crate) fn validate(&self) -> Result<(), syn::Error> {
        if self.content.is_none() && self.path.is_none() {
            return Err(syn::Error::new(
                Span::call_site(),
                "Missing content or path, add template(content = \"<content>\") or template(path = \"<path>\") to specify"
            ));
        }

        if self.content.is_some() && self.path.is_some() {
            return Err(syn::Error::new(
                Span::call_site(),
                "Both content and path provided, only one must be provided",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use syn::{parse_quote, Attribute};

    use super::TemplateOptions;

    #[test]
    fn parse_empty_options() {
        let attr: Attribute = parse_quote! {
            #[template()]
        };

        let result = attr.parse_args_with(TemplateOptions::parse_attr);

        assert!(result.is_err());
    }

    #[test]
    fn parse_content_shorthand() {
        let attr: Attribute = parse_quote! {
            #[template("Test")]
        };

        let result = attr.parse_args_with(TemplateOptions::parse_attr);

        assert!(result.is_ok_and(|o| {
            assert_eq!(o.content, Some("Test".to_string()));
            assert!(o.path.is_none());
            true
        }));
    }

    #[test]
    fn parse_content() {
        let attr: Attribute = parse_quote! {
            #[template(content = "Test")]
        };

        let result = attr.parse_args_with(TemplateOptions::parse_attr);

        assert!(result.is_ok_and(|o| {
            assert_eq!(o.content, Some("Test".to_string()));
            assert!(o.path.is_none());
            true
        }));
    }

    #[test]
    fn parse_path() {
        let attr: Attribute = parse_quote! {
            #[template(path = "test/foo.html")]
        };

        let result = attr.parse_args_with(TemplateOptions::parse_attr);

        assert!(result.is_ok_and(|o| {
            assert_eq!(o.path, Some("test/foo.html".to_string()));
            assert!(o.content.is_none());
            true
        }));
    }
}
