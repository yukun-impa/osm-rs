use crate::graph::graphelements::Link;
use crate::graph::graphelements::Node as GraphNode;
use crate::reader::osmparse::OSM;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

pub fn build_graph(osm: &OSM) -> Result<Graph<GraphNode, f64>, i32> {
    let mut graph = Graph::<GraphNode, f64>::with_capacity(osm.nodes.len(), 5 * osm.ways.len());
    let mut nodeid2index = HashMap::<usize, NodeIndex>::new();
    for node in &osm.nodes {
        let graphnode = GraphNode::from_osmnode(&node);
        nodeid2index.insert(graphnode.id, graph.add_node(graphnode));
    }

    for way in &osm.ways {
        let one_way = way.is_one_way();
        for link in &Link::from_way(way, &osm.nodes) {
            let from_index = nodeid2index[&link.from];
            let to_index = nodeid2index[&link.to];
            if one_way {
                graph.add_edge(from_index, to_index, link.length);
            } else {
                graph.add_edge(from_index, to_index, link.length);
                graph.add_edge(to_index, from_index, link.length);
            }
        }
    }
    Ok(graph)
}
