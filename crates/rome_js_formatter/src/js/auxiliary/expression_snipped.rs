use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::JsExpressionSnipped;
use rome_js_syntax::JsExpressionSnippedFields;

impl FormatNode for JsExpressionSnipped {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExpressionSnippedFields {
            expression,
            eof_token,
        } = self.as_fields();

        Ok(format_elements![
            expression.format(formatter)?,
            eof_token.format(formatter)?,
        ])
    }
}
