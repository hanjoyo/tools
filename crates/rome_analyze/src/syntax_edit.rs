use rome_js_syntax::{JsSyntaxElement, TextRange, TextSize};

/// A single insert or deletion of text.
pub struct Indel {
    pub text: String,
    pub range: TextRange,
}

#[derive(Debug, Clone)]
pub enum SyntaxEdit {
    Remove {
        target: JsSyntaxElement,
        trimmed: bool,
    },
    Insert {
        offset: TextSize,
        element: JsSyntaxElement,
    },
    Replace {
        target: JsSyntaxElement,
        replacement: JsSyntaxElement,
        trimmed: bool,
    },
}

impl From<SyntaxEdit> for Indel {
    fn from(edit: SyntaxEdit) -> Self {
        match edit {
            SyntaxEdit::Remove { target, trimmed } => {
                let range = if trimmed {
                    target.text_trimmed_range()
                } else {
                    target.text_range()
                };
                Indel {
                    text: String::default(),
                    range,
                }
            }
            SyntaxEdit::Insert { offset, element } => {
                let text = match element {
                    JsSyntaxElement::Node(it) => it.text_trimmed().into(),
                    JsSyntaxElement::Token(it) => it.text_trimmed().into(),
                };
                let range = TextRange::new(offset, offset);
                Indel { text, range }
            }
            SyntaxEdit::Replace {
                target,
                replacement,
                trimmed: true,
            } => {
                let text = match replacement {
                    JsSyntaxElement::Node(it) => it.text_trimmed().into(),
                    JsSyntaxElement::Token(it) => it.text_trimmed().into(),
                };
                let range = target.text_trimmed_range();
                Indel { text, range }
            }
            SyntaxEdit::Replace {
                target,
                replacement,
                trimmed: false,
            } => {
                let text = match replacement {
                    JsSyntaxElement::Node(it) => it.text().into(),
                    JsSyntaxElement::Token(it) => it.text().into(),
                };
                let range = target.text_range();
                Indel { text, range }
            }
        }
    }
}
