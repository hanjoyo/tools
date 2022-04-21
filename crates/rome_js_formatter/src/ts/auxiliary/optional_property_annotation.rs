use crate::format_traits::FormatOptional;
use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsOptionalPropertyAnnotation;
use rome_js_syntax::TsOptionalPropertyAnnotationFields;

impl FormatNode for TsOptionalPropertyAnnotation {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsOptionalPropertyAnnotationFields {
            question_mark_token,
            type_annotation,
        } = self.as_fields();

        Ok(format_elements![
            question_mark_token.format(formatter)?,
            type_annotation.format_or_empty(formatter)?
        ])
    }
}
