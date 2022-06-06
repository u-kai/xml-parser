use super::nodes::{node::XmlNode, node_interface::NodeInterface};

#[derive(Clone, Debug)]
pub struct XmlTree<'a, T: NodeInterface<'a>> {
    node: XmlNode<'a, T>,
    children: Option<Box<Vec<XmlTree<'a, T>>>>,
}
