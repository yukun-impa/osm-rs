use crate::router::router::RouteResult;
use std::fmt;
impl fmt::Display for RouteResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Route cost: {} meters", self.routecost)?;
        writeln!(f, "Route:")?;
        for node in self.route.clone() {
            writeln!(f, "{}, {}, {}", node.node_id, node.latitude, node.longitude)?;
        }
        Ok(())
    }
}