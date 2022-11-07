use crate::graph::graphelements::Node;
use map_3d::distance;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
pub trait Measure: Debug + PartialOrd + Add<Self, Output = Self> + Default + Clone {}
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::ops::Add;
pub trait Router<W>
where
    W: Measure + Copy,
{
    fn find_path(&self, source: NodeIndex, target: NodeIndex) -> Option<RouteResult<W>>;

    fn find_node_id(&self, id: usize) -> NodeIndex;
    //fn find_node(coordinate: (f64, f64)) -> NodeIndex;
}

#[derive(Debug)]
pub struct RouteResult<W>
where
    W: Measure + Copy,
{
    pub routecost: W,
    pub route: Vec<Node>,
}

impl<W> PartialEq for RouteResult<W>
where
    W: Measure + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route
    }
}

impl<W> Eq for RouteResult<W> where W: Measure + Copy {}

impl<W> Router<W> for Graph<Node, W>
where
    W: Measure + Copy,
{
    fn find_path(&self, source: NodeIndex, target: NodeIndex) -> Option<RouteResult<W>> {
        let path_cost = petgraph::algo::astar(
            &self,
            source,
            |finish| finish == target,
            |e| *e.weight(),
            |_| W::default(),
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

    fn find_node_id(&self, id: usize) -> NodeIndex
    where
        W: Measure + Copy,
    {
        self.node_indices()
            .find(|&n| self.node_weight(n).unwrap().id == id)
            .unwrap()
    }
}

pub fn find_node<W>(graph: &Graph<Node, W>, coordinate: (f64, f64)) -> NodeIndex
where
    W: Measure + Copy,
{
    graph
        .node_indices()
        .min_by(|&x, &y| {
            distance(
                (
                    graph.node_weight(x).unwrap().lat,
                    graph.node_weight(x).unwrap().lon,
                ),
                coordinate,
            )
            .partial_cmp(&distance(
                (
                    graph.node_weight(y).unwrap().lat,
                    graph.node_weight(y).unwrap().lon,
                ),
                coordinate,
            ))
            .unwrap()
        })
        .unwrap()
}
