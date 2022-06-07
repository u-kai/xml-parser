use crate::xml::nodes::node_interface::NodeInterface;
use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XmlTree<'a, T: NodeInterface<'a>> {
    node: T,
    children: Option<Box<Vec<XmlTree<'a, T>>>>,
    _marker: PhantomData<&'a ()>,
}

impl<'a, T: NodeInterface<'a>> XmlTree<'a, T> {
    pub fn append_children(&mut self, child: XmlTree<'a, T>) {
        if self.children.is_some() {
            self.children.as_mut().unwrap().push(child)
        } else {
            self.children = Some(Box::new(vec![child]))
        }
    }
    pub fn get_elements_by_key_value(&self, key: &str, value: &str) -> Vec<&XmlTree<'a, T>> {
        let mut result = vec![];
        if self.node.contains_key_value(key, value) {
            result.push(self);
        }
        if self.children.is_some() {
            self.children.as_ref().unwrap().iter().for_each(|child| {
                result.extend(child.get_elements_by_key_value(key, value));
            });
        }
        result
    }
    pub fn get_elements_by_node_value(&self, value: &str) -> Vec<&XmlTree<'a, T>> {
        let mut result = vec![];
        if self.node.value() == value {
            result.push(self);
        }
        if self.children.is_some() {
            self.children.as_ref().unwrap().iter().for_each(|child| {
                result.extend(child.get_elements_by_node_value(value));
            });
        }
        result
    }
    pub fn text_contents(&self) -> Option<Vec<&str>> {
        self.children.as_ref().map(|child| {
            child
                .iter()
                .filter(|child| child.node.is_text_type())
                .map(|child| child.node.value())
                .collect::<Vec<_>>()
        })
    }
}
#[cfg(test)]

mod xml_tree_tests {
    use crate::xml::nodes::{node::NodeType, node_interface::PropertyInterface};

    use super::{mock_node::MockNode, XmlTree};
    #[test]
    fn text_contents_test() {
        let mut root = XmlTree {
            node: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        let mut text_node = MockNode::new("text-content");
        text_node.change_type(NodeType::Text);
        let text_child = XmlTree {
            node: text_node,
            children: None,
            _marker: Default::default(),
        };
        root.append_children(text_child);
        assert_eq!(root.text_contents(), Some(vec!["text-content"]))
    }
    #[test]
    fn get_elements_by_key_value_test() {
        let mut root = XmlTree {
            node: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        let mut child = MockNode::new("child");
        child.add_property("id", "kai");
        child.add_property("class", "style1");
        child.add_property("class", "style2");
        let child = XmlTree {
            node: child,
            children: None,
            _marker: Default::default(),
        };
        root.append_children(child.clone());
        assert_eq!(root.get_elements_by_key_value("id", "kai"), vec![&child]);
        assert_eq!(
            root.get_elements_by_key_value("class", "style1"),
            vec![&child]
        );
        assert_eq!(
            root.get_elements_by_key_value("class", "style2"),
            vec![&child]
        );
    }
    #[test]
    fn get_elements_by_node_value_test() {
        let root = XmlTree {
            node: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        assert_eq!(root.get_elements_by_node_value("root"), vec![&root]);
        let mut root = XmlTree {
            node: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        root.append_children(XmlTree {
            node: MockNode::new("child"),
            children: None,
            _marker: Default::default(),
        });
        assert_eq!(root.get_elements_by_node_value("root"), vec![&root]);
        let mut root = XmlTree {
            node: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        let mut child = XmlTree {
            node: MockNode::new("child"),
            children: None,
            _marker: Default::default(),
        };
        let grand_child = XmlTree {
            node: MockNode::new("child"),
            children: None,
            _marker: Default::default(),
        };
        child.append_children(grand_child.clone());
        child.append_children(XmlTree {
            node: MockNode::new("dumy"),
            children: None,
            _marker: Default::default(),
        });
        root.append_children(child.clone());
        assert_eq!(
            root.get_elements_by_node_value("child"),
            vec![&child, &grand_child]
        );
    }
    #[test]
    fn append_child_test() {
        let mut root = XmlTree {
            node: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        let child = XmlTree {
            node: MockNode::new("child"),
            children: None,
            _marker: Default::default(),
        };
        root.append_children(child.clone());

        let tobe_root = XmlTree {
            node: MockNode::new("root"),
            children: Some(Box::new(vec![child.clone()])),
            _marker: Default::default(),
        };
        assert_eq!(root, tobe_root)
    }
}

mod mock_node {
    use std::collections::HashMap;

    use crate::xml::nodes::{
        node::{self, NodeType},
        node_interface::{
            ElementInterface, NodeInterface, PropertyInterface, PropertyKey, PropertyValue,
        },
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct MockNode<'a> {
        value: &'a str,
        key_value: HashMap<String, Vec<String>>,
        node_type: NodeType,
    }
    impl<'a> MockNode<'a> {
        pub fn new(value: &'a str) -> Self {
            MockNode {
                value,
                key_value: HashMap::new(),
                node_type: NodeType::Element,
            }
        }
        pub fn change_type(&mut self, node_type: NodeType) {
            self.node_type = node_type
        }
    }
    impl<'a> ElementInterface<'a> for MockNode<'a> {
        fn change(&mut self, value: &'a str) -> () {
            self.value = value
        }
        fn value(&self) -> &'a str {
            self.value
        }
    }
    impl<'a> PropertyInterface for MockNode<'a> {
        fn add_property(&mut self, key: &str, value: &str) -> () {
            if self.key_value.contains_key(key) {
                self.key_value
                    .get_mut(key)
                    .as_mut()
                    .unwrap()
                    .push(value.to_string());
            } else {
                self.key_value
                    .insert(key.to_string(), vec![value.to_string()]);
            }
        }
        fn contains_key(&self, key: &str) -> bool {
            self.key_value.contains_key(key)
        }
        fn contains_key_value(&self, key: &str, value: &str) -> bool {
            if self.key_value.contains_key(key) {
                self.key_value
                    .get(key)
                    .unwrap()
                    .contains(&value.to_string())
            } else {
                false
            }
        }
        fn keys(&self) -> Option<&Vec<PropertyKey>> {
            //if self.key_value.keys().len() == 0 {
            //None
            //} else {
            //self.key_value.keys()
            //}
            None
        }
        fn values(&self) -> Option<Vec<Vec<PropertyValue>>> {
            None
        }
    }
    impl<'a> NodeInterface<'a> for MockNode<'a> {
        fn is_element_type(&self) -> bool {
            self.node_type == NodeType::Element
        }
        fn is_text_type(&self) -> bool {
            self.node_type == NodeType::Text
        }
    }
}
