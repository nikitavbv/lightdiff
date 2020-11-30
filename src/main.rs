#[macro_use]
extern crate tera;

use std::collections::HashMap;

use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use tera::{Tera, Context};
use std::fs;

#[derive(Deserialize)]
struct LighthouseReport {
    audits: HashMap<String, LighthouseAudit>
}

#[derive(Deserialize)]
struct LighthouseAudit {
    id: String,
    title: String,
    description: String,
    score: Option<f64>,
    score_display_mode: Option<String>,
    numeric_value: Option<f64>,
    numeric_unit: Option<String>,
    display_value: Option<String>
}

fn main() {
    println!("Hello, world!");

    load_report();
    template_diff();
}

fn load_report() {
    let mut file = File::open("example.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let report: LighthouseReport = serde_json::from_str(&data).unwrap();

    println!("loaded {} audits", report.audits.len());
}

fn template_diff() {
    let tera = Tera::new("templates/*").unwrap();

    let mut context = Context::new();
    context.insert("test", "42");

    let res = tera.render("base.html", &context).unwrap();

    fs::write("result.html", res).unwrap();
}