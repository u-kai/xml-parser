use crate::xml::nodes::node_interface::PropertyInterface;

#[derive(Clone, Debug)]
pub struct NodeElement<'a, T: PropertyInterface> {
    value: &'a str,
    property: T,
}
