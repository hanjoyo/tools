use crate::format_traits::FormatOptional;
use crate::{
    format_elements, hard_group_elements, space_token, Format, FormatElement, FormatNode,
    FormatResult, Formatter,
};
use rome_js_syntax::TsFunctionType;
use rome_js_syntax::TsFunctionTypeFields;

impl FormatNode for TsFunctionType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsFunctionTypeFields {
            parameters,
            fat_arrow_token,
            type_parameters,
            return_type,
        } = self.as_fields();

        Ok(hard_group_elements(format_elements![
            type_parameters.format_or_empty(formatter)?,
            parameters.format(formatter)?,
            space_token(),
            fat_arrow_token.format(formatter)?,
            space_token(),
            return_type.format(formatter)?
        ]))
    }
}
