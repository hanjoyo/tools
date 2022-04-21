use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsPostUpdateExpression;
use rome_js_syntax::JsPostUpdateExpressionFields;

impl FormatNode for JsPostUpdateExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPostUpdateExpressionFields {
            operand,
            operator_token,
        } = self.as_fields();

        Ok(format_elements![
            operand.format(formatter)?,
            operator_token.format(formatter)?,
        ])
    }
}
