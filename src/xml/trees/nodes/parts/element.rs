use crate::xml::trees::nodes::node_interface::PropertyInterface;

#[derive(Clone, Debug)]
pub struct NodeElement<'a, T: PropertyInterface<'a>> {
    value: &'a str,
    property: T,
}
