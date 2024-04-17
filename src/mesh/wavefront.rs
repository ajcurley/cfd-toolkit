use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use flate2::read::GzDecoder;

use crate::mesh::io::{IOFace, IOMesh, IOPatch, IOVertex};

/// Import a WaveFront (OBJ) file. Both ASCII and compressed ASCII are
/// valid file formats.
pub fn import_obj(path: &str) -> std::io::Result<IOMesh> {
    let mut mesh = IOMesh::default();

    for line in read(path)?.lines() {
        let line = line.trim();
        let args = line.splitn(2, char::is_whitespace).collect::<Vec<&str>>();

        match args.first() {
            Some(&"v") => parse_vertex(&args[1], &mut mesh),
            Some(&"f") => parse_face(&args[1], &mut mesh),
            Some(&"g") => parse_group(&args[1], &mut mesh),
            _ => {}
        };
    }

    Ok(mesh)
}

/// Read the file contents to string
fn read(path: &str) -> std::io::Result<String> {
    let path = Path::new(path);
    let extension = path.extension().and_then(OsStr::to_str).unwrap();

    let mut contents = String::new();
    let mut file = File::open(&path)?;

    if extension.to_lowercase() == "gz" {
        let mut file = GzDecoder::new(file);
        file.read_to_string(&mut contents)?;
    } else {
        file.read_to_string(&mut contents)?;
    }

    Ok(contents)
}

/// Parse a vertex from a line entry
fn parse_vertex(line: &str, mesh: &mut IOMesh) {
    let values: Vec<f64> = line
        .split_whitespace()
        .map(|v| v.parse::<f64>().unwrap())
        .collect();

    assert_eq!(values.len(), 3);
    let vertex = IOVertex::from(&values);
    mesh.insert_vertex(vertex);
}

/// Parse a face from a line entry
fn parse_face(line: &str, mesh: &mut IOMesh) {
    let values: Vec<usize> = line
        .split_whitespace()
        .map(|v| v.splitn(2, "/").next().unwrap())
        .map(|v| v.parse::<usize>().unwrap() - 1)
        .collect();

    assert!(values.len() >= 3);
    let patch = mesh.latest_patch();
    let face = IOFace::new(values, patch);
    mesh.insert_face(face);
}

/// Parse a group (patch) from a line entry
fn parse_group(line: &str, mesh: &mut IOMesh) {
    let name = line.trim().to_string();
    let patch = IOPatch::new(name);
    mesh.insert_patch(patch);
}
