use crate::raw_language::RawLanguageKind::{COMMA_TOKEN, LITERAL_EXPRESSION};
///! Provides a sample language implementation that is useful in API explanation or tests
use crate::{
	Language, ParsedChildren, RawNodeSlots, RawSyntaxKind, RawSyntaxNode, SyntaxFactory,
	SyntaxKind, TreeBuilder,
};

#[doc(hidden)]
#[derive(Debug, Default, Hash, Copy, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct RawLanguage;

impl Language for RawLanguage {
	type Kind = RawLanguageKind;
}

#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
#[allow(bad_style)]
pub enum RawLanguageKind {
	ROOT = 0,
	EXPRESSION_LIST = 1,
	SEPARATED_EXPRESSION_LIST = 2,
	COMMA_TOKEN = 3,
	STRING_TOKEN = 4,
	NUMBER_TOKEN = 5,
	LITERAL_EXPRESSION = 6,
	UNKNOWN = 7,
	FOR_KW = 8,
	L_PAREN_TOKEN = 9,
	SEMICOLON_TOKEN = 10,
	R_PAREN_TOKEN = 11,
	EQUAL_TOKEN = 12,
	LET_TOKEN = 13,
	CONDITION = 14,
	PLUS_TOKEN = 15,
	__LAST,
}

impl SyntaxKind for RawLanguageKind {
	fn is_unknown(&self) -> bool {
		self == &RawLanguageKind::UNKNOWN
	}

	fn to_unknown(&self) -> Self {
		RawLanguageKind::UNKNOWN
	}

	fn to_raw(&self) -> RawSyntaxKind {
		RawSyntaxKind(*self as u16)
	}

	#[allow(unsafe_code)]
	fn from_raw(raw: RawSyntaxKind) -> Self {
		assert!(raw.0 < RawLanguageKind::__LAST as u16);

		unsafe { std::mem::transmute::<u16, RawLanguageKind>(raw.0) }
	}
}

#[doc(hidden)]
#[derive(Debug)]
pub struct RawLanguageSyntaxFactory;

impl SyntaxFactory for RawLanguageSyntaxFactory {
	type Kind = RawLanguageKind;

	fn make_syntax(
		kind: Self::Kind,
		children: ParsedChildren<Self::Kind>,
	) -> RawSyntaxNode<Self::Kind> {
		match kind {
			RawLanguageKind::UNKNOWN | RawLanguageKind::ROOT => {
				RawSyntaxNode::new(kind, children.into_iter().map(Some))
			}
			RawLanguageKind::EXPRESSION_LIST => {
				Self::make_node_list_syntax(kind, children, |kind| kind == LITERAL_EXPRESSION)
			}
			RawLanguageKind::SEPARATED_EXPRESSION_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				|kind| kind == LITERAL_EXPRESSION,
				COMMA_TOKEN,
				true,
			),
			RawLanguageKind::LITERAL_EXPRESSION => {
				let actual_len = children.len();

				if actual_len > 1 {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}

				let mut elements = children.into_iter();
				let current_element = elements.next();

				if let Some(element) = &current_element {
					if !matches!(
						element.kind(),
						RawLanguageKind::STRING_TOKEN | RawLanguageKind::NUMBER_TOKEN
					) {
						return RawSyntaxNode::new(
							kind.to_unknown(),
							std::iter::once(current_element),
						);
					}
				} else {
					return RawSyntaxNode::new(kind, std::iter::once(None));
				}

				return RawSyntaxNode::new(kind, std::iter::once(current_element));
			}

			RawLanguageKind::CONDITION => {
				let actual_len = children.len();

				if actual_len > 3 {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}

				let mut elements = (&children).into_iter();
				let mut current_element = elements.next();
				let mut slots: RawNodeSlots<3> = Default::default();

				if let Some(element) = &current_element {
					if element.kind() == RawLanguageKind::L_PAREN_TOKEN {
						slots.mark_present();
						current_element = elements.next();
					} else {
						slots.mark_absent();
					}
				} else {
					slots.mark_absent();
				}

				if let Some(element) = &current_element {
					if element.kind() == RawLanguageKind::LITERAL_EXPRESSION {
						slots.mark_present();
						current_element = elements.next();
					} else {
						slots.mark_absent();
					}
				} else {
					slots.mark_absent();
				}

				if let Some(element) = &current_element {
					if element.kind() == RawLanguageKind::R_PAREN_TOKEN {
						slots.mark_present();
						current_element = elements.next();
					} else {
						slots.mark_absent();
					}
				} else {
					slots.mark_absent();
				}

				if current_element.is_some() {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}

				slots.into_node(kind, children)
			}
			_ => unreachable!("{:?} is not a node kind", kind),
		}
	}
}

#[doc(hidden)]
pub type RawSyntaxTreeBuilder = TreeBuilder<'static, RawLanguage, RawLanguageSyntaxFactory>;