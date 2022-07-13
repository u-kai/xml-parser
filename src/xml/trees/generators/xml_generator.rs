use super::token_array::TokenArray;
use crate::xml::trees::{nodes::concreate_nodes::quick_node::QuickNode, tree::XmlTree};
pub struct XmlGenerator;
impl XmlGenerator {
    pub fn gen<'a>(source: &'a str) -> XmlTree<'a, QuickNode<'a>> {
        let token_array = TokenArray::new(source);
        token_array.to_tree()
    }
}
#[cfg(test)]
mod xml_generator_tests {
    use crate::xml::trees::generators::xml_generator::XmlGenerator;
    use crate::xml::trees::nodes::node_interface::PropertyInterface;
    use crate::xml::trees::{
        nodes::{concreate_nodes::quick_node::QuickNode, node_type::NodeType},
        tree::XmlTree,
    };

    #[test]
    fn gen_test() {
        let source = "<div>data</div>";
        let tree = XmlGenerator::gen(source);
        let data_node = XmlTree::new(QuickNode::new("data", NodeType::Text), None);
        let div_tree = XmlTree::new(
            QuickNode::new("div", NodeType::Element),
            Some(Box::new(vec![data_node])),
        );
        assert_eq!(tree, div_tree);
        let source = r#"<div id="data">data</div>"#;
        let tree = XmlGenerator::gen(source);
        let data_node = XmlTree::new(QuickNode::new("data", NodeType::Text), None);
        let mut div = QuickNode::new("div", NodeType::Element);
        div.add_property("id", "data");
        let div_tree = XmlTree::new(div, Some(Box::new(vec![data_node])));
        assert_eq!(tree, div_tree)
    }
}
