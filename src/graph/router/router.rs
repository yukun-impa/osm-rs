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
    W: Debug + PartialOrd + Add<W, Output = W> + Default + Clone + Copy,
{
    fn find_path_index(&self, source: NodeIndex, target: NodeIndex) -> Option<RouteResult<W>>;

    fn find_path_node_id(&self, source: usize, target: usize) -> Option<RouteResult<W>>;

    fn find_path_node_coor(&self, source: (f64, f64), target: (f64, f64)) -> Option<RouteResult<W>>;

    fn find_node_index(&self, id: usize) -> NodeIndex;

    fn find_node(&self, coordinate: (f64, f64)) -> NodeIndex;
}

#[derive(Debug)]
pub struct RouteResult<W>
where
    W: Debug + PartialOrd + Add<W, Output = W> + Default + Clone + Copy,
{
    pub routecost: W,
    pub route: Vec<Node>,
}

impl<W> PartialEq for RouteResult<W>
where
    W: Debug + PartialOrd + Add<W, Output = W> + Default + Clone + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route
    }
}

impl<W> Eq for RouteResult<W> where
    W: Debug + PartialOrd + Add<W, Output = W> + Default + Clone + Copy
{
}

impl<W> Router<W> for Graph<Node, W>
where
    W: Debug + PartialOrd + Add<W, Output = W> + Default + Clone + Copy,
{
    fn find_path_index(&self, source: NodeIndex, target: NodeIndex) -> Option<RouteResult<W>> {
        let path_cost = petgraph::algo::astar(
            &self,
            source,
            |finish| finish == target,
            |e| *e.weight(),
            |_| W::default(),
        )?;
        let (routecost, route) = path_cost;
        let route: Vec<Node> = route
            .into_iter()
            .map(|x| *self.node_weight(x).unwrap())
            .collect();
        Some(RouteResult { routecost, route })
    }

    fn find_path_node_id(&self, source: usize, target: usize) -> Option<RouteResult<W>> {
        let source = self.find_node_index(source);
        let target = self.find_node_index(target);
        self.find_path_index(source, target)
    }

    fn find_path_node_coor(&self, source: (f64, f64), target: (f64, f64)) -> Option<RouteResult<W>> {
        let source = self.find_node(source);
        let target = self.find_node(target);
        self.find_path_index(source, target)
    }

    fn find_node_index(&self, id: usize) -> NodeIndex
    {
        //todo: handle the case when node_id is not valid
        self.node_indices()
            .find(|&n| self.node_weight(n).unwrap().id == id)
            .unwrap()
    }


    fn find_node(&self, coordinate: (f64, f64)) -> NodeIndex
    {
        self
            .node_indices()
            .filter(|&n| self.neighbors_undirected(n).count() != 0)
            .min_by(|&x, &y| {
                distance(
                    (
                        self.node_weight(x).unwrap().lat,
                        self.node_weight(x).unwrap().lon,
                    ),
                    coordinate,
                )
                .partial_cmp(&distance(
                    (
                        self.node_weight(y).unwrap().lat,
                        self.node_weight(y).unwrap().lon,
                    ),
                    coordinate,
                ))
                .unwrap()
            })
            .unwrap()
    }

}