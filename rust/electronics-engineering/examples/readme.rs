//! Electronics Engineering: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p electronics-engineering-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&electronics_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "elx_butterworth_section_q", "inputs": {"n": "4"}}));
    println!();
    show(json!({"key": "elx_transition_frequency", "inputs": {"gm": "40 mS", "beta": "100", "Cpi": "10 pF", "Cmu": "2 pF"}}));
    println!();
    show(json!({"key": "elx_cmos_propagation_delay", "inputs": {"Rn": "10000 ohm", "Rp": "20000 ohm", "CL": "100 fF"}}));
    println!();
    show(json!({"key": "elx_adc_code", "inputs": {"Vin": "3.2 V", "Vref": "5 V", "n": "8"}}));
    println!();
    show(json!({"key": "elx_capacitor_filter_ripple", "inputs": {"Vp": "16.3 V", "f": "60 Hz", "RL": "500 ohm", "C": "1000 uF", "circuit": "full-wave"}}));
    println!();
    show(json!({"key": "elx_common_source_amplifier", "inputs": {"gm": "2 mS", "ro": "100000 ohm", "RD": "10000 ohm", "RL": "10000 ohm", "RS": "0 ohm", "RG": "1000000 ohm", "Rsig": "10000 ohm"}}));
    println!();
    show(json!({"key": "elx_closed_loop_bandwidth", "inputs": {"GBW": "1000000 Hz", "config": "non-inverting", "Rf": "90000 ohm", "R1": "10000 ohm"}}));
    println!();
    show(json!({"key": "elx_555_astable", "inputs": {"RA": "1000 ohm", "RB": "10000 ohm", "C": "10 uF"}}));
    println!();
    show(json!({"key": "elx_adjustable_regulator_output", "inputs": {"R1": "240 ohm", "R2": "720 ohm", "Vref": "1.25 V"}}));
    println!();
    show(json!({"key": "elx_builtin_potential", "inputs": {"NA": "1e+17 cm^-3", "ND": "1e+16 cm^-3", "ni": "1.5e+10 cm^-3", "T": "300 K"}}));
    println!();
    show(json!({"key": "elx_cmos_dynamic_power", "inputs": {"alpha": "0.1", "C": "1000 pF", "VDD": "1 V", "f": "1000000000 Hz"}}));
    println!();
    show(json!({"key": "elx_filter_order", "inputs": {"ftype": "Butterworth", "fp": "1000 Hz", "fs": "2500 Hz", "Ap": "1", "As": "40"}}));
    println!();
    show(json!({"key": "elx_bjt_cascode", "inputs": {"IC": "1 mA", "beta": "100", "VT": "25 mV", "VA": "100 V", "RL": "1000000 ohm"}}));
    println!();
    show(json!({"key": "elx_cmos_switching_threshold", "inputs": {"VDD": "2.5 V", "Vtn": "0.4 V", "Vtp": "0.4 V", "kn": "0.1 mA/V^2", "kp": "0.1 mA/V^2"}}));
    println!();
    show(json!({"key": "elx_adc_quantization_snr", "inputs": {"n": "12"}}));
}
