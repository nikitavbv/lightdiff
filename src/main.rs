use std::collections::HashMap;

struct LighthouseReport {
    audits: HashMap<String, LighthouseAudit>
}

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
}
