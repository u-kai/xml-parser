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
    use crate::xml::trees::nodes::{node_interface::PropertyInterface, node_type::NodeType};

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
    #[test]
    fn to_trees_test() {
        let data = "<div>
        <div>div-first
            <p>p-data</p>
            div-data
        </div>
    </div>";

        let expect = TokenArray::new(data).to_tree();
        let p = QuickNode::new("p", NodeType::Element);
        let mut p = XmlTree::new(p, None);
        let p_data = QuickNode::new("p-data", NodeType::Text);
        let p_data = XmlTree::new(p_data, None);
        p.append_children(p_data);
        let div = QuickNode::new("div", NodeType::Element);
        let mut div = XmlTree::new(div, None);
        let child_div = QuickNode::new("div", NodeType::Element);
        let mut child_div = XmlTree::new(child_div, None);
        let div_first = QuickNode::new("div-first", NodeType::Text);
        let div_first = XmlTree::new(div_first, None);
        let div_data = QuickNode::new("div-data", NodeType::Text);
        let div_data = XmlTree::new(div_data, None);
        child_div.append_children(div_first);
        child_div.append_children(p);
        child_div.append_children(div_data);
        div.append_children(child_div);
        assert_eq!(expect, div);
        let data = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <div>
                <div>
                    div-first
                    <p>p-data</p>
                    <data/>
                    div-data
                </div>
            </div>"#;
        let expect = TokenArray::new(data).to_tree();
        let mut root = QuickNode::new("?xml", NodeType::Element);
        root.add_property("version", "1.0");
        root.add_property("encoding", "UTF-8");
        root.add_property("standalone", "yes");
        root.add_property("?", "");
        let mut root = XmlTree::new(root, None);
        let p = QuickNode::new("p", NodeType::Element);
        let mut p = XmlTree::new(p, None);
        let p_data = QuickNode::new("p-data", NodeType::Text);
        let p_data = XmlTree::new(p_data, None);
        p.append_children(p_data);
        let single_data = QuickNode::new("data", NodeType::Element);
        let single_data = XmlTree::new(single_data, None);
        let div = QuickNode::new("div", NodeType::Element);
        let mut div = XmlTree::new(div, None);
        let child_div = QuickNode::new("div", NodeType::Element);
        let mut child_div = XmlTree::new(child_div, None);
        let div_first = QuickNode::new("div-first", NodeType::Text);
        let div_first = XmlTree::new(div_first, None);
        let div_data = QuickNode::new("div-data", NodeType::Text);
        let div_data = XmlTree::new(div_data, None);
        child_div.append_children(div_first);
        child_div.append_children(p);
        child_div.append_children(single_data);
        child_div.append_children(div_data);
        div.append_children(child_div);
        root.append_children(div);
        assert_eq!(expect, root)
    }
}
