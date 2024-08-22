use std::fmt::Display;

/**
 * RoutePoint struct that represents a point in a route. <br>
 * The struct fields are derived from the CSV file headers. <br>
 * Example CSV file headers:
 * ```csv
 * "Latitude (°)","Longitude (°)","Altitude (m)","Altitude WGS84 (m)","Speed (m/s)","Direction (°)","Distance (km)","Horizontal Accuracy (m)","Vertical Accuracy (m)","Satellites"
 * ```
 */
#[derive(Debug, serde::Deserialize)]
pub struct RoutePoint {
    #[serde(rename = "Time (s)")]
    pub time: f64,
    #[serde(rename = "Latitude (°)")]
    pub latitude: f64,
    #[serde(rename = "Longitude (°)")]
    pub longitude: f64,
    #[serde(rename = "Altitude WGS84 (m)")]
    pub altitude_wgs84: f64,
    #[serde(rename = "Speed (m/s)")]
    pub speed: f64,
    #[serde(rename = "Direction (°)")]
    pub direction: f64,
    #[serde(rename = "Horizontal Accuracy (m)")]
    pub h_accuracy: f64,
    #[serde(rename = "Vertical Accuracy (m)")]
    pub v_accuracy: f64,
    #[serde(rename = "Satellites")]
    pub satellites: f64,
}

impl RoutePoint {
    /// Use serde to deserialize the CSV file content into a RoutePoint struct
    pub fn from_str(data: &str) -> Vec<RoutePoint> {
        let mut rdr = csv::Reader::from_reader(data.as_bytes());
        let mut route_points = Vec::<RoutePoint>::new();

        for result in rdr.deserialize::<RoutePoint>() {
            if let Ok(record) = result {
                route_points.push(record);
            } else {
                eprintln!("Error parsing record: {:?}", result);
            }
        }

        route_points
    }
}

impl Display for RoutePoint {
    /**
     * Formats the RoutePoint struct into a string that can be written to a GPX file. <br>
     * Example (from the GPX 1.1 Topografix schema): 
     * ```xml
     *   <rtept lat="latitude" lon="longitude">
     *       <ele>altitudeWgs84</ele>
     *       <speed>speed</speed>
     *       <course>direction</course>
     *       <sat>satellites</sat>
     *       <hdop>hAccuracy</hdop>
     *       <vdop>vAccuracy</vdop>
     *   </rtept>
     * ```
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\t\t<rtept lat=\"{:.8}\" lon=\"{:.8}\">\n\t\t\t<ele>{:.6}</ele>\n\t\t\t<speed>{:.9}</speed>\n\t\t\t<course>{:.7}</course>\n\t\t\t<sat>{}</sat>\n\t\t\t<hdop>{:.9}</hdop>\n\t\t\t<vdop>{:.9}</vdop>\n\t\t</rtept>",
            self.latitude,
            self.longitude,
            self.altitude_wgs84,
            self.speed,
            self.direction,
            self.satellites,
            self.h_accuracy,
            self.v_accuracy
        )
    }
}
