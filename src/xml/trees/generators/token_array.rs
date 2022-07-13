use crate::xml::trees::{nodes::concreate_nodes::quick_node::QuickNode, tree::XmlTree};

use super::token::{Token, TokenType};
enum StateMachine {
    CharBlank,
    CharChar,
    StartStart,
    EndChar,
    StartChar,
    StartSlash,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TokenArray<'a>(Vec<Token<'a>>);

impl<'a> TokenArray<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut start_index = 0;
        let mut vec = Vec::new();
        let mut state = StateMachine::CharBlank;
        source.bytes().enumerate().for_each(|(i, c)| match state {
            StateMachine::CharBlank => match c {
                60 => {
                    state = StateMachine::StartStart;
                    start_index = i + 1;
                }
                _ => {
                    if !(c.is_ascii_whitespace()) {
                        state = StateMachine::CharChar;
                        start_index = i;
                    }
                }
            },
            StateMachine::CharChar => match c {
                60 => {
                    vec.push(Token::with_type(
                        source.get(start_index..i).unwrap(),
                        TokenType::Text,
                    ));
                    state = StateMachine::StartStart;
                    start_index = i + 1;
                }
                _ => {
                    if c.is_ascii_whitespace() {
                        vec.push(Token::with_type(
                            source.get(start_index..i).unwrap(),
                            TokenType::Text,
                        ));
                        state = StateMachine::CharBlank;
                    }
                }
            },
            StateMachine::StartStart => match c {
                47 => {
                    state = StateMachine::EndChar;
                    start_index += 1;
                }
                _ => {
                    if c.is_ascii_whitespace() {
                        return;
                    }
                    state = StateMachine::StartChar;
                }
            },
            StateMachine::EndChar => match c {
                62 => {
                    vec.push(Token::with_type(
                        source.get(start_index..i).expect(
                            format!(
                                "len {} start {} i {}, range {:?}",
                                source.len(),
                                start_index,
                                i,
                                source.get(190..)
                            )
                            .as_str(),
                        ),
                        TokenType::EndElement,
                    ));
                    state = StateMachine::CharBlank;
                }
                _ => (),
            },
            StateMachine::StartChar => match c {
                47 => {
                    state = StateMachine::StartSlash;
                }
                62 => {
                    state = StateMachine::CharBlank;
                    vec.push(Token::with_type(
                        source.get(start_index..i).unwrap(),
                        TokenType::Element,
                    ))
                }
                _ => (),
            },
            StateMachine::StartSlash => match c {
                62 => {
                    vec.push(Token::with_type(
                        source.get(start_index..i - 1).unwrap(),
                        TokenType::SingleElement,
                    ));
                    state = StateMachine::CharBlank;
                }
                _ => {
                    if !(c.is_ascii_whitespace()) {
                        state = StateMachine::StartChar;
                    }
                }
            },
        });

        TokenArray(vec)
    }
    pub fn to_tree(self) -> XmlTree<'a, QuickNode<'a>> {
        let mut parent_stack = Vec::new();
        for token in self.0 {
            match token.token_type {
                TokenType::Element => {
                    let node = token.to_node();
                    parent_stack.push(XmlTree::new(node, None))
                }
                TokenType::EndElement => {
                    let child = parent_stack.pop();
                    match child {
                        Some(node) => {
                            if parent_stack.len() == 0 {
                                return node;
                            }
                            parent_stack.last_mut().unwrap().append_children(node)
                        }
                        None => panic!("error: this case is not parse"),
                    }
                }
                _ => {
                    let node = token.to_node();
                    parent_stack
                        .last_mut()
                        .unwrap()
                        .append_children(XmlTree::new(node, None))
                }
            }
        }
        // case exist declear line
        if parent_stack.len() == 1 {
            let single_parent = parent_stack.pop().unwrap();
            return single_parent;
        }
        panic!("not had end tag this stack : {:?}", parent_stack)
    }
}

#[cfg(test)]
mod p_token_array_test {
    use super::*;

    #[test]
    fn build_test() {
        let source = r#"
                    <div>
                        hello world
                    </div>
                    "#;
        let token_array = TokenArray::new(source);
        assert_eq!(
            token_array,
            TokenArray(vec![
                Token::with_type("div", TokenType::Element),
                Token::with_type("hello", TokenType::Text),
                Token::with_type("world", TokenType::Text),
                Token::with_type("div", TokenType::EndElement),
            ])
        );
        let source = r#"
        <div id="name" class="style style2">
        hello world
        </div>
        "#;
        let token_array = TokenArray::new(source);
        assert_eq!(
            token_array,
            TokenArray(vec![
                Token::with_type(r#"div id="name" class="style style2""#, TokenType::Element),
                Token::with_type("hello", TokenType::Text),
                Token::with_type("world", TokenType::Text),
                Token::with_type("div", TokenType::EndElement),
            ])
        );
        let source = r#"
        <div id="name" class="style style2">
        <data />
        hello world
        <p> p desu </ p>
        </div>
        "#;
        let token_array = TokenArray::new(source);
        assert_eq!(
            token_array,
            TokenArray(vec![
                Token::with_type(r#"div id="name" class="style style2""#, TokenType::Element),
                Token::with_type("data ", TokenType::SingleElement),
                Token::with_type("hello", TokenType::Text),
                Token::with_type("world", TokenType::Text),
                Token::with_type("p", TokenType::Element),
                Token::with_type("p", TokenType::Text),
                Token::with_type("desu", TokenType::Text),
                Token::with_type(" p", TokenType::EndElement),
                Token::with_type("div", TokenType::EndElement),
            ])
        );
    }
}
