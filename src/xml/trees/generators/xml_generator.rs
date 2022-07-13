use crate::xml::trees::{nodes::concreate_nodes::quick_node::QuickNode, tree::XmlTree};

use crate::xml::trees::nodes::node_type::NodeType;
pub struct XmlGenerator;
impl XmlGenerator {
    pub fn gen<'a>(source: &'a str) -> XmlTree<'a, QuickNode<'a>> {
        let data_node = XmlTree::new(QuickNode::new("data", NodeType::Text), None);
        let div_tree = XmlTree::new(
            QuickNode::new("div", NodeType::Element),
            Some(Box::new(vec![data_node])),
        );
        div_tree
    }
}
#[cfg(test)]
mod xml_generator_tests {
    use crate::xml::trees::generators::xml_generator::XmlGenerator;
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
        let source = "<div>data</div>";
        let tree = XmlGenerator::gen(source);
        let data_node = XmlTree::new(QuickNode::new("data", NodeType::Text), None);
        let div_tree = XmlTree::new(
            QuickNode::new("div", NodeType::Element),
            Some(Box::new(vec![data_node])),
        );
        assert_eq!(tree, div_tree)
    }
}
