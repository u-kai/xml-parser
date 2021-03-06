use std::collections::HashMap;

use crate::xml::trees::nodes::{
    node_interface::{
        ElementInterface, NodeInterface, PropertyInterface, PropertyKey, PropertyValue,
    },
    node_type::NodeType,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuickNode<'a> {
    value: &'a str,
    property: Option<HashMap<PropertyKey<'a>, PropertyValue<'a>>>,
    node_type: NodeType,
}

impl<'a> QuickNode<'a> {
    pub fn new(value: &'a str, node_type: NodeType) -> Self {
        let property = if node_type == NodeType::Element {
            Some(HashMap::new())
        } else {
            None
        };
        QuickNode {
            value,
            node_type,
            property,
        }
    }
    pub fn with_property(
        value: &'a str,
        node_type: NodeType,
        property: Option<HashMap<PropertyKey<'a>, PropertyValue<'a>>>,
    ) -> Self {
        QuickNode {
            value,
            property,
            node_type,
        }
    }
}

impl<'a> ElementInterface<'a> for QuickNode<'a> {
    fn change(&mut self, value: &'a str) -> () {
        self.value = value
    }
    fn value(&self) -> &'a str {
        self.value
    }
}
impl<'a> PropertyInterface<'a> for QuickNode<'a> {
    fn keys(&self) -> Option<Vec<PropertyKey>> {
        self.property
            .as_ref()
            .map(|p| p.keys().map(|k| *k).collect())
    }
    fn values(&self) -> Option<Vec<&PropertyValue>> {
        self.property
            .as_ref()
            .map(|p| p.values().map(|v| v).collect())
    }
    fn contains_key(&self, key: &str) -> bool {
        if self.property.is_some() {
            self.property.as_ref().unwrap().contains_key(key)
        } else {
            false
        }
    }
    fn contains_key_value(&self, key: &str, value: &str) -> bool {
        if self.contains_key(key) {
            self.property
                .as_ref()
                .unwrap()
                .get(key)
                .unwrap()
                .contains(&value)
        } else {
            false
        }
    }
    fn add_property(&mut self, key: &'a str, value: &'a str) -> () {
        if self.property.is_some() {
            if self.property.as_ref().unwrap().contains_key(key) {
                self.property
                    .as_mut()
                    .unwrap()
                    .get_mut(key)
                    .as_mut()
                    .unwrap()
                    .push(value);
                return;
            }
            self.property.as_mut().unwrap().insert(key, vec![value]);
        }
    }
}

impl<'a> NodeInterface<'a> for QuickNode<'a> {
    fn is_element_type(&self) -> bool {
        self.node_type == NodeType::Element
    }
    fn is_text_type(&self) -> bool {
        self.node_type == NodeType::Text
    }
}

#[cfg(test)]
mod quick_node_test {
    use crate::xml::trees::nodes::{
        node_interface::{ElementInterface, NodeInterface, PropertyInterface},
        node_type::NodeType,
    };
    use std::collections::HashMap;

    use super::QuickNode;
    #[test]
    fn values_test() {
        let mut hash = HashMap::new();
        hash.insert("key", vec!["value"]);
        hash.insert("key2", vec!["value2"]);
        hash.insert("key3", vec!["value3"]);
        let node = QuickNode {
            value: "test",
            property: Some(hash),
            node_type: NodeType::Element,
        };
        let values = node.values();
        assert_eq!(values.clone().unwrap().contains(&&vec!["value"]), true);
        assert_eq!(values.clone().unwrap().contains(&&vec!["value2"]), true);
        assert_eq!(values.clone().unwrap().contains(&&vec!["value3"]), true);
    }
    #[test]
    fn keys_test() {
        let mut hash = HashMap::new();
        hash.insert("key", vec!["value"]);
        hash.insert("key2", vec!["value"]);
        hash.insert("key3", vec!["value"]);
        let node = QuickNode {
            value: "test",
            property: Some(hash),
            node_type: NodeType::Element,
        };
        let keys = node.keys();
        assert_eq!(keys.clone().unwrap().contains(&"key"), true);
        assert_eq!(keys.clone().unwrap().contains(&"key2"), true);
        assert_eq!(keys.clone().unwrap().contains(&"key3"), true);
    }
    #[test]
    fn containes_key_value_test() {
        let mut hash = HashMap::new();
        hash.insert("key", vec!["value"]);
        let node = QuickNode {
            value: "test",
            property: Some(hash),
            node_type: NodeType::Element,
        };
        assert_eq!(node.contains_key_value("key", "value"), true);
        assert_eq!(node.contains_key_value("key", "value2"), false);
    }
    #[test]
    fn containes_key_test() {
        let mut hash = HashMap::new();
        hash.insert("key", vec!["value"]);
        let node = QuickNode {
            value: "test",
            property: Some(hash),
            node_type: NodeType::Element,
        };
        assert_eq!(node.contains_key("key"), true);
        assert_eq!(node.contains_key("key2"), false);
        let node = QuickNode {
            value: "test",
            property: None,
            node_type: NodeType::Text,
        };
        assert_eq!(node.contains_key("key"), false);
    }
    #[test]
    fn add_property_test() {
        let mut node = QuickNode::new("test", NodeType::Element);
        node.add_property("key", "value");
        let mut hash = HashMap::new();
        hash.insert("key", vec!["value"]);
        assert_eq!(
            node,
            QuickNode {
                value: "test",
                property: Some(hash),
                node_type: NodeType::Element
            }
        );
    }
    #[test]
    fn change_test() {
        let mut node = QuickNode::new("test", NodeType::Element);
        node.change("data");
        assert_eq!(node.value(), "data");
    }

    #[test]
    fn is_element_type_test() {
        let node = QuickNode::new("test", NodeType::Element);
        assert_eq!(node.is_element_type(), true);
        let node = QuickNode::new("test", NodeType::Text);
        assert_eq!(node.is_element_type(), false);
    }
    #[test]
    fn is_text_type_test() {
        let node = QuickNode::new("test", NodeType::Text);
        assert_eq!(node.is_text_type(), true);
        let node = QuickNode::new("test", NodeType::Element);
        assert_eq!(node.is_text_type(), false);
    }
}
