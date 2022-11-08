use crate::graph::graphelements::Node;
use crate::graph::router::router::RouteResult;
use crate::reader::osmelements::Bbox;
use petgraph::graph::Graph;
use plotters::prelude::*;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::Index;
pub trait Draw<W>
where
    W: Debug + PartialOrd + Add<W, Output = W> + Default + Clone + Copy,
{
    fn draw_pic(
        &self,
        path: RouteResult<W>,
        bbox: Bbox,
        size: (u32, u32),
    ) -> Result<(), Box<dyn std::error::Error>>;
}
impl<W> Draw<W> for Graph<Node, W>
where
    W: Debug + PartialOrd + Add<W, Output = W> + Default + Clone + Copy,
{
    fn draw_pic(
        &self,
        path: RouteResult<W>,
        bbox: Bbox,
        size: (u32, u32),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut root = BitMapBackend::new("data/output.png", (size.0, size.1));
        root.draw_rect((0, 0), (size.0 as i32, size.1 as i32), &WHITE, true)?;
        for (_, node) in self.node_indices().enumerate() {
            for (_, target) in self.neighbors(node).enumerate() {
                let from = (self.index(node).lon, self.index(node).lat);
                let to = (self.index(target).lon, self.index(target).lat);
                let from = normalize(from, &bbox, size);
                let to = normalize(to, &bbox, size);
                root.draw_line(from, to, &BLUE.stroke_width(6))?;
            }
        }
        let source = (path.route[0].lon, path.route[0].lat);
        let target = (
            path.route[path.route.len() - 1].lon,
            path.route[path.route.len() - 1].lat,
        );
        let source = normalize(source, &bbox, size);
        let target = normalize(target, &bbox, size);
        root.draw_rect(
            (source.0 - 50, source.1 - 50),
            (source.0 + 50, source.1 + 50),
            &RED,
            true,
        )?;
        root.draw_circle((target.0, target.1), 45, &RED, true)?;
        root.present()?;
        for i in 0..path.route.len() - 1 {
            let from = (path.route[i].lon, path.route[i].lat);
            let to = (path.route[i + 1].lon, path.route[i + 1].lat);
            let from = normalize(from, &bbox, size);
            let to = normalize(to, &bbox, size);
            root.draw_line(from, to, &RED.stroke_width(15))?;
        }
        root.present()?;
        Ok(())
    }
}

fn normalize(p: (f64, f64), area: &Bbox, size: (u32, u32)) -> (i32, i32) {
    //todo! calculate Web Mercator projection
    let area_size = (area.right - area.left, area.top - area.bottom);
    let multiplier = (size.0 as f64 / area_size.0, size.1 as f64 / area_size.1);

    (
        ((p.0 - area.left) * multiplier.0) as i32,
        size.1 as i32 - ((p.1 - area.bottom) * multiplier.1) as i32,
    )
}
