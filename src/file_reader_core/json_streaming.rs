use crate::_Graph;
use crate::graph_core::disk_graph::DiskGraph;
use crate::file_reader_core::file_reader::{JsonConnection, JsonNode};
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};


struct NodeSeqSeed<'a, S: crate::graph_core::graph_structure_interface::IGraphStructure + Default> {
    graph: &'a mut _Graph<S>,
}

impl<'de, 'a, S: crate::graph_core::graph_structure_interface::IGraphStructure + Default> DeserializeSeed<'de> for NodeSeqSeed<'a, S> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        return deserializer.deserialize_seq(self);
    }
}

impl<'de, 'a, S: crate::graph_core::graph_structure_interface::IGraphStructure + Default> Visitor<'de> for NodeSeqSeed<'a, S> {
    type Value = ();
    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return f.write_str("array of nodes");
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        while let Some(node) = seq.next_element::<JsonNode>()? {
            if let (Some(x), Some(y)) = (node.x, node.y) {
                self.graph.add_node_with_pos(node.label, x, y);
            } else {
                self.graph.add_node(node.label);
            }
        }
        return Ok(());
    }
}

struct ConnSeqSeed<'a, S: crate::graph_core::graph_structure_interface::IGraphStructure + Default> {
    graph: &'a mut _Graph<S>,
    directed: bool,
}

impl<'de, 'a, S: crate::graph_core::graph_structure_interface::IGraphStructure + Default> DeserializeSeed<'de> for ConnSeqSeed<'a, S> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        return deserializer.deserialize_seq(self);
    }
}

impl<'de, 'a, S: crate::graph_core::graph_structure_interface::IGraphStructure + Default> Visitor<'de> for ConnSeqSeed<'a, S> {
    type Value = ();
    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return f.write_str("array of connections");
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        while let Some(conn) = seq.next_element::<JsonConnection>()? {
            self.graph.create_connection(conn.source, conn.target, conn.weight, Some(self.directed));
        }
        return Ok(());
    }
}

struct GraphSeed<'a, S: crate::graph_core::graph_structure_interface::IGraphStructure + Default> {
    graph: &'a mut _Graph<S>,
}

impl<'de, 'a, S: crate::graph_core::graph_structure_interface::IGraphStructure + Default> DeserializeSeed<'de> for GraphSeed<'a, S> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        return deserializer.deserialize_map(self);
    }
}

impl<'de, 'a, S: crate::graph_core::graph_structure_interface::IGraphStructure + Default> Visitor<'de> for GraphSeed<'a, S> {
    type Value = ();
    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return f.write_str("graph JSON object");
    }
    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<(), M::Error> {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "nodes" => map.next_value_seed(NodeSeqSeed { graph: self.graph })?,
                "edges" => map.next_value_seed(ConnSeqSeed { graph: self.graph, directed: false })?,
                "arcs"  => map.next_value_seed(ConnSeqSeed { graph: self.graph, directed: true })?,
                _       => { map.next_value::<serde_json::Value>()?; }
            }
        }
        return Ok(());
    }
}


pub fn read_json_file_streaming<S: crate::graph_core::graph_structure_interface::IGraphStructure + Default>(file_path: &str) -> Result<_Graph<S>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut graph = _Graph::default();

    let mut de = serde_json::Deserializer::from_reader(reader);
    GraphSeed { graph: &mut graph }
        .deserialize(&mut de)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    return Ok(graph);
}

pub fn read_json_file_streaming_disk(file_path: &str) -> Result<_Graph<DiskGraph>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut graph = _Graph::<DiskGraph>::default();

    let mut de = serde_json::Deserializer::from_reader(reader);
    GraphSeed { graph: &mut graph }
        .deserialize(&mut de)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    graph.structure.flush_to_disk();

    return Ok(graph);
}
