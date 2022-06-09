use crate::xml::trees::nodes::{
    node_interface::{
        ElementInterface, NodeInterface, PropertyInterface, PropertyKey, PropertyValue,
    },
    node_type::NodeType,
};

pub struct QuickNode<'a> {
    value: &'a str,
    node_type: NodeType,
}

impl<'a> QuickNode<'a> {
    pub fn new(value: &'a str, node_type: NodeType) -> Self {
        QuickNode { value, node_type }
    }
}

impl<'a> ElementInterface<'a> for QuickNode<'a> {
    fn change(&mut self, value: &'a str) -> () {}
    fn value(&self) -> &'a str {
        self.value
    }
}
impl<'a> PropertyInterface for QuickNode<'a> {
    fn keys(&self) -> Option<&Vec<PropertyKey>> {
        None
    }
    fn values(&self) -> Option<Vec<Vec<PropertyValue>>> {
        None
    }
    fn contains_key(&self, key: &str) -> bool {
        true
    }
    fn contains_key_value(&self, key: &str, value: &str) -> bool {
        true
    }
    fn add_property(&mut self, key: &str, value: &str) -> () {}
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
    use crate::xml::trees::nodes::{node_interface::NodeInterface, node_type::NodeType};

    use super::QuickNode;

    #[test]
    fn is_element_type_test() {
        let node = QuickNode::new("test", NodeType::Element);
        assert_eq!(node.is_element_type(), true);
        assert_eq!(node.is_element_type(), true);
        let node = QuickNode::new("test", NodeType::Text);
        assert_eq!(node.is_element_type(), false);
    }
}
