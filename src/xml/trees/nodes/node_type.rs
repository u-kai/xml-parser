#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeType {
    Element,
    Text,
    Comment,
    Script,
}
