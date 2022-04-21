use crate::format_traits::FormatOptional;
use crate::space_token;
use crate::utils::format_with_semicolon;
use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsImportEqualsDeclaration;
use rome_js_syntax::TsImportEqualsDeclarationFields;

impl FormatNode for TsImportEqualsDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsImportEqualsDeclarationFields {
            import_token,
            type_token,
            id,
            eq_token,
            module_reference,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(
            formatter,
            format_elements![
                import_token.format(formatter)?,
                space_token(),
                type_token.format_with_or_empty(formatter, |token| format_elements![
                    token,
                    space_token(),
                ])?,
                id.format(formatter)?,
                space_token(),
                eq_token.format(formatter)?,
                space_token(),
                module_reference.format(formatter)?,
            ],
            semicolon_token,
        )
    }
}
