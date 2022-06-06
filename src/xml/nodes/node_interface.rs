use super::parts::{element::ElementInterface, property::PropertyInterface};

pub trait NodeInterface<'a>: ElementInterface<'a> + PropertyInterface {}
