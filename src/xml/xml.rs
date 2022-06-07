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
}
#[cfg(test)]

mod xml_tree_tests {
    use super::{mock_node::MockNode, XmlTree};
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
    use crate::xml::nodes::node_interface::{
        ElementInterface, NodeInterface, PropertyInterface, PropertyKey, PropertyValue,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct MockNode<'a> {
        value: &'a str,
    }
    impl<'a> MockNode<'a> {
        pub fn new(value: &'a str) -> Self {
            MockNode { value }
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
        fn add_property(&mut self, key: &str, value: &str) -> () {}
        fn contains_key(&self, key: &str) -> bool {
            true
        }
        fn contains_key_value(&self, key: &str, value: &str) -> bool {
            true
        }
        fn keys(&self) -> Option<&Vec<PropertyKey>> {
            None
        }
        fn values(&self) -> Option<Vec<Vec<PropertyValue>>> {
            None
        }
    }
    impl<'a> NodeInterface<'a> for MockNode<'a> {}
}
