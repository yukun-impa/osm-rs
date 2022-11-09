use crate::reader::osmparse::OSM;

use crate::reader::osmelements::*;
use std::fs::File;
use std::io::Write;
use xml::writer::{EmitterConfig, EventWriter, Result, XmlEvent};
fn start_document_writer<W: Write>(w: &mut EventWriter<W>) -> Result<()> {
    let event: XmlEvent = XmlEvent::StartDocument {
        version: xml::common::XmlVersion::Version10,
        encoding: Some("UTF-8"),
        standalone: (None),
    };
    w.write(event)
}

fn bbox_witer<W: Write>(w: &mut EventWriter<W>, bbox: &Bbox) -> Result<()> {
    let bbox_info = format!(
        "bounds minlat=\"{}\" minlon=\"{}\" maxlat=\"{}\" maxlon=\"{}\"",
        bbox.left, bbox.bottom, bbox.right, bbox.top
    );
    let startevent = XmlEvent::start_element(&*bbox_info);
    w.write(startevent)?;
    let endevent: XmlEvent = XmlEvent::end_element().into();
    w.write(endevent)?;
    Ok(())
}

fn node_witer<W: Write>(w: &mut EventWriter<W>, node: &Node) -> Result<()> {
    let node_info = format!(
        "node id=\"{}\" lat=\"{}\" lon=\"{}\" user=\"{}\" uid=\"{}\" version=\"{}\" changeset=\"{}\"",
        node.id, node.lat, node.lon, node.user, node.uid, node.version, node.changeset
    );
    let startevent = XmlEvent::start_element(&*node_info);
    w.write(startevent)?;
    for tag in &node.tags {
        tag_witer(w, tag)?;
    }
    let endevent: XmlEvent = XmlEvent::end_element().into();
    w.write(endevent)?;
    Ok(())
}

fn way_writer<W: Write>(w: &mut EventWriter<W>, way: &Way) -> Result<()> {
    let way_info = format!(
        "way id=\"{}\" user=\"{}\" uid=\"{}\" version=\"{}\" changeset=\"{}\" timestamp=\"{}\"",
        way.id, way.user, way.uid, way.version, way.changeset, way.timestamp
    );
    let startevent = XmlEvent::start_element(&*way_info);
    w.write(startevent)?;
    for tag in &way.tags {
        tag_witer(w, tag)?;
    }
    let endevent: XmlEvent = XmlEvent::end_element().into();
    w.write(endevent)?;
    Ok(())
}

fn relation_witer<W: Write>(w: &mut EventWriter<W>, relation: &Relation) -> Result<()> {
    let relation_info = format!(
        "relation id=\"{}\" user=\"{}\" uid=\"{}\" version=\"{}\" changeset=\"{}\" timestamp=\"{}\"",
        relation.id, relation.user, relation.uid, relation.version, relation.changeset, relation.timestamp
    );
    let startevent = XmlEvent::start_element(&*relation_info);
    w.write(startevent)?;
    for tag in &relation.tags {
        tag_witer(w, tag)?;
    }
    for member in &relation.members {
        member_witer(w, member)?;
    }
    //let endevent = XmlEvent::EndElement { name: Some(Name::from("relation")) }

    let endevent: XmlEvent = XmlEvent::end_element().into();
    w.write(endevent)?;

    Ok(())
}

fn tag_witer<W: Write>(w: &mut EventWriter<W>, tag: &Tag) -> Result<()> {
    let tag_info = format!("tag k=\"{}\" v=\"{}\"", tag.key, tag.value);
    let startevent = XmlEvent::start_element(&*tag_info);
    w.write(startevent)?;
    let endevent: XmlEvent = XmlEvent::end_element().into();
    w.write(endevent)?;
    Ok(())
}

fn member_witer<W: Write>(w: &mut EventWriter<W>, member: &Member) -> Result<()> {
    let member_type = match member.member_type {
        OsmElement::Node => "node",
        OsmElement::Way => "way",
        OsmElement::Relation => "relation",
    };
    let tag_info = format!(
        "member type=\"{}\" ref=\"{}\" role=\"{}\"",
        member_type, member.ref_id, member.role
    );
    let startevent = XmlEvent::start_element(&*tag_info);
    w.write(startevent)?;
    let endevent: XmlEvent = XmlEvent::end_element().into();
    w.write(endevent)?;
    Ok(())
}

impl OSM {
    pub fn save(&self, path: &str) -> Result<()> {
        let mut file = File::create(path).unwrap();
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&mut file);

        start_document_writer(&mut writer)?;
        self.osm_writer(&mut writer)?;

        Ok(())
    }

    fn osm_writer<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()> {
        bbox_witer(w, &self.bbox)?;
        for node in &self.nodes {
            node_witer(w, node)?;
        }

        for way in &self.ways {
            way_writer(w, way)?;
        }

        for relation in &self.relations {
            relation_witer(w, relation)?;
        }
        Ok(())
    }
}
