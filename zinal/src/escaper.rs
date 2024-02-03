use std::{collections::HashMap, sync::OnceLock};

pub struct HtmlEscaper;

impl HtmlEscaper {
    pub fn escape<'a>(&self, value: std::borrow::Cow<'a, str>) -> std::borrow::Cow<'a, str> {
        let mut escaped = String::new();
        let escapes =
            ESCAPES.get_or_init(|| HashMap::from([('<', "&lt;"), ('>', "&gt;"), ('&', "&amp;")]));

        let mut previous_offset = 0;
        let mut offset = 0;

        for c in value.chars() {
            if let Some(replacement) = escapes.get(&c) {
                escaped.push_str(&value[previous_offset..offset]);
                escaped.push_str(replacement);

                offset += c.len_utf8();
                previous_offset = offset;
            } else {
                offset += c.len_utf8();
            }
        }

        if !escaped.is_empty() {
            escaped.push_str(&value[previous_offset..offset]);
            escaped.into()
        } else {
            value
        }
    }
}

static ESCAPES: OnceLock<HashMap<char, &'static str>> = OnceLock::new();

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::HtmlEscaper;

    #[test]
    fn escaper_unescaped() {
        let input = Cow::from("This does not need to be escaped");

        let output = HtmlEscaper.escape(input);

        assert_eq!(
            output,
            Cow::<str>::Borrowed("This does not need to be escaped")
        );
    }

    #[test]
    fn escaper_escaped() {
        let input = Cow::from("<&>");

        let output = HtmlEscaper.escape(input);

        assert_eq!(output, Cow::<str>::Owned(String::from("&lt;&amp;&gt;")));
    }

    #[test]
    fn escaper_mixed() {
        let input = Cow::from("< hello & world >");

        let output = HtmlEscaper.escape(input);

        assert_eq!(
            output,
            Cow::<str>::Owned(String::from("&lt; hello &amp; world &gt;"))
        );
    }
}
