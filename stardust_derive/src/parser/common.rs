use super::Error;

pub struct Source<'src> {
    source: &'src str,
}

impl<'src> Source<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source }
    }

    pub fn is_empty(&self) -> bool {
        self.source.is_empty()
    }

    pub fn parse_rust_expr(&mut self, end: &str, escape: &str) -> Result<syn::Expr, Error> {
        let mut expr = String::new();

        while !self.is_empty() {
            // Handle escapes
            if self.try_consume(escape) {
                expr.push_str(end);
                continue;
            }

            // Handle end of expression
            if self.try_consume(end) {
                return syn::parse_str(&expr).map_err(Error::from);
            }

            expr.push_str(self.consume(1)?);
        }

        Err(Error::premature_eof())
    }

    pub fn parse_rust_statement(
        &mut self,
        end: &str,
        escape: &str,
    ) -> Result<proc_macro2::TokenStream, Error> {
        todo!("Statements not yet done")
    }

    pub fn peek(&self, len: usize) -> &'src str {
        if self.source.len() >= len {
            &self.source[..len]
        } else {
            ""
        }
    }

    pub fn consume(&mut self, len: usize) -> Result<&'src str, Error> {
        if self.source.len() < len {
            return Err(Error::premature_eof());
        }

        let result = &self.source[..len];
        self.source = &self.source[len..];

        Ok(result)
    }

    pub fn consume_until(&mut self, chars: &[char], skip_first: bool) -> &'src str {
        // Pattern that matches a char in alist of characters, and allowing to skip
        // a given number of matches first
        let pattern = {
            let mut skip = skip_first && self.source.starts_with(chars);
            move |c| {
                let found = chars.contains(&c);
                if found && skip {
                    skip = false;
                    false
                } else {
                    found
                }
            }
        };

        match self.source.find(pattern) {
            Some(idx) => {
                let (consumed, rest) = self.source.split_at(idx);
                self.source = rest;
                consumed
            }
            None => {
                let consumed = self.source;
                self.source = "";
                consumed
            }
        }
    }

    pub fn try_consume(&mut self, token: &str) -> bool {
        if self.peek(token.len()) == token {
            self.consume(token.len()).expect("Peek");
            true
        } else {
            false
        }
    }
}
