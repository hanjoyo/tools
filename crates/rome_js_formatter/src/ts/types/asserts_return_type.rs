use crate::format_traits::FormatOptional;
use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsAssertsReturnType;
use rome_js_syntax::TsAssertsReturnTypeFields;

impl FormatNode for TsAssertsReturnType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAssertsReturnTypeFields {
            parameter_name,
            asserts_token,
            predicate,
        } = self.as_fields();
        Ok(format_elements![
            asserts_token.format(formatter)?,
            space_token(),
            parameter_name.format(formatter)?,
            space_token(),
            predicate.format_or_empty(formatter)?
        ])
    }
}
