use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::JsAnyClass;
use rome_js_syntax::JsClassExportDefaultDeclaration;

impl FormatNode for JsClassExportDefaultDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyClass::from(self.clone()).format(formatter)
    }
}
