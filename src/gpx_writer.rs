use std::{fs, io::Write};

use crate::route_points::RoutePoint;

pub fn write_gpx_file(filepath: &str, route_points: Vec<RoutePoint>) {
    println!("Writing GPX file: {}", filepath);
    //TODO: Implement writing logic here
    let mut file = fs::File::create(filepath).expect("Unable to create file");
    let mut content = String::new();

    content.push_str(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
    <gpx xmlns=\"http://www.topografix.com/GPX/1/1\" version=\"1.1\" creator=\"GPX Writer\">
        <rte>
            {}
        </rte>
    </gpx>",
        route_points
            .iter()
            .map(|route_point| route_point.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    ));

    file.write_all(content.as_bytes())
        .expect("Unable to write data to file");

    println!("GPX file written successfully!");
}
