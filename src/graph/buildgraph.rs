use crate::osmelements::Way;
use crate::osmelements::Node as OSMNode;
use crate::graph::graphelements::Node as GraphNode;
use crate::graph::graphelements::Link;
use petgraph::graph::NodeIndex;
use petgraph::graph::Graph;
use petgraph::stable_graph::GraphIndex;
use crate::osmparse::OSM;
use std::collections::HashMap;
use std::hash::Hash;
use map_3d::distance;


pub fn build_graph(osm: &OSM) -> Result<Graph<GraphNode, Link>, i32> {
    let mut graph = Graph::<GraphNode, Link>::with_capacity(osm.nodes.len(), 5 * osm.ways.len());
    let mut node2index = HashMap::<GraphNode, NodeIndex> ::new();
    for node in &osm.nodes {
        let graphnode = GraphNode::from_osmnode(&node);
        node2index.insert(graphnode, graph.add_node(graphnode));
    }

    for way in &osm.ways {
        for link in &Link::from_way(way) {
            //todo fixed link: from and link: to
            graph.add_edge(link.from, link.to, *link);
        }
    }
    Err(-1)
}
