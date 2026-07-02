use std::{fs, io::Write, path::PathBuf};

use crate::route_points::RoutePoint;

/// Writes the route points to a GPX file by mapping the route points array to a vector of strings.
pub fn write_gpx_file(filepath: PathBuf, route_points: Vec<RoutePoint>) {
    println!("Writing GPX file: {}", filepath.to_path_buf().display());

    let mut file = fs::File::create(filepath).expect("Unable to create file");
    let mut content = String::new();

    content.push_str(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<gpx xmlns=\"http://www.topografix.com/GPX/1/1\" version=\"1.1\" creator=\"GPX Writer\">\n\t<rte>\n{}\n\t</rte>\n</gpx>",
        route_points
            .iter()
            .map(|point| format!("{point}"))
            .collect::<Vec<String>>()
            .join("\n")
    ));

    file.write_all(content.as_bytes())
        .expect("Unable to write data to file");

    println!("GPX file written successfully!");
}
