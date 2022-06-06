use super::nodes::node::XmlNode;

#[derive(Clone, Debug)]
pub struct XmlTree<'a> {
    node: XmlNode<'a>,
    children: Option<Box<Vec<XmlTree<'a>>>>,
}
