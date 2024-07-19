mod gpx_writer;
mod route_points;

use std::{fs, time::Instant};

use route_points::RoutePoint;

fn main() {
    //Args for input file and output file paths
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        3 => {
            let input_file = &args[1];
            let output_file = &args[2];

            if !(input_file.ends_with(".csv") && output_file.ends_with(".gpx")) {
                eprintln!("Input file must be a CSV file and output file must be a GPX file");
                std::process::exit(1);
            }

            // Read the contents of the CSV file
            println!("Parsing file: {}", input_file);
            let contents =
                fs::read_to_string(input_file).expect("Something went wrong reading the file");

            // Parse the CSV file into a vector of RoutePoint structs.
            // This works by deserializing the CSV file content into RoutePoint structs using serde.
            let start = Instant::now();
            let mut route_points: Vec<route_points::RoutePoint> = RoutePoint::from_str(&contents);
            let duration = start.elapsed();
            println!("Parsing took: {:?}", duration);

            // Sort the route points by time in ascending order
            route_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

            // Write the route points to a GPX file
            gpx_writer::write_gpx_file(output_file, route_points);
        }
        _ => {
            eprintln!("Usage: {} <input file> <output file>", args[0]);
            std::process::exit(1);
        }
    }
}
