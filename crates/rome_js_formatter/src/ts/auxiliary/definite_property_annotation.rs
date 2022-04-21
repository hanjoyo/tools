use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsDefinitePropertyAnnotation;
use rome_js_syntax::TsDefinitePropertyAnnotationFields;

impl FormatNode for TsDefinitePropertyAnnotation {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDefinitePropertyAnnotationFields {
            excl_token,
            type_annotation,
        } = self.as_fields();
        Ok(format_elements![
            excl_token.format(formatter)?,
            type_annotation.format(formatter)?
        ])
    }
}
