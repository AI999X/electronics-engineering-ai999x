//! electronics_engineering: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/electronics_engineering/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "elx_555_astable",
    "elx_555_monostable",
    "elx_adc_code",
    "elx_adc_enob",
    "elx_adc_quantization_snr",
    "elx_adjustable_regulator_output",
    "elx_aperture_jitter_snr",
    "elx_bjt_cascode",
    "elx_bjt_collector_feedback_bias",
    "elx_bjt_current_relations",
    "elx_bjt_emitter_bias",
    "elx_bjt_fixed_bias",
    "elx_bjt_small_signal_parameters",
    "elx_bjt_voltage_divider_bias",
    "elx_builtin_potential",
    "elx_butterworth_section_q",
    "elx_capacitor_filter_ripple",
    "elx_chebyshev_sections",
    "elx_closed_loop_bandwidth",
    "elx_cmos_dynamic_power",
    "elx_cmos_propagation_delay",
    "elx_cmos_switching_threshold",
    "elx_colpitts_oscillator",
    "elx_common_base_amplifier",
    "elx_common_collector_amplifier",
    "elx_common_emitter_amplifier",
    "elx_common_source_amplifier",
    "elx_crystal_oscillator",
    "elx_current_mirror",
    "elx_dac_binary_weighted",
    "elx_dac_output",
    "elx_depletion_width",
    "elx_difference_amplifier",
    "elx_differential_pair",
    "elx_diode_operating_point",
    "elx_diode_voltage",
    "elx_doped_carrier_concentrations",
    "elx_drift_current",
    "elx_dual_slope_adc",
    "elx_einstein_diffusion",
    "elx_elmore_delay",
    "elx_filter_order",
    "elx_first_order_active_lowpass",
    "elx_hartley_oscillator",
    "elx_heat_sink_required",
    "elx_heat_sink_temperatures",
    "elx_instrumentation_amplifier",
    "elx_intrinsic_carrier_concentration",
    "elx_inverting_amplifier",
    "elx_inverting_amplifier_finite_gain",
    "elx_junction_capacitance",
    "elx_led_series_resistor",
    "elx_linear_regulator_efficiency",
    "elx_low_frequency_cutoff",
    "elx_metastability_mtbf",
    "elx_mfb_bandpass",
    "elx_mfb_lowpass",
    "elx_miller_high_cutoff",
    "elx_mosfet_body_effect",
    "elx_mosfet_divider_bias",
    "elx_mosfet_drain_current",
    "elx_mosfet_gm_ro",
    "elx_negative_feedback",
    "elx_noninverting_amplifier",
    "elx_opamp_dc_offset",
    "elx_opamp_differentiator",
    "elx_opamp_integrator",
    "elx_opamp_relaxation_oscillator",
    "elx_pll_loop_dynamics",
    "elx_rc_delay_rise_time",
    "elx_rc_phase_shift_oscillator",
    "elx_rectifier_averages",
    "elx_sallen_key_equal_component_design",
    "elx_sallen_key_highpass",
    "elx_sallen_key_lowpass",
    "elx_sample_hold_droop",
    "elx_sar_conversion",
    "elx_schmitt_trigger",
    "elx_semiconductor_conductivity",
    "elx_series_pass_regulator",
    "elx_setup_hold_timing",
    "elx_shockley_diode_current",
    "elx_slew_rate_limit",
    "elx_source_follower",
    "elx_summing_amplifier",
    "elx_thermal_voltage",
    "elx_transition_frequency",
    "elx_two_pole_phase_margin",
    "elx_wien_bridge_oscillator",
    "elx_wilson_current_mirror",
    "elx_zener_regulator",
    "elx_zener_series_resistor_design",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "elx_555_astable" => {
            let r = f_timer_astable(match v.get("RA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_19(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_555_monostable" => {
            let r = f_timer_monostable(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_63(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_adc_code" => {
            let r = f_adc_code(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_123(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_adc_enob" => {
            let r = f_adc_enob(match v.get("SINAD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_168(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_adc_quantization_snr" => {
            let r = f_adc_snr(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("OSR") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("A_frac") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_215(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_adjustable_regulator_output" => {
            let r = f_adj_regulator(match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Iadj") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_265(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_aperture_jitter_snr" => {
            let r = f_jitter_snr(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tj") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_313(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_bjt_cascode" => {
            let r = f_cascode(match v.get("IC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_359(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_bjt_collector_feedback_bias" => {
            let r = f_collector_feedback(match v.get("VCC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RF") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VBE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RE") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_413(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_bjt_current_relations" => {
            let r = f_bjt_currents(match v.get("IB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_457(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_bjt_emitter_bias" => {
            let r = f_emitter_bias(match v.get("VCC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VEE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VBE") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_505(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_bjt_fixed_bias" => {
            let r = f_fixed_bias(match v.get("VCC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VBE") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_553(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_bjt_small_signal_parameters" => {
            let r = f_bjt_small_signal(match v.get("IC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VA") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_597(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_bjt_voltage_divider_bias" => {
            let r = f_divider_bias(match v.get("VCC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VBE") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_646(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_builtin_potential" => {
            let r = f_builtin_potential(match v.get("NA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ND") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_695(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_butterworth_section_q" => {
            let r = f_butterworth_sections(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_739(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_capacitor_filter_ripple" => {
            let r = f_filter_ripple(match v.get("Vp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("circuit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_789(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_chebyshev_sections" => {
            let r = f_chebyshev_sections(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ripple") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_837(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_closed_loop_bandwidth" => {
            let r = f_closed_loop_bw(match v.get("GBW") { Some(x) => py::from_val(x), None => { return None } }, match v.get("config") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_885(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_cmos_dynamic_power" => {
            let r = f_cmos_power(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VDD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_932(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_cmos_propagation_delay" => {
            let r = f_cmos_delay(match v.get("Rn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_977(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_cmos_switching_threshold" => {
            let r = f_cmos_threshold(match v.get("VDD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vtn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vtp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kp") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1023(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_colpitts_oscillator" => {
            let r = f_colpitts(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1067(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_common_base_amplifier" => {
            let r = f_cb_amplifier(match v.get("IC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsig") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1113(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_common_collector_amplifier" => {
            let r = f_cc_amplifier(match v.get("IC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsig") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1159(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_common_emitter_amplifier" => {
            let r = f_ce_amplifier(match v.get("IC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RE1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsig") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1211(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_common_source_amplifier" => {
            let r = f_cs_amplifier(match v.get("gm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ro") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RG") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsig") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1262(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_crystal_oscillator" => {
            let r = f_crystal(match v.get("Lm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1306(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_current_mirror" => {
            let r = f_current_mirror(match v.get("VCC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VBE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1352(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_dac_binary_weighted" => {
            let r = f_dac_binary_weighted(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("code") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1404(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_dac_output" => {
            let r = f_dac_output(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("code") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vref") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1456(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_depletion_width" => {
            let r = f_depletion(match v.get("NA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ND") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vbi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps_r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1505(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_difference_amplifier" => {
            let r = f_difference_amp(match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R3") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R4") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1554(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_differential_pair" => {
            let r = f_diff_pair(match v.get("I_tail") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("REE") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1598(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_diode_operating_point" => {
            let r = f_diode_operating_point(match v.get("VS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Is") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1650(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_diode_voltage" => {
            let r = f_diode_voltage(match v.get("ID") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Is") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1699(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_doped_carrier_concentrations" => {
            let r = f_doped_carriers(match v.get("ND") { Some(x) => py::from_val(x), None => { return None } }, match v.get("NA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ni") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1757(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_drift_current" => {
            let r = f_drift_current(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1804(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_dual_slope_adc" => {
            let r = f_dual_slope(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fclk") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1853(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_einstein_diffusion" => {
            let r = f_einstein(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1897(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_elmore_delay" => {
            let r = f_elmore(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1956(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_filter_order" => {
            let r = f_filter_order(match v.get("ftype") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ap") { Some(x) => py::from_val(x), None => { return None } }, match v.get("As") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2008(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_first_order_active_lowpass" => {
            let r = f_first_order_lp(match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2052(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_hartley_oscillator" => {
            let r = f_hartley(match v.get("L1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2104(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_heat_sink_required" => {
            let r = f_heat_sink_required(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tj_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta_jc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta_cs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2150(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_heat_sink_temperatures" => {
            let r = f_heat_sink(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta_jc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta_cs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta_sa") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2194(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_instrumentation_amplifier" => {
            let r = f_inamp(match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RG") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R3") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2241(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_intrinsic_carrier_concentration" => {
            let r = f_intrinsic_concentration(match v.get("Nc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Eg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2286(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_inverting_amplifier" => {
            let r = f_inverting_amp(match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2333(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_inverting_amplifier_finite_gain" => {
            let r = f_finite_gain_inverting(match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2380(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_junction_capacitance" => {
            let r = f_junction_capacitance(match v.get("Cj0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vbi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2432(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_led_series_resistor" => {
            let r = f_led_resistor(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("If") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2481(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_linear_regulator_efficiency" => {
            let r = f_linear_regulator(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Iload") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Iq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2533(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_low_frequency_cutoff" => {
            let r = f_sctc_low_cutoff(match v.get("Rsig") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r_pi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CE") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2579(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_metastability_mtbf" => {
            let r = f_mtbf(match v.get("tr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fclk") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fdata") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2625(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_mfb_bandpass" => {
            let r = f_mfb_bandpass(match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R3") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2669(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_mfb_lowpass" => {
            let r = f_mfb_lowpass(match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R3") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2713(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_miller_high_cutoff" => {
            let r = f_miller_octc(match v.get("gm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r_pi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cpi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cmu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RLp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsigp") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2758(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_mosfet_body_effect" => {
            let r = f_body_effect(match v.get("Vt0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi2F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VSB") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2808(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_mosfet_divider_bias" => {
            let r = f_mosfet_divider_bias(match v.get("VDD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vt") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2862(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_mosfet_drain_current" => {
            let r = f_mosfet_id(match v.get("kn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VGS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VDS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2919(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_mosfet_gm_ro" => {
            let r = f_mosfet_gm_ro(match v.get("kn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ID") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vt") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2963(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_negative_feedback" => {
            let r = f_neg_feedback(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3007(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_noninverting_amplifier" => {
            let r = f_noninverting_amp(match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3054(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_opamp_dc_offset" => {
            let r = f_opamp_offset(match v.get("Vos") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ios") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rp") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3109(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_opamp_differentiator" => {
            let r = f_differentiator(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3153(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_opamp_integrator" => {
            let r = f_integrator(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3200(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_opamp_relaxation_oscillator" => {
            let r = f_relaxation_oscillator(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3244(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_pll_loop_dynamics" => {
            let r = f_pll(match v.get("Kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ko") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3288(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_rc_delay_rise_time" => {
            let r = f_rc_timing(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3332(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_rc_phase_shift_oscillator" => {
            let r = f_rc_phase_shift(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ladder") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3380(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_rectifier_averages" => {
            let r = f_rectifier(match v.get("Vm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("circuit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Von") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3439(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_sallen_key_equal_component_design" => {
            let r = f_sk_equal_design(match v.get("fc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3486(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_sallen_key_highpass" => {
            let r = f_sk_highpass(match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3532(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_sallen_key_lowpass" => {
            let r = f_sk_lowpass(match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3578(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_sample_hold_droop" => {
            let r = f_sample_hold(match v.get("I_leak") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_hold") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3626(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_sar_conversion" => {
            let r = f_sar_conversion(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3688(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_schmitt_trigger" => {
            let r = f_schmitt(match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VsatH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VsatL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vref") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3737(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_semiconductor_conductivity" => {
            let r = f_conductivity(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3789(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_series_pass_regulator" => {
            let r = f_series_pass(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VBE") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3838(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_setup_hold_timing" => {
            let r = f_setup_hold(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tcq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tpd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tsu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tskew") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tccq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tcd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("th") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3905(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_shockley_diode_current" => {
            let r = f_shockley(match v.get("Is") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3954(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_slew_rate_limit" => {
            let r = f_slew_rate(match v.get("SR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4000(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_source_follower" => {
            let r = f_source_follower(match v.get("gm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ro") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RG") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsig") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4045(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_summing_amplifier" => {
            let r = f_summing_amp(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rf") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4099(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_thermal_voltage" => {
            let r = f_thermal_voltage(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4143(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_transition_frequency" => {
            let r = f_transition_frequency(match v.get("gm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cpi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cmu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4187(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_two_pole_phase_margin" => {
            let r = f_phase_margin(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4233(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_wien_bridge_oscillator" => {
            let r = f_wien_bridge(match v.get("R1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4277(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_wilson_current_mirror" => {
            let r = f_wilson_mirror(match v.get("Iref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4321(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_zener_regulator" => {
            let r = f_zener_regulator(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rz") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4375(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elx_zener_series_resistor_design" => {
            let r = f_zener_design(match v.get("Vin_min") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL_min") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IL_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Iz_min") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4432(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_EPS0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.8541878128e-12)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_K_B() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.380649e-23)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LN2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.6931471805599453)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LN3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.0986122886681098)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_Q_E() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.602176634e-19)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_T0C() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(273.15)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_TWO_PI() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.283185307179586)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__RECT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("half-wave"), Py::str("bridge"), Py::str("center-tapped")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__active(mut v_VCE: Py) -> R {
    let _ = f__need({ let t632 = v_VCE.clone(); Py::Bool({ let t633 = Py::Float(0.2); if py::ge(&t632, &t633)? { true } else { false } }) }, Py::str("the transistor is saturated (VCE below 0.2 V): the active-region bias equations do not apply"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__beta(mut v_beta: Py) -> R {
    let _ = f__pos(Py::str("beta"), Py::tuple(vec![v_beta.clone()]))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bits(mut v_code: Py, mut v_n: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = Py::str("");
    v_i = py::sub(&v_n.clone(), &Py::Int(1i64))?;
    while ({ let t3686 = v_i.clone(); Py::Bool({ let t3687 = Py::Int(0i64); if py::ge(&t3686, &t3687)? { true } else { false } }) }).truthy() {
        v_s = py::iadd(&v_s, &(if ({ let t3684 = py::modulo(&py::floordiv(&v_code.clone(), &py::pow(&Py::Int(2i64), &v_i.clone())?)?, &Py::Int(2i64))?; Py::Bool({ let t3685 = Py::Int(1i64); if py::eq(&t3684, &t3685) { true } else { false } }) }).truthy() { Py::str("1") } else { Py::str("0") }))?;
        v_i = py::sub(&v_i, &Py::Int(1i64))?;
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__db(mut v_x: Py) -> R {
    return Ok(py::mul(&Py::Float(20.0), &py::math1("log10", &py::abs(&v_x.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4083 in py::iter(&(v_vals.clone()))? {
        v_v = t4083;
        let _ = f__need({ let t4085 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4085.truthy() { t4085 } else { { let t4084 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4084.truthy() { t4084 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str("every input must be a finite number"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hybrid(mut v_IC: Py, mut v_beta: Py, mut v_VT: Py) -> R {
    let mut v_gm: Py = Py::None;
    let _ = f__pos(Py::str("IC, beta and VT"), Py::tuple(vec![v_IC.clone(), v_beta.clone(), v_VT.clone()]))?;
    v_gm = py::truediv(&v_IC.clone(), &v_VT.clone())?;
    return Ok(Py::tuple(vec![v_gm.clone(), py::truediv(&v_beta.clone(), &v_gm.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4407 in py::iter(&(v_vals.clone()))? {
        v_v = t4407;
        let _ = f__need({ let t4412 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4412.truthy() { t4412 } else { { let t4411 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4411.truthy() { t4411 } else { { let t4410 = py::math1("isfinite", &v_v.clone())?; if !t4410.truthy() { t4410 } else { { let t4408 = v_v.clone(); Py::Bool({ let t4409 = Py::Int(0i64); if py::ge(&t4408, &t4409)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t4420 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4421 = (t4420).items()?;
if t4421.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4421[0].clone();
v_v = t4421[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__par(mut v_rs: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_s = Py::Float(0.0);
    for t4032 in py::iter(&(v_rs.clone()))? {
        v_x = t4032;
        v_s = py::iadd(&v_s, &py::truediv(&Py::Float(1.0), &v_x.clone())?)?;
    }
    return Ok(py::truediv(&Py::Float(1.0), &v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4401 in py::iter(&(v_vals.clone()))? {
        v_v = t4401;
        let _ = f__need({ let t4406 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4406.truthy() { t4406 } else { { let t4405 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4405.truthy() { t4405 } else { { let t4404 = py::math1("isfinite", &v_v.clone())?; if !t4404.truthy() { t4404 } else { { let t4402 = v_v.clone(); Py::Bool({ let t4403 = Py::Int(0i64); if py::gt(&t4402, &t4403)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t4425 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t4425.truthy() { t4425 } else { { let t4424 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t4424.truthy() { t4424 } else { { let t4422 = v_x.clone(); Py::Bool({ let t4423 = Py::None; if py::is(&t4422, &t4423) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t4426 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4426;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t4429 = { let t4427 = v_x.clone(); Py::Bool({ let t4428 = Py::Int(0i64); if py::eq(&t4427, &t4428) { true } else { false } }) }; if t4429.truthy() { t4429 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t4430 = v_x.clone(); Py::Bool({ let t4431 = Py::Int(0i64); if py::eq(&t4430, &t4431) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t4438 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t4438.truthy() { t4438 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t4442 = py::not(&{ let t4439 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t4439.truthy() { t4439 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t4442.truthy() { t4442 } else { { let t4440 = py::len(&v_a.clone())?; Py::Bool({ let t4441 = py::len(&v_b.clone())?; if py::ne(&t4440, &t4441) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t4443 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t4444 = (t4443).items()?;
if t4444.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t4444[0].clone();
v_y = t4444[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t4445 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t4445.truthy() { t4445 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t4446 = v_a.clone(); Py::Bool({ let t4447 = v_b.clone(); if py::eq(&t4446, &t4447) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t4449: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t4448 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t4448.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4448[0].clone();
            v_b = t4448[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4449 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t4450 = v_d.clone(); Py::Bool({ let t4451 = Py::Int(0i64); if py::eq(&t4450, &t4451) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4456 = { let t4452 = py::abs(&v_a.clone())?; Py::Bool({ let t4453 = Py::Float(1e-15); if py::lt(&t4452, &t4453)? { true } else { false } }) }; if !t4456.truthy() { t4456 } else { { let t4454 = py::abs(&v_b.clone())?; Py::Bool({ let t4455 = Py::Float(1e-15); if py::lt(&t4454, &t4455)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vt(mut v_T: Py) -> R {
    return Ok(py::truediv(&py::mul(&c_K_B()?, &v_T.clone())?, &c_Q_E()?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_adc_code(mut v_Vin: Py, mut v_Vref: Py, mut v_n: Py) -> R {
    let mut v_code: Py = Py::None;
    let mut v_lsb: Py = Py::None;
    let _ = f__pos(Py::str("Vref"), Py::tuple(vec![v_Vref.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_Vin.clone()]))?;
    let _ = f__need({ let t101 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t101.truthy() { t101 } else { { let t98 = Py::Int(1i64); Py::Bool({ let t99 = v_n.clone(); if py::le(&t98, &t99)? { { let t100 = Py::Int(16i64); if py::le(&t99, &t100)? { true } else { false } } } else { false } }) } } }, Py::str("the resolution must be a whole number of bits from 1 to 16"))?;
    let _ = f__need({ let t102 = Py::Float(0.0); Py::Bool({ let t103 = v_Vin.clone(); if py::le(&t102, &t103)? { { let t104 = v_Vref.clone(); if py::le(&t103, &t104)? { true } else { false } } } else { false } }) }, Py::str("the input must lie between 0 and Vref (unipolar converter)"))?;
    v_lsb = py::truediv(&v_Vref.clone(), &py::pow(&Py::Int(2i64), &v_n.clone())?)?;
    v_code = py::int(&py::math1("floor", &py::truediv(&py::mul(&v_Vin.clone(), &py::pow(&Py::Int(2i64), &v_n.clone())?)?, &v_Vref.clone())?)?)?;
    if ({ let t105 = v_code.clone(); Py::Bool({ let t106 = py::sub(&py::pow(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?; if py::gt(&t105, &t106)? { true } else { false } }) }).truthy() {
        v_code = py::sub(&py::pow(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("code"), v_code.clone()), (Py::str("bits"), f__bits(v_code.clone(), v_n.clone())?), (Py::str("LSB"), v_lsb.clone()), (Py::str("V_code"), py::mul(&v_code.clone(), &v_lsb.clone())?), (Py::str("error"), py::sub(&v_Vin.clone(), &py::mul(&v_code.clone(), &v_lsb.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_adc_enob(mut v_SINAD: Py, mut v_n: Py) -> R {
    let mut v_e: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_SINAD.clone()]))?;
    let _ = f__need({ let t155 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t155.truthy() { t155 } else { { let t152 = Py::Int(1i64); Py::Bool({ let t153 = v_n.clone(); if py::le(&t152, &t153)? { { let t154 = Py::Int(32i64); if py::le(&t153, &t154)? { true } else { false } } } else { false } }) } } }, Py::str("the nominal resolution must be a whole number of bits from 1 to 32"))?;
    v_e = py::truediv(&py::sub(&v_SINAD.clone(), &Py::Float(1.76))?, &Py::Float(6.02))?;
    return Ok(f__out(Py::dict(vec![(Py::str("ENOB"), v_e.clone()), (Py::str("bits_lost"), py::sub(&v_n.clone(), &v_e.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_adc_snr(mut v_n: Py, mut v_OSR: Py, mut v_A_frac: Py) -> R {
    let mut v_snr: Py = Py::None;
    let _ = f__need({ let t197 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t197.truthy() { t197 } else { { let t194 = Py::Int(1i64); Py::Bool({ let t195 = v_n.clone(); if py::le(&t194, &t195)? { { let t196 = Py::Int(32i64); if py::le(&t195, &t196)? { true } else { false } } } else { false } }) } } }, Py::str("the resolution must be a whole number of bits from 1 to 32"))?;
    let _ = f__need({ let t198 = v_OSR.clone(); Py::Bool({ let t199 = Py::Float(1.0); if py::ge(&t198, &t199)? { true } else { false } }) }, Py::str("the oversampling ratio must be at least 1"))?;
    let _ = f__need({ let t200 = Py::Float(0.0); Py::Bool({ let t201 = v_A_frac.clone(); if py::lt(&t200, &t201)? { { let t202 = Py::Float(1.0); if py::le(&t201, &t202)? { true } else { false } } } else { false } }) }, Py::str("the amplitude fraction must be in (0, 1] of full scale"))?;
    v_snr = py::add(&py::mul(&Py::Float(20.0), &py::math1("log10", &py::mul(&py::mul(&v_A_frac.clone(), &py::pow(&Py::Float(2.0), &v_n.clone())?)?, &py::math1("sqrt", &Py::Float(1.5))?)?)?)?, &py::mul(&Py::Float(10.0), &py::math1("log10", &v_OSR.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("SNR_dB"), v_snr.clone()), (Py::str("noise_rms_lsb"), py::truediv(&Py::Float(1.0), &py::math1("sqrt", &py::mul(&Py::Float(12.0), &v_OSR.clone())?)?)?), (Py::str("SNR_full_scale_dB"), py::mul(&Py::Float(20.0), &py::math1("log10", &py::mul(&py::pow(&Py::Float(2.0), &v_n.clone())?, &py::math1("sqrt", &Py::Float(1.5))?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_adj_regulator(mut v_R1: Py, mut v_R2: Py, mut v_Vref: Py, mut v_Iadj: Py) -> R {
    let _ = f__pos(Py::str("R1, R2 and Vref"), Py::tuple(vec![v_R1.clone(), v_R2.clone(), v_Vref.clone()]))?;
    let _ = f__nonneg(Py::str("Iadj"), Py::tuple(vec![v_Iadj.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vout"), py::add(&py::mul(&v_Vref.clone(), &py::add(&Py::Float(1.0), &py::truediv(&v_R2.clone(), &v_R1.clone())?)?)?, &py::mul(&v_Iadj.clone(), &v_R2.clone())?)?), (Py::str("I_R1"), py::truediv(&v_Vref.clone(), &v_R1.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bjt_currents(mut v_IB: Py, mut v_beta: Py) -> R {
    let _ = f__pos(Py::str("the base current IB"), Py::tuple(vec![v_IB.clone()]))?;
    let _ = f__beta(v_beta.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("IC"), py::mul(&v_beta.clone(), &v_IB.clone())?), (Py::str("IE"), py::mul(&py::add(&v_beta.clone(), &Py::Float(1.0))?, &v_IB.clone())?), (Py::str("alpha"), py::truediv(&v_beta.clone(), &py::add(&v_beta.clone(), &Py::Float(1.0))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bjt_small_signal(mut v_IC: Py, mut v_beta: Py, mut v_VT: Py, mut v_VA: Py) -> R {
    let mut v_gm: Py = Py::None;
    let _ = f__pos(Py::str("IC, beta, VT and VA"), Py::tuple(vec![v_IC.clone(), v_beta.clone(), v_VT.clone(), v_VA.clone()]))?;
    v_gm = py::truediv(&v_IC.clone(), &v_VT.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("gm"), v_gm.clone()), (Py::str("r_pi"), py::truediv(&v_beta.clone(), &v_gm.clone())?), (Py::str("r_e"), py::truediv(&py::mul(&v_VT.clone(), &v_beta.clone())?, &py::mul(&py::add(&v_beta.clone(), &Py::Float(1.0))?, &v_IC.clone())?)?), (Py::str("r_o"), py::truediv(&v_VA.clone(), &v_IC.clone())?), (Py::str("A0"), py::truediv(&v_VA.clone(), &v_VT.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_body_effect(mut v_Vt0: Py, mut v_gamma: Py, mut v_phi2F: Py, mut v_VSB: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("Vt0 and 2 phi_F"), Py::tuple(vec![v_Vt0.clone(), v_phi2F.clone()]))?;
    let _ = f__nonneg(Py::str("gamma and VSB"), Py::tuple(vec![v_gamma.clone(), v_VSB.clone()]))?;
    v_d = py::mul(&v_gamma.clone(), &py::sub(&py::math1("sqrt", &py::add(&v_phi2F.clone(), &v_VSB.clone())?)?, &py::math1("sqrt", &v_phi2F.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vt"), py::add(&v_Vt0.clone(), &v_d.clone())?), (Py::str("dVt"), v_d.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_builtin_potential(mut v_NA: Py, mut v_ND: Py, mut v_ni: Py, mut v_T: Py) -> R {
    let mut v_Vbi: Py = Py::None;
    let _ = f__pos(Py::str("NA, ND, ni and T"), Py::tuple(vec![v_NA.clone(), v_ND.clone(), v_ni.clone(), v_T.clone()]))?;
    let _ = f__need({ let t682 = { let t678 = v_NA.clone(); Py::Bool({ let t679 = py::mul(&Py::Float(100.0), &v_ni.clone())?; if py::ge(&t678, &t679)? { true } else { false } }) }; if !t682.truthy() { t682 } else { { let t680 = v_ND.clone(); Py::Bool({ let t681 = py::mul(&Py::Float(100.0), &v_ni.clone())?; if py::ge(&t680, &t681)? { true } else { false } }) } } }, Py::str("the doping on each side must be at least 100 times ni (the formula assumes NA, ND >> ni)"))?;
    v_Vbi = py::mul(&f__vt(v_T.clone())?, &py::math1("log", &py::truediv(&py::mul(&v_NA.clone(), &v_ND.clone())?, &py::mul(&v_ni.clone(), &v_ni.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vbi"), v_Vbi.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_butterworth_sections(mut v_n: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_qs: Py = Py::None;
    let _ = f__need({ let t724 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t724.truthy() { t724 } else { { let t721 = Py::Int(2i64); Py::Bool({ let t722 = v_n.clone(); if py::le(&t721, &t722)? { { let t723 = Py::Int(20i64); if py::le(&t722, &t723)? { true } else { false } } } else { false } }) } } }, Py::str("the order must be a whole number from 2 to 20"))?;
    v_qs = Py::list(vec![]);
    v_k = py::floordiv(&v_n.clone(), &Py::Int(2i64))?;
    while ({ let t725 = v_k.clone(); Py::Bool({ let t726 = Py::Int(1i64); if py::ge(&t725, &t726)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_qs.clone(), "append", &[py::truediv(&Py::Float(1.0), &py::mul(&Py::Float(2.0), &py::math1("sin", &py::truediv(&py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &Py::Float(std::f64::consts::PI))?, &py::mul(&Py::Int(2i64), &v_n.clone())?)?)?)?)?])?;
        v_k = py::sub(&v_k, &Py::Int(1i64))?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Q_max"), py::getitem(&v_qs.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("Q"), v_qs.clone()), (Py::str("sections"), py::floordiv(&v_n.clone(), &Py::Int(2i64))?), (Py::str("first_order"), py::modulo(&v_n.clone(), &Py::Int(2i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cascode(mut v_IC: Py, mut v_beta: Py, mut v_VT: Py, mut v_VA: Py, mut v_RL: Py) -> R {
    let mut v_Gm: Py = Py::None;
    let mut v_Rout: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_ro: Py = Py::None;
    let mut v_rpi: Py = Py::None;
    let _ = f__pos(Py::str("IC, beta, VT, VA and RL"), Py::tuple(vec![v_IC.clone(), v_beta.clone(), v_VT.clone(), v_VA.clone(), v_RL.clone()]))?;
    v_gm = py::truediv(&v_IC.clone(), &v_VT.clone())?;
    let t345 = (Py::tuple(vec![py::truediv(&v_beta.clone(), &v_gm.clone())?, py::truediv(&v_VA.clone(), &v_IC.clone())?])).items()?;
    if t345.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rpi = t345[0].clone();
    v_ro = t345[1].clone();
    v_Rout = py::add(&v_ro.clone(), &py::mul(&py::add(&Py::Float(1.0), &py::mul(&v_gm.clone(), &v_ro.clone())?)?, &f__par(Py::tuple(vec![v_ro.clone(), v_rpi.clone()]))?)?)?;
    v_Gm = py::truediv(&py::mul(&v_gm.clone(), &py::add(&v_gm.clone(), &py::truediv(&Py::Float(1.0), &v_ro.clone())?)?)?, &py::add(&py::add(&v_gm.clone(), &py::truediv(&Py::Float(2.0), &v_ro.clone())?)?, &py::truediv(&Py::Float(1.0), &v_rpi.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("gm"), v_gm.clone()), (Py::str("r_o"), v_ro.clone()), (Py::str("Rout"), v_Rout.clone()), (Py::str("Gm"), v_Gm.clone()), (Py::str("Av"), py::mul(&py::neg(&v_Gm.clone())?, &f__par(Py::tuple(vec![v_Rout.clone(), v_RL.clone()]))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cb_amplifier(mut v_IC: Py, mut v_beta: Py, mut v_VT: Py, mut v_RC: Py, mut v_RL: Py, mut v_RE: Py, mut v_Rsig: Py) -> R {
    let mut v_Av: Py = Py::None;
    let mut v_Rin: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_rpi: Py = Py::None;
    let _ = f__pos(Py::str("RC, RL, RE and Rsig"), Py::tuple(vec![v_RC.clone(), v_RL.clone(), v_RE.clone(), v_Rsig.clone()]))?;
    let t1099 = (f__hybrid(v_IC.clone(), v_beta.clone(), v_VT.clone())?).items()?;
    if t1099.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gm = t1099[0].clone();
    v_rpi = t1099[1].clone();
    v_re_ = py::truediv(&v_beta.clone(), &py::mul(&py::add(&v_beta.clone(), &Py::Float(1.0))?, &v_gm.clone())?)?;
    v_Rin = f__par(Py::tuple(vec![v_RE.clone(), v_re_.clone()]))?;
    v_Av = py::mul(&v_gm.clone(), &f__par(Py::tuple(vec![v_RC.clone(), v_RL.clone()]))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Av"), v_Av.clone()), (Py::str("Rin"), v_Rin.clone()), (Py::str("Rout"), v_RC.clone()), (Py::str("Gv"), py::truediv(&py::mul(&v_Av.clone(), &v_Rin.clone())?, &py::add(&v_Rin.clone(), &v_Rsig.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cc_amplifier(mut v_IC: Py, mut v_beta: Py, mut v_VT: Py, mut v_RE: Py, mut v_RL: Py, mut v_RB: Py, mut v_Rsig: Py) -> R {
    let mut v_Av: Py = Py::None;
    let mut v_Re: Py = Py::None;
    let mut v_Rin: Py = Py::None;
    let mut v_Rout: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_rin_b: Py = Py::None;
    let mut v_rpi: Py = Py::None;
    let _ = f__pos(Py::str("RE, RL, RB and Rsig"), Py::tuple(vec![v_RE.clone(), v_RL.clone(), v_RB.clone(), v_Rsig.clone()]))?;
    let t1145 = (f__hybrid(v_IC.clone(), v_beta.clone(), v_VT.clone())?).items()?;
    if t1145.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gm = t1145[0].clone();
    v_rpi = t1145[1].clone();
    v_Re = f__par(Py::tuple(vec![v_RE.clone(), v_RL.clone()]))?;
    v_rin_b = py::add(&v_rpi.clone(), &py::mul(&py::add(&v_beta.clone(), &Py::Float(1.0))?, &v_Re.clone())?)?;
    v_Av = py::truediv(&py::mul(&py::add(&v_beta.clone(), &Py::Float(1.0))?, &v_Re.clone())?, &v_rin_b.clone())?;
    v_Rin = f__par(Py::tuple(vec![v_RB.clone(), v_rin_b.clone()]))?;
    v_Rout = f__par(Py::tuple(vec![v_RE.clone(), py::truediv(&py::add(&v_rpi.clone(), &f__par(Py::tuple(vec![v_Rsig.clone(), v_RB.clone()]))?)?, &py::add(&v_beta.clone(), &Py::Float(1.0))?)?]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Av"), v_Av.clone()), (Py::str("Rin"), v_Rin.clone()), (Py::str("Rout"), v_Rout.clone()), (Py::str("Gv"), py::truediv(&py::mul(&v_Av.clone(), &v_Rin.clone())?, &py::add(&v_Rin.clone(), &v_Rsig.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ce_amplifier(mut v_IC: Py, mut v_beta: Py, mut v_VT: Py, mut v_RC: Py, mut v_RL: Py, mut v_RE1: Py, mut v_RB: Py, mut v_Rsig: Py) -> R {
    let mut v_Av: Py = Py::None;
    let mut v_Rin: Py = Py::None;
    let mut v_Rl: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_rin_b: Py = Py::None;
    let mut v_rpi: Py = Py::None;
    let _ = f__pos(Py::str("RC, RL, RB and Rsig"), Py::tuple(vec![v_RC.clone(), v_RL.clone(), v_RB.clone(), v_Rsig.clone()]))?;
    let _ = f__nonneg(Py::str("the unbypassed emitter resistance RE1"), Py::tuple(vec![v_RE1.clone()]))?;
    let t1197 = (f__hybrid(v_IC.clone(), v_beta.clone(), v_VT.clone())?).items()?;
    if t1197.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gm = t1197[0].clone();
    v_rpi = t1197[1].clone();
    v_Rl = f__par(Py::tuple(vec![v_RC.clone(), v_RL.clone()]))?;
    v_rin_b = py::add(&v_rpi.clone(), &py::mul(&py::add(&v_beta.clone(), &Py::Float(1.0))?, &v_RE1.clone())?)?;
    v_Av = py::truediv(&py::mul(&py::neg(&v_beta.clone())?, &v_Rl.clone())?, &v_rin_b.clone())?;
    v_Rin = f__par(Py::tuple(vec![v_RB.clone(), v_rin_b.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("gm"), v_gm.clone()), (Py::str("r_pi"), v_rpi.clone()), (Py::str("Av"), v_Av.clone()), (Py::str("Rin"), v_Rin.clone()), (Py::str("Rout"), v_RC.clone()), (Py::str("Gv"), py::truediv(&py::mul(&v_Av.clone(), &v_Rin.clone())?, &py::add(&v_Rin.clone(), &v_Rsig.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chebyshev_sections(mut v_n: Py, mut v_ripple: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_eps: Py = Py::None;
    let mut v_f0: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_pairs: Py = Py::None;
    let mut v_qs: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_w0: Py = Py::None;
    let _ = f__need({ let t818 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t818.truthy() { t818 } else { { let t815 = Py::Int(2i64); Py::Bool({ let t816 = v_n.clone(); if py::le(&t815, &t816)? { { let t817 = Py::Int(16i64); if py::le(&t816, &t817)? { true } else { false } } } else { false } }) } } }, Py::str("the order must be a whole number from 2 to 16"))?;
    let _ = f__need({ let t819 = Py::Float(0.001); Py::Bool({ let t820 = v_ripple.clone(); if py::le(&t819, &t820)? { { let t821 = Py::Float(6.0); if py::le(&t820, &t821)? { true } else { false } } } else { false } }) }, Py::str("the passband ripple must be between 0.001 and 6 dB"))?;
    v_eps = py::math1("sqrt", &py::sub(&py::pow(&Py::Float(10.0), &py::truediv(&v_ripple.clone(), &Py::Float(10.0))?)?, &Py::Float(1.0))?)?;
    v_a = py::truediv(&py::math1("asinh", &py::truediv(&Py::Float(1.0), &v_eps.clone())?)?, &v_n.clone())?;
    v_pairs = Py::list(vec![]);
    for t822 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::floordiv(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?).i()?]) {
        v_k = t822;
        v_th = py::truediv(&py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &Py::Float(std::f64::consts::PI))?, &py::mul(&Py::Int(2i64), &v_n.clone())?)?;
        v_s = py::mul(&py::math1("sinh", &v_a.clone())?, &py::math1("sin", &v_th.clone())?)?;
        v_w = py::mul(&py::math1("cosh", &v_a.clone())?, &py::math1("cos", &v_th.clone())?)?;
        v_w0 = py::math1("sqrt", &py::add(&py::mul(&v_s.clone(), &v_s.clone())?, &py::mul(&v_w.clone(), &v_w.clone())?)?)?;
        let _ = py::method(&v_pairs.clone(), "append", &[Py::tuple(vec![py::truediv(&v_w0.clone(), &py::mul(&Py::Float(2.0), &v_s.clone())?)?, v_w0.clone()])])?;
    }
    let _ = py::method_sort(&v_pairs.clone(), None, false)?;
    v_qs = { let mut out = vec![]; for t823 in py::iter(&(v_pairs.clone()))? { let mut v_p: Py; 
v_p = t823;
out.push(py::getitem(&v_p.clone(), &Py::Int(0i64))?); } Py::list(out) };
    v_f0 = { let mut out = vec![]; for t824 in py::iter(&(v_pairs.clone()))? { let mut v_p: Py; 
v_p = t824;
out.push(py::getitem(&v_p.clone(), &Py::Int(1i64))?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("Q_max"), py::getitem(&v_qs.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("Q"), v_qs.clone()), (Py::str("f0_ratio"), v_f0.clone()), (Py::str("sections"), py::floordiv(&v_n.clone(), &Py::Int(2i64))?), (Py::str("real_pole"), (if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() { py::math1("sinh", &v_a.clone())? } else { Py::Float(0.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_closed_loop_bw(mut v_GBW: Py, mut v_config: Py, mut v_Rf: Py, mut v_R1: Py) -> R {
    let mut v_G0: Py = Py::None;
    let mut v_NG: Py = Py::None;
    let _ = f__pos(Py::str("GBW, Rf and R1"), Py::tuple(vec![v_GBW.clone(), v_Rf.clone(), v_R1.clone()]))?;
    let _ = f__need({ let t869 = v_config.clone(); Py::Bool({ let t870 = Py::tuple(vec![Py::str("inverting"), Py::str("non-inverting")]); if py::contains(&t870, &t869)? { true } else { false } }) }, Py::str("config: inverting or non-inverting"))?;
    v_NG = py::add(&Py::Float(1.0), &py::truediv(&v_Rf.clone(), &v_R1.clone())?)?;
    v_G0 = (if ({ let t871 = v_config.clone(); Py::Bool({ let t872 = Py::str("inverting"); if py::eq(&t871, &t872) { true } else { false } }) }).truthy() { py::truediv(&py::neg(&v_Rf.clone())?, &v_R1.clone())? } else { v_NG.clone() });
    return Ok(f__out(Py::dict(vec![(Py::str("G0"), v_G0.clone()), (Py::str("noise_gain"), v_NG.clone()), (Py::str("f_3dB"), py::truediv(&v_GBW.clone(), &v_NG.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cmos_delay(mut v_Rn: Py, mut v_Rp: Py, mut v_CL: Py) -> R {
    let mut v_hl: Py = Py::None;
    let mut v_lh: Py = Py::None;
    let _ = f__pos(Py::str("Rn, Rp and CL"), Py::tuple(vec![v_Rn.clone(), v_Rp.clone(), v_CL.clone()]))?;
    let t964 = (Py::tuple(vec![py::mul(&py::mul(&c_LN2()?, &v_Rn.clone())?, &v_CL.clone())?, py::mul(&py::mul(&c_LN2()?, &v_Rp.clone())?, &v_CL.clone())?])).items()?;
    if t964.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_hl = t964[0].clone();
    v_lh = t964[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("tpHL"), v_hl.clone()), (Py::str("tpLH"), v_lh.clone()), (Py::str("tp"), py::truediv(&py::add(&v_hl.clone(), &v_lh.clone())?, &Py::Float(2.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cmos_power(mut v_alpha: Py, mut v_C: Py, mut v_VDD: Py, mut v_f: Py) -> R {
    let mut v_P: Py = Py::None;
    let _ = f__pos(Py::str("C, VDD and f"), Py::tuple(vec![v_C.clone(), v_VDD.clone(), v_f.clone()]))?;
    let _ = f__need({ let t917 = Py::Float(0.0); Py::Bool({ let t918 = v_alpha.clone(); if py::le(&t917, &t918)? { { let t919 = Py::Float(1.0); if py::le(&t918, &t919)? { true } else { false } } } else { false } }) }, Py::str("the activity factor must be between 0 and 1"))?;
    v_P = py::mul(&py::mul(&py::mul(&py::mul(&v_alpha.clone(), &v_C.clone())?, &v_VDD.clone())?, &v_VDD.clone())?, &v_f.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("E_supply"), py::mul(&py::mul(&v_C.clone(), &v_VDD.clone())?, &v_VDD.clone())?), (Py::str("E_stored"), py::truediv(&py::mul(&py::mul(&v_C.clone(), &v_VDD.clone())?, &v_VDD.clone())?, &Py::Float(2.0))?), (Py::str("I_avg"), py::truediv(&v_P.clone(), &v_VDD.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cmos_threshold(mut v_VDD: Py, mut v_Vtn: Py, mut v_Vtp: Py, mut v_kn: Py, mut v_kp: Py) -> R {
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("VDD, Vtn, |Vtp|, kn and kp"), Py::tuple(vec![v_VDD.clone(), v_Vtn.clone(), v_Vtp.clone(), v_kn.clone(), v_kp.clone()]))?;
    let _ = f__need({ let t1009 = py::add(&v_Vtn.clone(), &v_Vtp.clone())?; Py::Bool({ let t1010 = v_VDD.clone(); if py::lt(&t1009, &t1010)? { true } else { false } }) }, Py::str("VDD must exceed Vtn + |Vtp|"))?;
    v_r = py::math1("sqrt", &py::truediv(&v_kp.clone(), &v_kn.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("r"), v_r.clone()), (Py::str("VM"), py::truediv(&py::add(&v_Vtn.clone(), &py::mul(&v_r.clone(), &py::sub(&v_VDD.clone(), &v_Vtp.clone())?)?)?, &py::add(&Py::Float(1.0), &v_r.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_collector_feedback(mut v_VCC: Py, mut v_RC: Py, mut v_RF: Py, mut v_beta: Py, mut v_VBE: Py, mut v_RE: Py) -> R {
    let mut v_IB: Py = Py::None;
    let mut v_IE: Py = Py::None;
    let mut v_VC: Py = Py::None;
    let mut v_VE: Py = Py::None;
    let _ = f__pos(Py::str("VCC, RC, RF and VBE"), Py::tuple(vec![v_VCC.clone(), v_RC.clone(), v_RF.clone(), v_VBE.clone()]))?;
    let _ = f__nonneg(Py::str("RE"), Py::tuple(vec![v_RE.clone()]))?;
    let _ = f__beta(v_beta.clone())?;
    let _ = f__need({ let t397 = v_VCC.clone(); Py::Bool({ let t398 = v_VBE.clone(); if py::gt(&t397, &t398)? { true } else { false } }) }, Py::str("VCC must exceed VBE"))?;
    v_IB = py::truediv(&py::sub(&v_VCC.clone(), &v_VBE.clone())?, &py::add(&v_RF.clone(), &py::mul(&py::add(&v_beta.clone(), &Py::Float(1.0))?, &py::add(&v_RC.clone(), &v_RE.clone())?)?)?)?;
    v_IE = py::mul(&py::add(&v_beta.clone(), &Py::Float(1.0))?, &v_IB.clone())?;
    v_VC = py::sub(&v_VCC.clone(), &py::mul(&v_IE.clone(), &v_RC.clone())?)?;
    v_VE = py::mul(&v_IE.clone(), &v_RE.clone())?;
    let _ = f__active(py::sub(&v_VC.clone(), &v_VE.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("IB"), v_IB.clone()), (Py::str("IC"), py::mul(&v_beta.clone(), &v_IB.clone())?), (Py::str("IE"), v_IE.clone()), (Py::str("VC"), v_VC.clone()), (Py::str("VCE"), py::sub(&v_VC.clone(), &v_VE.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_colpitts(mut v_L: Py, mut v_C1: Py, mut v_C2: Py) -> R {
    let mut v_Ceq: Py = Py::None;
    let _ = f__pos(Py::str("L, C1 and C2"), Py::tuple(vec![v_L.clone(), v_C1.clone(), v_C2.clone()]))?;
    v_Ceq = py::truediv(&py::mul(&v_C1.clone(), &v_C2.clone())?, &py::add(&v_C1.clone(), &v_C2.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C_eq"), v_Ceq.clone()), (Py::str("f0"), py::truediv(&Py::Float(1.0), &py::mul(&c_TWO_PI()?, &py::math1("sqrt", &py::mul(&v_L.clone(), &v_Ceq.clone())?)?)?)?), (Py::str("beta"), py::truediv(&v_C1.clone(), &v_C2.clone())?), (Py::str("A_min"), py::truediv(&v_C2.clone(), &v_C1.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_conductivity(mut v_n: Py, mut v_p: Py, mut v_mu_n: Py, mut v_mu_p: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__nonneg(Py::str("the carrier concentrations n and p"), Py::tuple(vec![v_n.clone(), v_p.clone()]))?;
    let _ = f__pos(Py::str("the mobilities"), Py::tuple(vec![v_mu_n.clone(), v_mu_p.clone()]))?;
    let _ = f__need({ let t3775 = py::add(&v_n.clone(), &v_p.clone())?; Py::Bool({ let t3776 = Py::Int(0i64); if py::gt(&t3775, &t3776)? { true } else { false } }) }, Py::str("at least one carrier concentration must be positive"))?;
    v_s = py::mul(&c_Q_E()?, &py::add(&py::mul(&v_n.clone(), &v_mu_n.clone())?, &py::mul(&v_p.clone(), &v_mu_p.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sigma"), v_s.clone()), (Py::str("rho"), py::truediv(&Py::Float(1.0), &v_s.clone())?), (Py::str("rho_ohm_cm"), py::truediv(&Py::Float(100.0), &v_s.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_crystal(mut v_Lm: Py, mut v_Cm: Py, mut v_Rm: Py, mut v_C0: Py) -> R {
    let mut v_fp: Py = Py::None;
    let mut v_fs: Py = Py::None;
    let _ = f__pos(Py::str("Lm, Cm, Rm and C0"), Py::tuple(vec![v_Lm.clone(), v_Cm.clone(), v_Rm.clone(), v_C0.clone()]))?;
    v_fs = py::truediv(&Py::Float(1.0), &py::mul(&c_TWO_PI()?, &py::math1("sqrt", &py::mul(&v_Lm.clone(), &v_Cm.clone())?)?)?)?;
    v_fp = py::mul(&v_fs.clone(), &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::truediv(&v_Cm.clone(), &v_C0.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("fs"), v_fs.clone()), (Py::str("fp"), v_fp.clone()), (Py::str("delta_f"), py::mul(&v_fs.clone(), &py::sub(&py::math1("sqrt", &py::add(&Py::Float(1.0), &py::truediv(&v_Cm.clone(), &v_C0.clone())?)?)?, &Py::Float(1.0))?)?), (Py::str("Q"), py::truediv(&py::math1("sqrt", &py::truediv(&v_Lm.clone(), &v_Cm.clone())?)?, &v_Rm.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cs_amplifier(mut v_gm: Py, mut v_ro: Py, mut v_RD: Py, mut v_RL: Py, mut v_RS: Py, mut v_RG: Py, mut v_Rsig: Py) -> R {
    let mut v_Av: Py = Py::None;
    let mut v_Rd: Py = Py::None;
    let mut v_Rout: Py = Py::None;
    let _ = f__pos(Py::str("gm, ro, RD, RL, RG and Rsig"), Py::tuple(vec![v_gm.clone(), v_ro.clone(), v_RD.clone(), v_RL.clone(), v_RG.clone(), v_Rsig.clone()]))?;
    let _ = f__nonneg(Py::str("the unbypassed source resistance RS"), Py::tuple(vec![v_RS.clone()]))?;
    v_Rd = f__par(Py::tuple(vec![v_RD.clone(), v_RL.clone()]))?;
    v_Av = py::truediv(&py::mul(&py::mul(&py::neg(&v_gm.clone())?, &v_ro.clone())?, &v_Rd.clone())?, &py::add(&py::add(&v_Rd.clone(), &v_ro.clone())?, &py::mul(&v_RS.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_gm.clone(), &v_ro.clone())?)?)?)?)?;
    v_Rout = f__par(Py::tuple(vec![v_RD.clone(), py::add(&v_ro.clone(), &py::mul(&v_RS.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_gm.clone(), &v_ro.clone())?)?)?)?]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Av"), v_Av.clone()), (Py::str("Rin"), v_RG.clone()), (Py::str("Rout"), v_Rout.clone()), (Py::str("Gv"), py::truediv(&py::mul(&v_Av.clone(), &v_RG.clone())?, &py::add(&v_RG.clone(), &v_Rsig.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_current_mirror(mut v_VCC: Py, mut v_R: Py, mut v_VBE: Py, mut v_beta: Py) -> R {
    let mut v_Iout: Py = Py::None;
    let mut v_Iref: Py = Py::None;
    let _ = f__pos(Py::str("VCC, R and VBE"), Py::tuple(vec![v_VCC.clone(), v_R.clone(), v_VBE.clone()]))?;
    let _ = f__beta(v_beta.clone())?;
    let _ = f__need({ let t1338 = v_VCC.clone(); Py::Bool({ let t1339 = v_VBE.clone(); if py::gt(&t1338, &t1339)? { true } else { false } }) }, Py::str("VCC must exceed VBE"))?;
    v_Iref = py::truediv(&py::sub(&v_VCC.clone(), &v_VBE.clone())?, &v_R.clone())?;
    v_Iout = py::truediv(&py::mul(&v_Iref.clone(), &v_beta.clone())?, &py::add(&v_beta.clone(), &Py::Float(2.0))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Iref"), v_Iref.clone()), (Py::str("Iout"), v_Iout.clone()), (Py::str("ratio"), py::truediv(&v_beta.clone(), &py::add(&v_beta.clone(), &Py::Float(2.0))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dac_binary_weighted(mut v_n: Py, mut v_code: Py, mut v_Vref: Py, mut v_R: Py, mut v_Rf: Py) -> R {
    let mut v_I: Py = Py::None;
    let _ = f__need({ let t1381 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1381.truthy() { t1381 } else { { let t1378 = Py::Int(1i64); Py::Bool({ let t1379 = v_n.clone(); if py::le(&t1378, &t1379)? { { let t1380 = Py::Int(12i64); if py::le(&t1379, &t1380)? { true } else { false } } } else { false } }) } } }, Py::str("the resolution must be a whole number of bits from 1 to 12"))?;
    let _ = f__need({ let t1385 = Py::Bool(matches!(v_code.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1385.truthy() { t1385 } else { { let t1382 = Py::Int(0i64); Py::Bool({ let t1383 = v_code.clone(); if py::le(&t1382, &t1383)? { { let t1384 = py::sub(&py::pow(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?; if py::le(&t1383, &t1384)? { true } else { false } } } else { false } }) } } }, Py::str("the code must be a whole number from 0 to 2^n - 1"))?;
    let _ = f__pos(Py::str("Vref, R and Rf"), Py::tuple(vec![v_Vref.clone(), v_R.clone(), v_Rf.clone()]))?;
    v_I = py::truediv(&py::mul(&v_Vref.clone(), &v_code.clone())?, &py::mul(&v_R.clone(), &py::pow(&Py::Int(2i64), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vout"), py::mul(&py::neg(&v_Rf.clone())?, &v_I.clone())?), (Py::str("I_total"), v_I.clone()), (Py::str("R_LSB"), py::mul(&v_R.clone(), &py::pow(&Py::Int(2i64), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dac_output(mut v_n: Py, mut v_code: Py, mut v_Vref: Py) -> R {
    let mut v_lsb: Py = Py::None;
    let _ = f__need({ let t1433 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1433.truthy() { t1433 } else { { let t1430 = Py::Int(1i64); Py::Bool({ let t1431 = v_n.clone(); if py::le(&t1430, &t1431)? { { let t1432 = Py::Int(16i64); if py::le(&t1431, &t1432)? { true } else { false } } } else { false } }) } } }, Py::str("the resolution must be a whole number of bits from 1 to 16"))?;
    let _ = f__need({ let t1437 = Py::Bool(matches!(v_code.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1437.truthy() { t1437 } else { { let t1434 = Py::Int(0i64); Py::Bool({ let t1435 = v_code.clone(); if py::le(&t1434, &t1435)? { { let t1436 = py::sub(&py::pow(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?; if py::le(&t1435, &t1436)? { true } else { false } } } else { false } }) } } }, Py::str("the code must be a whole number from 0 to 2^n - 1"))?;
    let _ = f__pos(Py::str("Vref"), Py::tuple(vec![v_Vref.clone()]))?;
    v_lsb = py::truediv(&v_Vref.clone(), &py::pow(&Py::Int(2i64), &v_n.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vout"), py::mul(&v_code.clone(), &v_lsb.clone())?), (Py::str("LSB"), v_lsb.clone()), (Py::str("V_full_scale"), py::mul(&py::sub(&py::pow(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?, &v_lsb.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_depletion(mut v_NA: Py, mut v_ND: Py, mut v_Vbi: Py, mut v_VR: Py, mut v_eps_r: Py) -> R {
    let mut v_Emax: Py = Py::None;
    let mut v_Vt: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_eps: Py = Py::None;
    let mut v_xn: Py = Py::None;
    let mut v_xp: Py = Py::None;
    let _ = f__pos(Py::str("NA, ND, Vbi and eps_r"), Py::tuple(vec![v_NA.clone(), v_ND.clone(), v_Vbi.clone(), v_eps_r.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_VR.clone()]))?;
    v_Vt = py::add(&v_Vbi.clone(), &v_VR.clone())?;
    let _ = f__need({ let t1491 = v_Vt.clone(); Py::Bool({ let t1492 = Py::Int(0i64); if py::gt(&t1491, &t1492)? { true } else { false } }) }, Py::str("Vbi + VR must be positive (a forward bias below the built-in potential)"))?;
    v_eps = py::mul(&v_eps_r.clone(), &c_EPS0()?)?;
    v_W = py::math1("sqrt", &py::mul(&py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &v_eps.clone())?, &v_Vt.clone())?, &c_Q_E()?)?, &py::add(&py::truediv(&Py::Float(1.0), &v_NA.clone())?, &py::truediv(&Py::Float(1.0), &v_ND.clone())?)?)?)?;
    v_xn = py::truediv(&py::mul(&v_W.clone(), &v_NA.clone())?, &py::add(&v_NA.clone(), &v_ND.clone())?)?;
    v_xp = py::truediv(&py::mul(&v_W.clone(), &v_ND.clone())?, &py::add(&v_NA.clone(), &v_ND.clone())?)?;
    v_Emax = py::truediv(&py::mul(&py::mul(&c_Q_E()?, &v_ND.clone())?, &v_xn.clone())?, &v_eps.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("W"), v_W.clone()), (Py::str("W_um"), py::mul(&v_W.clone(), &Py::Float(1000000.0))?), (Py::str("xn"), v_xn.clone()), (Py::str("xp"), v_xp.clone()), (Py::str("Emax"), v_Emax.clone()), (Py::str("Cj_area"), py::truediv(&v_eps.clone(), &v_W.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_diff_pair(mut v_I_tail: Py, mut v_beta: Py, mut v_VT: Py, mut v_RC: Py, mut v_REE: Py) -> R {
    let mut v_Acm: Py = Py::None;
    let mut v_Ad_se: Py = Py::None;
    let mut v_IC: Py = Py::None;
    let mut v_cmrr: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_rpi: Py = Py::None;
    let _ = f__pos(Py::str("I_tail, beta, VT, RC and REE"), Py::tuple(vec![v_I_tail.clone(), v_beta.clone(), v_VT.clone(), v_RC.clone(), v_REE.clone()]))?;
    v_IC = py::truediv(&py::mul(&py::truediv(&v_beta.clone(), &py::add(&v_beta.clone(), &Py::Float(1.0))?)?, &v_I_tail.clone())?, &Py::Float(2.0))?;
    v_gm = py::truediv(&v_IC.clone(), &v_VT.clone())?;
    v_rpi = py::truediv(&v_beta.clone(), &v_gm.clone())?;
    v_Ad_se = py::truediv(&py::mul(&py::neg(&v_gm.clone())?, &v_RC.clone())?, &Py::Float(2.0))?;
    v_Acm = py::truediv(&py::mul(&py::neg(&v_gm.clone())?, &v_RC.clone())?, &py::add(&Py::Float(1.0), &py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &v_REE.clone())?, &py::add(&v_beta.clone(), &Py::Float(1.0))?)?, &v_rpi.clone())?)?)?;
    v_cmrr = py::abs(&py::truediv(&v_Ad_se.clone(), &v_Acm.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("IC"), v_IC.clone()), (Py::str("gm"), v_gm.clone()), (Py::str("Ad_diff"), py::mul(&py::neg(&v_gm.clone())?, &v_RC.clone())?), (Py::str("Ad_se"), v_Ad_se.clone()), (Py::str("Acm"), v_Acm.clone()), (Py::str("CMRR"), v_cmrr.clone()), (Py::str("CMRR_dB"), f__db(v_cmrr.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_difference_amp(mut v_R1: Py, mut v_R2: Py, mut v_R3: Py, mut v_R4: Py, mut v_V1: Py, mut v_V2: Py) -> R {
    let mut v_Acm: Py = Py::None;
    let mut v_Ad: Py = Py::None;
    let mut v_a1: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let _ = f__pos(Py::str("R1, R2, R3 and R4"), Py::tuple(vec![v_R1.clone(), v_R2.clone(), v_R3.clone(), v_R4.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_V1.clone(), v_V2.clone()]))?;
    v_a1 = py::truediv(&py::neg(&v_R2.clone())?, &v_R1.clone())?;
    v_a2 = py::mul(&py::truediv(&v_R4.clone(), &py::add(&v_R3.clone(), &v_R4.clone())?)?, &py::add(&Py::Float(1.0), &py::truediv(&v_R2.clone(), &v_R1.clone())?)?)?;
    v_Ad = py::truediv(&py::sub(&v_a2.clone(), &v_a1.clone())?, &Py::Float(2.0))?;
    v_Acm = py::add(&v_a1.clone(), &v_a2.clone())?;
    if ({ let t1540 = py::abs(&v_Acm.clone())?; Py::Bool({ let t1541 = py::mul(&Py::Float(1e-12), &py::abs(&v_Ad.clone())?)?; if py::lt(&t1540, &t1541)? { true } else { false } }) }).truthy() {
        v_Acm = Py::Float(0.0);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Vout"), py::add(&py::mul(&v_a1.clone(), &v_V1.clone())?, &py::mul(&v_a2.clone(), &v_V2.clone())?)?), (Py::str("Ad"), v_Ad.clone()), (Py::str("Acm"), v_Acm.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_differentiator(mut v_R: Py, mut v_C: Py, mut v_A: Py, mut v_f: Py) -> R {
    let mut v_g: Py = Py::None;
    let _ = f__pos(Py::str("R, C, A and f"), Py::tuple(vec![v_R.clone(), v_C.clone(), v_A.clone(), v_f.clone()]))?;
    v_g = py::mul(&py::mul(&py::mul(&c_TWO_PI()?, &v_f.clone())?, &v_R.clone())?, &v_C.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("gain"), v_g.clone()), (Py::str("Vout_peak"), py::mul(&v_g.clone(), &v_A.clone())?), (Py::str("gain_dB"), f__db(v_g.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_diode_operating_point(mut v_VS: Py, mut v_R: Py, mut v_Is: Py, mut v_n: Py, mut v_T: Py) -> R {
    let mut v_ID: Py = Py::None;
    let mut v_VD: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_nVT: Py = Py::None;
    let _ = f__pos(Py::str("VS, R, Is, n and T"), Py::tuple(vec![v_VS.clone(), v_R.clone(), v_Is.clone(), v_n.clone(), v_T.clone()]))?;
    v_nVT = py::mul(&v_n.clone(), &f__vt(v_T.clone())?)?;
    v_lo = Py::Float(0.0);
    v_hi = py::minmax(&[v_VS.clone(), py::mul(&v_nVT.clone(), &py::math1("log1p", &py::truediv(&v_VS.clone(), &py::mul(&v_R.clone(), &v_Is.clone())?)?)?)?], false)?;
    for t1630 in py::range_iter(&[(Py::Int(400i64)).i()?]) {
        v__ = t1630;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t1635 = { let t1631 = v_mid.clone(); Py::Bool({ let t1632 = v_lo.clone(); if py::eq(&t1631, &t1632) { true } else { false } }) }; if t1635.truthy() { t1635 } else { { let t1633 = v_mid.clone(); Py::Bool({ let t1634 = v_hi.clone(); if py::eq(&t1633, &t1634) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_g = py::sub(&py::mul(&v_Is.clone(), &py::math1("expm1", &py::truediv(&v_mid.clone(), &v_nVT.clone())?)?)?, &py::truediv(&py::sub(&v_VS.clone(), &v_mid.clone())?, &v_R.clone())?)?;
        if ({ let t1636 = v_g.clone(); Py::Bool({ let t1637 = Py::Int(0i64); if py::gt(&t1636, &t1637)? { true } else { false } }) }).truthy() {
            v_hi = v_mid.clone();
        } else {
            v_lo = v_mid.clone();
        }
    }
    v_VD = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
    v_ID = py::truediv(&py::sub(&v_VS.clone(), &v_VD.clone())?, &v_R.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("ID"), v_ID.clone()), (Py::str("VD"), v_VD.clone()), (Py::str("P_D"), py::mul(&v_VD.clone(), &v_ID.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_diode_voltage(mut v_ID: Py, mut v_Is: Py, mut v_n: Py, mut v_T: Py) -> R {
    let mut v_VT: Py = Py::None;
    let _ = f__pos(Py::str("Is, n and T"), Py::tuple(vec![v_Is.clone(), v_n.clone(), v_T.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_ID.clone()]))?;
    let _ = f__need({ let t1685 = v_ID.clone(); Py::Bool({ let t1686 = py::neg(&v_Is.clone())?; if py::gt(&t1685, &t1686)? { true } else { false } }) }, Py::str("a diode current cannot be below -Is"))?;
    v_VT = f__vt(v_T.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("VD"), py::mul(&py::mul(&v_n.clone(), &v_VT.clone())?, &py::math1("log1p", &py::truediv(&v_ID.clone(), &v_Is.clone())?)?)?), (Py::str("VT"), v_VT.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_divider_bias(mut v_VCC: Py, mut v_R1: Py, mut v_R2: Py, mut v_RC: Py, mut v_RE: Py, mut v_beta: Py, mut v_VBE: Py) -> R {
    let mut v_IB: Py = Py::None;
    let mut v_IC: Py = Py::None;
    let mut v_IE: Py = Py::None;
    let mut v_RTH: Py = Py::None;
    let mut v_VC: Py = Py::None;
    let mut v_VE: Py = Py::None;
    let mut v_VTH: Py = Py::None;
    let _ = f__pos(Py::str("VCC, R1, R2, RC, RE and VBE"), Py::tuple(vec![v_VCC.clone(), v_R1.clone(), v_R2.clone(), v_RC.clone(), v_RE.clone(), v_VBE.clone()]))?;
    let _ = f__beta(v_beta.clone())?;
    v_VTH = py::truediv(&py::mul(&v_VCC.clone(), &v_R2.clone())?, &py::add(&v_R1.clone(), &v_R2.clone())?)?;
    v_RTH = py::truediv(&py::mul(&v_R1.clone(), &v_R2.clone())?, &py::add(&v_R1.clone(), &v_R2.clone())?)?;
    let _ = f__need({ let t629 = v_VTH.clone(); Py::Bool({ let t630 = v_VBE.clone(); if py::gt(&t629, &t630)? { true } else { false } }) }, Py::str("the divider voltage is below VBE: the transistor is cut off"))?;
    v_IB = py::truediv(&py::sub(&v_VTH.clone(), &v_VBE.clone())?, &py::add(&v_RTH.clone(), &py::mul(&py::add(&v_beta.clone(), &Py::Float(1.0))?, &v_RE.clone())?)?)?;
    let t631 = (Py::tuple(vec![py::mul(&v_beta.clone(), &v_IB.clone())?, py::mul(&py::add(&v_beta.clone(), &Py::Float(1.0))?, &v_IB.clone())?])).items()?;
    if t631.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_IC = t631[0].clone();
    v_IE = t631[1].clone();
    v_VE = py::mul(&v_IE.clone(), &v_RE.clone())?;
    v_VC = py::sub(&v_VCC.clone(), &py::mul(&v_IC.clone(), &v_RC.clone())?)?;
    let _ = f__active(py::sub(&v_VC.clone(), &v_VE.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("VTH"), v_VTH.clone()), (Py::str("RTH"), v_RTH.clone()), (Py::str("IB"), v_IB.clone()), (Py::str("IC"), v_IC.clone()), (Py::str("IE"), v_IE.clone()), (Py::str("VB"), py::add(&v_VE.clone(), &v_VBE.clone())?), (Py::str("VE"), v_VE.clone()), (Py::str("VC"), v_VC.clone()), (Py::str("VCE"), py::sub(&v_VC.clone(), &v_VE.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_doped_carriers(mut v_ND: Py, mut v_NA: Py, mut v_ni: Py) -> R {
    let mut v_big: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__nonneg(Py::str("the donor and acceptor concentrations"), Py::tuple(vec![v_ND.clone(), v_NA.clone()]))?;
    let _ = f__pos(Py::str("the intrinsic concentration ni"), Py::tuple(vec![v_ni.clone()]))?;
    v_d = py::sub(&v_ND.clone(), &v_NA.clone())?;
    v_h = py::mul(&Py::Float(0.5), &py::abs(&v_d.clone())?)?;
    v_big = py::add(&v_h.clone(), &py::math1("sqrt", &py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::mul(&v_ni.clone(), &v_ni.clone())?)?)?)?;
    if ({ let t1737 = v_d.clone(); Py::Bool({ let t1738 = Py::Int(0i64); if py::ge(&t1737, &t1738)? { true } else { false } }) }).truthy() {
        let t1739 = (Py::tuple(vec![v_big.clone(), py::truediv(&py::mul(&v_ni.clone(), &v_ni.clone())?, &v_big.clone())?])).items()?;
        if t1739.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t1739[0].clone();
        v_p = t1739[1].clone();
    } else {
        let t1740 = (Py::tuple(vec![v_big.clone(), py::truediv(&py::mul(&v_ni.clone(), &v_ni.clone())?, &v_big.clone())?])).items()?;
        if t1740.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t1740[0].clone();
        v_n = t1740[1].clone();
    }
    v_t = (if ({ let t1741 = v_d.clone(); Py::Bool({ let t1742 = Py::Int(0i64); if py::gt(&t1741, &t1742)? { true } else { false } }) }).truthy() { Py::str("n-type") } else { (if ({ let t1743 = v_d.clone(); Py::Bool({ let t1744 = Py::Int(0i64); if py::lt(&t1743, &t1744)? { true } else { false } }) }).truthy() { Py::str("p-type") } else { Py::str("intrinsic") }) });
    return Ok(f__out(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("p"), v_p.clone()), (Py::str("n_cm3"), py::mul(&v_n.clone(), &Py::Float(1e-06))?), (Py::str("p_cm3"), py::mul(&v_p.clone(), &Py::Float(1e-06))?), (Py::str("type"), v_t.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_drift_current(mut v_n: Py, mut v_mu: Py, mut v_V: Py, mut v_L: Py, mut v_A: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_J: Py = Py::None;
    let mut v_vd: Py = Py::None;
    let _ = f__pos(Py::str("n, mu, L and A"), Py::tuple(vec![v_n.clone(), v_mu.clone(), v_L.clone(), v_A.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_V.clone()]))?;
    v_E = py::truediv(&v_V.clone(), &v_L.clone())?;
    v_vd = py::mul(&v_mu.clone(), &v_E.clone())?;
    v_J = py::mul(&py::mul(&c_Q_E()?, &v_n.clone())?, &v_vd.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("v_d"), v_vd.clone()), (Py::str("J"), v_J.clone()), (Py::str("I"), py::mul(&v_J.clone(), &v_A.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dual_slope(mut v_Vin: Py, mut v_Vref: Py, mut v_N1: Py, mut v_fclk: Py, mut v_R: Py, mut v_C: Py) -> R {
    let mut v_T1: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vref, fclk, R and C"), Py::tuple(vec![v_Vin.clone(), v_Vref.clone(), v_fclk.clone(), v_R.clone(), v_C.clone()]))?;
    let _ = f__need({ let t1838 = Py::Bool(matches!(v_N1.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1838.truthy() { t1838 } else { { let t1836 = v_N1.clone(); Py::Bool({ let t1837 = Py::Int(1i64); if py::ge(&t1836, &t1837)? { true } else { false } }) } } }, Py::str("the fixed count N1 must be a whole number"))?;
    let _ = f__need({ let t1839 = v_Vin.clone(); Py::Bool({ let t1840 = v_Vref.clone(); if py::le(&t1839, &t1840)? { true } else { false } }) }, Py::str("the input must not exceed the reference"))?;
    v_T1 = py::truediv(&v_N1.clone(), &v_fclk.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("T1"), v_T1.clone()), (Py::str("T2"), py::truediv(&py::mul(&v_T1.clone(), &v_Vin.clone())?, &v_Vref.clone())?), (Py::str("N2"), py::truediv(&py::mul(&v_N1.clone(), &v_Vin.clone())?, &v_Vref.clone())?), (Py::str("V_peak"), py::truediv(&py::mul(&v_Vin.clone(), &v_T1.clone())?, &py::mul(&v_R.clone(), &v_C.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_einstein(mut v_mu: Py, mut v_T: Py, mut v_tau: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Ld: Py = Py::None;
    let _ = f__pos(Py::str("mu, T and tau"), Py::tuple(vec![v_mu.clone(), v_T.clone(), v_tau.clone()]))?;
    v_D = py::mul(&v_mu.clone(), &f__vt(v_T.clone())?)?;
    v_Ld = py::math1("sqrt", &py::mul(&v_D.clone(), &v_tau.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("D_cm2"), py::mul(&v_D.clone(), &Py::Float(10000.0))?), (Py::str("L_diff"), v_Ld.clone()), (Py::str("L_um"), py::mul(&v_Ld.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elmore(mut v_R: Py, mut v_C: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_delays: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_path: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__need({ let t1927 = { let t1923 = py::len(&v_R.clone())?; Py::Bool({ let t1924 = py::len(&v_C.clone())?; if py::eq(&t1923, &t1924) { true } else { false } }) }; if !t1927.truthy() { t1927 } else { { let t1925 = py::len(&v_R.clone())?; Py::Bool({ let t1926 = Py::Int(1i64); if py::ge(&t1925, &t1926)? { true } else { false } }) } } }, Py::str("give one capacitance per resistor segment"))?;
    v_path = Py::list(vec![]);
    v_s = Py::Float(0.0);
    for t1928 in py::iter(&(v_R.clone()))? {
        v_r = t1928;
        let _ = f__pos(Py::str("every resistance"), Py::tuple(vec![v_r.clone()]))?;
        v_s = py::iadd(&v_s, &v_r.clone())?;
        let _ = py::method(&v_path.clone(), "append", &[v_s.clone()])?;
    }
    for t1935 in py::iter(&(v_C.clone()))? {
        v_c = t1935;
        let _ = f__nonneg(Py::str("every capacitance"), Py::tuple(vec![v_c.clone()]))?;
    }
    v_delays = Py::list(vec![]);
    for t1942 in py::range_iter(&[(py::len(&v_R.clone())?).i()?]) {
        v_k = t1942;
        v_d = Py::Float(0.0);
        for t1943 in py::range_iter(&[(py::len(&v_C.clone())?).i()?]) {
            v_i = t1943;
            v_d = py::iadd(&v_d, &py::mul(&py::mul(&py::getitem(&v_C.clone(), &v_i.clone())?, &Py::Float(1e-12))?, &py::getitem(&v_path.clone(), &py::minmax(&[v_i.clone(), v_k.clone()], false)?)?)?)?;
        }
        let _ = py::method(&v_delays.clone(), "append", &[v_d.clone()])?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("tau_D"), py::getitem(&v_delays.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("t50"), py::mul(&c_LN2()?, &py::getitem(&v_delays.clone(), &py::neg(&Py::Int(1i64))?)?)?), (Py::str("delays"), v_delays.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_emitter_bias(mut v_VCC: Py, mut v_VEE: Py, mut v_RB: Py, mut v_RC: Py, mut v_RE: Py, mut v_beta: Py, mut v_VBE: Py) -> R {
    let mut v_IB: Py = Py::None;
    let mut v_IC: Py = Py::None;
    let mut v_IE: Py = Py::None;
    let mut v_VB: Py = Py::None;
    let mut v_VC: Py = Py::None;
    let mut v_VE: Py = Py::None;
    let _ = f__pos(Py::str("VCC, VEE, RB, RC, RE and VBE"), Py::tuple(vec![v_VCC.clone(), v_VEE.clone(), v_RB.clone(), v_RC.clone(), v_RE.clone(), v_VBE.clone()]))?;
    let _ = f__beta(v_beta.clone())?;
    let _ = f__need({ let t489 = v_VEE.clone(); Py::Bool({ let t490 = v_VBE.clone(); if py::gt(&t489, &t490)? { true } else { false } }) }, Py::str("VEE must exceed VBE"))?;
    v_IE = py::truediv(&py::sub(&v_VEE.clone(), &v_VBE.clone())?, &py::add(&v_RE.clone(), &py::truediv(&v_RB.clone(), &py::add(&v_beta.clone(), &Py::Float(1.0))?)?)?)?;
    v_IB = py::truediv(&v_IE.clone(), &py::add(&v_beta.clone(), &Py::Float(1.0))?)?;
    v_IC = py::mul(&v_beta.clone(), &v_IB.clone())?;
    v_VB = py::mul(&py::neg(&v_IB.clone())?, &v_RB.clone())?;
    v_VE = py::sub(&v_VB.clone(), &v_VBE.clone())?;
    v_VC = py::sub(&v_VCC.clone(), &py::mul(&v_IC.clone(), &v_RC.clone())?)?;
    let _ = f__active(py::sub(&v_VC.clone(), &v_VE.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("IB"), v_IB.clone()), (Py::str("IC"), v_IC.clone()), (Py::str("IE"), v_IE.clone()), (Py::str("VB"), v_VB.clone()), (Py::str("VE"), v_VE.clone()), (Py::str("VC"), v_VC.clone()), (Py::str("VCE"), py::sub(&v_VC.clone(), &v_VE.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_filter_order(mut v_ftype: Py, mut v_fp: Py, mut v_fs: Py, mut v_Ap: Py, mut v_As: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("fp, fs, Ap and As"), Py::tuple(vec![v_fp.clone(), v_fs.clone(), v_Ap.clone(), v_As.clone()]))?;
    let _ = f__need({ let t1988 = v_ftype.clone(); Py::Bool({ let t1989 = Py::tuple(vec![Py::str("Butterworth"), Py::str("Chebyshev")]); if py::contains(&t1989, &t1988)? { true } else { false } }) }, Py::str("type: butterworth or chebyshev"))?;
    let _ = f__need({ let t1990 = v_fs.clone(); Py::Bool({ let t1991 = v_fp.clone(); if py::gt(&t1990, &t1991)? { true } else { false } }) }, Py::str("the stopband edge must be above the passband edge (low-pass)"))?;
    let _ = f__need({ let t1992 = v_As.clone(); Py::Bool({ let t1993 = v_Ap.clone(); if py::gt(&t1992, &t1993)? { true } else { false } }) }, Py::str("the stopband attenuation must exceed the passband ripple"))?;
    v_d = py::truediv(&py::sub(&py::pow(&Py::Float(10.0), &py::truediv(&v_As.clone(), &Py::Float(10.0))?)?, &Py::Float(1.0))?, &py::sub(&py::pow(&Py::Float(10.0), &py::truediv(&v_Ap.clone(), &Py::Float(10.0))?)?, &Py::Float(1.0))?)?;
    if ({ let t1994 = v_ftype.clone(); Py::Bool({ let t1995 = Py::str("Butterworth"); if py::eq(&t1994, &t1995) { true } else { false } }) }).truthy() {
        v_x = py::truediv(&py::math1("log10", &v_d.clone())?, &py::mul(&Py::Float(2.0), &py::math1("log10", &py::truediv(&v_fs.clone(), &v_fp.clone())?)?)?)?;
    } else {
        v_x = py::truediv(&py::math1("acosh", &py::math1("sqrt", &v_d.clone())?)?, &py::math1("acosh", &py::truediv(&v_fs.clone(), &v_fp.clone())?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("n"), py::int(&py::math1("ceil", &v_x.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_filter_ripple(mut v_Vp: Py, mut v_f: Py, mut v_RL: Py, mut v_C: Py, mut v_circuit: Py) -> R {
    let mut v_Vdc: Py = Py::None;
    let mut v_Vr: Py = Py::None;
    let mut v_fr: Py = Py::None;
    let mut v_rms: Py = Py::None;
    let _ = f__pos(Py::str("Vp, f, RL and C"), Py::tuple(vec![v_Vp.clone(), v_f.clone(), v_RL.clone(), v_C.clone()]))?;
    let _ = f__need({ let t771 = v_circuit.clone(); Py::Bool({ let t772 = Py::tuple(vec![Py::str("half-wave"), Py::str("full-wave")]); if py::contains(&t772, &t771)? { true } else { false } }) }, Py::str("circuit: half-wave or full-wave"))?;
    v_fr = (if ({ let t773 = v_circuit.clone(); Py::Bool({ let t774 = Py::str("half-wave"); if py::eq(&t773, &t774) { true } else { false } }) }).truthy() { v_f.clone() } else { py::mul(&Py::Float(2.0), &v_f.clone())? });
    v_Vr = py::truediv(&v_Vp.clone(), &py::mul(&py::mul(&v_fr.clone(), &v_RL.clone())?, &v_C.clone())?)?;
    let _ = f__need({ let t775 = v_Vr.clone(); Py::Bool({ let t776 = v_Vp.clone(); if py::lt(&t775, &t776)? { true } else { false } }) }, Py::str("the ripple would exceed the peak: the small-ripple formula does not apply"))?;
    v_Vdc = py::sub(&v_Vp.clone(), &py::truediv(&v_Vr.clone(), &Py::Float(2.0))?)?;
    v_rms = py::truediv(&v_Vr.clone(), &py::mul(&Py::Float(2.0), &py::math1("sqrt", &Py::Float(3.0))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("f_ripple"), v_fr.clone()), (Py::str("Vr"), v_Vr.clone()), (Py::str("Vdc"), v_Vdc.clone()), (Py::str("Vr_rms"), v_rms.clone()), (Py::str("ripple_factor"), py::truediv(&v_rms.clone(), &v_Vdc.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_finite_gain_inverting(mut v_Rf: Py, mut v_R1: Py, mut v_A0: Py, mut v_Vin: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_Gi: Py = Py::None;
    let _ = f__pos(Py::str("Rf, R1 and A0"), Py::tuple(vec![v_Rf.clone(), v_R1.clone(), v_A0.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_Vin.clone()]))?;
    v_Gi = py::truediv(&py::neg(&v_Rf.clone())?, &v_R1.clone())?;
    v_G = py::truediv(&v_Gi.clone(), &py::add(&Py::Float(1.0), &py::truediv(&py::add(&Py::Float(1.0), &py::truediv(&v_Rf.clone(), &v_R1.clone())?)?, &v_A0.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("G"), v_G.clone()), (Py::str("G_ideal"), v_Gi.clone()), (Py::str("Vout"), py::mul(&v_G.clone(), &v_Vin.clone())?), (Py::str("error"), py::sub(&Py::Float(1.0), &py::truediv(&v_G.clone(), &v_Gi.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_order_lp(mut v_R1: Py, mut v_Rf: Py, mut v_C: Py, mut v_f: Py) -> R {
    let mut v_H0: Py = Py::None;
    let mut v_fc: Py = Py::None;
    let mut v_g: Py = Py::None;
    let _ = f__pos(Py::str("R1, Rf, C and f"), Py::tuple(vec![v_R1.clone(), v_Rf.clone(), v_C.clone(), v_f.clone()]))?;
    v_H0 = py::truediv(&py::neg(&v_Rf.clone())?, &v_R1.clone())?;
    v_fc = py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&c_TWO_PI()?, &v_Rf.clone())?, &v_C.clone())?)?;
    v_g = py::truediv(&py::abs(&v_H0.clone())?, &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::pow(&py::truediv(&v_f.clone(), &v_fc.clone())?, &Py::Int(2i64))?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("H0"), v_H0.clone()), (Py::str("fc"), v_fc.clone()), (Py::str("gain_f"), v_g.clone()), (Py::str("gain_dB"), f__db(v_g.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fixed_bias(mut v_VCC: Py, mut v_RB: Py, mut v_RC: Py, mut v_beta: Py, mut v_VBE: Py) -> R {
    let mut v_IB: Py = Py::None;
    let mut v_IC: Py = Py::None;
    let mut v_VCE: Py = Py::None;
    let _ = f__pos(Py::str("VCC, RB, RC and VBE"), Py::tuple(vec![v_VCC.clone(), v_RB.clone(), v_RC.clone(), v_VBE.clone()]))?;
    let _ = f__beta(v_beta.clone())?;
    let _ = f__need({ let t537 = v_VCC.clone(); Py::Bool({ let t538 = v_VBE.clone(); if py::gt(&t537, &t538)? { true } else { false } }) }, Py::str("VCC must exceed VBE"))?;
    v_IB = py::truediv(&py::sub(&v_VCC.clone(), &v_VBE.clone())?, &v_RB.clone())?;
    v_IC = py::mul(&v_beta.clone(), &v_IB.clone())?;
    v_VCE = py::sub(&v_VCC.clone(), &py::mul(&v_IC.clone(), &v_RC.clone())?)?;
    let _ = f__active(v_VCE.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("IB"), v_IB.clone()), (Py::str("IC"), v_IC.clone()), (Py::str("IE"), py::add(&v_IB.clone(), &v_IC.clone())?), (Py::str("VCE"), v_VCE.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hartley(mut v_L1: Py, mut v_L2: Py, mut v_M: Py, mut v_C: Py) -> R {
    let mut v_Leq: Py = Py::None;
    let _ = f__pos(Py::str("L1, L2 and C"), Py::tuple(vec![v_L1.clone(), v_L2.clone(), v_C.clone()]))?;
    let _ = f__nonneg(Py::str("the mutual inductance M"), Py::tuple(vec![v_M.clone()]))?;
    let _ = f__need({ let t2090 = v_M.clone(); Py::Bool({ let t2091 = py::math1("sqrt", &py::mul(&v_L1.clone(), &v_L2.clone())?)?; if py::le(&t2090, &t2091)? { true } else { false } }) }, Py::str("M cannot exceed sqrt(L1 L2)"))?;
    v_Leq = py::add(&py::add(&v_L1.clone(), &v_L2.clone())?, &py::mul(&Py::Float(2.0), &v_M.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("L_eq"), v_Leq.clone()), (Py::str("f0"), py::truediv(&Py::Float(1.0), &py::mul(&c_TWO_PI()?, &py::math1("sqrt", &py::mul(&v_Leq.clone(), &v_C.clone())?)?)?)?), (Py::str("A_min"), py::truediv(&py::add(&v_L1.clone(), &v_M.clone())?, &py::add(&v_L2.clone(), &v_M.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_heat_sink(mut v_P: Py, mut v_Ta: Py, mut v_theta_jc: Py, mut v_theta_cs: Py, mut v_theta_sa: Py) -> R {
    let mut v_Tc: Py = Py::None;
    let mut v_Tj: Py = Py::None;
    let mut v_Ts: Py = Py::None;
    let _ = f__pos(Py::str("P, Ta and the thermal resistances"), Py::tuple(vec![v_P.clone(), v_Ta.clone(), v_theta_jc.clone(), v_theta_cs.clone(), v_theta_sa.clone()]))?;
    v_Ts = py::add(&v_Ta.clone(), &py::mul(&v_P.clone(), &v_theta_sa.clone())?)?;
    v_Tc = py::add(&v_Ts.clone(), &py::mul(&v_P.clone(), &v_theta_cs.clone())?)?;
    v_Tj = py::add(&v_Tc.clone(), &py::mul(&v_P.clone(), &v_theta_jc.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Tj"), v_Tj.clone()), (Py::str("Tc"), v_Tc.clone()), (Py::str("Ts"), v_Ts.clone()), (Py::str("Tj_C"), py::sub(&v_Tj.clone(), &c_T0C()?)?), (Py::str("theta_ja"), py::add(&py::add(&v_theta_jc.clone(), &v_theta_cs.clone())?, &v_theta_sa.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_heat_sink_required(mut v_P: Py, mut v_Ta: Py, mut v_Tj_max: Py, mut v_theta_jc: Py, mut v_theta_cs: Py) -> R {
    let mut v_tja: Py = Py::None;
    let mut v_tsa: Py = Py::None;
    let _ = f__pos(Py::str("P, Ta, Tj_max and the thermal resistances"), Py::tuple(vec![v_P.clone(), v_Ta.clone(), v_Tj_max.clone(), v_theta_jc.clone(), v_theta_cs.clone()]))?;
    v_tja = py::truediv(&py::sub(&v_Tj_max.clone(), &v_Ta.clone())?, &v_P.clone())?;
    v_tsa = py::sub(&py::sub(&v_tja.clone(), &v_theta_jc.clone())?, &v_theta_cs.clone())?;
    let _ = f__need({ let t2136 = v_tsa.clone(); Py::Bool({ let t2137 = Py::Int(0i64); if py::gt(&t2136, &t2137)? { true } else { false } }) }, Py::str("no heat sink can keep the junction below Tj_max at this power (the device and interface alone are too hot)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta_ja_max"), v_tja.clone()), (Py::str("theta_sa_max"), v_tsa.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inamp(mut v_R1: Py, mut v_RG: Py, mut v_R2: Py, mut v_R3: Py, mut v_V1: Py, mut v_V2: Py) -> R {
    let mut v_G: Py = Py::None;
    let _ = f__pos(Py::str("R1, RG, R2 and R3"), Py::tuple(vec![v_R1.clone(), v_RG.clone(), v_R2.clone(), v_R3.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_V1.clone(), v_V2.clone()]))?;
    v_G = py::mul(&py::add(&Py::Float(1.0), &py::truediv(&py::mul(&Py::Float(2.0), &v_R1.clone())?, &v_RG.clone())?)?, &py::truediv(&v_R3.clone(), &v_R2.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("G"), v_G.clone()), (Py::str("G_stage1"), py::add(&Py::Float(1.0), &py::truediv(&py::mul(&Py::Float(2.0), &v_R1.clone())?, &v_RG.clone())?)?), (Py::str("Vout"), py::mul(&v_G.clone(), &py::sub(&v_V2.clone(), &v_V1.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_integrator(mut v_R: Py, mut v_C: Py, mut v_Vin: Py, mut v_t: Py, mut v_V0: Py) -> R {
    let mut v_slope: Py = Py::None;
    let _ = f__pos(Py::str("R, C and t"), Py::tuple(vec![v_R.clone(), v_C.clone(), v_t.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_Vin.clone(), v_V0.clone()]))?;
    v_slope = py::truediv(&py::neg(&v_Vin.clone())?, &py::mul(&v_R.clone(), &v_C.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vout"), py::add(&v_V0.clone(), &py::mul(&v_slope.clone(), &v_t.clone())?)?), (Py::str("slope"), v_slope.clone()), (Py::str("f_unity"), py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&c_TWO_PI()?, &v_R.clone())?, &v_C.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_intrinsic_concentration(mut v_Nc: Py, mut v_Nv: Py, mut v_Eg: Py, mut v_T: Py) -> R {
    let mut v_kT: Py = Py::None;
    let mut v_nc: Py = Py::None;
    let mut v_ni: Py = Py::None;
    let mut v_nv: Py = Py::None;
    let mut v_off: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("Nc, Nv, Eg and T"), Py::tuple(vec![v_Nc.clone(), v_Nv.clone(), v_Eg.clone(), v_T.clone()]))?;
    v_s = py::pow(&py::truediv(&v_T.clone(), &Py::Float(300.0))?, &Py::Float(1.5))?;
    let t2273 = (Py::tuple(vec![py::mul(&v_Nc.clone(), &v_s.clone())?, py::mul(&v_Nv.clone(), &v_s.clone())?])).items()?;
    if t2273.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nc = t2273[0].clone();
    v_nv = t2273[1].clone();
    v_kT = py::mul(&c_K_B()?, &v_T.clone())?;
    v_ni = py::mul(&py::math1("sqrt", &py::mul(&v_nc.clone(), &v_nv.clone())?)?, &py::math1("exp", &py::truediv(&py::neg(&v_Eg.clone())?, &py::mul(&Py::Float(2.0), &v_kT.clone())?)?)?)?;
    v_off = py::truediv(&py::mul(&py::mul(&Py::Float(0.5), &v_kT.clone())?, &py::math1("log", &py::truediv(&v_nv.clone(), &v_nc.clone())?)?)?, &c_Q_E()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("ni"), v_ni.clone()), (Py::str("ni_cm3"), py::mul(&v_ni.clone(), &Py::Float(1e-06))?), (Py::str("Ei_minus_midgap_eV"), v_off.clone()), (Py::str("Nc_T"), v_nc.clone()), (Py::str("Nv_T"), v_nv.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inverting_amp(mut v_Rf: Py, mut v_Rin: Py, mut v_Vin: Py) -> R {
    let mut v_G: Py = Py::None;
    let _ = f__pos(Py::str("Rf and Rin"), Py::tuple(vec![v_Rf.clone(), v_Rin.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_Vin.clone()]))?;
    v_G = py::truediv(&py::neg(&v_Rf.clone())?, &v_Rin.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("G"), v_G.clone()), (Py::str("Vout"), py::mul(&v_G.clone(), &v_Vin.clone())?), (Py::str("Zin"), v_Rin.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_jitter_snr(mut v_f: Py, mut v_tj: Py, mut v_n: Py) -> R {
    let mut v_sj: Py = Py::None;
    let mut v_sq: Py = Py::None;
    let mut v_st: Py = Py::None;
    let _ = f__pos(Py::str("f and tj"), Py::tuple(vec![v_f.clone(), v_tj.clone()]))?;
    let _ = f__need({ let t300 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t300.truthy() { t300 } else { { let t297 = Py::Int(1i64); Py::Bool({ let t298 = v_n.clone(); if py::le(&t297, &t298)? { { let t299 = Py::Int(32i64); if py::le(&t298, &t299)? { true } else { false } } } else { false } }) } } }, Py::str("the resolution must be a whole number of bits from 1 to 32"))?;
    v_sj = py::mul(&py::neg(&Py::Float(20.0))?, &py::math1("log10", &py::mul(&py::mul(&c_TWO_PI()?, &v_f.clone())?, &v_tj.clone())?)?)?;
    v_sq = py::mul(&Py::Float(20.0), &py::math1("log10", &py::mul(&py::pow(&Py::Float(2.0), &v_n.clone())?, &py::math1("sqrt", &Py::Float(1.5))?)?)?)?;
    v_st = py::mul(&py::neg(&Py::Float(10.0))?, &py::math1("log10", &py::add(&py::pow(&Py::Float(10.0), &py::truediv(&py::neg(&v_sj.clone())?, &Py::Float(10.0))?)?, &py::pow(&Py::Float(10.0), &py::truediv(&py::neg(&v_sq.clone())?, &Py::Float(10.0))?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("SNR_jitter_dB"), v_sj.clone()), (Py::str("SNR_quant_dB"), v_sq.clone()), (Py::str("SNR_total_dB"), v_st.clone()), (Py::str("ENOB_total"), py::truediv(&py::sub(&v_st.clone(), &Py::Float(1.76))?, &Py::Float(6.02))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_junction_capacitance(mut v_Cj0: Py, mut v_Vbi: Py, mut v_VR: Py, mut v_m: Py) -> R {
    let mut v_ratio: Py = Py::None;
    let _ = f__pos(Py::str("Cj0 and Vbi"), Py::tuple(vec![v_Cj0.clone(), v_Vbi.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_VR.clone(), v_m.clone()]))?;
    let _ = f__need({ let t2415 = Py::Float(0.0); Py::Bool({ let t2416 = v_m.clone(); if py::lt(&t2415, &t2416)? { { let t2417 = Py::Float(1.0); if py::lt(&t2416, &t2417)? { true } else { false } } } else { false } }) }, Py::str("the grading coefficient m must lie between 0 and 1 (1/2 abrupt, 1/3 linearly graded)"))?;
    let _ = f__need({ let t2418 = v_VR.clone(); Py::Bool({ let t2419 = py::neg(&v_Vbi.clone())?; if py::gt(&t2418, &t2419)? { true } else { false } }) }, Py::str("VR must exceed -Vbi"))?;
    v_ratio = py::pow(&py::add(&Py::Float(1.0), &py::truediv(&v_VR.clone(), &v_Vbi.clone())?)?, &py::neg(&v_m.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Cj"), py::mul(&v_Cj0.clone(), &v_ratio.clone())?), (Py::str("ratio"), v_ratio.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_led_resistor(mut v_Vs: Py, mut v_Vf: Py, mut v_If: Py, mut v_n: Py) -> R {
    let mut v_R: Py = Py::None;
    let _ = f__pos(Py::str("Vs, Vf and If"), Py::tuple(vec![v_Vs.clone(), v_Vf.clone(), v_If.clone()]))?;
    let _ = f__need({ let t2466 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2466.truthy() { t2466 } else { { let t2464 = v_n.clone(); Py::Bool({ let t2465 = Py::Int(1i64); if py::ge(&t2464, &t2465)? { true } else { false } }) } } }, Py::str("the number of LEDs in series must be a whole number of at least 1"))?;
    let _ = f__need({ let t2467 = v_Vs.clone(); Py::Bool({ let t2468 = py::mul(&v_n.clone(), &v_Vf.clone())?; if py::gt(&t2467, &t2468)? { true } else { false } }) }, Py::str("the supply must exceed the total forward voltage"))?;
    v_R = py::truediv(&py::sub(&v_Vs.clone(), &py::mul(&v_n.clone(), &v_Vf.clone())?)?, &v_If.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("P_R"), py::mul(&py::mul(&v_If.clone(), &v_If.clone())?, &v_R.clone())?), (Py::str("P_LED"), py::mul(&py::mul(&v_n.clone(), &v_Vf.clone())?, &v_If.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_regulator(mut v_Vin: Py, mut v_Vout: Py, mut v_Iload: Py, mut v_Iq: Py) -> R {
    let mut v_Pin: Py = Py::None;
    let mut v_Pout: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout and Iload"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_Iload.clone()]))?;
    let _ = f__nonneg(Py::str("the quiescent current Iq"), Py::tuple(vec![v_Iq.clone()]))?;
    let _ = f__need({ let t2519 = v_Vin.clone(); Py::Bool({ let t2520 = v_Vout.clone(); if py::gt(&t2519, &t2520)? { true } else { false } }) }, Py::str("the input must exceed the output"))?;
    v_Pin = py::mul(&v_Vin.clone(), &py::add(&v_Iload.clone(), &v_Iq.clone())?)?;
    v_Pout = py::mul(&v_Vout.clone(), &v_Iload.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("P_in"), v_Pin.clone()), (Py::str("P_out"), v_Pout.clone()), (Py::str("P_diss"), py::sub(&v_Pin.clone(), &v_Pout.clone())?), (Py::str("efficiency"), py::truediv(&v_Pout.clone(), &v_Pin.clone())?), (Py::str("efficiency_pct"), py::truediv(&py::mul(&Py::Float(100.0), &v_Pout.clone())?, &v_Pin.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mfb_bandpass(mut v_R1: Py, mut v_R2: Py, mut v_R3: Py, mut v_C1: Py, mut v_C2: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_f0: Py = Py::None;
    let mut v_w0: Py = Py::None;
    let _ = f__pos(Py::str("R1, R2, R3, C1 and C2"), Py::tuple(vec![v_R1.clone(), v_R2.clone(), v_R3.clone(), v_C1.clone(), v_C2.clone()]))?;
    v_w0 = py::math1("sqrt", &py::truediv(&py::add(&v_R1.clone(), &v_R2.clone())?, &py::mul(&py::mul(&py::mul(&py::mul(&v_R1.clone(), &v_R2.clone())?, &v_R3.clone())?, &v_C1.clone())?, &v_C2.clone())?)?)?;
    v_Q = py::truediv(&py::mul(&py::mul(&py::mul(&v_w0.clone(), &v_C1.clone())?, &v_C2.clone())?, &v_R3.clone())?, &py::add(&v_C1.clone(), &v_C2.clone())?)?;
    v_f0 = py::truediv(&v_w0.clone(), &c_TWO_PI()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("f0"), v_f0.clone()), (Py::str("Q"), v_Q.clone()), (Py::str("BW"), py::truediv(&v_f0.clone(), &v_Q.clone())?), (Py::str("H_peak"), py::truediv(&py::mul(&py::neg(&v_C1.clone())?, &v_R3.clone())?, &py::mul(&v_R1.clone(), &py::add(&v_C1.clone(), &v_C2.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mfb_lowpass(mut v_R1: Py, mut v_R2: Py, mut v_R3: Py, mut v_C1: Py, mut v_C2: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let _ = f__pos(Py::str("R1, R2, R3, C1 and C2"), Py::tuple(vec![v_R1.clone(), v_R2.clone(), v_R3.clone(), v_C1.clone(), v_C2.clone()]))?;
    v_a = py::mul(&py::mul(&py::mul(&v_R2.clone(), &v_R3.clone())?, &v_C1.clone())?, &v_C2.clone())?;
    v_b = py::mul(&v_C2.clone(), &py::add(&py::add(&v_R2.clone(), &v_R3.clone())?, &py::truediv(&py::mul(&v_R2.clone(), &v_R3.clone())?, &v_R1.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("f0"), py::truediv(&Py::Float(1.0), &py::mul(&c_TWO_PI()?, &py::math1("sqrt", &v_a.clone())?)?)?), (Py::str("Q"), py::truediv(&py::math1("sqrt", &v_a.clone())?, &v_b.clone())?), (Py::str("H0"), py::truediv(&py::neg(&v_R2.clone())?, &v_R1.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_miller_octc(mut v_gm: Py, mut v_r_pi: Py, mut v_Cpi: Py, mut v_Cmu: Py, mut v_RLp: Py, mut v_Rsigp: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_CM: Py = Py::None;
    let mut v_Rp: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let _ = f__pos(Py::str("gm, r_pi, Cpi, Cmu, RLp and Rsigp"), Py::tuple(vec![v_gm.clone(), v_r_pi.clone(), v_Cpi.clone(), v_Cmu.clone(), v_RLp.clone(), v_Rsigp.clone()]))?;
    v_A = py::mul(&py::neg(&v_gm.clone())?, &v_RLp.clone())?;
    v_CM = py::mul(&v_Cmu.clone(), &py::sub(&Py::Float(1.0), &v_A.clone())?)?;
    v_Rp = f__par(Py::tuple(vec![v_r_pi.clone(), v_Rsigp.clone()]))?;
    v_fm = py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&c_TWO_PI()?, &v_Rp.clone())?, &py::add(&v_Cpi.clone(), &v_CM.clone())?)?)?;
    v_tau = py::add(&py::mul(&v_Cpi.clone(), &v_Rp.clone())?, &py::mul(&v_Cmu.clone(), &py::add(&py::add(&v_Rp.clone(), &v_RLp.clone())?, &py::mul(&py::mul(&v_gm.clone(), &v_Rp.clone())?, &v_RLp.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("A_mid"), v_A.clone()), (Py::str("C_M"), v_CM.clone()), (Py::str("R_pi_eq"), v_Rp.clone()), (Py::str("f_H_miller"), v_fm.clone()), (Py::str("tau_oc"), v_tau.clone()), (Py::str("f_H_octc"), py::truediv(&Py::Float(1.0), &py::mul(&c_TWO_PI()?, &v_tau.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mosfet_divider_bias(mut v_VDD: Py, mut v_R1: Py, mut v_R2: Py, mut v_RD: Py, mut v_RS: Py, mut v_kn: Py, mut v_Vt: Py) -> R {
    let mut v_ID: Py = Py::None;
    let mut v_VD: Py = Py::None;
    let mut v_VG: Py = Py::None;
    let mut v_VOV: Py = Py::None;
    let mut v_VS: Py = Py::None;
    let mut v_a: Py = Py::None;
    let _ = f__pos(Py::str("VDD, R1, R2, RD, kn and Vt"), Py::tuple(vec![v_VDD.clone(), v_R1.clone(), v_R2.clone(), v_RD.clone(), v_kn.clone(), v_Vt.clone()]))?;
    let _ = f__nonneg(Py::str("RS"), Py::tuple(vec![v_RS.clone()]))?;
    v_VG = py::truediv(&py::mul(&v_VDD.clone(), &v_R2.clone())?, &py::add(&v_R1.clone(), &v_R2.clone())?)?;
    let _ = f__need({ let t2846 = v_VG.clone(); Py::Bool({ let t2847 = v_Vt.clone(); if py::gt(&t2846, &t2847)? { true } else { false } }) }, Py::str("the gate voltage is below the threshold: the transistor is off"))?;
    v_a = py::sub(&v_VG.clone(), &v_Vt.clone())?;
    v_VOV = py::truediv(&py::mul(&Py::Float(2.0), &v_a.clone())?, &py::add(&Py::Float(1.0), &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::mul(&py::mul(&py::mul(&Py::Float(2.0), &v_kn.clone())?, &v_RS.clone())?, &v_a.clone())?)?)?)?)?;
    v_ID = py::mul(&py::mul(&py::truediv(&v_kn.clone(), &Py::Float(2.0))?, &v_VOV.clone())?, &v_VOV.clone())?;
    v_VS = py::mul(&v_ID.clone(), &v_RS.clone())?;
    v_VD = py::sub(&v_VDD.clone(), &py::mul(&v_ID.clone(), &v_RD.clone())?)?;
    let _ = f__need({ let t2848 = py::sub(&v_VD.clone(), &v_VS.clone())?; Py::Bool({ let t2849 = v_VOV.clone(); if py::ge(&t2848, &t2849)? { true } else { false } }) }, Py::str("the transistor is in the triode region (VDS < VOV): the saturation bias equations do not apply"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("VG"), v_VG.clone()), (Py::str("VGS"), py::add(&v_Vt.clone(), &v_VOV.clone())?), (Py::str("VOV"), v_VOV.clone()), (Py::str("ID"), v_ID.clone()), (Py::str("VS"), v_VS.clone()), (Py::str("VD"), v_VD.clone()), (Py::str("VDS"), py::sub(&v_VD.clone(), &v_VS.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mosfet_gm_ro(mut v_kn: Py, mut v_ID: Py, mut v_lam: Py, mut v_Vt: Py) -> R {
    let mut v_VOV: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_ro: Py = Py::None;
    let _ = f__pos(Py::str("kn, ID, lambda and Vt"), Py::tuple(vec![v_kn.clone(), v_ID.clone(), v_lam.clone(), v_Vt.clone()]))?;
    v_VOV = py::math1("sqrt", &py::truediv(&py::mul(&Py::Float(2.0), &v_ID.clone())?, &v_kn.clone())?)?;
    v_gm = py::math1("sqrt", &py::mul(&py::mul(&Py::Float(2.0), &v_kn.clone())?, &v_ID.clone())?)?;
    v_ro = py::truediv(&Py::Float(1.0), &py::mul(&v_lam.clone(), &v_ID.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("VOV"), v_VOV.clone()), (Py::str("VGS"), py::add(&v_Vt.clone(), &v_VOV.clone())?), (Py::str("gm"), v_gm.clone()), (Py::str("r_o"), v_ro.clone()), (Py::str("A0"), py::mul(&v_gm.clone(), &v_ro.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mosfet_id(mut v_kn: Py, mut v_Vt: Py, mut v_VGS: Py, mut v_VDS: Py, mut v_lam: Py) -> R {
    let mut v_VOV: Py = Py::None;
    let _ = f__pos(Py::str("kn and Vt"), Py::tuple(vec![v_kn.clone(), v_Vt.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_VGS.clone()]))?;
    let _ = f__nonneg(Py::str("VDS and lambda"), Py::tuple(vec![v_VDS.clone(), v_lam.clone()]))?;
    v_VOV = py::sub(&v_VGS.clone(), &v_Vt.clone())?;
    if ({ let t2903 = v_VOV.clone(); Py::Bool({ let t2904 = Py::Int(0i64); if py::le(&t2903, &t2904)? { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("region"), Py::str("cutoff")), (Py::str("ID"), Py::Float(0.0)), (Py::str("VOV"), v_VOV.clone()), (Py::str("VDS_sat"), Py::Float(0.0))]))?);
    }
    if ({ let t2917 = v_VDS.clone(); Py::Bool({ let t2918 = v_VOV.clone(); if py::lt(&t2917, &t2918)? { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("region"), Py::str("triode")), (Py::str("ID"), py::mul(&v_kn.clone(), &py::sub(&py::mul(&v_VOV.clone(), &v_VDS.clone())?, &py::truediv(&py::mul(&v_VDS.clone(), &v_VDS.clone())?, &Py::Float(2.0))?)?)?), (Py::str("VOV"), v_VOV.clone()), (Py::str("VDS_sat"), v_VOV.clone())]))?);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("region"), Py::str("saturation")), (Py::str("ID"), py::mul(&py::mul(&py::mul(&py::truediv(&v_kn.clone(), &Py::Float(2.0))?, &v_VOV.clone())?, &v_VOV.clone())?, &py::add(&Py::Float(1.0), &py::mul(&v_lam.clone(), &v_VDS.clone())?)?)?), (Py::str("VOV"), v_VOV.clone()), (Py::str("VDS_sat"), v_VOV.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mtbf(mut v_tr: Py, mut v_tau: Py, mut v_T0: Py, mut v_fclk: Py, mut v_fdata: Py) -> R {
    let mut v_m: Py = Py::None;
    let _ = f__pos(Py::str("tr, tau, T0, fclk and fdata"), Py::tuple(vec![v_tr.clone(), v_tau.clone(), v_T0.clone(), v_fclk.clone(), v_fdata.clone()]))?;
    let _ = f__need({ let t2611 = py::truediv(&v_tr.clone(), &v_tau.clone())?; Py::Bool({ let t2612 = Py::Float(700.0); if py::le(&t2611, &t2612)? { true } else { false } }) }, Py::str("tr/tau is beyond the range of a double-precision exponential"))?;
    v_m = py::truediv(&py::math1("exp", &py::truediv(&v_tr.clone(), &v_tau.clone())?)?, &py::mul(&py::mul(&v_T0.clone(), &v_fclk.clone())?, &v_fdata.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("MTBF_s"), v_m.clone()), (Py::str("MTBF_years"), py::truediv(&v_m.clone(), &py::mul(&Py::Float(365.25), &Py::Float(86400.0))?)?), (Py::str("log10_MTBF_s"), py::sub(&py::truediv(&py::truediv(&v_tr.clone(), &v_tau.clone())?, &py::math1("log", &Py::Float(10.0))?)?, &py::math1("log10", &py::mul(&py::mul(&v_T0.clone(), &v_fclk.clone())?, &v_fdata.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_neg_feedback(mut v_A: Py, mut v_beta: Py, mut v_fH: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_L: Py = Py::None;
    let _ = f__pos(Py::str("A, beta and fH"), Py::tuple(vec![v_A.clone(), v_beta.clone(), v_fH.clone()]))?;
    v_L = py::mul(&v_A.clone(), &v_beta.clone())?;
    v_D = py::add(&Py::Float(1.0), &v_L.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Af"), py::truediv(&v_A.clone(), &v_D.clone())?), (Py::str("loop_gain"), v_L.clone()), (Py::str("desensitivity"), v_D.clone()), (Py::str("desensitivity_dB"), f__db(v_D.clone())?), (Py::str("fH_closed"), py::mul(&v_fH.clone(), &v_D.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_noninverting_amp(mut v_Rf: Py, mut v_Rg: Py, mut v_Vin: Py) -> R {
    let mut v_G: Py = Py::None;
    let _ = f__pos(Py::str("Rf and Rg"), Py::tuple(vec![v_Rf.clone(), v_Rg.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_Vin.clone()]))?;
    v_G = py::add(&Py::Float(1.0), &py::truediv(&v_Rf.clone(), &v_Rg.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("G"), v_G.clone()), (Py::str("Vout"), py::mul(&v_G.clone(), &v_Vin.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_opamp_offset(mut v_Vos: Py, mut v_IB: Py, mut v_Ios: Py, mut v_Rf: Py, mut v_R1: Py, mut v_Rp: Py) -> R {
    let mut v_IB1: Py = Py::None;
    let mut v_IB2: Py = Py::None;
    let mut v_NG: Py = Py::None;
    let mut v_vb: Py = Py::None;
    let mut v_vv: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_Vos.clone(), v_IB.clone(), v_Ios.clone()]))?;
    let _ = f__pos(Py::str("Rf and R1"), Py::tuple(vec![v_Rf.clone(), v_R1.clone()]))?;
    let _ = f__nonneg(Py::str("Rp"), Py::tuple(vec![v_Rp.clone()]))?;
    v_NG = py::add(&Py::Float(1.0), &py::truediv(&v_Rf.clone(), &v_R1.clone())?)?;
    let t3095 = (Py::tuple(vec![py::add(&v_IB.clone(), &py::truediv(&v_Ios.clone(), &Py::Float(2.0))?)?, py::sub(&v_IB.clone(), &py::truediv(&v_Ios.clone(), &Py::Float(2.0))?)?])).items()?;
    if t3095.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_IB1 = t3095[0].clone();
    v_IB2 = t3095[1].clone();
    v_vv = py::mul(&v_Vos.clone(), &v_NG.clone())?;
    v_vb = py::sub(&py::mul(&v_IB1.clone(), &v_Rf.clone())?, &py::mul(&py::mul(&v_IB2.clone(), &v_Rp.clone())?, &v_NG.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vout_Vos"), v_vv.clone()), (Py::str("Vout_bias"), v_vb.clone()), (Py::str("Vout_total"), py::add(&v_vv.clone(), &v_vb.clone())?), (Py::str("Rp_optimal"), f__par(Py::tuple(vec![v_R1.clone(), v_Rf.clone()]))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_phase_margin(mut v_A0: Py, mut v_f1: Py, mut v_f2: Py, mut v_beta: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_fc: Py = Py::None;
    let mut v_pm: Py = Py::None;
    let mut v_u: Py = Py::None;
    let _ = f__pos(Py::str("A0, f1, f2 and beta"), Py::tuple(vec![v_A0.clone(), v_f1.clone(), v_f2.clone(), v_beta.clone()]))?;
    v_K = py::mul(&v_A0.clone(), &v_beta.clone())?;
    let _ = f__need({ let t4219 = v_K.clone(); Py::Bool({ let t4220 = Py::Float(1.0); if py::gt(&t4219, &t4220)? { true } else { false } }) }, Py::str("the loop gain A0 beta must exceed 1 for a crossover to exist"))?;
    v_a = py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&py::mul(&v_f1.clone(), &v_f1.clone())?, &v_f2.clone())?, &v_f2.clone())?)?;
    v_b = py::add(&py::truediv(&Py::Float(1.0), &py::mul(&v_f1.clone(), &v_f1.clone())?)?, &py::truediv(&Py::Float(1.0), &py::mul(&v_f2.clone(), &v_f2.clone())?)?)?;
    v_u = py::truediv(&py::mul(&Py::Float(2.0), &py::sub(&py::mul(&v_K.clone(), &v_K.clone())?, &Py::Float(1.0))?)?, &py::add(&v_b.clone(), &py::math1("sqrt", &py::add(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&py::mul(&Py::Float(4.0), &v_a.clone())?, &py::sub(&py::mul(&v_K.clone(), &v_K.clone())?, &Py::Float(1.0))?)?)?)?)?)?;
    v_fc = py::math1("sqrt", &v_u.clone())?;
    v_pm = py::sub(&py::sub(&Py::Float(180.0), &py::math1("degrees", &py::math1("atan", &py::truediv(&v_fc.clone(), &v_f1.clone())?)?)?)?, &py::math1("degrees", &py::math1("atan", &py::truediv(&v_fc.clone(), &v_f2.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("f_c"), v_fc.clone()), (Py::str("PM_deg"), v_pm.clone()), (Py::str("Af"), py::truediv(&v_A0.clone(), &py::add(&Py::Float(1.0), &v_K.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pll(mut v_Kd: Py, mut v_Ko: Py, mut v_tau1: Py, mut v_tau2: Py) -> R {
    let mut v_wn: Py = Py::None;
    let _ = f__pos(Py::str("Kd, Ko, tau1 and tau2"), Py::tuple(vec![v_Kd.clone(), v_Ko.clone(), v_tau1.clone(), v_tau2.clone()]))?;
    v_wn = py::math1("sqrt", &py::truediv(&py::mul(&v_Kd.clone(), &v_Ko.clone())?, &v_tau1.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("wn"), v_wn.clone()), (Py::str("fn"), py::truediv(&v_wn.clone(), &c_TWO_PI()?)?), (Py::str("zeta"), py::truediv(&py::mul(&v_tau2.clone(), &v_wn.clone())?, &Py::Float(2.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rc_phase_shift(mut v_R: Py, mut v_C: Py, mut v_ladder: Py) -> R {
    let mut v_f: Py = Py::None;
    let _ = f__pos(Py::str("R and C"), Py::tuple(vec![v_R.clone(), v_C.clone()]))?;
    let _ = f__need({ let t3364 = v_ladder.clone(); Py::Bool({ let t3365 = Py::tuple(vec![Py::str("high-pass"), Py::str("low-pass")]); if py::contains(&t3365, &t3364)? { true } else { false } }) }, Py::str("ladder: highpass (series C, shunt R) or lowpass (series R, shunt C)"))?;
    v_f = (if ({ let t3366 = v_ladder.clone(); Py::Bool({ let t3367 = Py::str("high-pass"); if py::eq(&t3366, &t3367) { true } else { false } }) }).truthy() { py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&py::mul(&c_TWO_PI()?, &py::math1("sqrt", &Py::Float(6.0))?)?, &v_R.clone())?, &v_C.clone())?)? } else { py::truediv(&py::math1("sqrt", &Py::Float(6.0))?, &py::mul(&py::mul(&c_TWO_PI()?, &v_R.clone())?, &v_C.clone())?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("f0"), v_f.clone()), (Py::str("A_min"), Py::Float(29.0)), (Py::str("beta"), py::truediv(&Py::Float(1.0), &Py::Float(29.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rc_timing(mut v_R: Py, mut v_C: Py) -> R {
    let mut v_f3: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let _ = f__pos(Py::str("R and C"), Py::tuple(vec![v_R.clone(), v_C.clone()]))?;
    v_tau = py::mul(&v_R.clone(), &v_C.clone())?;
    v_tr = py::mul(&py::math1("log", &Py::Float(9.0))?, &v_tau.clone())?;
    v_f3 = py::truediv(&Py::Float(1.0), &py::mul(&c_TWO_PI()?, &v_tau.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("tau"), v_tau.clone()), (Py::str("t50"), py::mul(&c_LN2()?, &v_tau.clone())?), (Py::str("t_rise"), v_tr.clone()), (Py::str("f_3dB"), v_f3.clone()), (Py::str("tr_times_bw"), py::mul(&v_tr.clone(), &v_f3.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rectifier(mut v_Vm: Py, mut v_circuit: Py, mut v_Von: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dc: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_piv: Py = Py::None;
    let mut v_span: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let _ = f__pos(Py::str("the peak voltage Vm"), Py::tuple(vec![v_Vm.clone()]))?;
    let _ = f__nonneg(Py::str("the diode drop Von"), Py::tuple(vec![v_Von.clone()]))?;
    let _ = f__need({ let t3418 = v_circuit.clone(); Py::Bool({ let t3419 = c__RECT()?; if py::contains(&t3419, &t3418)? { true } else { false } }) }, Py::str("circuit: half-wave, bridge or center-tapped"))?;
    v_V = (if ({ let t3420 = v_circuit.clone(); Py::Bool({ let t3421 = Py::str("bridge"); if py::eq(&t3420, &t3421) { true } else { false } }) }).truthy() { py::mul(&Py::Float(2.0), &v_Von.clone())? } else { v_Von.clone() });
    let _ = f__need({ let t3422 = v_V.clone(); Py::Bool({ let t3423 = v_Vm.clone(); if py::lt(&t3422, &t3423)? { true } else { false } }) }, Py::str("the diode drops must be below the peak voltage"))?;
    v_t1 = py::math1("asin", &py::truediv(&v_V.clone(), &v_Vm.clone())?)?;
    v_c = py::math1("cos", &v_t1.clone())?;
    v_span = py::sub(&Py::Float(std::f64::consts::PI), &py::mul(&Py::Float(2.0), &v_t1.clone())?)?;
    v_dc = py::truediv(&py::sub(&py::mul(&py::mul(&Py::Float(2.0), &v_Vm.clone())?, &v_c.clone())?, &py::mul(&v_V.clone(), &v_span.clone())?)?, &c_TWO_PI()?)?;
    v_ms = py::truediv(&py::add(&py::sub(&py::mul(&py::mul(&v_Vm.clone(), &v_Vm.clone())?, &py::add(&py::truediv(&v_span.clone(), &Py::Float(2.0))?, &py::truediv(&py::math1("sin", &py::mul(&Py::Float(2.0), &v_t1.clone())?)?, &Py::Float(2.0))?)?)?, &py::mul(&py::mul(&py::mul(&Py::Float(4.0), &v_Vm.clone())?, &v_V.clone())?, &v_c.clone())?)?, &py::mul(&py::mul(&v_V.clone(), &v_V.clone())?, &v_span.clone())?)?, &c_TWO_PI()?)?;
    if ({ let t3424 = v_circuit.clone(); Py::Bool({ let t3425 = Py::str("half-wave"); if py::ne(&t3424, &t3425) { true } else { false } }) }).truthy() {
        let t3426 = (Py::tuple(vec![py::mul(&Py::Float(2.0), &v_dc.clone())?, py::mul(&Py::Float(2.0), &v_ms.clone())?])).items()?;
        if t3426.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_dc = t3426[0].clone();
        v_ms = t3426[1].clone();
    }
    v_piv = py::getitem(&Py::dict(vec![(Py::str("half-wave"), v_Vm.clone()), (Py::str("bridge"), py::sub(&v_Vm.clone(), &v_Von.clone())?), (Py::str("center-tapped"), py::sub(&py::mul(&Py::Float(2.0), &v_Vm.clone())?, &v_Von.clone())?)]), &v_circuit.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vp_out"), py::sub(&v_Vm.clone(), &v_V.clone())?), (Py::str("Vdc"), v_dc.clone()), (Py::str("Vrms"), py::math1("sqrt", &v_ms.clone())?), (Py::str("ripple_factor"), py::math1("sqrt", &py::sub(&py::truediv(&v_ms.clone(), &py::mul(&v_dc.clone(), &v_dc.clone())?)?, &Py::Float(1.0))?)?), (Py::str("PIV"), v_piv.clone()), (Py::str("conduction_deg"), py::math1("degrees", &v_span.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_relaxation_oscillator(mut v_R: Py, mut v_C: Py, mut v_R1: Py, mut v_R2: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_b: Py = Py::None;
    let _ = f__pos(Py::str("R, C, R1 and R2"), Py::tuple(vec![v_R.clone(), v_C.clone(), v_R1.clone(), v_R2.clone()]))?;
    v_b = py::truediv(&v_R1.clone(), &py::add(&v_R1.clone(), &v_R2.clone())?)?;
    v_T = py::mul(&py::mul(&py::mul(&Py::Float(2.0), &v_R.clone())?, &v_C.clone())?, &py::math1("log", &py::truediv(&py::add(&Py::Float(1.0), &v_b.clone())?, &py::sub(&Py::Float(1.0), &v_b.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("beta"), v_b.clone()), (Py::str("T"), v_T.clone()), (Py::str("f"), py::truediv(&Py::Float(1.0), &v_T.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sample_hold(mut v_I_leak: Py, mut v_C: Py, mut v_t_hold: Py, mut v_Vref: Py, mut v_n: Py) -> R {
    let mut v_lsb: Py = Py::None;
    let mut v_rate: Py = Py::None;
    let _ = f__pos(Py::str("I_leak, C, t_hold and Vref"), Py::tuple(vec![v_I_leak.clone(), v_C.clone(), v_t_hold.clone(), v_Vref.clone()]))?;
    let _ = f__need({ let t3613 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3613.truthy() { t3613 } else { { let t3610 = Py::Int(1i64); Py::Bool({ let t3611 = v_n.clone(); if py::le(&t3610, &t3611)? { { let t3612 = Py::Int(24i64); if py::le(&t3611, &t3612)? { true } else { false } } } else { false } }) } } }, Py::str("the resolution must be a whole number of bits from 1 to 24"))?;
    v_rate = py::truediv(&v_I_leak.clone(), &v_C.clone())?;
    v_lsb = py::truediv(&v_Vref.clone(), &py::pow(&Py::Int(2i64), &v_n.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("droop_rate"), v_rate.clone()), (Py::str("droop"), py::mul(&v_rate.clone(), &v_t_hold.clone())?), (Py::str("droop_lsb"), py::truediv(&py::mul(&v_rate.clone(), &v_t_hold.clone())?, &v_lsb.clone())?), (Py::str("t_max_half_lsb"), py::truediv(&py::truediv(&v_lsb.clone(), &Py::Float(2.0))?, &v_rate.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sar_conversion(mut v_Vin: Py, mut v_Vref: Py, mut v_n: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_code: Py = Py::None;
    let mut v_lsb: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_trials: Py = Py::None;
    let _ = f__pos(Py::str("Vref"), Py::tuple(vec![v_Vref.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_Vin.clone()]))?;
    let _ = f__need({ let t3664 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3664.truthy() { t3664 } else { { let t3661 = Py::Int(1i64); Py::Bool({ let t3662 = v_n.clone(); if py::le(&t3661, &t3662)? { { let t3663 = Py::Int(16i64); if py::le(&t3662, &t3663)? { true } else { false } } } else { false } }) } } }, Py::str("the resolution must be a whole number of bits from 1 to 16"))?;
    let _ = f__need({ let t3665 = Py::Float(0.0); Py::Bool({ let t3666 = v_Vin.clone(); if py::le(&t3665, &t3666)? { { let t3667 = v_Vref.clone(); if py::lt(&t3666, &t3667)? { true } else { false } } } else { false } }) }, Py::str("the input must lie in [0, Vref)"))?;
    v_lsb = py::truediv(&v_Vref.clone(), &py::pow(&Py::Int(2i64), &v_n.clone())?)?;
    v_code = Py::Int(0i64);
    v_trials = Py::list(vec![]);
    v_b = py::sub(&v_n.clone(), &Py::Int(1i64))?;
    while ({ let t3670 = v_b.clone(); Py::Bool({ let t3671 = Py::Int(0i64); if py::ge(&t3670, &t3671)? { true } else { false } }) }).truthy() {
        v_t = py::add(&v_code.clone(), &py::pow(&Py::Int(2i64), &v_b.clone())?)?;
        let _ = py::method(&v_trials.clone(), "append", &[py::mul(&v_t.clone(), &v_lsb.clone())?])?;
        if ({ let t3668 = v_Vin.clone(); Py::Bool({ let t3669 = py::mul(&v_t.clone(), &v_lsb.clone())?; if py::ge(&t3668, &t3669)? { true } else { false } }) }).truthy() {
            v_code = v_t.clone();
        }
        v_b = py::sub(&v_b, &Py::Int(1i64))?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("code"), v_code.clone()), (Py::str("bits"), f__bits(v_code.clone(), v_n.clone())?), (Py::str("trials"), v_trials.clone()), (Py::str("V_dac"), py::mul(&v_code.clone(), &v_lsb.clone())?), (Py::str("cycles"), v_n.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_schmitt(mut v_Rf: Py, mut v_R1: Py, mut v_VsatH: Py, mut v_VsatL: Py, mut v_Vref: Py) -> R {
    let mut v_th: Py = Py::None;
    let mut v_tl: Py = Py::None;
    let _ = f__pos(Py::str("Rf and R1"), Py::tuple(vec![v_Rf.clone(), v_R1.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_VsatH.clone(), v_VsatL.clone(), v_Vref.clone()]))?;
    let _ = f__need({ let t3723 = v_VsatH.clone(); Py::Bool({ let t3724 = v_VsatL.clone(); if py::gt(&t3723, &t3724)? { true } else { false } }) }, Py::str("VsatH must exceed VsatL"))?;
    v_th = py::truediv(&py::add(&py::mul(&v_VsatH.clone(), &v_R1.clone())?, &py::mul(&v_Vref.clone(), &v_Rf.clone())?)?, &py::add(&v_R1.clone(), &v_Rf.clone())?)?;
    v_tl = py::truediv(&py::add(&py::mul(&v_VsatL.clone(), &v_R1.clone())?, &py::mul(&v_Vref.clone(), &v_Rf.clone())?)?, &py::add(&v_R1.clone(), &v_Rf.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V_TH"), v_th.clone()), (Py::str("V_TL"), v_tl.clone()), (Py::str("hysteresis"), py::sub(&v_th.clone(), &v_tl.clone())?), (Py::str("center"), py::truediv(&py::add(&v_th.clone(), &v_tl.clone())?, &Py::Float(2.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sctc_low_cutoff(mut v_Rsig: Py, mut v_RB: Py, mut v_r_pi: Py, mut v_beta: Py, mut v_RE: Py, mut v_RC: Py, mut v_RL: Py, mut v_C1: Py, mut v_C2: Py, mut v_CE: Py) -> R {
    let mut v_R1: Py = Py::None;
    let mut v_R2: Py = Py::None;
    let mut v_RCE: Py = Py::None;
    let mut v_f1: Py = Py::None;
    let mut v_f2: Py = Py::None;
    let mut v_fe: Py = Py::None;
    let _ = f__pos(Py::str("Rsig, RB, r_pi, beta, RE, RC, RL, C1, C2 and CE"), Py::tuple(vec![v_Rsig.clone(), v_RB.clone(), v_r_pi.clone(), v_beta.clone(), v_RE.clone(), v_RC.clone(), v_RL.clone(), v_C1.clone(), v_C2.clone(), v_CE.clone()]))?;
    v_R1 = py::add(&v_Rsig.clone(), &f__par(Py::tuple(vec![v_RB.clone(), v_r_pi.clone()]))?)?;
    v_RCE = f__par(Py::tuple(vec![v_RE.clone(), py::truediv(&py::add(&v_r_pi.clone(), &f__par(Py::tuple(vec![v_Rsig.clone(), v_RB.clone()]))?)?, &py::add(&v_beta.clone(), &Py::Float(1.0))?)?]))?;
    v_R2 = py::add(&v_RC.clone(), &v_RL.clone())?;
    let t2566 = (Py::tuple(vec![py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&c_TWO_PI()?, &v_R1.clone())?, &v_C1.clone())?)?, py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&c_TWO_PI()?, &v_RCE.clone())?, &v_CE.clone())?)?, py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&c_TWO_PI()?, &v_R2.clone())?, &v_C2.clone())?)?])).items()?;
    if t2566.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_f1 = t2566[0].clone();
    v_fe = t2566[1].clone();
    v_f2 = t2566[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("R_C1"), v_R1.clone()), (Py::str("R_CE"), v_RCE.clone()), (Py::str("R_C2"), v_R2.clone()), (Py::str("f_C1"), v_f1.clone()), (Py::str("f_CE"), v_fe.clone()), (Py::str("f_C2"), v_f2.clone()), (Py::str("f_L"), py::add(&py::add(&v_f1.clone(), &v_fe.clone())?, &v_f2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_pass(mut v_Vin: Py, mut v_Vz: Py, mut v_Rs: Py, mut v_RL: Py, mut v_beta: Py, mut v_VBE: Py) -> R {
    let mut v_IB: Py = Py::None;
    let mut v_IC: Py = Py::None;
    let mut v_IL: Py = Py::None;
    let mut v_IRs: Py = Py::None;
    let mut v_Iz: Py = Py::None;
    let mut v_Vo: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vz, Rs, RL, beta and VBE"), Py::tuple(vec![v_Vin.clone(), v_Vz.clone(), v_Rs.clone(), v_RL.clone(), v_beta.clone(), v_VBE.clone()]))?;
    let _ = f__need({ let t3821 = v_Vin.clone(); Py::Bool({ let t3822 = v_Vz.clone(); if py::gt(&t3821, &t3822)? { { let t3823 = v_VBE.clone(); if py::gt(&t3822, &t3823)? { true } else { false } } } else { false } }) }, Py::str("need Vin > Vz > VBE"))?;
    v_Vo = py::sub(&v_Vz.clone(), &v_VBE.clone())?;
    v_IL = py::truediv(&v_Vo.clone(), &v_RL.clone())?;
    v_IB = py::truediv(&v_IL.clone(), &py::add(&v_beta.clone(), &Py::Float(1.0))?)?;
    v_IRs = py::truediv(&py::sub(&v_Vin.clone(), &v_Vz.clone())?, &v_Rs.clone())?;
    v_Iz = py::sub(&v_IRs.clone(), &v_IB.clone())?;
    let _ = f__need({ let t3824 = v_Iz.clone(); Py::Bool({ let t3825 = Py::Int(0i64); if py::gt(&t3824, &t3825)? { true } else { false } }) }, Py::str("the Zener is starved (Rs too large for this load): no regulation"))?;
    v_IC = py::mul(&v_beta.clone(), &v_IB.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vout"), v_Vo.clone()), (Py::str("I_L"), v_IL.clone()), (Py::str("I_B"), v_IB.clone()), (Py::str("I_z"), v_Iz.clone()), (Py::str("I_Rs"), v_IRs.clone()), (Py::str("P_Q"), py::mul(&py::sub(&v_Vin.clone(), &v_Vo.clone())?, &v_IC.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_setup_hold(mut v_T: Py, mut v_tcq: Py, mut v_tpd: Py, mut v_tsu: Py, mut v_tskew: Py, mut v_tccq: Py, mut v_tcd: Py, mut v_th: Py) -> R {
    let mut v_hs: Py = Py::None;
    let mut v_need: Py = Py::None;
    let mut v_ss: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("the clock period T"), Py::tuple(vec![v_T.clone()]))?;
    let _ = f__nonneg(Py::str("tcq, tpd, tsu, tccq, tcd and th"), Py::tuple(vec![v_tcq.clone(), v_tpd.clone(), v_tsu.clone(), v_tccq.clone(), v_tcd.clone(), v_th.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_tskew.clone()]))?;
    v_ss = py::sub(&py::add(&v_T.clone(), &v_tskew.clone())?, &py::add(&py::add(&v_tcq.clone(), &v_tpd.clone())?, &v_tsu.clone())?)?;
    v_hs = py::sub(&py::sub(&py::add(&v_tccq.clone(), &v_tcd.clone())?, &v_th.clone())?, &v_tskew.clone())?;
    v_need = py::sub(&py::add(&py::add(&v_tcq.clone(), &v_tpd.clone())?, &v_tsu.clone())?, &v_tskew.clone())?;
    let _ = f__need({ let t3879 = v_need.clone(); Py::Bool({ let t3880 = Py::Int(0i64); if py::gt(&t3879, &t3880)? { true } else { false } }) }, Py::str("the path needs no time: f_max is undefined"))?;
    if ({ let t3885 = { let t3881 = v_ss.clone(); Py::Bool({ let t3882 = Py::Int(0i64); if py::ge(&t3881, &t3882)? { true } else { false } }) }; if !t3885.truthy() { t3885 } else { { let t3883 = v_hs.clone(); Py::Bool({ let t3884 = Py::Int(0i64); if py::ge(&t3883, &t3884)? { true } else { false } }) } } }).truthy() {
        v_v = Py::str("meets timing");
    } else {
        if ({ let t3890 = { let t3886 = v_ss.clone(); Py::Bool({ let t3887 = Py::Int(0i64); if py::lt(&t3886, &t3887)? { true } else { false } }) }; if !t3890.truthy() { t3890 } else { { let t3888 = v_hs.clone(); Py::Bool({ let t3889 = Py::Int(0i64); if py::lt(&t3888, &t3889)? { true } else { false } }) } } }).truthy() {
            v_v = Py::str("setup and hold violations");
        } else {
            if ({ let t3891 = v_ss.clone(); Py::Bool({ let t3892 = Py::Int(0i64); if py::lt(&t3891, &t3892)? { true } else { false } }) }).truthy() {
                v_v = Py::str("setup violation");
            } else {
                v_v = Py::str("hold violation");
            }
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("setup_slack"), v_ss.clone()), (Py::str("hold_slack"), v_hs.clone()), (Py::str("f_max"), py::truediv(&Py::Float(1.0), &v_need.clone())?), (Py::str("verdict"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shockley(mut v_Is: Py, mut v_n: Py, mut v_VD: Py, mut v_T: Py) -> R {
    let mut v_VT: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("Is, n and T"), Py::tuple(vec![v_Is.clone(), v_n.clone(), v_T.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_VD.clone()]))?;
    v_VT = f__vt(v_T.clone())?;
    v_x = py::truediv(&v_VD.clone(), &py::mul(&v_n.clone(), &v_VT.clone())?)?;
    let _ = f__need({ let t3940 = v_x.clone(); Py::Bool({ let t3941 = Py::Float(700.0); if py::lt(&t3940, &t3941)? { true } else { false } }) }, Py::str("the forward voltage is far beyond any real diode (exp overflow)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("ID"), py::mul(&v_Is.clone(), &py::math1("expm1", &v_x.clone())?)?), (Py::str("VT"), v_VT.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sk_equal_design(mut v_fc: Py, mut v_Q: Py, mut v_C: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_R: Py = Py::None;
    let _ = f__pos(Py::str("fc, Q and C"), Py::tuple(vec![v_fc.clone(), v_Q.clone(), v_C.clone()]))?;
    let _ = f__need({ let t3471 = Py::Float(0.5); Py::Bool({ let t3472 = v_Q.clone(); if py::le(&t3471, &t3472)? { { let t3473 = Py::Float(20.0); if py::le(&t3472, &t3473)? { true } else { false } } } else { false } }) }, Py::str("the equal-component design needs 0.5 <= Q <= 20 (gain K = 3 - 1/Q between 1 and 3)"))?;
    v_R = py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&c_TWO_PI()?, &v_fc.clone())?, &v_C.clone())?)?;
    v_K = py::sub(&Py::Float(3.0), &py::truediv(&Py::Float(1.0), &v_Q.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("K"), v_K.clone()), (Py::str("Rf_over_Rg"), py::sub(&v_K.clone(), &Py::Float(1.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sk_highpass(mut v_C1: Py, mut v_C2: Py, mut v_R1: Py, mut v_R2: Py, mut v_K: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let _ = f__pos(Py::str("C1, C2, R1, R2 and K"), Py::tuple(vec![v_C1.clone(), v_C2.clone(), v_R1.clone(), v_R2.clone(), v_K.clone()]))?;
    v_a = py::mul(&py::mul(&py::mul(&v_R1.clone(), &v_R2.clone())?, &v_C1.clone())?, &v_C2.clone())?;
    v_b = py::add(&py::mul(&v_R1.clone(), &py::add(&v_C1.clone(), &v_C2.clone())?)?, &py::mul(&py::mul(&v_R2.clone(), &v_C2.clone())?, &py::sub(&Py::Float(1.0), &v_K.clone())?)?)?;
    let _ = f__need({ let t3518 = v_b.clone(); Py::Bool({ let t3519 = Py::Int(0i64); if py::gt(&t3518, &t3519)? { true } else { false } }) }, Py::str("the gain K makes the filter unstable (the s coefficient is not positive)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("f0"), py::truediv(&Py::Float(1.0), &py::mul(&c_TWO_PI()?, &py::math1("sqrt", &v_a.clone())?)?)?), (Py::str("Q"), py::truediv(&py::math1("sqrt", &v_a.clone())?, &v_b.clone())?), (Py::str("H_inf"), v_K.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sk_lowpass(mut v_R1: Py, mut v_R2: Py, mut v_C1: Py, mut v_C2: Py, mut v_K: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let _ = f__pos(Py::str("R1, R2, C1, C2 and K"), Py::tuple(vec![v_R1.clone(), v_R2.clone(), v_C1.clone(), v_C2.clone(), v_K.clone()]))?;
    v_a = py::mul(&py::mul(&py::mul(&v_R1.clone(), &v_R2.clone())?, &v_C1.clone())?, &v_C2.clone())?;
    v_b = py::add(&py::add(&py::mul(&v_R1.clone(), &v_C2.clone())?, &py::mul(&v_R2.clone(), &v_C2.clone())?)?, &py::mul(&py::mul(&v_R1.clone(), &v_C1.clone())?, &py::sub(&Py::Float(1.0), &v_K.clone())?)?)?;
    let _ = f__need({ let t3564 = v_b.clone(); Py::Bool({ let t3565 = Py::Int(0i64); if py::gt(&t3564, &t3565)? { true } else { false } }) }, Py::str("the gain K makes the filter unstable (the s coefficient is not positive)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("f0"), py::truediv(&Py::Float(1.0), &py::mul(&c_TWO_PI()?, &py::math1("sqrt", &v_a.clone())?)?)?), (Py::str("Q"), py::truediv(&py::math1("sqrt", &v_a.clone())?, &v_b.clone())?), (Py::str("H0"), v_K.clone()), (Py::str("peak_gain"), py::truediv(&py::mul(&v_K.clone(), &py::math1("sqrt", &v_a.clone())?)?, &v_b.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_slew_rate(mut v_SR: Py, mut v_Vp: Py, mut v_f: Py) -> R {
    let mut v_need: Py = Py::None;
    let _ = f__pos(Py::str("SR, Vp and f"), Py::tuple(vec![v_SR.clone(), v_Vp.clone(), v_f.clone()]))?;
    v_need = py::mul(&py::mul(&c_TWO_PI()?, &v_f.clone())?, &v_Vp.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("f_max"), py::truediv(&v_SR.clone(), &py::mul(&c_TWO_PI()?, &v_Vp.clone())?)?), (Py::str("Vp_max"), py::truediv(&v_SR.clone(), &py::mul(&c_TWO_PI()?, &v_f.clone())?)?), (Py::str("slope_needed"), v_need.clone()), (Py::str("limited"), (if ({ let t3998 = v_need.clone(); Py::Bool({ let t3999 = v_SR.clone(); if py::gt(&t3998, &t3999)? { true } else { false } }) }).truthy() { Py::str("slew-limited") } else { Py::str("not slew-limited") }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_source_follower(mut v_gm: Py, mut v_ro: Py, mut v_RS: Py, mut v_RL: Py, mut v_RG: Py, mut v_Rsig: Py) -> R {
    let mut v_Av: Py = Py::None;
    let mut v_R: Py = Py::None;
    let _ = f__pos(Py::str("gm, ro, RS, RL, RG and Rsig"), Py::tuple(vec![v_gm.clone(), v_ro.clone(), v_RS.clone(), v_RL.clone(), v_RG.clone(), v_Rsig.clone()]))?;
    v_R = f__par(Py::tuple(vec![v_RS.clone(), v_RL.clone(), v_ro.clone()]))?;
    v_Av = py::truediv(&py::mul(&v_gm.clone(), &v_R.clone())?, &py::add(&Py::Float(1.0), &py::mul(&v_gm.clone(), &v_R.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Av"), v_Av.clone()), (Py::str("Rin"), v_RG.clone()), (Py::str("Rout"), f__par(Py::tuple(vec![v_RS.clone(), v_ro.clone(), py::truediv(&Py::Float(1.0), &v_gm.clone())?]))?), (Py::str("Gv"), py::truediv(&py::mul(&v_Av.clone(), &v_RG.clone())?, &py::add(&v_RG.clone(), &v_Rsig.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_summing_amp(mut v_V: Py, mut v_R: Py, mut v_Rf: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("Rf"), Py::tuple(vec![v_Rf.clone()]))?;
    let _ = f__need({ let t4081 = { let t4077 = py::len(&v_V.clone())?; Py::Bool({ let t4078 = py::len(&v_R.clone())?; if py::eq(&t4077, &t4078) { true } else { false } }) }; if !t4081.truthy() { t4081 } else { { let t4079 = py::len(&v_V.clone())?; Py::Bool({ let t4080 = Py::Int(2i64); if py::ge(&t4079, &t4080)? { true } else { false } }) } } }, Py::str("give one resistor per input voltage (at least two inputs)"))?;
    v_s = Py::Float(0.0);
    for t4082 in py::iter(&(py::zip(&[v_V.clone(), v_R.clone()])?))? {
        let t4086 = (t4082).items()?;
        if t4086.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_v = t4086[0].clone();
        v_r = t4086[1].clone();
        let _ = f__fin(Py::tuple(vec![v_v.clone()]))?;
        let _ = f__pos(Py::str("every input resistor"), Py::tuple(vec![v_r.clone()]))?;
        v_s = py::iadd(&v_s, &py::truediv(&v_v.clone(), &py::mul(&v_r.clone(), &Py::Float(1000.0))?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Vout"), py::mul(&py::neg(&v_Rf.clone())?, &v_s.clone())?), (Py::str("I_f"), v_s.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thermal_voltage(mut v_T: Py) -> R {
    let mut v_VT: Py = Py::None;
    let _ = f__pos(Py::str("the temperature T"), Py::tuple(vec![v_T.clone()]))?;
    v_VT = f__vt(v_T.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("VT"), v_VT.clone()), (Py::str("VT_mV"), py::mul(&v_VT.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_timer_astable(mut v_RA: Py, mut v_RB: Py, mut v_C: Py) -> R {
    let mut v_tH: Py = Py::None;
    let mut v_tL: Py = Py::None;
    let _ = f__pos(Py::str("RA, RB and C"), Py::tuple(vec![v_RA.clone(), v_RB.clone(), v_C.clone()]))?;
    v_tH = py::mul(&py::mul(&c_LN2()?, &py::add(&v_RA.clone(), &v_RB.clone())?)?, &v_C.clone())?;
    v_tL = py::mul(&py::mul(&c_LN2()?, &v_RB.clone())?, &v_C.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("t_high"), v_tH.clone()), (Py::str("t_low"), v_tL.clone()), (Py::str("T"), py::add(&v_tH.clone(), &v_tL.clone())?), (Py::str("f"), py::truediv(&Py::Float(1.0), &py::add(&v_tH.clone(), &v_tL.clone())?)?), (Py::str("duty"), py::truediv(&v_tH.clone(), &py::add(&v_tH.clone(), &v_tL.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_timer_monostable(mut v_R: Py, mut v_C: Py) -> R {
    let _ = f__pos(Py::str("R and C"), Py::tuple(vec![v_R.clone(), v_C.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("T"), py::mul(&py::mul(&c_LN3()?, &v_R.clone())?, &v_C.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transition_frequency(mut v_gm: Py, mut v_beta: Py, mut v_Cpi: Py, mut v_Cmu: Py) -> R {
    let mut v_fb: Py = Py::None;
    let mut v_rpi: Py = Py::None;
    let _ = f__pos(Py::str("gm, beta, Cpi and Cmu"), Py::tuple(vec![v_gm.clone(), v_beta.clone(), v_Cpi.clone(), v_Cmu.clone()]))?;
    v_rpi = py::truediv(&v_beta.clone(), &v_gm.clone())?;
    v_fb = py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&c_TWO_PI()?, &v_rpi.clone())?, &py::add(&v_Cpi.clone(), &v_Cmu.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("r_pi"), v_rpi.clone()), (Py::str("f_beta"), v_fb.clone()), (Py::str("f_T"), py::mul(&v_beta.clone(), &v_fb.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wien_bridge(mut v_R1: Py, mut v_C1: Py, mut v_R2: Py, mut v_C2: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_f0: Py = Py::None;
    let _ = f__pos(Py::str("R1, C1, R2 and C2"), Py::tuple(vec![v_R1.clone(), v_C1.clone(), v_R2.clone(), v_C2.clone()]))?;
    v_f0 = py::truediv(&Py::Float(1.0), &py::mul(&c_TWO_PI()?, &py::math1("sqrt", &py::mul(&py::mul(&py::mul(&v_R1.clone(), &v_R2.clone())?, &v_C1.clone())?, &v_C2.clone())?)?)?)?;
    v_A = py::add(&py::add(&Py::Float(1.0), &py::truediv(&v_R1.clone(), &v_R2.clone())?)?, &py::truediv(&v_C2.clone(), &v_C1.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("f0"), v_f0.clone()), (Py::str("beta"), py::truediv(&Py::Float(1.0), &v_A.clone())?), (Py::str("A_min"), v_A.clone()), (Py::str("Rf_over_Rg"), py::sub(&v_A.clone(), &Py::Float(1.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wilson_mirror(mut v_Iref: Py, mut v_beta: Py) -> R {
    let mut v_ratio: Py = Py::None;
    let _ = f__pos(Py::str("Iref"), Py::tuple(vec![v_Iref.clone()]))?;
    let _ = f__beta(v_beta.clone())?;
    v_ratio = py::sub(&Py::Float(1.0), &py::truediv(&Py::Float(2.0), &py::add(&py::add(&py::mul(&v_beta.clone(), &v_beta.clone())?, &py::mul(&Py::Float(2.0), &v_beta.clone())?)?, &Py::Float(2.0))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Iout"), py::mul(&v_Iref.clone(), &v_ratio.clone())?), (Py::str("ratio"), v_ratio.clone()), (Py::str("error"), py::sub(&Py::Float(1.0), &v_ratio.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zener_design(mut v_Vin_min: Py, mut v_Vin_max: Py, mut v_Vz: Py, mut v_IL_min: Py, mut v_IL_max: Py, mut v_Iz_min: Py) -> R {
    let mut v_Izmax: Py = Py::None;
    let mut v_Rs: Py = Py::None;
    let _ = f__pos(Py::str("Vin_min, Vin_max, Vz and Iz_min"), Py::tuple(vec![v_Vin_min.clone(), v_Vin_max.clone(), v_Vz.clone(), v_Iz_min.clone()]))?;
    let _ = f__nonneg(Py::str("the load currents"), Py::tuple(vec![v_IL_min.clone(), v_IL_max.clone()]))?;
    let _ = f__need({ let t4413 = v_Vin_min.clone(); Py::Bool({ let t4414 = v_Vz.clone(); if py::gt(&t4413, &t4414)? { true } else { false } }) }, Py::str("Vin_min must exceed Vz"))?;
    let _ = f__need({ let t4419 = { let t4415 = v_Vin_max.clone(); Py::Bool({ let t4416 = v_Vin_min.clone(); if py::ge(&t4415, &t4416)? { true } else { false } }) }; if !t4419.truthy() { t4419 } else { { let t4417 = v_IL_max.clone(); Py::Bool({ let t4418 = v_IL_min.clone(); if py::ge(&t4417, &t4418)? { true } else { false } }) } } }, Py::str("the maxima must not be below the minima"))?;
    v_Rs = py::truediv(&py::sub(&v_Vin_min.clone(), &v_Vz.clone())?, &py::add(&v_Iz_min.clone(), &v_IL_max.clone())?)?;
    v_Izmax = py::sub(&py::truediv(&py::sub(&v_Vin_max.clone(), &v_Vz.clone())?, &v_Rs.clone())?, &v_IL_min.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Rs_max"), v_Rs.clone()), (Py::str("Iz_max"), v_Izmax.clone()), (Py::str("Pz_max"), py::mul(&v_Vz.clone(), &v_Izmax.clone())?), (Py::str("PRs_max"), py::truediv(&py::pow(&py::sub(&v_Vin_max.clone(), &v_Vz.clone())?, &Py::Int(2i64))?, &v_Rs.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zener_regulator(mut v_Vin: Py, mut v_Rs: Py, mut v_Vz: Py, mut v_RL: Py, mut v_rz: Py) -> R {
    let mut v_Iz: Py = Py::None;
    let mut v_V: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Rs, Vz and RL"), Py::tuple(vec![v_Vin.clone(), v_Rs.clone(), v_Vz.clone(), v_RL.clone()]))?;
    let _ = f__nonneg(Py::str("the Zener resistance rz"), Py::tuple(vec![v_rz.clone()]))?;
    let _ = f__need({ let t4359 = py::truediv(&py::mul(&v_Vin.clone(), &v_RL.clone())?, &py::add(&v_Rs.clone(), &v_RL.clone())?)?; Py::Bool({ let t4360 = v_Vz.clone(); if py::gt(&t4359, &t4360)? { true } else { false } }) }, Py::str("the Zener is not in breakdown: the open-circuit voltage across the load is below Vz"))?;
    if ({ let t4361 = v_rz.clone(); Py::Bool({ let t4362 = Py::Int(0i64); if py::eq(&t4361, &t4362) { true } else { false } }) }).truthy() {
        v_V = v_Vz.clone();
        v_Iz = py::sub(&py::truediv(&py::sub(&v_Vin.clone(), &v_Vz.clone())?, &v_Rs.clone())?, &py::truediv(&v_Vz.clone(), &v_RL.clone())?)?;
    } else {
        v_V = py::truediv(&py::add(&py::truediv(&v_Vin.clone(), &v_Rs.clone())?, &py::truediv(&v_Vz.clone(), &v_rz.clone())?)?, &py::add(&py::add(&py::truediv(&Py::Float(1.0), &v_Rs.clone())?, &py::truediv(&Py::Float(1.0), &v_RL.clone())?)?, &py::truediv(&Py::Float(1.0), &v_rz.clone())?)?)?;
        v_Iz = py::truediv(&py::sub(&v_V.clone(), &v_Vz.clone())?, &v_rz.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Vout"), v_V.clone()), (Py::str("I_s"), py::truediv(&py::sub(&v_Vin.clone(), &v_V.clone())?, &v_Rs.clone())?), (Py::str("I_L"), py::truediv(&v_V.clone(), &v_RL.clone())?), (Py::str("I_z"), v_Iz.clone()), (Py::str("P_z"), py::mul(&v_V.clone(), &v_Iz.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1023(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1025: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1024 = v_inputs.clone(); f_cmos_threshold(py::getitem(&t1024, &Py::str("VDD"))?, py::getitem(&t1024, &Py::str("Vtn"))?, py::getitem(&t1024, &Py::str("Vtp"))?, py::getitem(&t1024, &Py::str("kn"))?, py::getitem(&t1024, &Py::str("kp"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1025 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1026 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1048 = (t1026).items()?;
        if t1048.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1048[0].clone();
        v_v = t1048[1].clone();
        if ({ let t1027 = v_k.clone(); Py::Bool({ let t1028 = v_outputs.clone(); if !py::contains(&t1028, &t1027)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1067(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1069: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1068 = v_inputs.clone(); f_colpitts(py::getitem(&t1068, &Py::str("L"))?, py::getitem(&t1068, &Py::str("C1"))?, py::getitem(&t1068, &Py::str("C2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1069 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1070 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1092 = (t1070).items()?;
        if t1092.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1092[0].clone();
        v_v = t1092[1].clone();
        if ({ let t1071 = v_k.clone(); Py::Bool({ let t1072 = v_outputs.clone(); if !py::contains(&t1072, &t1071)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1113(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1115: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1114 = v_inputs.clone(); f_cb_amplifier(py::getitem(&t1114, &Py::str("IC"))?, py::getitem(&t1114, &Py::str("beta"))?, py::getitem(&t1114, &Py::str("VT"))?, py::getitem(&t1114, &Py::str("RC"))?, py::getitem(&t1114, &Py::str("RL"))?, py::getitem(&t1114, &Py::str("RE"))?, py::getitem(&t1114, &Py::str("Rsig"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1115 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1116 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1138 = (t1116).items()?;
        if t1138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1138[0].clone();
        v_v = t1138[1].clone();
        if ({ let t1117 = v_k.clone(); Py::Bool({ let t1118 = v_outputs.clone(); if !py::contains(&t1118, &t1117)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1159(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1161: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1160 = v_inputs.clone(); f_cc_amplifier(py::getitem(&t1160, &Py::str("IC"))?, py::getitem(&t1160, &Py::str("beta"))?, py::getitem(&t1160, &Py::str("VT"))?, py::getitem(&t1160, &Py::str("RE"))?, py::getitem(&t1160, &Py::str("RL"))?, py::getitem(&t1160, &Py::str("RB"))?, py::getitem(&t1160, &Py::str("Rsig"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1161 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1162 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1184 = (t1162).items()?;
        if t1184.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1184[0].clone();
        v_v = t1184[1].clone();
        if ({ let t1163 = v_k.clone(); Py::Bool({ let t1164 = v_outputs.clone(); if !py::contains(&t1164, &t1163)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1211(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1213: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1212 = v_inputs.clone(); f_ce_amplifier(py::getitem(&t1212, &Py::str("IC"))?, py::getitem(&t1212, &Py::str("beta"))?, py::getitem(&t1212, &Py::str("VT"))?, py::getitem(&t1212, &Py::str("RC"))?, py::getitem(&t1212, &Py::str("RL"))?, py::getitem(&t1212, &Py::str("RE1"))?, py::getitem(&t1212, &Py::str("RB"))?, py::getitem(&t1212, &Py::str("Rsig"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1213 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1214 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1236 = (t1214).items()?;
        if t1236.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1236[0].clone();
        v_v = t1236[1].clone();
        if ({ let t1215 = v_k.clone(); Py::Bool({ let t1216 = v_outputs.clone(); if !py::contains(&t1216, &t1215)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_123(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t125: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t124 = v_inputs.clone(); f_adc_code(py::getitem(&t124, &Py::str("Vin"))?, py::getitem(&t124, &Py::str("Vref"))?, py::getitem(&t124, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t125 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t126 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t148 = (t126).items()?;
        if t148.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t148[0].clone();
        v_v = t148[1].clone();
        if ({ let t127 = v_k.clone(); Py::Bool({ let t128 = v_outputs.clone(); if !py::contains(&t128, &t127)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1262(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1264: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1263 = v_inputs.clone(); f_cs_amplifier(py::getitem(&t1263, &Py::str("gm"))?, py::getitem(&t1263, &Py::str("ro"))?, py::getitem(&t1263, &Py::str("RD"))?, py::getitem(&t1263, &Py::str("RL"))?, py::getitem(&t1263, &Py::str("RS"))?, py::getitem(&t1263, &Py::str("RG"))?, py::getitem(&t1263, &Py::str("Rsig"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1264 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1265 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1287 = (t1265).items()?;
        if t1287.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1287[0].clone();
        v_v = t1287[1].clone();
        if ({ let t1266 = v_k.clone(); Py::Bool({ let t1267 = v_outputs.clone(); if !py::contains(&t1267, &t1266)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1306(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1308: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1307 = v_inputs.clone(); f_crystal(py::getitem(&t1307, &Py::str("Lm"))?, py::getitem(&t1307, &Py::str("Cm"))?, py::getitem(&t1307, &Py::str("Rm"))?, py::getitem(&t1307, &Py::str("C0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1308 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1309 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1331 = (t1309).items()?;
        if t1331.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1331[0].clone();
        v_v = t1331[1].clone();
        if ({ let t1310 = v_k.clone(); Py::Bool({ let t1311 = v_outputs.clone(); if !py::contains(&t1311, &t1310)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1352(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1354: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1353 = v_inputs.clone(); f_current_mirror(py::getitem(&t1353, &Py::str("VCC"))?, py::getitem(&t1353, &Py::str("R"))?, py::getitem(&t1353, &Py::str("VBE"))?, py::getitem(&t1353, &Py::str("beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1354 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1355 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1377 = (t1355).items()?;
        if t1377.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1377[0].clone();
        v_v = t1377[1].clone();
        if ({ let t1356 = v_k.clone(); Py::Bool({ let t1357 = v_outputs.clone(); if !py::contains(&t1357, &t1356)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1404(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1406: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1405 = v_inputs.clone(); f_dac_binary_weighted(py::getitem(&t1405, &Py::str("n"))?, py::getitem(&t1405, &Py::str("code"))?, py::getitem(&t1405, &Py::str("Vref"))?, py::getitem(&t1405, &Py::str("R"))?, py::getitem(&t1405, &Py::str("Rf"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1406 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1407 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1429 = (t1407).items()?;
        if t1429.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1429[0].clone();
        v_v = t1429[1].clone();
        if ({ let t1408 = v_k.clone(); Py::Bool({ let t1409 = v_outputs.clone(); if !py::contains(&t1409, &t1408)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1456(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1458: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1457 = v_inputs.clone(); f_dac_output(py::getitem(&t1457, &Py::str("n"))?, py::getitem(&t1457, &Py::str("code"))?, py::getitem(&t1457, &Py::str("Vref"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1458 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1459 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1481 = (t1459).items()?;
        if t1481.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1481[0].clone();
        v_v = t1481[1].clone();
        if ({ let t1460 = v_k.clone(); Py::Bool({ let t1461 = v_outputs.clone(); if !py::contains(&t1461, &t1460)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1505(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1507: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1506 = v_inputs.clone(); f_depletion(py::getitem(&t1506, &Py::str("NA"))?, py::getitem(&t1506, &Py::str("ND"))?, py::getitem(&t1506, &Py::str("Vbi"))?, py::getitem(&t1506, &Py::str("VR"))?, py::getitem(&t1506, &Py::str("eps_r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1507 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1508 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1530 = (t1508).items()?;
        if t1530.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1530[0].clone();
        v_v = t1530[1].clone();
        if ({ let t1509 = v_k.clone(); Py::Bool({ let t1510 = v_outputs.clone(); if !py::contains(&t1510, &t1509)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1554(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1556: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1555 = v_inputs.clone(); f_difference_amp(py::getitem(&t1555, &Py::str("R1"))?, py::getitem(&t1555, &Py::str("R2"))?, py::getitem(&t1555, &Py::str("R3"))?, py::getitem(&t1555, &Py::str("R4"))?, py::getitem(&t1555, &Py::str("V1"))?, py::getitem(&t1555, &Py::str("V2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1556 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1557 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1579 = (t1557).items()?;
        if t1579.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1579[0].clone();
        v_v = t1579[1].clone();
        if ({ let t1558 = v_k.clone(); Py::Bool({ let t1559 = v_outputs.clone(); if !py::contains(&t1559, &t1558)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1598(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1600: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1599 = v_inputs.clone(); f_diff_pair(py::getitem(&t1599, &Py::str("I_tail"))?, py::getitem(&t1599, &Py::str("beta"))?, py::getitem(&t1599, &Py::str("VT"))?, py::getitem(&t1599, &Py::str("RC"))?, py::getitem(&t1599, &Py::str("REE"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1600 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1601 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1623 = (t1601).items()?;
        if t1623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1623[0].clone();
        v_v = t1623[1].clone();
        if ({ let t1602 = v_k.clone(); Py::Bool({ let t1603 = v_outputs.clone(); if !py::contains(&t1603, &t1602)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1650(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1652: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1651 = v_inputs.clone(); f_diode_operating_point(py::getitem(&t1651, &Py::str("VS"))?, py::getitem(&t1651, &Py::str("R"))?, py::getitem(&t1651, &Py::str("Is"))?, py::getitem(&t1651, &Py::str("n"))?, py::getitem(&t1651, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1652 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1653 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1675 = (t1653).items()?;
        if t1675.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1675[0].clone();
        v_v = t1675[1].clone();
        if ({ let t1654 = v_k.clone(); Py::Bool({ let t1655 = v_outputs.clone(); if !py::contains(&t1655, &t1654)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_168(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t170: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t169 = v_inputs.clone(); f_adc_enob(py::getitem(&t169, &Py::str("SINAD"))?, py::getitem(&t169, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t170 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t171 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t193 = (t171).items()?;
        if t193.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t193[0].clone();
        v_v = t193[1].clone();
        if ({ let t172 = v_k.clone(); Py::Bool({ let t173 = v_outputs.clone(); if !py::contains(&t173, &t172)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1699(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1701: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1700 = v_inputs.clone(); f_diode_voltage(py::getitem(&t1700, &Py::str("ID"))?, py::getitem(&t1700, &Py::str("Is"))?, py::getitem(&t1700, &Py::str("n"))?, py::getitem(&t1700, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1701 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1702 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1724 = (t1702).items()?;
        if t1724.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1724[0].clone();
        v_v = t1724[1].clone();
        if ({ let t1703 = v_k.clone(); Py::Bool({ let t1704 = v_outputs.clone(); if !py::contains(&t1704, &t1703)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1757(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1759: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1758 = v_inputs.clone(); f_doped_carriers(py::getitem(&t1758, &Py::str("ND"))?, py::getitem(&t1758, &Py::str("NA"))?, py::getitem(&t1758, &Py::str("ni"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1759 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1760 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1782 = (t1760).items()?;
        if t1782.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1782[0].clone();
        v_v = t1782[1].clone();
        if ({ let t1761 = v_k.clone(); Py::Bool({ let t1762 = v_outputs.clone(); if !py::contains(&t1762, &t1761)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1804(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1806: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1805 = v_inputs.clone(); f_drift_current(py::getitem(&t1805, &Py::str("n"))?, py::getitem(&t1805, &Py::str("mu"))?, py::getitem(&t1805, &Py::str("V"))?, py::getitem(&t1805, &Py::str("L"))?, py::getitem(&t1805, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1806 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1807 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1829 = (t1807).items()?;
        if t1829.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1829[0].clone();
        v_v = t1829[1].clone();
        if ({ let t1808 = v_k.clone(); Py::Bool({ let t1809 = v_outputs.clone(); if !py::contains(&t1809, &t1808)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1853(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1855: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1854 = v_inputs.clone(); f_dual_slope(py::getitem(&t1854, &Py::str("Vin"))?, py::getitem(&t1854, &Py::str("Vref"))?, py::getitem(&t1854, &Py::str("N1"))?, py::getitem(&t1854, &Py::str("fclk"))?, py::getitem(&t1854, &Py::str("R"))?, py::getitem(&t1854, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1855 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1856 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1878 = (t1856).items()?;
        if t1878.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1878[0].clone();
        v_v = t1878[1].clone();
        if ({ let t1857 = v_k.clone(); Py::Bool({ let t1858 = v_outputs.clone(); if !py::contains(&t1858, &t1857)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1897(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1899: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1898 = v_inputs.clone(); f_einstein(py::getitem(&t1898, &Py::str("mu"))?, py::getitem(&t1898, &Py::str("T"))?, py::getitem(&t1898, &Py::str("tau"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1899 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1900 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1922 = (t1900).items()?;
        if t1922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1922[0].clone();
        v_v = t1922[1].clone();
        if ({ let t1901 = v_k.clone(); Py::Bool({ let t1902 = v_outputs.clone(); if !py::contains(&t1902, &t1901)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_19(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t21: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t20 = v_inputs.clone(); f_timer_astable(py::getitem(&t20, &Py::str("RA"))?, py::getitem(&t20, &Py::str("RB"))?, py::getitem(&t20, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t21 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t22 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t44 = (t22).items()?;
        if t44.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t44[0].clone();
        v_v = t44[1].clone();
        if ({ let t23 = v_k.clone(); Py::Bool({ let t24 = v_outputs.clone(); if !py::contains(&t24, &t23)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1956(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1958: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1957 = v_inputs.clone(); f_elmore(py::getitem(&t1957, &Py::str("R"))?, py::getitem(&t1957, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1958 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1959 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1981 = (t1959).items()?;
        if t1981.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1981[0].clone();
        v_v = t1981[1].clone();
        if ({ let t1960 = v_k.clone(); Py::Bool({ let t1961 = v_outputs.clone(); if !py::contains(&t1961, &t1960)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2008(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2010: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2009 = v_inputs.clone(); f_filter_order(py::getitem(&t2009, &Py::str("ftype"))?, py::getitem(&t2009, &Py::str("fp"))?, py::getitem(&t2009, &Py::str("fs"))?, py::getitem(&t2009, &Py::str("Ap"))?, py::getitem(&t2009, &Py::str("As"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2010 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2011 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2033 = (t2011).items()?;
        if t2033.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2033[0].clone();
        v_v = t2033[1].clone();
        if ({ let t2012 = v_k.clone(); Py::Bool({ let t2013 = v_outputs.clone(); if !py::contains(&t2013, &t2012)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2052(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2054: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2053 = v_inputs.clone(); f_first_order_lp(py::getitem(&t2053, &Py::str("R1"))?, py::getitem(&t2053, &Py::str("Rf"))?, py::getitem(&t2053, &Py::str("C"))?, py::getitem(&t2053, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2054 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2055 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2077 = (t2055).items()?;
        if t2077.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2077[0].clone();
        v_v = t2077[1].clone();
        if ({ let t2056 = v_k.clone(); Py::Bool({ let t2057 = v_outputs.clone(); if !py::contains(&t2057, &t2056)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2104(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2106: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2105 = v_inputs.clone(); f_hartley(py::getitem(&t2105, &Py::str("L1"))?, py::getitem(&t2105, &Py::str("L2"))?, py::getitem(&t2105, &Py::str("M"))?, py::getitem(&t2105, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2106 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2107 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2129 = (t2107).items()?;
        if t2129.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2129[0].clone();
        v_v = t2129[1].clone();
        if ({ let t2108 = v_k.clone(); Py::Bool({ let t2109 = v_outputs.clone(); if !py::contains(&t2109, &t2108)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_215(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t217: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t216 = v_inputs.clone(); f_adc_snr(py::getitem(&t216, &Py::str("n"))?, py::method(&t216, "get", &[Py::str("OSR"), Py::Float(1.0)])?, py::method(&t216, "get", &[Py::str("A_frac"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t217 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t218 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t240 = (t218).items()?;
        if t240.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t240[0].clone();
        v_v = t240[1].clone();
        if ({ let t219 = v_k.clone(); Py::Bool({ let t220 = v_outputs.clone(); if !py::contains(&t220, &t219)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2150(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2152: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2151 = v_inputs.clone(); f_heat_sink_required(py::getitem(&t2151, &Py::str("P"))?, py::getitem(&t2151, &Py::str("Ta"))?, py::getitem(&t2151, &Py::str("Tj_max"))?, py::getitem(&t2151, &Py::str("theta_jc"))?, py::getitem(&t2151, &Py::str("theta_cs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2152 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2153 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2175 = (t2153).items()?;
        if t2175.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2175[0].clone();
        v_v = t2175[1].clone();
        if ({ let t2154 = v_k.clone(); Py::Bool({ let t2155 = v_outputs.clone(); if !py::contains(&t2155, &t2154)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2194(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2196: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2195 = v_inputs.clone(); f_heat_sink(py::getitem(&t2195, &Py::str("P"))?, py::getitem(&t2195, &Py::str("Ta"))?, py::getitem(&t2195, &Py::str("theta_jc"))?, py::getitem(&t2195, &Py::str("theta_cs"))?, py::getitem(&t2195, &Py::str("theta_sa"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2196 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2197 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2219 = (t2197).items()?;
        if t2219.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2219[0].clone();
        v_v = t2219[1].clone();
        if ({ let t2198 = v_k.clone(); Py::Bool({ let t2199 = v_outputs.clone(); if !py::contains(&t2199, &t2198)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2241(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2243: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2242 = v_inputs.clone(); f_inamp(py::getitem(&t2242, &Py::str("R1"))?, py::getitem(&t2242, &Py::str("RG"))?, py::getitem(&t2242, &Py::str("R2"))?, py::getitem(&t2242, &Py::str("R3"))?, py::getitem(&t2242, &Py::str("V1"))?, py::getitem(&t2242, &Py::str("V2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2243 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2244 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2266 = (t2244).items()?;
        if t2266.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2266[0].clone();
        v_v = t2266[1].clone();
        if ({ let t2245 = v_k.clone(); Py::Bool({ let t2246 = v_outputs.clone(); if !py::contains(&t2246, &t2245)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2286(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2288: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2287 = v_inputs.clone(); f_intrinsic_concentration(py::getitem(&t2287, &Py::str("Nc"))?, py::getitem(&t2287, &Py::str("Nv"))?, py::getitem(&t2287, &Py::str("Eg"))?, py::getitem(&t2287, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2288 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2289 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2311 = (t2289).items()?;
        if t2311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2311[0].clone();
        v_v = t2311[1].clone();
        if ({ let t2290 = v_k.clone(); Py::Bool({ let t2291 = v_outputs.clone(); if !py::contains(&t2291, &t2290)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2333(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2335: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2334 = v_inputs.clone(); f_inverting_amp(py::getitem(&t2334, &Py::str("Rf"))?, py::getitem(&t2334, &Py::str("Rin"))?, py::getitem(&t2334, &Py::str("Vin"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2335 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2336 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2358 = (t2336).items()?;
        if t2358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2358[0].clone();
        v_v = t2358[1].clone();
        if ({ let t2337 = v_k.clone(); Py::Bool({ let t2338 = v_outputs.clone(); if !py::contains(&t2338, &t2337)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2380(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2382: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2381 = v_inputs.clone(); f_finite_gain_inverting(py::getitem(&t2381, &Py::str("Rf"))?, py::getitem(&t2381, &Py::str("R1"))?, py::getitem(&t2381, &Py::str("A0"))?, py::getitem(&t2381, &Py::str("Vin"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2382 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2383 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2405 = (t2383).items()?;
        if t2405.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2405[0].clone();
        v_v = t2405[1].clone();
        if ({ let t2384 = v_k.clone(); Py::Bool({ let t2385 = v_outputs.clone(); if !py::contains(&t2385, &t2384)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2432(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2434: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2433 = v_inputs.clone(); f_junction_capacitance(py::getitem(&t2433, &Py::str("Cj0"))?, py::getitem(&t2433, &Py::str("Vbi"))?, py::getitem(&t2433, &Py::str("VR"))?, py::getitem(&t2433, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2434 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2435 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2457 = (t2435).items()?;
        if t2457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2457[0].clone();
        v_v = t2457[1].clone();
        if ({ let t2436 = v_k.clone(); Py::Bool({ let t2437 = v_outputs.clone(); if !py::contains(&t2437, &t2436)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2481(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2483: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2482 = v_inputs.clone(); f_led_resistor(py::getitem(&t2482, &Py::str("Vs"))?, py::getitem(&t2482, &Py::str("Vf"))?, py::getitem(&t2482, &Py::str("If"))?, py::method(&t2482, "get", &[Py::str("n"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2483 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2484 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2506 = (t2484).items()?;
        if t2506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2506[0].clone();
        v_v = t2506[1].clone();
        if ({ let t2485 = v_k.clone(); Py::Bool({ let t2486 = v_outputs.clone(); if !py::contains(&t2486, &t2485)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2533(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2535: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2534 = v_inputs.clone(); f_linear_regulator(py::getitem(&t2534, &Py::str("Vin"))?, py::getitem(&t2534, &Py::str("Vout"))?, py::getitem(&t2534, &Py::str("Iload"))?, py::getitem(&t2534, &Py::str("Iq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2535 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2536 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2558 = (t2536).items()?;
        if t2558.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2558[0].clone();
        v_v = t2558[1].clone();
        if ({ let t2537 = v_k.clone(); Py::Bool({ let t2538 = v_outputs.clone(); if !py::contains(&t2538, &t2537)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2579(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2581: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2580 = v_inputs.clone(); f_sctc_low_cutoff(py::getitem(&t2580, &Py::str("Rsig"))?, py::getitem(&t2580, &Py::str("RB"))?, py::getitem(&t2580, &Py::str("r_pi"))?, py::getitem(&t2580, &Py::str("beta"))?, py::getitem(&t2580, &Py::str("RE"))?, py::getitem(&t2580, &Py::str("RC"))?, py::getitem(&t2580, &Py::str("RL"))?, py::getitem(&t2580, &Py::str("C1"))?, py::getitem(&t2580, &Py::str("C2"))?, py::getitem(&t2580, &Py::str("CE"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2581 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2582 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2604 = (t2582).items()?;
        if t2604.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2604[0].clone();
        v_v = t2604[1].clone();
        if ({ let t2583 = v_k.clone(); Py::Bool({ let t2584 = v_outputs.clone(); if !py::contains(&t2584, &t2583)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2625(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2627: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2626 = v_inputs.clone(); f_mtbf(py::getitem(&t2626, &Py::str("tr"))?, py::getitem(&t2626, &Py::str("tau"))?, py::getitem(&t2626, &Py::str("T0"))?, py::getitem(&t2626, &Py::str("fclk"))?, py::getitem(&t2626, &Py::str("fdata"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2627 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2628 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2650 = (t2628).items()?;
        if t2650.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2650[0].clone();
        v_v = t2650[1].clone();
        if ({ let t2629 = v_k.clone(); Py::Bool({ let t2630 = v_outputs.clone(); if !py::contains(&t2630, &t2629)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_265(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t267: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t266 = v_inputs.clone(); f_adj_regulator(py::getitem(&t266, &Py::str("R1"))?, py::getitem(&t266, &Py::str("R2"))?, py::getitem(&t266, &Py::str("Vref"))?, py::method(&t266, "get", &[Py::str("Iadj"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t267 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t268 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t290 = (t268).items()?;
        if t290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t290[0].clone();
        v_v = t290[1].clone();
        if ({ let t269 = v_k.clone(); Py::Bool({ let t270 = v_outputs.clone(); if !py::contains(&t270, &t269)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2669(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2671: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2670 = v_inputs.clone(); f_mfb_bandpass(py::getitem(&t2670, &Py::str("R1"))?, py::getitem(&t2670, &Py::str("R2"))?, py::getitem(&t2670, &Py::str("R3"))?, py::getitem(&t2670, &Py::str("C1"))?, py::getitem(&t2670, &Py::str("C2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2671 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2672 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2694 = (t2672).items()?;
        if t2694.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2694[0].clone();
        v_v = t2694[1].clone();
        if ({ let t2673 = v_k.clone(); Py::Bool({ let t2674 = v_outputs.clone(); if !py::contains(&t2674, &t2673)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2713(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2715: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2714 = v_inputs.clone(); f_mfb_lowpass(py::getitem(&t2714, &Py::str("R1"))?, py::getitem(&t2714, &Py::str("R2"))?, py::getitem(&t2714, &Py::str("R3"))?, py::getitem(&t2714, &Py::str("C1"))?, py::getitem(&t2714, &Py::str("C2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2715 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2716 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2738 = (t2716).items()?;
        if t2738.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2738[0].clone();
        v_v = t2738[1].clone();
        if ({ let t2717 = v_k.clone(); Py::Bool({ let t2718 = v_outputs.clone(); if !py::contains(&t2718, &t2717)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2758(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2760: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2759 = v_inputs.clone(); f_miller_octc(py::getitem(&t2759, &Py::str("gm"))?, py::getitem(&t2759, &Py::str("r_pi"))?, py::getitem(&t2759, &Py::str("Cpi"))?, py::getitem(&t2759, &Py::str("Cmu"))?, py::getitem(&t2759, &Py::str("RLp"))?, py::getitem(&t2759, &Py::str("Rsigp"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2760 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2761 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2783 = (t2761).items()?;
        if t2783.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2783[0].clone();
        v_v = t2783[1].clone();
        if ({ let t2762 = v_k.clone(); Py::Bool({ let t2763 = v_outputs.clone(); if !py::contains(&t2763, &t2762)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2808(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2810: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2809 = v_inputs.clone(); f_body_effect(py::getitem(&t2809, &Py::str("Vt0"))?, py::getitem(&t2809, &Py::str("gamma"))?, py::getitem(&t2809, &Py::str("phi2F"))?, py::getitem(&t2809, &Py::str("VSB"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2810 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2811 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2833 = (t2811).items()?;
        if t2833.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2833[0].clone();
        v_v = t2833[1].clone();
        if ({ let t2812 = v_k.clone(); Py::Bool({ let t2813 = v_outputs.clone(); if !py::contains(&t2813, &t2812)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2862(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2864: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2863 = v_inputs.clone(); f_mosfet_divider_bias(py::getitem(&t2863, &Py::str("VDD"))?, py::getitem(&t2863, &Py::str("R1"))?, py::getitem(&t2863, &Py::str("R2"))?, py::getitem(&t2863, &Py::str("RD"))?, py::getitem(&t2863, &Py::str("RS"))?, py::getitem(&t2863, &Py::str("kn"))?, py::getitem(&t2863, &Py::str("Vt"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2864 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2865 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2887 = (t2865).items()?;
        if t2887.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2887[0].clone();
        v_v = t2887[1].clone();
        if ({ let t2866 = v_k.clone(); Py::Bool({ let t2867 = v_outputs.clone(); if !py::contains(&t2867, &t2866)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2919(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2921: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2920 = v_inputs.clone(); f_mosfet_id(py::getitem(&t2920, &Py::str("kn"))?, py::getitem(&t2920, &Py::str("Vt"))?, py::getitem(&t2920, &Py::str("VGS"))?, py::getitem(&t2920, &Py::str("VDS"))?, py::method(&t2920, "get", &[Py::str("lam"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2921 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2922 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2944 = (t2922).items()?;
        if t2944.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2944[0].clone();
        v_v = t2944[1].clone();
        if ({ let t2923 = v_k.clone(); Py::Bool({ let t2924 = v_outputs.clone(); if !py::contains(&t2924, &t2923)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2963(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2965: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2964 = v_inputs.clone(); f_mosfet_gm_ro(py::getitem(&t2964, &Py::str("kn"))?, py::getitem(&t2964, &Py::str("ID"))?, py::getitem(&t2964, &Py::str("lam"))?, py::getitem(&t2964, &Py::str("Vt"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2965 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2966 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2988 = (t2966).items()?;
        if t2988.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2988[0].clone();
        v_v = t2988[1].clone();
        if ({ let t2967 = v_k.clone(); Py::Bool({ let t2968 = v_outputs.clone(); if !py::contains(&t2968, &t2967)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3007(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3009: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3008 = v_inputs.clone(); f_neg_feedback(py::getitem(&t3008, &Py::str("A"))?, py::getitem(&t3008, &Py::str("beta"))?, py::getitem(&t3008, &Py::str("fH"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3009 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3010 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3032 = (t3010).items()?;
        if t3032.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3032[0].clone();
        v_v = t3032[1].clone();
        if ({ let t3011 = v_k.clone(); Py::Bool({ let t3012 = v_outputs.clone(); if !py::contains(&t3012, &t3011)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3054(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3056: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3055 = v_inputs.clone(); f_noninverting_amp(py::getitem(&t3055, &Py::str("Rf"))?, py::getitem(&t3055, &Py::str("Rg"))?, py::getitem(&t3055, &Py::str("Vin"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3056 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3057 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3079 = (t3057).items()?;
        if t3079.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3079[0].clone();
        v_v = t3079[1].clone();
        if ({ let t3058 = v_k.clone(); Py::Bool({ let t3059 = v_outputs.clone(); if !py::contains(&t3059, &t3058)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3109(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3111: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3110 = v_inputs.clone(); f_opamp_offset(py::getitem(&t3110, &Py::str("Vos"))?, py::getitem(&t3110, &Py::str("IB"))?, py::getitem(&t3110, &Py::str("Ios"))?, py::getitem(&t3110, &Py::str("Rf"))?, py::getitem(&t3110, &Py::str("R1"))?, py::getitem(&t3110, &Py::str("Rp"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3111 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3112 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3134 = (t3112).items()?;
        if t3134.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3134[0].clone();
        v_v = t3134[1].clone();
        if ({ let t3113 = v_k.clone(); Py::Bool({ let t3114 = v_outputs.clone(); if !py::contains(&t3114, &t3113)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_313(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t315: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t314 = v_inputs.clone(); f_jitter_snr(py::getitem(&t314, &Py::str("f"))?, py::getitem(&t314, &Py::str("tj"))?, py::getitem(&t314, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t315 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t316 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t338 = (t316).items()?;
        if t338.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t338[0].clone();
        v_v = t338[1].clone();
        if ({ let t317 = v_k.clone(); Py::Bool({ let t318 = v_outputs.clone(); if !py::contains(&t318, &t317)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3153(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3155: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3154 = v_inputs.clone(); f_differentiator(py::getitem(&t3154, &Py::str("R"))?, py::getitem(&t3154, &Py::str("C"))?, py::getitem(&t3154, &Py::str("A"))?, py::getitem(&t3154, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3155 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3156 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3178 = (t3156).items()?;
        if t3178.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3178[0].clone();
        v_v = t3178[1].clone();
        if ({ let t3157 = v_k.clone(); Py::Bool({ let t3158 = v_outputs.clone(); if !py::contains(&t3158, &t3157)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3200(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3202: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3201 = v_inputs.clone(); f_integrator(py::getitem(&t3201, &Py::str("R"))?, py::getitem(&t3201, &Py::str("C"))?, py::getitem(&t3201, &Py::str("Vin"))?, py::getitem(&t3201, &Py::str("t"))?, py::method(&t3201, "get", &[Py::str("V0"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3202 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3203 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3225 = (t3203).items()?;
        if t3225.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3225[0].clone();
        v_v = t3225[1].clone();
        if ({ let t3204 = v_k.clone(); Py::Bool({ let t3205 = v_outputs.clone(); if !py::contains(&t3205, &t3204)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3244(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3246: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3245 = v_inputs.clone(); f_relaxation_oscillator(py::getitem(&t3245, &Py::str("R"))?, py::getitem(&t3245, &Py::str("C"))?, py::getitem(&t3245, &Py::str("R1"))?, py::getitem(&t3245, &Py::str("R2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3246 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3247 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3269 = (t3247).items()?;
        if t3269.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3269[0].clone();
        v_v = t3269[1].clone();
        if ({ let t3248 = v_k.clone(); Py::Bool({ let t3249 = v_outputs.clone(); if !py::contains(&t3249, &t3248)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3288(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3290: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3289 = v_inputs.clone(); f_pll(py::getitem(&t3289, &Py::str("Kd"))?, py::getitem(&t3289, &Py::str("Ko"))?, py::getitem(&t3289, &Py::str("tau1"))?, py::getitem(&t3289, &Py::str("tau2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3290 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3291 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3313 = (t3291).items()?;
        if t3313.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3313[0].clone();
        v_v = t3313[1].clone();
        if ({ let t3292 = v_k.clone(); Py::Bool({ let t3293 = v_outputs.clone(); if !py::contains(&t3293, &t3292)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3332(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3334: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3333 = v_inputs.clone(); f_rc_timing(py::getitem(&t3333, &Py::str("R"))?, py::getitem(&t3333, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3334 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3335 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3357 = (t3335).items()?;
        if t3357.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3357[0].clone();
        v_v = t3357[1].clone();
        if ({ let t3336 = v_k.clone(); Py::Bool({ let t3337 = v_outputs.clone(); if !py::contains(&t3337, &t3336)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3380(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3382: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3381 = v_inputs.clone(); f_rc_phase_shift(py::getitem(&t3381, &Py::str("R"))?, py::getitem(&t3381, &Py::str("C"))?, py::getitem(&t3381, &Py::str("ladder"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3382 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3383 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3405 = (t3383).items()?;
        if t3405.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3405[0].clone();
        v_v = t3405[1].clone();
        if ({ let t3384 = v_k.clone(); Py::Bool({ let t3385 = v_outputs.clone(); if !py::contains(&t3385, &t3384)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3439(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3441: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3440 = v_inputs.clone(); f_rectifier(py::getitem(&t3440, &Py::str("Vm"))?, py::getitem(&t3440, &Py::str("circuit"))?, py::getitem(&t3440, &Py::str("Von"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3441 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3442 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3464 = (t3442).items()?;
        if t3464.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3464[0].clone();
        v_v = t3464[1].clone();
        if ({ let t3443 = v_k.clone(); Py::Bool({ let t3444 = v_outputs.clone(); if !py::contains(&t3444, &t3443)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3486(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3488: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3487 = v_inputs.clone(); f_sk_equal_design(py::getitem(&t3487, &Py::str("fc"))?, py::getitem(&t3487, &Py::str("Q"))?, py::getitem(&t3487, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3488 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3489 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3511 = (t3489).items()?;
        if t3511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3511[0].clone();
        v_v = t3511[1].clone();
        if ({ let t3490 = v_k.clone(); Py::Bool({ let t3491 = v_outputs.clone(); if !py::contains(&t3491, &t3490)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3532(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3534: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3533 = v_inputs.clone(); f_sk_highpass(py::getitem(&t3533, &Py::str("C1"))?, py::getitem(&t3533, &Py::str("C2"))?, py::getitem(&t3533, &Py::str("R1"))?, py::getitem(&t3533, &Py::str("R2"))?, py::getitem(&t3533, &Py::str("K"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3534 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3535 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3557 = (t3535).items()?;
        if t3557.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3557[0].clone();
        v_v = t3557[1].clone();
        if ({ let t3536 = v_k.clone(); Py::Bool({ let t3537 = v_outputs.clone(); if !py::contains(&t3537, &t3536)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3578(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3580: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3579 = v_inputs.clone(); f_sk_lowpass(py::getitem(&t3579, &Py::str("R1"))?, py::getitem(&t3579, &Py::str("R2"))?, py::getitem(&t3579, &Py::str("C1"))?, py::getitem(&t3579, &Py::str("C2"))?, py::getitem(&t3579, &Py::str("K"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3580 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3581 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3603 = (t3581).items()?;
        if t3603.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3603[0].clone();
        v_v = t3603[1].clone();
        if ({ let t3582 = v_k.clone(); Py::Bool({ let t3583 = v_outputs.clone(); if !py::contains(&t3583, &t3582)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_359(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t361: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t360 = v_inputs.clone(); f_cascode(py::getitem(&t360, &Py::str("IC"))?, py::getitem(&t360, &Py::str("beta"))?, py::getitem(&t360, &Py::str("VT"))?, py::getitem(&t360, &Py::str("VA"))?, py::getitem(&t360, &Py::str("RL"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t361 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t362 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t384 = (t362).items()?;
        if t384.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t384[0].clone();
        v_v = t384[1].clone();
        if ({ let t363 = v_k.clone(); Py::Bool({ let t364 = v_outputs.clone(); if !py::contains(&t364, &t363)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3626(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3628: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3627 = v_inputs.clone(); f_sample_hold(py::getitem(&t3627, &Py::str("I_leak"))?, py::getitem(&t3627, &Py::str("C"))?, py::getitem(&t3627, &Py::str("t_hold"))?, py::getitem(&t3627, &Py::str("Vref"))?, py::getitem(&t3627, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3628 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3629 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3651 = (t3629).items()?;
        if t3651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3651[0].clone();
        v_v = t3651[1].clone();
        if ({ let t3630 = v_k.clone(); Py::Bool({ let t3631 = v_outputs.clone(); if !py::contains(&t3631, &t3630)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3688(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3690: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3689 = v_inputs.clone(); f_sar_conversion(py::getitem(&t3689, &Py::str("Vin"))?, py::getitem(&t3689, &Py::str("Vref"))?, py::getitem(&t3689, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3690 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3691 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3713 = (t3691).items()?;
        if t3713.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3713[0].clone();
        v_v = t3713[1].clone();
        if ({ let t3692 = v_k.clone(); Py::Bool({ let t3693 = v_outputs.clone(); if !py::contains(&t3693, &t3692)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3737(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3739: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3738 = v_inputs.clone(); f_schmitt(py::getitem(&t3738, &Py::str("Rf"))?, py::getitem(&t3738, &Py::str("R1"))?, py::getitem(&t3738, &Py::str("VsatH"))?, py::getitem(&t3738, &Py::str("VsatL"))?, py::getitem(&t3738, &Py::str("Vref"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3739 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3740 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3762 = (t3740).items()?;
        if t3762.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3762[0].clone();
        v_v = t3762[1].clone();
        if ({ let t3741 = v_k.clone(); Py::Bool({ let t3742 = v_outputs.clone(); if !py::contains(&t3742, &t3741)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3789(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3791: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3790 = v_inputs.clone(); f_conductivity(py::getitem(&t3790, &Py::str("n"))?, py::getitem(&t3790, &Py::str("p"))?, py::getitem(&t3790, &Py::str("mu_n"))?, py::getitem(&t3790, &Py::str("mu_p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3791 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3792 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3814 = (t3792).items()?;
        if t3814.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3814[0].clone();
        v_v = t3814[1].clone();
        if ({ let t3793 = v_k.clone(); Py::Bool({ let t3794 = v_outputs.clone(); if !py::contains(&t3794, &t3793)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3838(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3840: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3839 = v_inputs.clone(); f_series_pass(py::getitem(&t3839, &Py::str("Vin"))?, py::getitem(&t3839, &Py::str("Vz"))?, py::getitem(&t3839, &Py::str("Rs"))?, py::getitem(&t3839, &Py::str("RL"))?, py::getitem(&t3839, &Py::str("beta"))?, py::getitem(&t3839, &Py::str("VBE"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3840 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3841 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3863 = (t3841).items()?;
        if t3863.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3863[0].clone();
        v_v = t3863[1].clone();
        if ({ let t3842 = v_k.clone(); Py::Bool({ let t3843 = v_outputs.clone(); if !py::contains(&t3843, &t3842)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3905(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3907: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3906 = v_inputs.clone(); f_setup_hold(py::getitem(&t3906, &Py::str("T"))?, py::getitem(&t3906, &Py::str("tcq"))?, py::getitem(&t3906, &Py::str("tpd"))?, py::getitem(&t3906, &Py::str("tsu"))?, py::getitem(&t3906, &Py::str("tskew"))?, py::getitem(&t3906, &Py::str("tccq"))?, py::getitem(&t3906, &Py::str("tcd"))?, py::getitem(&t3906, &Py::str("th"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3907 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3908 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3930 = (t3908).items()?;
        if t3930.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3930[0].clone();
        v_v = t3930[1].clone();
        if ({ let t3909 = v_k.clone(); Py::Bool({ let t3910 = v_outputs.clone(); if !py::contains(&t3910, &t3909)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3954(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3956: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3955 = v_inputs.clone(); f_shockley(py::getitem(&t3955, &Py::str("Is"))?, py::getitem(&t3955, &Py::str("n"))?, py::getitem(&t3955, &Py::str("VD"))?, py::getitem(&t3955, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3956 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3957 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3979 = (t3957).items()?;
        if t3979.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3979[0].clone();
        v_v = t3979[1].clone();
        if ({ let t3958 = v_k.clone(); Py::Bool({ let t3959 = v_outputs.clone(); if !py::contains(&t3959, &t3958)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4000(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4002: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4001 = v_inputs.clone(); f_slew_rate(py::getitem(&t4001, &Py::str("SR"))?, py::getitem(&t4001, &Py::str("Vp"))?, py::getitem(&t4001, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4002 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4003 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4025 = (t4003).items()?;
        if t4025.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4025[0].clone();
        v_v = t4025[1].clone();
        if ({ let t4004 = v_k.clone(); Py::Bool({ let t4005 = v_outputs.clone(); if !py::contains(&t4005, &t4004)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4045(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4047: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4046 = v_inputs.clone(); f_source_follower(py::getitem(&t4046, &Py::str("gm"))?, py::getitem(&t4046, &Py::str("ro"))?, py::getitem(&t4046, &Py::str("RS"))?, py::getitem(&t4046, &Py::str("RL"))?, py::getitem(&t4046, &Py::str("RG"))?, py::getitem(&t4046, &Py::str("Rsig"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4047 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4048 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4070 = (t4048).items()?;
        if t4070.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4070[0].clone();
        v_v = t4070[1].clone();
        if ({ let t4049 = v_k.clone(); Py::Bool({ let t4050 = v_outputs.clone(); if !py::contains(&t4050, &t4049)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4099(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4101: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4100 = v_inputs.clone(); f_summing_amp(py::getitem(&t4100, &Py::str("V"))?, py::getitem(&t4100, &Py::str("R"))?, py::getitem(&t4100, &Py::str("Rf"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4101 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4102 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4124 = (t4102).items()?;
        if t4124.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4124[0].clone();
        v_v = t4124[1].clone();
        if ({ let t4103 = v_k.clone(); Py::Bool({ let t4104 = v_outputs.clone(); if !py::contains(&t4104, &t4103)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_413(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t415: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t414 = v_inputs.clone(); f_collector_feedback(py::getitem(&t414, &Py::str("VCC"))?, py::getitem(&t414, &Py::str("RC"))?, py::getitem(&t414, &Py::str("RF"))?, py::getitem(&t414, &Py::str("beta"))?, py::getitem(&t414, &Py::str("VBE"))?, py::method(&t414, "get", &[Py::str("RE"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t415 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t416 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t438 = (t416).items()?;
        if t438.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t438[0].clone();
        v_v = t438[1].clone();
        if ({ let t417 = v_k.clone(); Py::Bool({ let t418 = v_outputs.clone(); if !py::contains(&t418, &t417)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4143(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4145: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4144 = v_inputs.clone(); f_thermal_voltage(py::getitem(&t4144, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4145 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4146 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4168 = (t4146).items()?;
        if t4168.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4168[0].clone();
        v_v = t4168[1].clone();
        if ({ let t4147 = v_k.clone(); Py::Bool({ let t4148 = v_outputs.clone(); if !py::contains(&t4148, &t4147)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4187(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4189: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4188 = v_inputs.clone(); f_transition_frequency(py::getitem(&t4188, &Py::str("gm"))?, py::getitem(&t4188, &Py::str("beta"))?, py::getitem(&t4188, &Py::str("Cpi"))?, py::getitem(&t4188, &Py::str("Cmu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4189 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4190 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4212 = (t4190).items()?;
        if t4212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4212[0].clone();
        v_v = t4212[1].clone();
        if ({ let t4191 = v_k.clone(); Py::Bool({ let t4192 = v_outputs.clone(); if !py::contains(&t4192, &t4191)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4233(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4235: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4234 = v_inputs.clone(); f_phase_margin(py::getitem(&t4234, &Py::str("A0"))?, py::getitem(&t4234, &Py::str("f1"))?, py::getitem(&t4234, &Py::str("f2"))?, py::getitem(&t4234, &Py::str("beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4235 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4236 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4258 = (t4236).items()?;
        if t4258.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4258[0].clone();
        v_v = t4258[1].clone();
        if ({ let t4237 = v_k.clone(); Py::Bool({ let t4238 = v_outputs.clone(); if !py::contains(&t4238, &t4237)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4277(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4279: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4278 = v_inputs.clone(); f_wien_bridge(py::getitem(&t4278, &Py::str("R1"))?, py::getitem(&t4278, &Py::str("C1"))?, py::getitem(&t4278, &Py::str("R2"))?, py::getitem(&t4278, &Py::str("C2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4279 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4280 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4302 = (t4280).items()?;
        if t4302.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4302[0].clone();
        v_v = t4302[1].clone();
        if ({ let t4281 = v_k.clone(); Py::Bool({ let t4282 = v_outputs.clone(); if !py::contains(&t4282, &t4281)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4321(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4323: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4322 = v_inputs.clone(); f_wilson_mirror(py::getitem(&t4322, &Py::str("Iref"))?, py::getitem(&t4322, &Py::str("beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4323 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4324 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4346 = (t4324).items()?;
        if t4346.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4346[0].clone();
        v_v = t4346[1].clone();
        if ({ let t4325 = v_k.clone(); Py::Bool({ let t4326 = v_outputs.clone(); if !py::contains(&t4326, &t4325)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4375(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4377: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4376 = v_inputs.clone(); f_zener_regulator(py::getitem(&t4376, &Py::str("Vin"))?, py::getitem(&t4376, &Py::str("Rs"))?, py::getitem(&t4376, &Py::str("Vz"))?, py::getitem(&t4376, &Py::str("RL"))?, py::method(&t4376, "get", &[Py::str("rz"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4377 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4378 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4400 = (t4378).items()?;
        if t4400.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4400[0].clone();
        v_v = t4400[1].clone();
        if ({ let t4379 = v_k.clone(); Py::Bool({ let t4380 = v_outputs.clone(); if !py::contains(&t4380, &t4379)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4432(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4434: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4433 = v_inputs.clone(); f_zener_design(py::getitem(&t4433, &Py::str("Vin_min"))?, py::getitem(&t4433, &Py::str("Vin_max"))?, py::getitem(&t4433, &Py::str("Vz"))?, py::getitem(&t4433, &Py::str("IL_min"))?, py::getitem(&t4433, &Py::str("IL_max"))?, py::getitem(&t4433, &Py::str("Iz_min"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4434 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4435 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4457 = (t4435).items()?;
        if t4457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4457[0].clone();
        v_v = t4457[1].clone();
        if ({ let t4436 = v_k.clone(); Py::Bool({ let t4437 = v_outputs.clone(); if !py::contains(&t4437, &t4436)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_457(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t459: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t458 = v_inputs.clone(); f_bjt_currents(py::getitem(&t458, &Py::str("IB"))?, py::getitem(&t458, &Py::str("beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t459 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t460 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t482 = (t460).items()?;
        if t482.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t482[0].clone();
        v_v = t482[1].clone();
        if ({ let t461 = v_k.clone(); Py::Bool({ let t462 = v_outputs.clone(); if !py::contains(&t462, &t461)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_505(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t507: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t506 = v_inputs.clone(); f_emitter_bias(py::getitem(&t506, &Py::str("VCC"))?, py::getitem(&t506, &Py::str("VEE"))?, py::getitem(&t506, &Py::str("RB"))?, py::getitem(&t506, &Py::str("RC"))?, py::getitem(&t506, &Py::str("RE"))?, py::getitem(&t506, &Py::str("beta"))?, py::getitem(&t506, &Py::str("VBE"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t507 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t508 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t530 = (t508).items()?;
        if t530.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t530[0].clone();
        v_v = t530[1].clone();
        if ({ let t509 = v_k.clone(); Py::Bool({ let t510 = v_outputs.clone(); if !py::contains(&t510, &t509)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_553(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t555: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t554 = v_inputs.clone(); f_fixed_bias(py::getitem(&t554, &Py::str("VCC"))?, py::getitem(&t554, &Py::str("RB"))?, py::getitem(&t554, &Py::str("RC"))?, py::getitem(&t554, &Py::str("beta"))?, py::getitem(&t554, &Py::str("VBE"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t555 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t556 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t578 = (t556).items()?;
        if t578.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t578[0].clone();
        v_v = t578[1].clone();
        if ({ let t557 = v_k.clone(); Py::Bool({ let t558 = v_outputs.clone(); if !py::contains(&t558, &t557)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_597(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t599: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t598 = v_inputs.clone(); f_bjt_small_signal(py::getitem(&t598, &Py::str("IC"))?, py::getitem(&t598, &Py::str("beta"))?, py::getitem(&t598, &Py::str("VT"))?, py::getitem(&t598, &Py::str("VA"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t599 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t600 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t622 = (t600).items()?;
        if t622.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t622[0].clone();
        v_v = t622[1].clone();
        if ({ let t601 = v_k.clone(); Py::Bool({ let t602 = v_outputs.clone(); if !py::contains(&t602, &t601)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_63(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t65: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t64 = v_inputs.clone(); f_timer_monostable(py::getitem(&t64, &Py::str("R"))?, py::getitem(&t64, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t65 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t66 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t88 = (t66).items()?;
        if t88.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t88[0].clone();
        v_v = t88[1].clone();
        if ({ let t67 = v_k.clone(); Py::Bool({ let t68 = v_outputs.clone(); if !py::contains(&t68, &t67)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_646(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t648: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t647 = v_inputs.clone(); f_divider_bias(py::getitem(&t647, &Py::str("VCC"))?, py::getitem(&t647, &Py::str("R1"))?, py::getitem(&t647, &Py::str("R2"))?, py::getitem(&t647, &Py::str("RC"))?, py::getitem(&t647, &Py::str("RE"))?, py::getitem(&t647, &Py::str("beta"))?, py::getitem(&t647, &Py::str("VBE"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t648 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t649 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t671 = (t649).items()?;
        if t671.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t671[0].clone();
        v_v = t671[1].clone();
        if ({ let t650 = v_k.clone(); Py::Bool({ let t651 = v_outputs.clone(); if !py::contains(&t651, &t650)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_695(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t697: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t696 = v_inputs.clone(); f_builtin_potential(py::getitem(&t696, &Py::str("NA"))?, py::getitem(&t696, &Py::str("ND"))?, py::getitem(&t696, &Py::str("ni"))?, py::getitem(&t696, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t697 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t698 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t720 = (t698).items()?;
        if t720.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t720[0].clone();
        v_v = t720[1].clone();
        if ({ let t699 = v_k.clone(); Py::Bool({ let t700 = v_outputs.clone(); if !py::contains(&t700, &t699)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_739(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t741: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t740 = v_inputs.clone(); f_butterworth_sections(py::getitem(&t740, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t741 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t742 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t764 = (t742).items()?;
        if t764.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t764[0].clone();
        v_v = t764[1].clone();
        if ({ let t743 = v_k.clone(); Py::Bool({ let t744 = v_outputs.clone(); if !py::contains(&t744, &t743)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_789(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t791: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t790 = v_inputs.clone(); f_filter_ripple(py::getitem(&t790, &Py::str("Vp"))?, py::getitem(&t790, &Py::str("f"))?, py::getitem(&t790, &Py::str("RL"))?, py::getitem(&t790, &Py::str("C"))?, py::getitem(&t790, &Py::str("circuit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t791 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t792 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t814 = (t792).items()?;
        if t814.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t814[0].clone();
        v_v = t814[1].clone();
        if ({ let t793 = v_k.clone(); Py::Bool({ let t794 = v_outputs.clone(); if !py::contains(&t794, &t793)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_837(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t839: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t838 = v_inputs.clone(); f_chebyshev_sections(py::getitem(&t838, &Py::str("n"))?, py::getitem(&t838, &Py::str("ripple"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t839 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t840 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t862 = (t840).items()?;
        if t862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t862[0].clone();
        v_v = t862[1].clone();
        if ({ let t841 = v_k.clone(); Py::Bool({ let t842 = v_outputs.clone(); if !py::contains(&t842, &t841)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_885(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t887: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t886 = v_inputs.clone(); f_closed_loop_bw(py::getitem(&t886, &Py::str("GBW"))?, py::getitem(&t886, &Py::str("config"))?, py::getitem(&t886, &Py::str("Rf"))?, py::getitem(&t886, &Py::str("R1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t887 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t888 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t910 = (t888).items()?;
        if t910.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t910[0].clone();
        v_v = t910[1].clone();
        if ({ let t889 = v_k.clone(); Py::Bool({ let t890 = v_outputs.clone(); if !py::contains(&t890, &t889)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_932(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t934: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t933 = v_inputs.clone(); f_cmos_power(py::getitem(&t933, &Py::str("alpha"))?, py::getitem(&t933, &Py::str("C"))?, py::getitem(&t933, &Py::str("VDD"))?, py::getitem(&t933, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t934 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t935 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t957 = (t935).items()?;
        if t957.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t957[0].clone();
        v_v = t957[1].clone();
        if ({ let t936 = v_k.clone(); Py::Bool({ let t937 = v_outputs.clone(); if !py::contains(&t937, &t936)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_977(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t979: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t978 = v_inputs.clone(); f_cmos_delay(py::getitem(&t978, &Py::str("Rn"))?, py::getitem(&t978, &Py::str("Rp"))?, py::getitem(&t978, &Py::str("CL"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t979 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t980 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1002 = (t980).items()?;
        if t1002.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1002[0].clone();
        v_v = t1002[1].clone();
        if ({ let t981 = v_k.clone(); Py::Bool({ let t982 = v_outputs.clone(); if !py::contains(&t982, &t981)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
