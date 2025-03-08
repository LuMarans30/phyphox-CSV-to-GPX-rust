mod gpx_writer;
mod route_points;

use std::{fs, path::PathBuf, time::Instant};

use route_points::RoutePoint;

use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input CSV file path
    #[clap(short, long, required = true)]
    input: PathBuf,

    /// Output GPX file path
    #[clap(short, long, required = true)]
    output: PathBuf,
}

/**
 * Main function that reads the input CSV file, parses the route points, sorts them by time, and writes them to a GPX file. <br>
 * The input CSV file path and output GPX file path are passed as command line arguments. <br>
 * Example usage:
 * ```sh
 * csv2gpx input.csv output.gpx
 * ```
 */
fn main() {
    let args = Args::parse();

    let (input_file, output_file) = (args.input, args.output);

    if !(input_file.extension().unwrap().eq_ignore_ascii_case("csv")
        && output_file.extension().unwrap().eq_ignore_ascii_case("gpx"))
    {
        eprintln!("Input file must be a CSV file and output file must be a GPX file");
        std::process::exit(1);
    }

    if !input_file.exists() {
        eprintln!("Input file does not exist");
        std::process::exit(1);
    }

    // Read the contents of the CSV file
    println!("Parsing file: {}", input_file.as_path().display());
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");

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
