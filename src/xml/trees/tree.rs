use crate::xml::trees::nodes::node_interface::NodeInterface;
use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq, Eq)]

pub struct XmlTree<'a, T: NodeInterface<'a>> {
    root: T,
    children: Option<Box<Vec<XmlTree<'a, T>>>>,
    _marker: PhantomData<&'a ()>,
}

impl<'a, T: NodeInterface<'a>> XmlTree<'a, T> {
    pub fn new(root: T, children: Option<Box<Vec<XmlTree<'a, T>>>>) -> Self {
        XmlTree {
            root,
            _marker: Default::default(),
            children,
        }
    }
}
impl<'a, T: NodeInterface<'a>> XmlTree<'a, T> {
    //pub fn change_property(&mut self, key: &'a str, new_value: &'a str) {
    //self.root.change_property(key, new_value)
    //}
    pub fn append_children(&mut self, child: XmlTree<'a, T>) {
        if self.children.is_some() {
            self.children.as_mut().unwrap().push(child)
        } else {
            self.children = Some(Box::new(vec![child]))
        }
    }
    pub fn get_elements_by_key_value(&self, key: &str, value: &str) -> Vec<&XmlTree<'a, T>> {
        let mut result = vec![];
        if self.root.contains_key_value(key, value) {
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
        if self.root.value() == value {
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
                .filter(|child| child.root.is_text_type())
                .map(|child| child.root.value())
                .collect::<Vec<_>>()
        })
    }
    ///
    /// Return child all text
    /// ## Example
    /// ```rust
    ///     let source = r#"<div>
    ///                         <p>my name</p>
    ///                         <div>
    ///                             <p>is</p>
    ///                             <p>kai</p>
    ///                         </div>
    ///                    </div>"#;
    ///     //let xml = XmlTree::from(source);
    ///     //asserteq!(xml.concat_all_text(),"my name is kai")
    /// ```
    ///
    pub fn concat_all_text(&self) -> String {
        if self.root.is_text_type() {
            return format!("{}", self.root.value());
        }
        if self.children.is_none() {
            return "".to_string();
        }
        self.children
            .as_ref()
            .unwrap()
            .iter()
            .fold("".to_string(), |acc, cur| {
                if acc.len() == 0 {
                    format!("{}", cur.concat_all_text())
                } else {
                    format!("{} {}", acc, cur.concat_all_text())
                }
            })
    }
}
#[cfg(test)]

mod xml_tree_tests {
    use crate::xml::trees::nodes::{node_interface::PropertyInterface, node_type::NodeType};

    use super::{mock_node::MockNode, XmlTree};
    //#[test]
    //fn change_property_test() {
    //let mut root = MockNode::new("root");
    //root.add_property("id", "kai");

    //let mut root = XmlTree {
    //root: root,
    //children: None,
    //_marker: Default::default(),
    //};
    //root.change_property("id", "iak");
    //let mut tobe = MockNode::new("root");
    //tobe.add_property("id", "iak");

    //let mut tobe = XmlTree {
    //root: tobe,
    //children: None,
    //_marker: Default::default(),
    //};
    //assert_eq!(root, tobe);
    //}
    #[test]
    fn concat_all_text_test() {
        let mut root = XmlTree {
            root: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        let mut text_node = MockNode::new("text-content");
        text_node.change_type(NodeType::Text);
        let text_child = XmlTree {
            root: text_node,
            children: None,
            _marker: Default::default(),
        };
        root.append_children(text_child);
        assert_eq!(root.concat_all_text(), "text-content".to_string());

        let mut root = XmlTree {
            root: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        let mut text_node = MockNode::new("hello");
        text_node.change_type(NodeType::Text);
        let text_child = XmlTree {
            root: text_node,
            children: None,
            _marker: Default::default(),
        };
        root.append_children(text_child);
        let span = MockNode::new("span");
        let mut text_node = MockNode::new("world");
        text_node.change_type(NodeType::Text);
        let text_node = XmlTree {
            root: text_node,
            children: None,
            _marker: Default::default(),
        };
        let mut span = XmlTree {
            root: span,
            children: None,
            _marker: Default::default(),
        };
        span.append_children(text_node);
        root.append_children(span);
        assert_eq!(root.concat_all_text(), "hello world");
    }
    #[test]
    fn text_contents_test() {
        let mut root = XmlTree {
            root: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        let mut text_node = MockNode::new("text-content");
        text_node.change_type(NodeType::Text);
        let text_child = XmlTree {
            root: text_node,
            children: None,
            _marker: Default::default(),
        };
        root.append_children(text_child);
        assert_eq!(root.text_contents(), Some(vec!["text-content"]))
    }
    #[test]
    fn get_elements_by_key_value_test() {
        let mut root = XmlTree {
            root: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        let mut child = MockNode::new("child");
        child.add_property("id", "kai");
        child.add_property("class", "style1");
        child.add_property("class", "style2");
        let child = XmlTree {
            root: child,
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
            root: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        assert_eq!(root.get_elements_by_node_value("root"), vec![&root]);
        let mut root = XmlTree {
            root: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        root.append_children(XmlTree {
            root: MockNode::new("child"),
            children: None,
            _marker: Default::default(),
        });
        assert_eq!(root.get_elements_by_node_value("root"), vec![&root]);
        let mut root = XmlTree {
            root: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        let mut child = XmlTree {
            root: MockNode::new("child"),
            children: None,
            _marker: Default::default(),
        };
        let grand_child = XmlTree {
            root: MockNode::new("child"),
            children: None,
            _marker: Default::default(),
        };
        child.append_children(grand_child.clone());
        child.append_children(XmlTree {
            root: MockNode::new("dumy"),
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
            root: MockNode::new("root"),
            children: None,
            _marker: Default::default(),
        };
        let child = XmlTree {
            root: MockNode::new("child"),
            children: None,
            _marker: Default::default(),
        };
        root.append_children(child.clone());

        let tobe_root = XmlTree {
            root: MockNode::new("root"),
            children: Some(Box::new(vec![child.clone()])),
            _marker: Default::default(),
        };
        assert_eq!(root, tobe_root)
    }
}

mod mock_node {
    use std::collections::HashMap;

    use crate::xml::trees::nodes::{
        node_interface::{
            ElementInterface, NodeInterface, PropertyInterface, PropertyKey, PropertyValue,
        },
        node_type::NodeType,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct MockNode<'a> {
        value: &'a str,
        key_value: HashMap<String, Vec<String>>,
        node_type: NodeType,
        stack: Vec<&'a str>,
    }
    impl<'a> MockNode<'a> {
        pub fn new(value: &'a str) -> Self {
            MockNode {
                value,
                key_value: HashMap::new(),
                node_type: NodeType::Element,
                stack: Vec::new(),
            }
        }
        pub fn change_type(&mut self, node_type: NodeType) {
            self.node_type = node_type
        }
        pub fn save_property_value(&mut self, value: &'a str) {
            self.stack.push(value)
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
    impl<'a> PropertyInterface<'a> for MockNode<'a> {
        //fn change_property(&mut self, key: &'a str, new_value: &'a str) -> () {
        //self.remove_property(key);
        //self.add_property(key, new_value);
        //}
        //fn remove_property(&mut self, key: &'a str) -> Option<PropertyValue> {
        //if self.contains_key(key) {
        //self.remove_property(key);
        //Some(self.stack.clone())
        //} else {
        //None
        //}
        //}
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
        fn keys(&self) -> Option<Vec<PropertyKey>> {
            //if self.key_value.keys().len() == 0 {
            //None
            //} else {
            //self.key_value.keys()
            //}
            None
        }
        fn values(&self) -> Option<Vec<&PropertyValue>> {
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
