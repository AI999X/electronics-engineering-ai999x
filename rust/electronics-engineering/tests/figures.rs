//! Electronics Engineering's own live figures (src/figures.rs): every capability's worked example gives each published
//! figure kind the model draws itself, a changed input changes the drawing, and no drawing holds a non-finite number or
//! text outside its canvas. (The kinds the generic engine draws, decision and series, are measured by
//! tools/figure_parity.py on the WebAssembly module: natively its sweeps stop at a time limit.)

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use electronics_engineering_ai999x::pack;

/// The published figure kinds the model draws itself (publish/website/models/electronics-engineering-ai999x/figures).
const KINDS: &[(&str, &[&str])] = &[
    ("elx_555_astable", &["f-vs-ra"]),
    ("elx_555_monostable", &["field"]),
    ("elx_adc_code", &["code-vs-vin"]),
    ("elx_bjt_current_relations", &["field"]),
    ("elx_diode_operating_point", &["id-vs-vs"]),
    ("elx_rc_delay_rise_time", &["field"]),
    ("elx_rc_phase_shift_oscillator", &["field"]),
    ("elx_sallen_key_lowpass", &["q-vs-r1"]),
    ("elx_wilson_current_mirror", &["field"]),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="x").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            let mut depth = 0i32;
            while i < cs.len() && !(depth == 0 && (cs[i] == ',' || cs[i] == ')')) {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                }
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        assert!(ax_engine::solve(p, key, &given, None).solved(), "{key}: the worked example does not solve");
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind && !f.slug.contains("-live-")) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// The first number of a quantity's text multiplied by f ("1000 ohm" -> "1370 ohm").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let end = t.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == 'e' || c == 'E')).unwrap_or(t.len());
    let x: f64 = t[..end].parse().ok()?;
    (x != 0.0).then(|| format!("{}{}", x * f, &t[end..]))
}

/// The model's own figure of a kind.
fn own<'a>(v: &'a [Figure], kind: &str) -> Option<&'a Figure> {
    v.iter().find(|x| x.kind == kind && !x.slug.contains("-live-"))
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut ok = false;
        'tries: for f in [1.37, 0.73] {
            for i in 0..given.len() {
                let Input::Text(t) = &given[i].1 else { continue };
                let Some(t2) = scale_text(t, f) else { continue };
                let mut g = given.clone();
                g[i].1 = Input::Text(t2);
                if !ax_engine::solve(p, key, &g, None).solved() {
                    continue;
                }
                let figs = figures(p, key, &g, None, None);
                let differs = kinds.iter().all(|kind| {
                    matches!((own(&base, kind), own(&figs, kind)), (Some(a), Some(b)) if a.svg != b.svg)
                });
                if differs {
                    for x in figs.iter().filter(|x| !x.slug.contains("-live-")) {
                        check_texts(x);
                    }
                    ok = true;
                    break 'tries;
                }
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    // a negative resistance is refused by the model: no figure at all, never a wrong one
    let p = pack();
    let given: Vec<(String, Input)> = [("R", "-1000 ohm"), ("C", "1e-11 F")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    assert!(!ax_engine::solve(p, "elx_rc_delay_rise_time", &given, None).solved());
    assert!(figures(p, "elx_rc_delay_rise_time", &given, None, None).is_empty());
}
