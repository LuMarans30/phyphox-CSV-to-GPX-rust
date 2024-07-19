# Phyphox CSV to GPX in Rust

![GitHub](https://img.shields.io/github/license/LuMarans30/phyphox-CSV-to-GPX-rust)
![GitHub repo size](https://img.shields.io/github/repo-size/LuMarans30/phyphox-CSV-to-GPX-rust)
![GitHub issues](https://img.shields.io/github/issues/LuMarans30/phyphox-CSV-to-GPX-rust)
![GitHub last commit](https://img.shields.io/github/last-commit/LuMarans30/phyphox-CSV-to-GPX-rust)

A simple Rust tool to convert CSV files containing route data into GPX format.

## Overview

This project aims to provide a straightforward way to transform Phyphox CSV files with route information into GPX files.

## Features

- Reads CSV files with specific route data columns
- Converts the data into GPX format
- Sorts route points by time
- Writes the result to a GPX file

## Usage

To use this tool, run it from the command line with two arguments:
cargo run <input_file.csv> <output_file.gpx>

Alternatively, [precompiled binaries for Windows](https://github.com/LuMarans30/phyphox-CSV-to-GPX-rust/releases/latest) are available.

## CSV Format

The tool expects the input CSV to have the following columns:

- Time (s)
- Latitude (°)
- Longitude (°)
- Altitude WGS84 (m)
- Speed (m/s)
- Direction (°)
- Horizontal Accuracy (m)
- Vertical Accuracy (m)
- Satellites

## Limitations

This is a basic tool and may not handle all edge cases or other CSV formats. It's been created for personal use and shared in case others find it helpful.

## Contributing

While this project is primarily for personal use, suggestions and improvements are welcome. Feel free to open an issue or submit a pull request if you have ideas to enhance its functionality.

## License

This project is open-source and available under the [MIT License](LICENSE).
