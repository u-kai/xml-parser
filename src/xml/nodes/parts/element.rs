use super::property::PropertyInterface;

#[derive(Clone, Debug)]
pub struct NodeElement<'a, T: PropertyInterface> {
    value: &'a str,
    property: T,
}

pub trait ElementInterface<'a> {
    fn change(&mut self, value: &'a str) -> ();
    fn value(&self) -> &'a str;
}
