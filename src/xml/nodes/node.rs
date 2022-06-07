use std::marker::PhantomData;

use super::node_interface::NodeInterface;

#[derive(Clone, Debug)]
pub struct XmlNode<'a, T: NodeInterface<'a>> {
    node: T,
    node_type: NodeType,
    _marker: PhantomData<&'a ()>,
}

#[derive(Clone, Debug)]
enum NodeType {
    Element,
    Text,
    Comment,
    Script,
}

#[cfg(test)]
mod xml_node_tests {}
