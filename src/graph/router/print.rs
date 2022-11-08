use crate::graph::router::router::RouteResult;
use std::fmt::{Debug, Display};
use std::ops::Add;
use std::fmt;
impl<W> fmt::Display for RouteResult<W>
where 
    W: Display + Debug + PartialOrd + Add<W, Output = W> + Default + Clone + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Route cost: {} meters", self.routecost)?;
        writeln!(f, "Route:")?;
        for node in self.route.clone() {
            writeln!(f, "{}, {}, {}", node.id, node.lat, node.lon)?;
        }
        Ok(())
    }
}

