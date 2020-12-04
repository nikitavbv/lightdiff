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
    #[serde(rename="finalUrl")]
    final_url: String,
    audits: HashMap<String, LighthouseAudit>,
}

#[derive(Deserialize, Clone)]
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

struct LighthouseReportDiff {
    matched_audits: HashMap<String, (LighthouseAudit, LighthouseAudit)>,
    changed_audits: HashMap<String, (LighthouseAudit, LighthouseAudit)>
}

fn main() {
    println!("Hello, world!");

    let before = load_report("before.json");
    let after = load_report("after.json");

    let report_diff = report_diff(&before, &after);

    println!("found {} matching and {} changed audits", report_diff.matched_audits.len(), report_diff.changed_audits.len());

    template_diff(&before, &after, &report_diff);
}

fn report_diff(before: &LighthouseReport, after: &LighthouseReport) -> LighthouseReportDiff {
    let matched_audits: HashMap<String, (LighthouseAudit, LighthouseAudit)> = before.audits.iter()
        .filter(|v| after.audits.contains_key(v.0) && audit_matches(v.1, after.audits.get(v.0).unwrap()))
        .map(|v| (v.0.clone(), (v.1.clone(), after.audits.get(v.0).unwrap().clone())))
        .collect();

    let changed_audits: HashMap<String, (LighthouseAudit, LighthouseAudit)> = before.audits.iter()
        .filter(|v| after.audits.contains_key(v.0) && !audit_matches(v.1, after.audits.get(v.0).unwrap()))
        .map(|v| (v.0.clone(), (v.1.clone(), after.audits.get(v.0).unwrap().clone())))
        .collect();

    LighthouseReportDiff {
        matched_audits,
        changed_audits
    }
}

fn audit_matches(before: &LighthouseAudit, after: &LighthouseAudit) -> bool {
    before.score == after.score
}

fn load_report(file_name: &str) -> LighthouseReport {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    serde_json::from_str(&data).unwrap()
}

fn template_diff(before: &LighthouseReport, after: &LighthouseReport, diff: &LighthouseReportDiff) {
    let tera = Tera::new("templates/*").unwrap();

    let mut context = Context::new();
    context.insert("test", "42");

    let res = tera.render("base.html", &context).unwrap();

    fs::write("result.html", res).unwrap();
}