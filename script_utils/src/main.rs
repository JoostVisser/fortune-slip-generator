use std::fs::File;

use serde_json::Value;

fn main() {
    let jscpd_file = File::open("report/jscpd-report.json").unwrap();
    let jscpd_report: Value = serde_json::from_reader(jscpd_file).unwrap();
    let dupl_percentage = jscpd_report["statistics"]["total"]["percentage"]
        .as_f64()
        .unwrap();

    print!("DUPL={:.2}", dupl_percentage);
}
