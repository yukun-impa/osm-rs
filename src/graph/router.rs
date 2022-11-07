use crate::graph::graphelements::{Link, Node};
use map_3d::distance;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
pub trait Measure: Debug + PartialOrd + Add<Self, Output = Self> + Default + Clone {}
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::ops::Add;
use std::path;
pub trait Router {
    fn find_path(
        &self,
        source: (f64, f64,),
        target: (f64, f64),
    ) -> Option<RouteResult>;
    

    //fn find_node(coordinate: (f64, f64)) -> NodeIndex;
}

#[derive(Debug)]
pub struct RouteResult {
    pub routecost: f64,
    pub route: Vec<Node>,
}

impl PartialEq for RouteResult {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route
    }
}

impl Eq for RouteResult {}

impl Router for Graph<Node, f64> {
    fn find_path(
            &self,
            source: (f64, f64,),
            target: (f64, f64),
        ) -> Option<RouteResult> {
        let source_index = find_node(source);
        let target_index = find_node(target);
        let path_cost = petgraph::algo::astar(
            &self,
            source_index,
            |finish| finish == target_index,
            |e| *e.weight(),
            |_| 0.0,
        );
        if path_cost.is_none() {
            return None;
        }
        let (routecost, route) = path_cost.unwrap();
        let route: Vec<Node> = route
            .into_iter()
            .map(|x| *self.node_weight(x).unwrap())
            .collect();
        Some(RouteResult { routecost, route })
    }

}
fn find_node(coordinate: (f64, f64)) -> NodeIndex {
    todo!()
}