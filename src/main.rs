use std::collections::HashMap;

use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
struct LighthouseReport {
    audits: HashMap<String, LighthouseAudit>
}

#[derive(Deserialize)]
struct LighthouseAudit {
    id: String,
    title: String,
    description: String,
    score: f64,
    score_display_mode: String,
    numeric_value: Option<f64>,
    numeric_unit: Option<String>,
    display_value: Option<String>
}

fn main() {
    println!("Hello, world!");

    load_report();
}

fn load_report() {
    let mut file = File::open("example.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let report: LighthouseReport = serde_json::from_str(&data).unwrap();

    println!("loaded {} audits", report.audits.len());
}
