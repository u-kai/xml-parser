use super::node_interface::NodeInterface;

#[derive(Clone, Debug)]
pub enum XmlNode<'a, T: NodeInterface<'a>> {
    Element(T),
    Text(&'a str),
    Comment(&'a str),
}
