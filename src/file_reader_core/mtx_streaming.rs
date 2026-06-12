use crate::_Graph;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};


struct MtxHeader {
    is_pattern: bool,
    directed: bool,
}

fn parse_mtx_header(header_line: &str) -> Result<MtxHeader, Error> {
    let lower = header_line.to_lowercase();
    if !lower.starts_with("%%matrixmarket") {
        return Err(Error::new(ErrorKind::InvalidData, "Not a MatrixMarket file: missing %%MatrixMarket header"));
    }

    let parts: Vec<&str> = header_line.split_whitespace().collect();

    let format = parts.get(2).map(|s| s.to_lowercase()).unwrap_or_else(|| "coordinate".to_string());
    if format != "coordinate" {
        return Err(Error::new(ErrorKind::InvalidData, "Only 'coordinate' MatrixMarket format is supported"));
    }

    let type_field = parts.get(3).map(|s| s.to_lowercase()).unwrap_or_else(|| "real".to_string());
    let is_pattern = type_field == "pattern";

    let symmetry = parts.get(4).map(|s| s.to_lowercase()).unwrap_or_else(|| "general".to_string());
    let directed = symmetry == "general" || symmetry == "skew-symmetric";

    return Ok(MtxHeader { is_pattern, directed });
}

fn parse_mtx_size_line(line: &str) -> Result<(usize, usize), Error> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid MatrixMarket size line"));
    }
    let n_rows: usize = parts[0].parse()
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid row count in size line"))?;
    let n_cols: usize = parts[1].parse()
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid col count in size line"))?;
    return Ok((n_rows, n_cols));
}

pub fn read_mtx_file_streaming<S: crate::graph_core::graph_structure_interface::IGraphStructure + Default>(
    file_path: &str,
) -> Result<_Graph<S>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut graph = _Graph::default();

    let mut lines = reader.lines();


    let header_line = lines
        .next()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Empty .mtx file"))??;
    let header = parse_mtx_header(&header_line)?;


    let mut size_line = String::new();
    for raw in lines.by_ref() {
        let line = raw?;
        let trimmed = line.trim().to_string();
        if trimmed.starts_with('%') || trimmed.is_empty() {
            continue;
        }
        size_line = trimmed;
        break;
    }

    if size_line.is_empty() {
        return Err(Error::new(ErrorKind::InvalidData, "Missing size line in .mtx file"));
    }

    let (n_rows, n_cols) = parse_mtx_size_line(&size_line)?;
    let n_nodes = n_rows.max(n_cols);

    for i in 1..=n_nodes {
        graph.add_node(i.to_string());
    }


    for raw in lines {
        let line = raw?;
        let trimmed = line.trim();
        if trimmed.starts_with('%') || trimmed.is_empty() {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let row: usize = parts[0].parse()
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid row index in edge line"))?;
        let col: usize = parts[1].parse()
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid col index in edge line"))?;

        let weight: f32 = if header.is_pattern || parts.len() < 3 {
            1.0
        } else {
            parts[2].parse()
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid weight in edge line"))?
        };

        graph.create_connection(row.to_string(), col.to_string(), weight, Some(header.directed));
    }

    return Ok(graph);
}
