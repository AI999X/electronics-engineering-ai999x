//! Electronics Engineering's own live figures (ax_engine::figkit): every published figure kind of a capability, drawn
//! for the visitor's inputs from what the model computed. A curve or a shaded field is a set of further solves of the
//! capability itself; a figure the inputs make impossible is left out. Written by hand for the Rust build.

use ax_engine::figkit::{self as k, eq, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "elx_555_astable" => vec![against(s, "f-vs-ra", "RA", "f", R::Near)],
        "elx_adc_code" => vec![against(s, "code-vs-vin", "Vin", "code", R::Near)],
        "elx_diode_operating_point" => vec![against(s, "id-vs-vs", "VS", "ID", R::Near)],
        "elx_sallen_key_lowpass" => vec![against(s, "q-vs-r1", "R1", "Q", R::Near)],
        "elx_555_monostable" => vec![field(s, "R", R::Near, "C", R::Near, "T")],
        "elx_bjt_current_relations" => vec![field(s, "IB", R::Near, "beta", R::Near, "IC")],
        "elx_rc_delay_rise_time" => vec![field(s, "R", R::Near, "C", R::Near, "t_rise")],
        "elx_rc_phase_shift_oscillator" => vec![field(s, "R", R::Near, "C", R::Near, "f0")],
        "elx_wilson_current_mirror" => vec![field(s, "Iref", R::Near, "beta", R::Decade, "Iout")],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

/// The range an input is swept over, from its given value.
#[derive(Clone, Copy)]
enum R {
    /// half to one and a half times the value
    Near,
    /// a tenth to ten times (logarithmic)
    Decade,
}

fn range(x0: f64, r: R) -> Option<(f64, f64, bool)> {
    match r {
        R::Near if x0 != 0.0 => Some(((x0 * 0.5).min(x0 * 1.5), (x0 * 0.5).max(x0 * 1.5), false)),
        R::Decade if x0 > 0.0 => Some((x0 / 10.0, x0 * 10.0, true)),
        _ => None,
    }
}

fn is_int(s: &Solved, sym: &str) -> bool {
    s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).is_some_and(|i| i.ty == "integer")
}

/// A number input as read (an integer input too).
fn given(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym).or_else(|| s.val(sym).map(|v| v.num()).filter(|x| x.is_finite()))
}

/// The input value to solve with: an integer input as a whole number.
fn input(s: &Solved, sym: &str, x: f64) -> Input {
    if is_int(s, sym) { Input::Int(x.round() as i64) } else { Input::Float(x) }
}

fn unit_of_kind(s: &Solved, kind: &str) -> String {
    match s.p.kinds.get(kind).map(|x| x.unit.clone()) {
        Some(u) if u != "1" => u,
        _ => String::new(),
    }
}

/// (meaning, unit) of an input or output.
fn about(s: &Solved, sym: &str) -> (String, String) {
    let Some(p) = s.p.procedure(s.key) else { return (sym.into(), String::new()) };
    if let Some(i) = p.inputs.iter().find(|i| i.symbol == sym) {
        return (i.meaning.clone(), unit_of_kind(s, &i.kind));
    }
    if let Some(o) = p.outputs.iter().find(|o| o.symbol == sym) {
        return (o.meaning.clone(), unit_of_kind(s, &o.kind));
    }
    (sym.into(), String::new())
}

fn label(sym: &str, unit: &str) -> String {
    let u = if unit == "ohm" { "Ω" } else { unit };
    if u.is_empty() { sym.to_string() } else { format!("{sym} ({u})") }
}

fn name(s: &Solved) -> String {
    s.p.procedure(s.key).map(|p| p.name.clone()).unwrap_or_default()
}

/// One output against one input over its range (31 solves of the model), the given point marked.
fn against(s: &Solved, kind: &str, x: &str, y: &str, r: R) -> Option<Figure> {
    let (x0, y0) = (given(s, x)?, s.out(y)?);
    let (lo, hi, log) = range(x0, r)?;
    let mut pts: Vec<(f64, f64)> = vec![];
    for xv in k::span(lo, hi, 31, log) {
        let xv = if is_int(s, x) { xv.round() } else { xv };
        if pts.last().is_some_and(|p| p.0 == xv) {
            continue;
        }
        if let Some(yv) = s.with_input(&[(x, input(s, x, xv))]).and_then(|a| k::num_of(&a, y)) {
            pts.push((xv, yv));
        }
    }
    if pts.len() < 3 {
        return None;
    }
    let (mx, ux) = about(s, x);
    let (my, uy) = about(s, y);
    let n = pts.len();
    let body = k::lines_y(&[(y.to_string(), pts)], &label(x, &ux), &label(y, &uy), &eq(y, y0, &uy), Some((x0, y0)),
                          &k::q(y0, &uy), log, true);
    Some(s.chart(kind, &format!("{y} against {x}: {}", name(s)),
                 &format!("{} ({y}) against {} ({x}) from {} to {}, the other inputs as given: {n} points, each computed \
                           by the model. Marked: the inputs given, {x} = {}, {y} = {}.", cap(&my), mx, k::q(lo, &ux),
                          k::q(hi, &ux), k::q(x0, &ux), k::q(y0, &uy)), &body))
}

fn cap(t: &str) -> String {
    let mut c = t.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// One output over two inputs (17 x 13 solves of the model), each cell shaded by its value, the given point marked.
fn field(s: &Solved, xs_: &str, xr: R, ys_: &str, yr: R, out: &str) -> Option<Figure> {
    let (x0, y0) = (given(s, xs_)?, given(s, ys_)?);
    let z0 = s.out(out)?;
    let (xl, xh, xlog) = range(x0, xr)?;
    let (yl, yh, ylog) = range(y0, yr)?;
    let xs = k::span(xl, xh, 17, xlog);
    let ys = k::span(yl, yh, 13, ylog);
    let mut vals = vec![];
    let mut n = 0;
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let v = s.with_input(&[(xs_, input(s, xs_, *x)), (ys_, input(s, ys_, *y))]).and_then(|a| k::num_of(&a, out));
            n += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if n < 4 {
        return None;
    }
    let (mx, ux) = about(s, xs_);
    let (my, uy) = about(s, ys_);
    let (mz, uz) = about(s, out);
    let body = k::field(&xs, &ys, &vals, (x0, y0), &label(xs_, &ux), &label(ys_, &uy), &eq(out, z0, &uz),
                        (xlog, ylog));
    Some(s.chart("field", &format!("{}: {mz} over {mx} and {my}", name(s)),
                 &format!("{} ({out}) over {xs_} from {} to {} and {ys_} from {} to {}{}: each of the {n} cells is one \
                           computation of the model, shaded by its value (darker is larger; a blank cell: inputs the \
                           model refuses). Marked: the inputs given, {out} = {}.", cap(&mz), k::q(xl, &ux),
                          k::q(xh, &ux), k::q(yl, &uy), k::q(yh, &uy),
                          if xlog || ylog { " (logarithmic axes)" } else { "" }, k::q(z0, &uz)), &body))
}
