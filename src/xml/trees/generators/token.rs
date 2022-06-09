#[derive(Clone, Debug)]
pub(super) struct Token<'a> {
    value: &'a str,
    token_type: TokenType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TokenType {
    Element,
    SingleElement,
    Text,
}
