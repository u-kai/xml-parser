use std::collections::HashMap;

use crate::xml::trees::nodes::{concreate_nodes::quick_node::QuickNode, node_type::NodeType};

#[derive(Clone, Debug)]
pub(super) struct Token<'a> {
    value: &'a str,
    token_type: TokenType,
}

impl<'a> Token<'a> {
    fn with_type(value: &'a str, token_type: TokenType) -> Self {
        Token { value, token_type }
    }
    pub fn to_node(self) -> QuickNode<'a> {
        match &self.token_type {
            TokenType::Element => self.element_token_to_node(),
            TokenType::SingleElement => self.single_element_token_to_node(),
            TokenType::Text => QuickNode::new(self.value, NodeType::Text),
        }
    }
    fn element_token_to_node(self) -> QuickNode<'a> {
        start_or_single_token_to_node(self)
    }
    fn single_element_token_to_node(self) -> QuickNode<'a> {
        start_or_single_token_to_node(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum TokenType {
    Element,
    SingleElement,
    Text,
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum StateMachine {
    ValueBlank,
    ValueChar,
    EleKeyBlank,
    EleKeyChar,
    EleValBlank,
    EleValStart,
    EleValChar,
    EleValSplit,
}

fn start_or_single_token_to_node<'a>(token: Token<'a>) -> QuickNode<'a> {
    let mut key_range = 0..0;
    let mut value_range_list = vec![];
    let mut property = HashMap::new();
    let mut start_index = 0;
    let mut node_char_range = start_index..start_index;
    let mut state = StateMachine::ValueBlank;
    let source = token.value;
    source.bytes().enumerate().for_each(|(i, c)| match state {
        StateMachine::ValueBlank => {
            if c.is_ascii_whitespace() {
                start_index += 1;
                return;
            }
            state = StateMachine::ValueChar
        }
        StateMachine::ValueChar => {
            if c.is_ascii_whitespace() {
                node_char_range = start_index..i;
                state = StateMachine::EleKeyBlank
            }
        }
        StateMachine::EleKeyBlank => {
            if !(c.is_ascii_whitespace()) {
                start_index = i;
                state = StateMachine::EleKeyChar;
            }
        }
        StateMachine::EleKeyChar => {
            if c.is_ascii_whitespace() {
                key_range = start_index..i;
                state = StateMachine::EleKeyBlank;
                return;
            }
            if c == b'=' {
                key_range = start_index..i;
                state = StateMachine::EleValBlank;
            }
        }
        StateMachine::EleValBlank => {
            if c == b'"' {
                start_index = i + 1;
                state = StateMachine::EleValStart;
            }
        }
        StateMachine::EleValStart => {
            if !(c.is_ascii_whitespace()) {
                start_index = i;
                state = StateMachine::EleValChar;
            }
        }
        StateMachine::EleValChar => {
            if c == b'"' {
                value_range_list.push(start_index..i);
                let mut v = vec![];
                for range in &value_range_list {
                    v.push(source.get(range.clone()).unwrap());
                }
                value_range_list = vec![];
                property.insert(source.get(key_range.clone()).unwrap(), v);
                state = StateMachine::EleKeyBlank;
                return;
            }
            if c.is_ascii_whitespace() {
                value_range_list.push(start_index..i);
                state = StateMachine::EleValSplit;
            }
        }
        StateMachine::EleValSplit => {
            if c == b'"' {
                value_range_list.push(start_index..i);
                state = StateMachine::EleKeyBlank;
            }
            if !(c.is_ascii_whitespace()) {
                start_index = i;
                state = StateMachine::EleValChar;
                return;
            }
        }
    });
    let node_type = match token.token_type {
        TokenType::SingleElement => NodeType::Element,
        TokenType::Element => NodeType::Element,
        _ => panic!("not consider end and character type"),
    };
    if start_index == 0 {
        node_char_range = 0..(source.len())
    }
    if state == StateMachine::EleKeyChar {
        let key = source.get(start_index..source.len()).unwrap();

        property.insert(key, vec![""]);
    }
    let node = QuickNode::with_property(
        &source.get(node_char_range).unwrap(),
        node_type,
        Some(property),
    );

    node
}

#[cfg(test)]
mod token_to_node_tests {

    use crate::xml::trees::nodes::node_interface::PropertyInterface;

    use super::*;
    #[test]
    fn token_to_node_case_only_element_key_test() {
        let token = Token::with_type(
            r#"div id="kai" class="style style2" only"#,
            TokenType::Element,
        );
        let mut node = QuickNode::new("div", NodeType::Element);
        node.add_property("id", "kai");

        node.add_property("class", "style");
        node.add_property("class", "style2");
        node.add_property("only", "");
        assert_eq!(token.to_node(), node)
    }
    #[test]
    fn token_to_node_case_element_test() {
        let token = Token::with_type(r#"div id="kai" class="style style2""#, TokenType::Element);
        let mut node = QuickNode::new("div", NodeType::Element);
        node.add_property("id", "kai");
        node.add_property("class", "style");
        node.add_property("class", "style2");
        assert_eq!(token.to_node(), node)
    }
    #[test]
    fn token_to_node_case_single_test() {
        let token = Token::with_type("div", TokenType::SingleElement);
        assert_eq!(token.to_node(), QuickNode::new("div", NodeType::Element));
    }
    #[test]
    fn token_to_node_case_charcter_test() {
        let token = Token::with_type("char", TokenType::Text);
        assert_eq!(token.to_node(), QuickNode::new("char", NodeType::Text));
    }
    #[test]
    fn token_to_node_case_workbook_test() {
        let token = Token::with_type(
            r#"workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x15 xr xr6 xr10 xr2" xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr6="http://schemas.microsoft.com/office/spreadsheetml/2016/revision6" xmlns:xr10="http://schemas.microsoft.com/office/spreadsheetml/2016/revision10" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2""#,
            TokenType::Element,
        );
        let mut node = QuickNode::new("workbook", NodeType::Element);
        node.add_property(
            "xmlns",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        );
        node.add_property(
            "xmlns:r",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
        );
        node.add_property(
            "xmlns:mc",
            "http://schemas.openxmlformats.org/markup-compatibility/2006",
        );
        node.add_property("mc:Ignorable", "x15");
        node.add_property("mc:Ignorable", "xr");
        node.add_property("mc:Ignorable", "xr6");
        node.add_property("mc:Ignorable", "xr10");
        node.add_property("mc:Ignorable", "xr2");
        node.add_property(
            "xmlns:x15",
            "http://schemas.microsoft.com/office/spreadsheetml/2010/11/main",
        );
        node.add_property(
            "xmlns:xr",
            "http://schemas.microsoft.com/office/spreadsheetml/2014/revision",
        );
        node.add_property(
            "xmlns:xr6",
            "http://schemas.microsoft.com/office/spreadsheetml/2016/revision6",
        );
        node.add_property(
            "xmlns:xr10",
            "http://schemas.microsoft.com/office/spreadsheetml/2016/revision10",
        );
        node.add_property(
            "xmlns:xr2",
            "http://schemas.microsoft.com/office/spreadsheetml/2015/revision2",
        );

        assert_eq!(token.to_node(), node);
    }
}
