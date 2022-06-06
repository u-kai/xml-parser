use super::parts::element::NodeElement;

#[derive(Clone, Debug)]
pub enum XmlNode<'a> {
    Element(NodeElement),
    Text(&'a str),
    Comment(&'a str),
}
