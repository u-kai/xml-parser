pub trait NodeInterface<'a>: ElementInterface<'a> + PropertyInterface {}
pub type PropertyKey<'a> = &'a str;
pub type PropertyValue<'a> = Vec<&'a str>;
pub trait ElementInterface<'a> {
    fn change(&mut self, value: &'a str) -> ();
    fn value(&self) -> &'a str;
}

pub trait PropertyInterface {
    fn keys(&self) -> Option<&Vec<PropertyKey>>;
    fn values(&self) -> Option<Vec<Vec<PropertyValue>>>;
    fn contains_key(&self, key: &str) -> bool;
    fn contains_key_value(&self, key: &str, value: &str) -> bool;
    fn add_property(&mut self, key: &str, value: &str) -> ();
}
