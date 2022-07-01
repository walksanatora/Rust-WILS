use dotavious::{Dot, Edge, Graph, GraphBuilder, Node};

use crate::huffman;

pub fn generate_graph(node: &huffman::Node,path: Option<String>, left: Option<bool>,dot: Option<GraphBuilder>) -> GraphBuilder {
	let mut graph = dot.unwrap_or(GraphBuilder::new_named_directed("huffman"));
	let label = left.unwrap_or(false).to_string();
	if node.is_leaf() {
		graph.add_node(Node::new(label));
		generate_graph(
			node.clone().left.unwrap().as_ref(),
			Some(path.unwrap_or(String::from("")) + "0"),
			Some(true),
			Some(graph)
		);
	}

	return graph
}