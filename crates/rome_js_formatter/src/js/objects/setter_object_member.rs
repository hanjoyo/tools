use crate::{
    format_elements, hard_group_elements, space_token, Format, FormatElement, FormatNode,
    FormatResult, Formatter,
};

use rome_js_syntax::JsSetterObjectMember;
use rome_js_syntax::JsSetterObjectMemberFields;

impl FormatNode for JsSetterObjectMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSetterObjectMemberFields {
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            body,
        } = self.as_fields();

        Ok(hard_group_elements(format_elements![
            set_token.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            l_paren_token.format(formatter)?,
            parameter.format(formatter)?,
            r_paren_token.format(formatter)?,
            space_token(),
            body.format(formatter)?,
        ]))
    }
}
