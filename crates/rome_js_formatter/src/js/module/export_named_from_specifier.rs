use crate::format_traits::FormatOptional;

use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};

use rome_js_syntax::JsExportNamedFromSpecifier;
use rome_js_syntax::JsExportNamedFromSpecifierFields;

impl FormatNode for JsExportNamedFromSpecifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedFromSpecifierFields {
            type_token,
            source_name,
            export_as,
        } = self.as_fields();

        let type_token = type_token.format_with_or_empty(formatter, |type_token| {
            format_elements![type_token, space_token()]
        })?;

        let source_name = source_name.format(formatter)?;

        let export_as = export_as.format_with_or_empty(formatter, |export_as| {
            format_elements![space_token(), export_as]
        })?;

        Ok(format_elements![type_token, source_name, export_as])
    }
}
