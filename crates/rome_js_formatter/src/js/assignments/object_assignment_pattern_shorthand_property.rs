use crate::format_traits::FormatOptional;

use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};

use rome_js_syntax::JsObjectAssignmentPatternShorthandProperty;
use rome_js_syntax::JsObjectAssignmentPatternShorthandPropertyFields;

impl FormatNode for JsObjectAssignmentPatternShorthandProperty {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternShorthandPropertyFields { identifier, init } =
            self.as_fields();

        let init_node =
            init.format_with_or_empty(formatter, |node| format_elements![space_token(), node])?;
        Ok(format_elements![identifier.format(formatter)?, init_node])
    }
}
