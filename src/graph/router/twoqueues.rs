use petgraph::stable_graph::IndexType;
use std::fmt::Debug;
use std::ops::Add;
pub trait Measure: Debug + PartialOrd + Add<Self, Output = Self> + Default + Clone {}
use petgraph::visit::{EdgeRef, IntoEdges, Visitable};
use queues::*;
trait IsEmpty<T> {
    fn is_empty(&self) -> bool;
}
impl<T> IsEmpty<T> for Queue<T>
where
    T: Clone,
{
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::hash::Hash;
#[derive(Clone, Copy)]
enum Label {
    Inqueue,
    Scanned,
    Unscanned,
}
use petgraph::visit::NodeCount;

pub fn two_queue_shortest_path<G, F, K>(
    graph: G,
    start: G::NodeId,
    goal: Option<G::NodeId>,
    mut edge_cost: F,
) -> Option<(K, Vec<G::NodeId>)>
where
    G: IntoEdges + Visitable + NodeCount,
    G::NodeId: Eq + Hash + IndexType,
    F: FnMut(G::EdgeRef) -> K,
    K: Debug + PartialOrd + Default + Clone + std::ops::Add<Output = K> + Copy,
{
    let nodes_num = graph.node_count();
    let mut label = vec![Label::Unscanned; nodes_num];
    let mut scores = HashMap::new();
    // initialize hash map that maps to parent Node
    let mut came_from = HashMap::<G::NodeId, G::NodeId>::new();
    //
    let zero_score = K::default();
    scores.insert(start, zero_score);
    let mut front_queue: Queue<G::NodeId> = queue![];
    let mut back_queue: Queue<G::NodeId> = queue![];
    if let Err(e) = front_queue.add(start) {
        panic!("{}", e);
    }
    while !front_queue.is_empty() || !back_queue.is_empty() {
        let node = if !back_queue.is_empty() {
            back_queue.remove().unwrap()
        } else {
            front_queue.remove().unwrap()
        };
        label[node.index()] = Label::Scanned;
        for edge in graph.edges(node) {
            let next = edge.target();
            let next_score = scores[&node] + edge_cost(edge);
            match scores.entry(next) {
                Occupied(ent) => {
                    if next_score < *ent.get() {
                        *ent.into_mut() = next_score;
                        came_from.insert(next, node);
                        match label[next.index()] {
                            Label::Scanned => {
                                label[next.index()] = Label::Inqueue;
                                if let Err(e) = front_queue.add(next) {
                                    panic!("{}", e);
                                }
                            }
                            Label::Unscanned => {
                                label[next.index()] = Label::Inqueue;
                                if let Err(e) = back_queue.add(next) {
                                    panic!("{}", e);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Vacant(ent) => {
                    ent.insert(next_score);
                    came_from.insert(next, node);

                    match label[next.index()] {
                        Label::Scanned => {
                            label[next.index()] = Label::Inqueue;
                            if let Err(e) = front_queue.add(next) {
                                panic!("{}", e);
                            }
                        }
                        Label::Unscanned => {
                            label[next.index()] = Label::Inqueue;
                            if let Err(e) = back_queue.add(next) {
                                panic!("{}", e);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    //scores was return value
    //return scores;
    //code downbelow is used to get the route
    let mut path = vec![goal.unwrap()];
    let mut current = goal.unwrap();
    while let Some(&previous) = came_from.get(&current) {
        path.push(previous);
        current = previous;
    }
    path.reverse();
    match scores.entry(goal.unwrap()) {
        Occupied(ent) => Some((*ent.get(), path)),
        Vacant(_) => None,
    }

    //end of route find part
    //deal with enum of goal
}

//todo! make a router struct
