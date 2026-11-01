"""Electronics engineering procedures: deterministic capabilities with typed inputs and several outputs (domain code,
loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Topics: semiconductors (thermal voltage, intrinsic carrier concentration, conductivity, drift, the Einstein relation
and diffusion length, carrier concentrations of doped material, the built-in potential, the depletion region, junction
capacitance); diodes (the Shockley equation and its inverse, the operating point of a diode and resistor, rectifier
averages, capacitor-filter ripple, Zener regulators and their design, LED series resistors); bipolar transistors
(current relations, fixed, voltage-divider, emitter and collector-feedback bias, small-signal parameters, common-emitter,
common-collector and common-base amplifiers, the transition frequency, the Miller effect and open-circuit time
constants, low-frequency cutoff by short-circuit time constants, current mirrors, the Wilson mirror, the cascode, the
differential pair); MOSFETs and CMOS (drain current in every region, gm and ro, divider bias, common-source and
source-follower amplifiers, the body effect, the inverter switching threshold, propagation delay); op-amps (inverting,
non-inverting, summing, difference and instrumentation amplifiers, finite open-loop gain, integrator, differentiator,
gain-bandwidth and slew-rate limits, DC offsets, Schmitt triggers, negative feedback, two-pole phase margin); active
filters (first-order, Sallen-Key low- and high-pass, multiple-feedback low-pass and band-pass, equal-component design,
Butterworth and Chebyshev order and section Q values); oscillators (RC phase-shift, Wien bridge, Colpitts, Hartley,
crystal, op-amp relaxation, 555 astable and monostable, PLL loop dynamics); data converters (ADC code, quantisation
SNR with oversampling, ENOB, aperture jitter, R-2R and binary-weighted DACs, successive approximation, dual slope,
sample-and-hold droop); timing (RC delay and rise time, Elmore delay, CMOS dynamic power, setup and hold slack,
metastability MTBF); power supplies (linear-regulator efficiency, adjustable regulators, series-pass regulators,
heat-sink temperatures and the required heat sink).

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question form it parses, worked
examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are imported inside
the reference functions) and a deliberately wrong variant that the checks must reject. The shipped code (run, verify,
parse) is plain Python (math, re).

How the references differ from the shipped code:
- every circuit is re-solved by modified nodal analysis (numpy linear solve of the circuit's netlist: resistors,
  capacitors and inductors as admittances, independent and controlled sources, ideal op-amps); bipolar transistors are
  a base-emitter voltage source with a current-controlled collector source, small-signal transistors the hybrid-pi
  network; resistances seen by capacitors come from test sources in the same netlist;
- frequencies of filters and oscillators come from the netlist's complex response (scipy brentq on a zero phase or a
  -3 dB point; second-order coefficients read from the response at one frequency); scipy.signal for Butterworth and
  Chebyshev prototypes, order selection and loop responses;
- timing from exponential crossings found with brentq; averages and noise powers by scipy quad; semiconductor results
  by the Fermi-level and charge-neutrality equations solved with brentq, the depletion region by integrating Poisson's
  equation; the diode operating point with scipy's Wright omega function; derivatives (gm, ro, junction capacitance)
  by sympy differentiation; the ADC code by numpy digitize, the SAR result by exact rational arithmetic, the R-2R DAC by
  solving the whole ladder.
- weaker references (the same published formula evaluated a different way) are listed in procedures_certified.json.

Conventions (fixed, so run and reference agree exactly):
- SI internally; companion outputs give customary units (mV, mA, kHz, dB, degC, cm^-3).
- Constants: k = 1.380649e-23 J/K, q = 1.602176634e-19 C (exact SI values), eps0 = 8.8541878128e-12 F/m.
- Thermal voltage VT = kT/q from the stated temperature (semiconductor procedures); small-signal procedures take VT as
  stated (25 mV or 26 mV are both common).
- Carrier models: non-degenerate, complete ionisation; effective densities of states scale as (T/300 K)^1.5 with a
  temperature-independent band gap; abrupt junctions in the depletion approximation.
- Bipolar transistors: constant VBE in the active region, IC = beta IB; a bias whose VCE falls below 0.2 V is refused as
  saturated. Small-signal amplifiers neglect ro unless ro is an input. MOSFETs: square law, ID = (kn/2) VOV^2 (1 + lambda
  VDS) in saturation with kn = mu Cox W/L; triode without lambda; NMOS signs (PMOS by magnitudes).
- 555 timer: thresholds at Vcc/3 and 2Vcc/3, so tH = ln 2 (RA + RB) C, tL = ln 2 RB C, monostable T = ln 3 R C.
- ADC: unipolar, truncating, LSB = Vref/2^n, code clipped at 2^n - 1. DAC: Vout = Vref D/2^n. ENOB = (SINAD - 1.76)/6.02
  (IEEE 1241 rounded constants); SNR of an ideal converter uses the exact 20 log10(2^n sqrt(1.5)).
- CMOS dynamic power P = alpha C VDD^2 f with alpha the 0 -> 1 transition probability per clock cycle.
- Lists carry no unit: their unit is stated in the input (volts, kilohms, ohms, picofarads).
- Results built from exp, log or roots are rounded to 12 significant digits and exact zeros are written as 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Shockley diode current: Is = 1e-14 A, n = 1.5, VD = 0.65 V, T = 300 K`). Physical values carry a unit; fractions may
be written with %; decibels with dB; lists in brackets; choices as words. A thermal resistance may be written in degC/W
(read as K/W). Any other number, an unknown name or a name given twice is refused, never guessed.
"""
from __future__ import annotations

import math
import re

# ================================================================================================ constants
K_B = 1.380649e-23               # J/K
Q_E = 1.602176634e-19            # C
EPS0 = 8.8541878128e-12          # F/m
T0C = 273.15                     # K
LN2 = math.log(2.0)
LN3 = math.log(3.0)
TWO_PI = 2.0 * math.pi


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str) or x is None:
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] + [0.0])
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    if abs(a) < 1e-15 and abs(b) < 1e-15:
        return 0.0
    return d / max(abs(a), abs(b))


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _fin(*vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v), "every input must be a finite number")


def _pos(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v > 0,
              f"{name} must be a positive number")


def _nonneg(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v >= 0,
              f"{name} must be zero or positive")


def _par(*rs):
    """Parallel combination of resistances (all positive)."""
    s = 0.0
    for x in rs:
        s += 1.0 / x
    return 1.0 / s


def _db(x):
    return 20.0 * math.log10(abs(x))


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _sig(x, n=4):
    """x rounded to n significant digits (random inputs written as a user would write them)."""
    return float(f"{x:.{n}g}")


# ---------------------------------------------------------------------------------------------- reference helpers
def _brentq(f, a, b):
    from scipy.optimize import brentq
    return float(brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500))


def _quad(f, a, b, points=None):
    from scipy.integrate import quad
    v, _ = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=400, points=points)
    return float(v)


def _mna(n, elems, s=0.0):
    """Reference helper: modified nodal analysis with numpy. Nodes 1..n (0 is ground). Elements:
    ("R", a, b, ohm) ("Y", a, b, siemens) ("C", a, b, farad) ("L", a, b, henry)
    ("I", a, b, amp)            a current source: the current flows from a to b through the source (into b)
    ("V", a, b, volt)           V(a) - V(b) = volt; its unknown current flows from a through the source to b
    ("E", a, b, c, d, gain)     V(a) - V(b) = gain (V(c) - V(d))
    ("Gm", a, b, c, d, g)       a current g (V(c) - V(d)) flows from a to b through the source
    ("F", a, b, k, gain)        a current gain * (current of the source elems[k]) flows from a to b through the source
    ("OA", out, p, m)           an ideal op-amp: V(p) = V(m), any output current
    Returns (v, cur): node voltages (v[0] = 0) and a dict of source currents by element index."""
    import numpy as np
    rows = {}
    k = n
    for i, e in enumerate(elems):
        if e[0] in ("V", "E", "OA"):
            rows[i] = k
            k += 1
    A = np.zeros((k, k), dtype=complex)
    z = np.zeros(k, dtype=complex)

    def y(a, b, val):
        if a:
            A[a - 1, a - 1] += val
        if b:
            A[b - 1, b - 1] += val
        if a and b:
            A[a - 1, b - 1] -= val
            A[b - 1, a - 1] -= val

    for i, e in enumerate(elems):
        t = e[0]
        if t == "R":
            y(e[1], e[2], 1.0 / e[3])
        elif t == "Y":
            y(e[1], e[2], e[3])
        elif t == "C":
            y(e[1], e[2], s * e[3])
        elif t == "L":
            y(e[1], e[2], 1.0 / (s * e[3]))
        elif t == "I":
            a, b, cur = e[1], e[2], e[3]
            if a:
                z[a - 1] -= cur
            if b:
                z[b - 1] += cur
        elif t == "Gm":
            a, b, c, d, g = e[1:]
            for node, sgn in ((a, 1.0), (b, -1.0)):
                if node:
                    if c:
                        A[node - 1, c - 1] += sgn * g
                    if d:
                        A[node - 1, d - 1] -= sgn * g
        elif t == "V":
            a, b, val = e[1], e[2], e[3]
            r = rows[i]
            if a:
                A[a - 1, r] += 1.0
                A[r, a - 1] += 1.0
            if b:
                A[b - 1, r] -= 1.0
                A[r, b - 1] -= 1.0
            z[r] = val
        elif t == "E":
            a, b, c, d, g = e[1:]
            r = rows[i]
            if a:
                A[a - 1, r] += 1.0
                A[r, a - 1] += 1.0
            if b:
                A[b - 1, r] -= 1.0
                A[r, b - 1] -= 1.0
            if c:
                A[r, c - 1] -= g
            if d:
                A[r, d - 1] += g
        elif t == "OA":
            out, p, m = e[1:]
            r = rows[i]
            if out:
                A[out - 1, r] += 1.0
            if p:
                A[r, p - 1] += 1.0
            if m:
                A[r, m - 1] -= 1.0
    for i, e in enumerate(elems):                       # current-controlled sources, once every row is known
        if e[0] == "F":
            a, b, kk, g = e[1:]
            r = rows[kk]
            if a:
                A[a - 1, r] += g
            if b:
                A[b - 1, r] -= g
    x = np.linalg.solve(A, z)
    v = [0j] + [complex(x[j]) for j in range(n)]
    cur = {i: complex(x[r]) for i, r in rows.items()}
    return v, cur


def _re(v):
    return [float(c.real) for c in v]


# ================================================================================================ units
_KU = {"temperature": "K", "voltage": "V", "current": "A", "resistance": "ohm", "capacitance": "F", "inductance": "H",
       "frequency": "Hz", "angular_frequency": "rad/s", "time": "s", "power": "W", "length": "m", "area": "m^2",
       "energy": "J", "speed": "m/s", "number_density": "1/m^3", "carrier_mobility": "m^2/(V*s)",
       "diffusion_coefficient": "m^2/s", "electrical_conductivity": "1/(ohm*m)", "resistivity": "ohm*m",
       "current_density": "A/m^2", "electric_field": "N/C", "capacitance_per_area": "F/m^2", "conductance": "S",
       "transconductance_parameter": "A/V^2", "inverse_voltage": "1/V", "voltage_slew_rate": "V/s",
       "thermal_resistance": "K/W", "number": ""}

# how a value of a kind is written in templates and questions (unit, factor from SI); SI when absent
_SHOW = {"number_density": ("cm^-3", 1e-6), "carrier_mobility": ("cm^2/(V*s)", 1e4), "diffusion_coefficient": ("cm^2/s", 1e4),
         "energy": ("eV", 1.0 / Q_E), "conductance": ("mS", 1e3), "transconductance_parameter": ("mA/V^2", 1e3),
         "voltage_slew_rate": ("V/us", 1e-6), "capacitance_per_area": ("F/m^2", 1.0)}

_THERMAL = re.compile(r"^(?:°C|degC|deg C|C)\s*/\s*W$")


def _written_unit(num: str, unit: str, kind: str):
    """'value unit' for the runtime: a thermal resistance in degC/W is the same number in K/W."""
    u = unit.strip()
    if kind == "thermal_resistance" and _THERMAL.match(u):
        u = "K/W"
    return f"{num} {u}"


# ================================================================================================ the question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[[^\]]*\])\s*$")
_PCT = {"%": 0.01, "percent": 0.01, "per cent": 0.01}


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _squash(text):
    """A choice word without case, spaces, hyphens or underscores ('Non-inverting' and 'non inverting' agree)."""
    s = ""
    for ch in text.lower():
        if ch not in " -_":
            s += ch
    return s


def _choice_value(text, choices):
    """The documented choice a written word names, compared without case, spaces, hyphens or underscores; a word
    that names no choice is passed on as written (the runtime then refuses it as an invalid choice)."""
    w = _squash(text.strip())
    for c in choices:
        if _squash(c) == w:
            return c
    return text.strip()


def _reader(phrases, inputs, aliases=None):
    """The one documented question form, '<phrase>: name = value, ...', for a procedure with these inputs.

    The reading guard is built in: every stated number is bound to a named input (any other text or number refuses
    the whole question); the capability phrase must open the question and be followed by the colon; a physical value
    without its unit, or a unit on a pure number, is refused; values are passed with their units so the runtime
    converts each exactly once."""
    rx = re.compile(r"\s*(?:" + "|".join(re.escape(p) for p in phrases) + r")\s*:\s*", re.I)
    names = {}
    by = {}
    for i in inputs:
        names[i["symbol"]] = i["symbol"]
        by[i["symbol"]] = i
    for i in inputs:                       # a lower-case spelling only where it names no other input
        names.setdefault(i["symbol"].lower(), i["symbol"])
    for a, s in (aliases or {}).items():
        names[a] = s
        names[a.lower()] = s

    def parse(question):
        if not isinstance(question, str):
            return None
        m = rx.match(question)
        if not m:
            return None
        rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
        if not rest:
            return {}
        out = {}
        for seg in _split(rest):
            if not seg.strip():
                return None
            mv = _VEC.match(seg)
            if mv:
                key = names.get(mv.group(1), names.get(mv.group(1).lower()))
                if key is None or key in out or by[key].get("type") != "vector":
                    return None
                out[key] = mv.group(2).strip()
                continue
            mw = _WORD.match(seg)
            if mw:
                key = names.get(mw.group(1), names.get(mw.group(1).lower()))
                if key is not None and by[key].get("type") == "choice":
                    if key in out:
                        return None
                    out[key] = _choice_value(mw.group(2), by[key]["choices"])
                    continue
                return None
            ms = _SEG.match(seg)
            if not ms:
                return None
            key = names.get(ms.group(1), names.get(ms.group(1).lower()))
            if key is None or key in out:
                return None
            spec = by[key]
            t = spec.get("type", "number")
            num, unit = ms.group(2), ms.group(3).strip()
            if t in ("vector", "choice", "text"):
                return None
            kind = spec.get("kind", "number")
            if t == "integer":
                if unit:
                    return None
                out[key] = num
                continue
            if kind != "number":
                if not unit:
                    return None
                out[key] = _written_unit(num, unit, kind)
                continue
            if unit:
                if spec.get("pct") and unit.lower() in _PCT:
                    out[key] = float(num) * _PCT[unit.lower()]
                    continue
                if unit in spec.get("words", ()):
                    out[key] = float(num)
                    continue
                return None
            if spec.get("pct") and abs(float(num)) > 1:
                return None                       # '20' for a fraction: 20 % or 20? refused; write 20% or 0.2
            if spec.get("words") and spec.get("unit_required"):
                return None                       # a decibel value without 'dB' is refused
            out[key] = float(num)
        return out
    return parse


def _fmt(x):
    return f"{x:.10g}"


def _written(spec, v):
    """An input value as a user writes it in the documented form."""
    t = spec.get("type", "number")
    if t == "vector":
        return "[" + ", ".join(_fmt(float(x)) for x in v) + "]"
    if t in ("choice", "text"):
        return str(v)
    if t == "integer":
        return str(int(v))
    kind = spec.get("kind", "number")
    if kind != "number":
        unit, f = spec.get("show") or _SHOW.get(kind) or (_KU[kind], 1.0)
        if unit == "degC":
            return f"{_fmt(float(v) - T0C)} degC"
        return f"{_fmt(float(v) * f)} {unit}"
    if spec.get("words"):
        return f"{_fmt(float(v))} {spec['words'][0]}"
    return _fmt(float(v))


# ================================================================================================ input/output specs
def _phys(sym, kind, meaning, required=True, show=None):
    _KU[kind]                                   # every physical kind has its SI unit here
    d = {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}
    if show:
        d["show"] = show
    return d


def _numin(sym, meaning, required=True, pct=False, words=None, unit_required=False):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}
    if pct:
        d["pct"] = True
    if words:
        d["words"] = list(words)
    if unit_required:
        d["unit_required"] = True
    return d


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _vecin(sym, meaning, lo=1, hi=50, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}
LEVELS: dict = {}
WEAK: dict = {}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rand,
         phrases, examples, level, conditions=(), aliases=None, extra_q=(), verify=None, weak=None):
    """Register a procedure: the documented question form is built from its phrases and inputs; templates and
    questions are written from the worked examples exactly as a user writes them."""
    parse = _reader(phrases, inputs, aliases)
    by = {i["symbol"]: i for i in inputs}

    def ask_text(ex_inputs):
        return f"{phrases[0]}: " + ", ".join(f"{k} = {_written(by[k], v)}" for k, v in ex_inputs.items())
    templates = [ask_text(e["inputs"]) for e in examples[:2]]
    questions = [{"q": ask_text(e["inputs"]), "outputs": e["outputs"]} for e in examples]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if req and examples:
        partial = {k: v for k, v in examples[0]["inputs"].items() if k != req[0]}
        questions.append({"q": ask_text(partial), "outcome": "NEEDS_INPUT"})
        questions.append({"q": ask_text(examples[0]["inputs"]) + ", zz = 1", "outcome": "UNRECOGNIZED_FORMAT"})
    questions += [{"q": q["q"], "outputs": examples[q["like"]]["outputs"]} if "like" in q else q for q in extra_q]
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or _recheck(run), "parse": parse, "reference": reference, "wrong": _safe(wrong),
        "random_inputs": rand, "templates": templates, "examples": list(examples), "questions": questions}
    LEVELS[key] = level
    if weak:
        WEAK[key] = weak


def _ex(inputs, outputs=None):
    return {"inputs": inputs, "outputs": dict(outputs or {})}


def _vt(T):
    return K_B * T / Q_E


# ================================================================================================ SEMICONDUCTORS
def thermal_voltage(T):
    _pos("the temperature T", T)
    VT = _vt(T)
    return _out(VT=VT, VT_mV=VT * 1e3)


def _ref_thermal_voltage(T):
    from scipy.constants import k, e
    VT = k * T / e
    return {"VT": VT, "VT_mV": VT * 1e3}


_add("elx_thermal_voltage",
     name="Thermal voltage kT/q", topic="semiconductors / thermal voltage",
     purpose="The thermal voltage VT = kT/q at an absolute temperature (about 25.85 mV at 300 K), the voltage scale of every diode and transistor equation.",
     inputs=[_phys("T", "temperature", "absolute temperature (degC is converted)")],
     outputs=[_o("VT", "voltage", "thermal voltage"), _o("VT_mV", "number", "thermal voltage in millivolts")],
     primary="VT", conditions=["T > 0 K"], method="VT = k T / q",
     latex=r"V_T = \frac{kT}{q}",
     run=thermal_voltage, reference=_ref_thermal_voltage,
     wrong=lambda T: thermal_voltage(T - T0C),
     rand=lambda rng: {"T": _sig(rng.uniform(290.0, 500.0), 4)},
     phrases=("Thermal voltage", "Thermal voltage kT/q"),
     examples=[_ex({"T": 300.0}), _ex({"T": 350.0})],
     level="beginner",
     extra_q=[{"q": "Thermal voltage: T = 26.85 degC", "like": 0}],
     weak="the same definition evaluated with scipy.constants")


def intrinsic_concentration(Nc, Nv, Eg, T):
    _pos("Nc, Nv, Eg and T", Nc, Nv, Eg, T)
    s = (T / 300.0) ** 1.5
    nc, nv = Nc * s, Nv * s
    kT = K_B * T
    ni = math.sqrt(nc * nv) * math.exp(-Eg / (2.0 * kT))
    off = 0.5 * kT * math.log(nv / nc) / Q_E
    return _out(ni=ni, ni_cm3=ni * 1e-6, Ei_minus_midgap_eV=off, Nc_T=nc, Nv_T=nv)


def _ref_intrinsic_concentration(Nc, Nv, Eg, T):
    import numpy as np
    nc, nv = Nc * np.power(T / 300.0, 1.5), Nv * np.power(T / 300.0, 1.5)
    kT = K_B * T
    # intrinsic Fermi level (measured from the valence band edge): electrons n = Nc exp(-(Ec-EF)/kT) equal holes
    # p = Nv exp(-(EF-Ev)/kT); found by root finding, then ni = n at that level
    f = lambda ef: (math.log(nc) - (Eg - ef) / kT) - (math.log(nv) - ef / kT)
    ef = _brentq(f, -Eg, 2.0 * Eg)
    ni = nc * math.exp(-(Eg - ef) / kT)
    return {"ni": ni, "ni_cm3": ni * 1e-6, "Ei_minus_midgap_eV": (ef - 0.5 * Eg) / Q_E, "Nc_T": float(nc), "Nv_T": float(nv)}


def _wrong_intrinsic(Nc, Nv, Eg, T):
    r = intrinsic_concentration(Nc, Nv, Eg, T)
    s = (T / 300.0) ** 1.5
    ni = math.sqrt(Nc * Nv) * s * math.exp(-Eg / (K_B * T))          # the factor 2 in the exponent dropped
    return dict(r, ni=_r(ni), ni_cm3=_r(ni * 1e-6))


_add("elx_intrinsic_carrier_concentration",
     name="Intrinsic carrier concentration", topic="semiconductors / carrier statistics",
     purpose="The intrinsic carrier concentration ni = sqrt(Nc Nv) exp(-Eg/2kT) at a temperature, from the effective densities of states at 300 K (scaled as T^1.5) and the band gap; also the intrinsic level's offset from midgap.",
     inputs=[_phys("Nc", "number_density", "effective density of states in the conduction band at 300 K"),
             _phys("Nv", "number_density", "effective density of states in the valence band at 300 K"),
             _phys("Eg", "energy", "band gap energy"),
             _phys("T", "temperature", "absolute temperature")],
     outputs=[_o("ni", "number_density", "intrinsic carrier concentration"), _o("ni_cm3", "number", "ni in cm^-3"),
              _o("Ei_minus_midgap_eV", "number", "intrinsic Fermi level above midgap, eV"),
              _o("Nc_T", "number_density", "Nc at T"), _o("Nv_T", "number_density", "Nv at T")],
     primary="ni", conditions=["Nc, Nv, Eg, T > 0", "non-degenerate statistics, band gap independent of T"],
     method="Nc(T) = Nc (T/300)^1.5, Nv likewise; ni = sqrt(Nc Nv) exp(-Eg/(2kT)); Ei - Emid = (kT/2) ln(Nv/Nc)",
     latex=r"n_i = \sqrt{N_cN_v}\,e^{-E_g/2kT},\quad N_{c,v}(T) = N_{c,v}(300)\left(\tfrac{T}{300}\right)^{3/2}",
     run=intrinsic_concentration, reference=_ref_intrinsic_concentration, wrong=_wrong_intrinsic,
     rand=lambda rng: {"Nc": _sig(_lu(rng, 1e24, 5e25), 3), "Nv": _sig(_lu(rng, 1e24, 2e25), 3),
                       "Eg": _sig(rng.uniform(0.6, 1.8), 3) * Q_E, "T": _sig(rng.uniform(250.0, 500.0), 4)},
     phrases=("Intrinsic carrier concentration", "Intrinsic concentration"),
     examples=[_ex({"Nc": 2.8e25, "Nv": 1.04e25, "Eg": 1.12 * Q_E, "T": 300.0}),
               _ex({"Nc": 4.7e23, "Nv": 7.0e24, "Eg": 1.42 * Q_E, "T": 400.0})],
     level="advanced")


def conductivity(n, p, mu_n, mu_p):
    _nonneg("the carrier concentrations n and p", n, p)
    _pos("the mobilities", mu_n, mu_p)
    _need(n + p > 0, "at least one carrier concentration must be positive")
    s = Q_E * (n * mu_n + p * mu_p)
    return _out(sigma=s, rho=1.0 / s, rho_ohm_cm=100.0 / s)


def _ref_conductivity(n, p, mu_n, mu_p):
    from scipy.constants import e
    import numpy as np
    s = float(np.dot([n, p], [mu_n, mu_p])) * e
    return {"sigma": s, "rho": 1.0 / s, "rho_ohm_cm": 100.0 / s}


_add("elx_semiconductor_conductivity",
     name="Conductivity and resistivity of a semiconductor", topic="semiconductors / conduction",
     purpose="The conductivity sigma = q (n mu_n + p mu_p) and the resistivity of a semiconductor from its electron and hole concentrations and mobilities.",
     inputs=[_phys("n", "number_density", "electron concentration"), _phys("p", "number_density", "hole concentration"),
             _phys("mu_n", "carrier_mobility", "electron mobility"), _phys("mu_p", "carrier_mobility", "hole mobility")],
     outputs=[_o("sigma", "electrical_conductivity", "conductivity"), _o("rho", "resistivity", "resistivity"),
              _o("rho_ohm_cm", "number", "resistivity in ohm cm")],
     primary="sigma", conditions=["n, p >= 0 (not both zero)", "mobilities > 0"],
     method="sigma = q (n mu_n + p mu_p); rho = 1/sigma",
     latex=r"\sigma = q\,(n\mu_n + p\mu_p),\quad \rho = 1/\sigma",
     run=conductivity, reference=_ref_conductivity,
     wrong=lambda n, p, mu_n, mu_p: (lambda s: _out(sigma=s, rho=1.0 / s, rho_ohm_cm=100.0 / s))(Q_E * (n + p) * mu_n),
     rand=lambda rng: (lambda n: {"n": n, "p": _sig(n * _lu(rng, 0.1, 10.0), 3), "mu_n": _sig(rng.uniform(0.05, 0.4), 3),
                                  "mu_p": _sig(rng.uniform(0.01, 0.05), 3)})(_sig(_lu(rng, 1e18, 1e24), 3)),
     phrases=("Semiconductor conductivity", "Conductivity of a semiconductor"),
     examples=[_ex({"n": 1e22, "p": 2.25e10, "mu_n": 0.135, "mu_p": 0.048}),
               _ex({"n": 1.5e16, "p": 1.5e16, "mu_n": 0.135, "mu_p": 0.048})],
     level="beginner",
     weak="the same definition evaluated as a numpy dot product with scipy's elementary charge")


def drift_current(n, mu, V, L, A):
    _pos("n, mu, L and A", n, mu, L, A)
    _fin(V)
    E = V / L
    vd = mu * E
    J = Q_E * n * vd
    return _out(E=E, v_d=vd, J=J, I=J * A)


def _ref_drift_current(n, mu, V, L, A):
    from scipy.constants import e
    # the conductance of the bar, G = sigma A / L, then Ohm's law
    sigma = e * n * mu
    I = sigma * A / L * V
    return {"E": V / L, "v_d": I / (e * n * A), "J": I / A, "I": I}


_add("elx_drift_current",
     name="Drift velocity and drift current in a semiconductor bar", topic="semiconductors / drift",
     purpose="The electric field, the carrier drift velocity, the drift current density and the current when a voltage is applied along a uniformly doped bar.",
     inputs=[_phys("n", "number_density", "concentration of the carriers that drift"),
             _phys("mu", "carrier_mobility", "their mobility"), _phys("V", "voltage", "voltage applied along the bar"),
             _phys("L", "length", "bar length"), _phys("A", "area", "cross-sectional area")],
     outputs=[_o("E", "electric_field", "electric field (V/m)"), _o("v_d", "speed", "drift velocity"),
              _o("J", "current_density", "drift current density"), _o("I", "current", "current")],
     primary="J", conditions=["n, mu, L, A > 0", "low field (constant mobility)"],
     method="E = V/L; v_d = mu E; J = q n v_d; I = J A",
     latex=r"J = q\,n\,\mu\,E,\quad E = V/L",
     run=drift_current, reference=_ref_drift_current,
     wrong=lambda n, mu, V, L, A: (lambda J: _out(E=V / L, v_d=mu * V / L, J=J, I=J * A))(Q_E * n * mu * V),
     rand=lambda rng: {"n": _sig(_lu(rng, 1e20, 1e24), 3), "mu": _sig(rng.uniform(0.01, 0.4), 3), "V": _sig(rng.uniform(0.1, 10.0), 3),
                       "L": _sig(_lu(rng, 1e-5, 1e-2), 3), "A": _sig(_lu(rng, 1e-10, 1e-6), 3)},
     phrases=("Drift current", "Drift current in a semiconductor"),
     examples=[_ex({"n": 1e22, "mu": 0.135, "V": 5.0, "L": 1e-3, "A": 1e-8}),
               _ex({"n": 5e21, "mu": 0.048, "V": 2.0, "L": 5e-4, "A": 2.5e-9})],
     level="beginner",
     weak="Ohm's law for the bar (conductance sigma A / L) instead of the field and drift velocity")


def einstein(mu, T, tau):
    _pos("mu, T and tau", mu, T, tau)
    D = mu * _vt(T)
    Ld = math.sqrt(D * tau)
    return _out(D=D, D_cm2=D * 1e4, L_diff=Ld, L_um=Ld * 1e6)


def _ref_einstein(mu, T, tau):
    from scipy.constants import k, e
    import numpy as np
    D = mu * k * T / e
    Ld = float(np.sqrt(D * tau))
    return {"D": D, "D_cm2": D * 1e4, "L_diff": Ld, "L_um": Ld * 1e6}


_add("elx_einstein_diffusion",
     name="Einstein relation and diffusion length", topic="semiconductors / diffusion",
     purpose="The diffusion coefficient D = mu kT/q of carriers from their mobility (Einstein relation) and the diffusion length L = sqrt(D tau) from the minority-carrier lifetime.",
     inputs=[_phys("mu", "carrier_mobility", "carrier mobility"), _phys("T", "temperature", "absolute temperature"),
             _phys("tau", "time", "minority-carrier lifetime", show=("us", 1e6))],
     outputs=[_o("D", "diffusion_coefficient", "diffusion coefficient"), _o("D_cm2", "number", "D in cm^2/s"),
              _o("L_diff", "length", "diffusion length"), _o("L_um", "number", "diffusion length in micrometres")],
     primary="D", conditions=["mu, T, tau > 0", "non-degenerate material"],
     method="D = mu kT/q; L = sqrt(D tau)",
     latex=r"D = \mu\frac{kT}{q},\quad L = \sqrt{D\tau}",
     run=einstein, reference=_ref_einstein,
     wrong=lambda mu, T, tau: (lambda D: _out(D=D, D_cm2=D * 1e4, L_diff=math.sqrt(D * tau), L_um=math.sqrt(D * tau) * 1e6))(mu / _vt(T) * 1e-3),
     rand=lambda rng: {"mu": _sig(rng.uniform(0.01, 0.4), 3), "T": _sig(rng.uniform(250.0, 450.0), 4), "tau": _sig(_lu(rng, 1e-8, 1e-4), 3)},
     phrases=("Einstein relation", "Diffusion coefficient and diffusion length"),
     examples=[_ex({"mu": 0.135, "T": 300.0, "tau": 1e-6}), _ex({"mu": 0.048, "T": 300.0, "tau": 1e-5})],
     level="intermediate",
     weak="the same relations evaluated with scipy.constants and numpy")


def doped_carriers(ND, NA, ni):
    _nonneg("the donor and acceptor concentrations", ND, NA)
    _pos("the intrinsic concentration ni", ni)
    d = ND - NA
    h = 0.5 * abs(d)
    big = h + math.sqrt(h * h + ni * ni)
    if d >= 0:
        n, p = big, ni * ni / big
    else:
        p, n = big, ni * ni / big
    t = "n-type" if d > 0 else ("p-type" if d < 0 else "intrinsic")
    return _out(n=n, p=p, n_cm3=n * 1e-6, p_cm3=p * 1e-6, type=t)


def _ref_doped_carriers(ND, NA, ni):
    # charge neutrality n - p - (ND - NA) = 0 with mass action p = ni^2/n, solved for ln n
    d = ND - NA
    f = lambda x: math.exp(x) - ni * ni * math.exp(-x) - d
    x = _brentq(f, math.log(ni) - 80.0, math.log(ni) + 80.0)
    n = math.exp(x)
    p = ni * ni / n
    t = "n-type" if d > 0 else ("p-type" if d < 0 else "intrinsic")
    return {"n": n, "p": p, "n_cm3": n * 1e-6, "p_cm3": p * 1e-6, "type": t}


def _wrong_doped(ND, NA, ni):
    d = ND - NA                                     # complete-ionisation shortcut: the majority density is |ND - NA|
    if d > 0:
        n, p = d, ni * ni / d
    elif d < 0:
        p, n = -d, ni * ni / -d
    else:
        n = p = ni
    return _out(n=n, p=p, n_cm3=n * 1e-6, p_cm3=p * 1e-6, type="n-type" if d > 0 else ("p-type" if d < 0 else "intrinsic"))


def _rand_doped(rng):
    ni = _sig(_lu(rng, 1e14, 1e19), 3)
    if rng.random() < 0.8:
        NA = _sig(ni * _lu(rng, 0.1, 1e3), 6)
        ND = _sig(max(0.0, NA + rng.choice([-1, 1]) * ni * _lu(rng, 0.05, 20.0)), 6)
    else:
        NA, ND = _sig(_lu(rng, 1e20, 1e24), 3), _sig(_lu(rng, 1e20, 1e24), 3)
    return {"ND": ND, "NA": NA, "ni": ni}


_add("elx_doped_carrier_concentrations",
     name="Electron and hole concentrations of doped (compensated) material", topic="semiconductors / carrier statistics",
     purpose="The equilibrium electron and hole concentrations of a semiconductor with donors and acceptors, from charge neutrality and the mass-action law (exact, so it also holds for light or compensated doping).",
     inputs=[_phys("ND", "number_density", "donor concentration"), _phys("NA", "number_density", "acceptor concentration"),
             _phys("ni", "number_density", "intrinsic carrier concentration")],
     outputs=[_o("n", "number_density", "electron concentration"), _o("p", "number_density", "hole concentration"),
              _o("n_cm3", "number", "n in cm^-3"), _o("p_cm3", "number", "p in cm^-3"),
              _o("type", "number", "n-type, p-type or intrinsic")],
     primary="n", conditions=["ND, NA >= 0; ni > 0", "complete ionisation, non-degenerate"],
     method="n - p = ND - NA and n p = ni^2: majority = |ND-NA|/2 + sqrt((ND-NA)^2/4 + ni^2), minority = ni^2/majority",
     latex=r"n = \frac{N_D - N_A}{2} + \sqrt{\left(\frac{N_D - N_A}{2}\right)^2 + n_i^2},\quad p = \frac{n_i^2}{n}",
     run=doped_carriers, reference=_ref_doped_carriers, wrong=_wrong_doped, rand=_rand_doped,
     phrases=("Carrier concentrations of doped material", "Doped carrier concentrations"),
     examples=[_ex({"ND": 1e22, "NA": 0.0, "ni": 1.5e16}), _ex({"ND": 3e16, "NA": 5e16, "ni": 1.5e16})],
     level="intermediate")


def builtin_potential(NA, ND, ni, T):
    _pos("NA, ND, ni and T", NA, ND, ni, T)
    _need(NA >= 100.0 * ni and ND >= 100.0 * ni, "the doping on each side must be at least 100 times ni (the formula assumes NA, ND >> ni)")
    Vbi = _vt(T) * math.log(NA * ND / (ni * ni))
    return _out(Vbi=Vbi)


def _ref_builtin_potential(NA, ND, ni, T):
    # Fermi-level positions on each side from exact neutrality: EF_n - Ei = kT ln(n/ni), Ei - EF_p = kT ln(p/ni)
    from scipy.constants import k, e
    nn = _ref_doped_carriers(ND, 0.0, ni)["n"]
    pp = _ref_doped_carriers(0.0, NA, ni)["p"]
    return {"Vbi": k * T / e * (math.log(nn / ni) + math.log(pp / ni))}


_add("elx_builtin_potential",
     name="Built-in potential of a pn junction", topic="semiconductors / pn junction",
     purpose="The built-in (contact) potential of an abrupt pn junction from the doping on each side, the intrinsic concentration and the temperature.",
     inputs=[_phys("NA", "number_density", "acceptor concentration on the p side"),
             _phys("ND", "number_density", "donor concentration on the n side"),
             _phys("ni", "number_density", "intrinsic carrier concentration"), _phys("T", "temperature", "absolute temperature")],
     outputs=[_o("Vbi", "voltage", "built-in potential")],
     primary="Vbi", conditions=["NA, ND >= 100 ni", "non-degenerate, complete ionisation"],
     method="Vbi = (kT/q) ln(NA ND / ni^2)",
     latex=r"V_{bi} = \frac{kT}{q}\ln\frac{N_AN_D}{n_i^2}",
     run=builtin_potential, reference=_ref_builtin_potential,
     wrong=lambda NA, ND, ni, T: _out(Vbi=_vt(T) * math.log(NA * ND / ni)),
     rand=lambda rng: (lambda ni: {"NA": _sig(ni * _lu(rng, 1e5, 1e9), 3), "ND": _sig(ni * _lu(rng, 1e5, 1e9), 3), "ni": ni,
                                   "T": _sig(rng.uniform(250.0, 400.0), 4)})(_sig(_lu(rng, 1e14, 1e17), 3)),
     phrases=("Built-in potential", "Built-in potential of a pn junction"),
     examples=[_ex({"NA": 1e23, "ND": 1e22, "ni": 1.5e16, "T": 300.0}), _ex({"NA": 1e24, "ND": 1e21, "ni": 1e16, "T": 300.0})],
     level="intermediate")


def depletion(NA, ND, Vbi, VR, eps_r):
    _pos("NA, ND, Vbi and eps_r", NA, ND, Vbi, eps_r)
    _fin(VR)
    Vt = Vbi + VR
    _need(Vt > 0, "Vbi + VR must be positive (a forward bias below the built-in potential)")
    eps = eps_r * EPS0
    W = math.sqrt(2.0 * eps * Vt / Q_E * (1.0 / NA + 1.0 / ND))
    xn = W * NA / (NA + ND)
    xp = W * ND / (NA + ND)
    Emax = Q_E * ND * xn / eps
    return _out(W=W, W_um=W * 1e6, xn=xn, xp=xp, Emax=Emax, Cj_area=eps / W)


def _ref_depletion(NA, ND, Vbi, VR, eps_r):
    # Poisson's equation: the field grows linearly through each charged layer; the potential drop is the integral of
    # the field; find the n-side width that drops Vbi + VR with charge neutrality NA xp = ND xn
    eps = eps_r * EPS0
    Vt = Vbi + VR

    def drop(xn):
        xp = ND * xn / NA
        e_p = lambda x: Q_E * NA * (x + xp) / eps
        e_n = lambda x: Q_E * ND * (xn - x) / eps
        return _quad(e_p, -xp, 0.0) + _quad(e_n, 0.0, xn) - Vt
    xn = _brentq(drop, 0.0, 1e-1)
    xp = ND * xn / NA
    W = xn + xp
    Emax = _quad(lambda x: Q_E * ND / eps, 0.0, xn)
    return {"W": W, "W_um": W * 1e6, "xn": xn, "xp": xp, "Emax": Emax, "Cj_area": eps / W}


_add("elx_depletion_width",
     name="Depletion width, field and capacitance of an abrupt pn junction", topic="semiconductors / pn junction",
     purpose="The depletion-region width (and its parts on each side), the peak electric field and the depletion capacitance per area of an abrupt pn junction at a reverse bias (negative VR for a forward bias below Vbi).",
     inputs=[_phys("NA", "number_density", "acceptor concentration (p side)"), _phys("ND", "number_density", "donor concentration (n side)"),
             _phys("Vbi", "voltage", "built-in potential"), _phys("VR", "voltage", "reverse bias (negative for forward)"),
             _numin("eps_r", "relative permittivity of the semiconductor (11.7 for silicon)")],
     outputs=[_o("W", "length", "depletion width"), _o("W_um", "number", "depletion width in micrometres"),
              _o("xn", "length", "width on the n side"), _o("xp", "length", "width on the p side"),
              _o("Emax", "electric_field", "peak electric field (V/m)"), _o("Cj_area", "capacitance_per_area", "depletion capacitance per area")],
     primary="W", conditions=["Vbi + VR > 0", "abrupt junction, depletion approximation"],
     method="W = sqrt(2 eps (Vbi + VR)/q (1/NA + 1/ND)); xn = W NA/(NA+ND); Emax = q ND xn/eps; C' = eps/W",
     latex=r"W = \sqrt{\frac{2\varepsilon(V_{bi}+V_R)}{q}\left(\frac{1}{N_A}+\frac{1}{N_D}\right)}",
     run=depletion, reference=_ref_depletion,
     wrong=lambda NA, ND, Vbi, VR, eps_r: (lambda W: _out(W=W, W_um=W * 1e6, xn=W * NA / (NA + ND), xp=W * ND / (NA + ND),
                                                          Emax=Q_E * ND * W * NA / (NA + ND) / (eps_r * EPS0), Cj_area=eps_r * EPS0 / W))(
         math.sqrt(eps_r * EPS0 * (Vbi + VR) / Q_E * (1.0 / NA + 1.0 / ND))),
     rand=lambda rng: {"NA": _sig(_lu(rng, 1e21, 1e24), 3), "ND": _sig(_lu(rng, 1e21, 1e24), 3), "Vbi": _sig(rng.uniform(0.5, 0.9), 3),
                       "VR": _sig(rng.uniform(-0.3, 10.0), 3), "eps_r": _sig(rng.uniform(10.0, 16.0), 3)},
     phrases=("Depletion width", "Depletion region of a pn junction"),
     examples=[_ex({"NA": 1e23, "ND": 1e22, "Vbi": 0.81, "VR": 5.0, "eps_r": 11.7}),
               _ex({"NA": 1e22, "ND": 1e22, "Vbi": 0.7, "VR": 0.0, "eps_r": 11.7})],
     level="advanced")


def junction_capacitance(Cj0, Vbi, VR, m):
    _pos("Cj0 and Vbi", Cj0, Vbi)
    _fin(VR, m)
    _need(0.0 < m < 1.0, "the grading coefficient m must lie between 0 and 1 (1/2 abrupt, 1/3 linearly graded)")
    _need(VR > -Vbi, "VR must exceed -Vbi")
    ratio = (1.0 + VR / Vbi) ** (-m)
    return _out(Cj=Cj0 * ratio, ratio=ratio)


_JC = {}


def _ref_junction_capacitance(Cj0, Vbi, VR, m):
    # the depletion charge Q(V) = Cj0 Vbi/(1-m) (1 - (1 + V/Vbi)^(1-m)); the capacitance is |dQ/dV| (sympy)
    if "f" not in _JC:
        import sympy as sp
        c0, vb, v, mm = sp.symbols("c0 vb v mm", positive=True)
        Q = c0 * vb / (1 - mm) * (1 - (1 + v / vb) ** (1 - mm))
        _JC["f"] = sp.lambdify((c0, vb, v, mm), -sp.diff(Q, v), "mpmath")
    import mpmath
    mpmath.mp.dps = 30
    Cj = float(_JC["f"](mpmath.mpf(Cj0), mpmath.mpf(Vbi), mpmath.mpf(VR), mpmath.mpf(m)))
    return {"Cj": Cj, "ratio": Cj / Cj0}


_add("elx_junction_capacitance",
     name="Junction (depletion) capacitance versus bias", topic="semiconductors / pn junction",
     purpose="The depletion capacitance of a pn junction at a reverse bias from its zero-bias value, the built-in potential and the grading coefficient (1/2 abrupt, 1/3 linearly graded), as used for varactors.",
     inputs=[_phys("Cj0", "capacitance", "zero-bias junction capacitance", show=("pF", 1e12)),
             _phys("Vbi", "voltage", "built-in potential"), _phys("VR", "voltage", "reverse bias (negative for forward)"),
             _numin("m", "grading coefficient (0.5 abrupt, 0.33 linearly graded)")],
     outputs=[_o("Cj", "capacitance", "junction capacitance"), _o("ratio", "number", "Cj/Cj0")],
     primary="Cj", conditions=["0 < m < 1", "VR > -Vbi"],
     method="Cj = Cj0 / (1 + VR/Vbi)^m",
     latex=r"C_j = \frac{C_{j0}}{\left(1 + V_R/V_{bi}\right)^{m}}",
     run=junction_capacitance, reference=_ref_junction_capacitance,
     wrong=lambda Cj0, Vbi, VR, m: _out(Cj=Cj0 * (1.0 + VR / Vbi) ** m, ratio=(1.0 + VR / Vbi) ** m),
     rand=lambda rng: {"Cj0": _sig(_lu(rng, 1e-13, 1e-10), 3), "Vbi": _sig(rng.uniform(0.5, 0.9), 3), "VR": _sig(rng.uniform(-0.3, 12.0), 3),
                       "m": _sig(rng.uniform(0.3, 0.6), 3)},
     phrases=("Junction capacitance", "Depletion capacitance"),
     examples=[_ex({"Cj0": 2e-12, "Vbi": 0.75, "VR": 5.0, "m": 0.5}), _ex({"Cj0": 50e-12, "Vbi": 0.7, "VR": 2.0, "m": 0.33})],
     level="advanced")


# ================================================================================================ DIODES
def shockley(Is, n, VD, T):
    _pos("Is, n and T", Is, n, T)
    _fin(VD)
    VT = _vt(T)
    x = VD / (n * VT)
    _need(x < 700.0, "the forward voltage is far beyond any real diode (exp overflow)")
    return _out(ID=Is * math.expm1(x), VT=VT)


def _ref_shockley(Is, n, VD, T):
    import mpmath
    from scipy.constants import k, e
    mpmath.mp.dps = 30
    VT = mpmath.mpf(k) * T / mpmath.mpf(e)
    return {"ID": float(Is * (mpmath.exp(VD / (n * VT)) - 1)), "VT": float(VT)}


_add("elx_shockley_diode_current",
     name="Diode current from the Shockley equation", topic="diodes / Shockley equation",
     purpose="The current of a pn-junction diode at a terminal voltage from its saturation current, ideality factor and temperature (forward or reverse).",
     inputs=[_phys("Is", "current", "saturation current"), _numin("n", "ideality (emission) factor, 1 to 2"),
             _phys("VD", "voltage", "diode voltage (negative for reverse bias)"), _phys("T", "temperature", "junction temperature")],
     outputs=[_o("ID", "current", "diode current"), _o("VT", "voltage", "thermal voltage")],
     primary="ID", conditions=["Is, n, T > 0"],
     method="ID = Is (exp(VD/(n VT)) - 1), VT = kT/q",
     latex=r"I_D = I_S\left(e^{V_D/nV_T} - 1\right)",
     run=shockley, reference=_ref_shockley,
     wrong=lambda Is, n, VD, T: _out(ID=Is * math.expm1(VD / _vt(T)), VT=_vt(T)),
     rand=lambda rng: {"Is": _sig(_lu(rng, 1e-15, 1e-9), 3), "n": _sig(rng.uniform(1.05, 2.0), 3), "VD": _sig(rng.uniform(-0.5, 0.75), 3),
                       "T": _sig(rng.uniform(250.0, 400.0), 4)},
     phrases=("Shockley diode current", "Diode current"),
     examples=[_ex({"Is": 1e-14, "n": 1.0, "VD": 0.65, "T": 300.0}), _ex({"Is": 2e-12, "n": 1.8, "VD": -2.0, "T": 300.0})],
     level="beginner",
     weak="the same equation evaluated in 30-digit mpmath")


def diode_voltage(ID, Is, n, T):
    _pos("Is, n and T", Is, n, T)
    _fin(ID)
    _need(ID > -Is, "a diode current cannot be below -Is")
    VT = _vt(T)
    return _out(VD=n * VT * math.log1p(ID / Is), VT=VT)


def _ref_diode_voltage(ID, Is, n, T):
    from scipy.constants import k, e
    VT = k * T / e
    VD = _brentq(lambda v: Is * math.expm1(v / (n * VT)) - ID, -60.0 * n * VT, 80.0 * n * VT)
    return {"VD": VD, "VT": VT}


_add("elx_diode_voltage",
     name="Diode voltage for a given current", topic="diodes / Shockley equation",
     purpose="The voltage across a pn-junction diode carrying a given current (the inverse of the Shockley equation).",
     inputs=[_phys("ID", "current", "diode current"), _phys("Is", "current", "saturation current"),
             _numin("n", "ideality factor"), _phys("T", "temperature", "junction temperature")],
     outputs=[_o("VD", "voltage", "diode voltage"), _o("VT", "voltage", "thermal voltage")],
     primary="VD", conditions=["ID > -Is", "Is, n, T > 0"],
     method="VD = n VT ln(1 + ID/Is)",
     latex=r"V_D = nV_T\ln\left(1 + \frac{I_D}{I_S}\right)",
     run=diode_voltage, reference=_ref_diode_voltage,
     wrong=lambda ID, Is, n, T: _out(VD=_vt(T) * math.log(ID / Is), VT=_vt(T)),
     rand=lambda rng: (lambda Is: {"ID": _sig(_lu(rng, 1e-6, 0.1), 3) if rng.random() < 0.85 else -_sig(Is * rng.uniform(0.05, 0.9), 3),
                                   "Is": Is, "n": _sig(rng.uniform(1.05, 2.0), 3), "T": _sig(rng.uniform(250.0, 400.0), 4)})(_sig(_lu(rng, 1e-15, 1e-9), 3)),
     phrases=("Diode voltage for a current", "Diode forward voltage"),
     examples=[_ex({"ID": 1e-3, "Is": 1e-14, "n": 1.0, "T": 300.0}), _ex({"ID": 0.02, "Is": 5e-12, "n": 1.7, "T": 320.0})],
     level="beginner")


def diode_operating_point(VS, R, Is, n, T):
    _pos("VS, R, Is, n and T", VS, R, Is, n, T)
    nVT = n * _vt(T)
    lo = 0.0
    hi = min(VS, nVT * math.log1p(VS / (R * Is)))     # at hi the diode alone would carry VS/R
    for _ in range(400):                              # bisection on the diode voltage: diode current = resistor current
        mid = 0.5 * (lo + hi)
        if mid == lo or mid == hi:
            break
        g = Is * math.expm1(mid / nVT) - (VS - mid) / R
        if g > 0:
            hi = mid
        else:
            lo = mid
    VD = 0.5 * (lo + hi)
    ID = (VS - VD) / R
    return _out(ID=ID, VD=VD, P_D=VD * ID)


def _ref_diode_operating_point(VS, R, Is, n, T):
    # closed form with the Lambert W function in its Wright-omega form: ID = (nVT/R) W(...) - Is
    from scipy.special import wrightomega
    from scipy.constants import k, e
    nVT = n * k * T / e
    z = math.log(Is * R / nVT) + (VS + Is * R) / nVT
    ID = nVT / R * float(wrightomega(z).real) - Is
    VD = VS - ID * R
    return {"ID": ID, "VD": VD, "P_D": VD * ID}


_add("elx_diode_operating_point",
     name="Operating point of a diode in series with a resistor (exact)", topic="diodes / operating point",
     purpose="The current and voltage of a forward-biased diode fed from a DC source through a resistor, solving the Shockley equation and Kirchhoff's voltage law together (no constant-drop approximation).",
     inputs=[_phys("VS", "voltage", "source voltage"), _phys("R", "resistance", "series resistance"),
             _phys("Is", "current", "saturation current"), _numin("n", "ideality factor"), _phys("T", "temperature", "junction temperature")],
     outputs=[_o("ID", "current", "diode current"), _o("VD", "voltage", "diode voltage"), _o("P_D", "power", "power in the diode")],
     primary="ID", conditions=["VS > 0 (forward bias)", "R, Is, n, T > 0"],
     method="solve VS = ID R + n VT ln(1 + ID/Is) (bisection on VD)",
     latex=r"V_S = I_DR + nV_T\ln\left(1 + \frac{I_D}{I_S}\right)",
     run=diode_operating_point, reference=_ref_diode_operating_point,
     wrong=lambda VS, R, Is, n, T: _out(ID=(VS - 0.7) / R, VD=0.7, P_D=0.7 * (VS - 0.7) / R),
     rand=lambda rng: {"VS": _sig(rng.uniform(1.0, 20.0), 3), "R": _sig(_lu(rng, 100.0, 1e5), 3), "Is": _sig(_lu(rng, 1e-15, 1e-9), 3),
                       "n": _sig(rng.uniform(1.0, 2.0), 3), "T": _sig(rng.uniform(270.0, 400.0), 4)},
     phrases=("Diode operating point", "Diode and resistor operating point"),
     examples=[_ex({"VS": 5.0, "R": 1000.0, "Is": 1e-14, "n": 1.0, "T": 300.0}),
               _ex({"VS": 12.0, "R": 2200.0, "Is": 3e-9, "n": 1.9, "T": 300.0})],
     level="intermediate")


_RECT = ("half-wave", "bridge", "center-tapped")


def rectifier(Vm, circuit, Von):
    _pos("the peak voltage Vm", Vm)
    _nonneg("the diode drop Von", Von)
    _need(circuit in _RECT, "circuit: half-wave, bridge or center-tapped")
    V = 2.0 * Von if circuit == "bridge" else Von
    _need(V < Vm, "the diode drops must be below the peak voltage")
    t1 = math.asin(V / Vm)
    c = math.cos(t1)
    span = math.pi - 2.0 * t1
    dc = (2.0 * Vm * c - V * span) / TWO_PI
    ms = (Vm * Vm * (span / 2.0 + math.sin(2.0 * t1) / 2.0) - 4.0 * Vm * V * c + V * V * span) / TWO_PI
    if circuit != "half-wave":
        dc, ms = 2.0 * dc, 2.0 * ms
    piv = {"half-wave": Vm, "bridge": Vm - Von, "center-tapped": 2.0 * Vm - Von}[circuit]
    return _out(Vp_out=Vm - V, Vdc=dc, Vrms=math.sqrt(ms), ripple_factor=math.sqrt(ms / (dc * dc) - 1.0), PIV=piv,
                conduction_deg=math.degrees(span))


def _ref_rectifier(Vm, circuit, Von):
    V = 2.0 * Von if circuit == "bridge" else Von
    t1 = _brentq(lambda t: Vm * math.sin(t) - V, 0.0, math.pi / 2) if V > 0 else 0.0
    if circuit == "half-wave":
        w = lambda t: max(0.0, Vm * math.sin(t) - V)
    else:
        w = lambda t: max(0.0, Vm * abs(math.sin(t)) - V)
    pts = sorted(set(p for p in (t1, math.pi - t1, math.pi, math.pi + t1, 2 * math.pi - t1) if 1e-12 < p < TWO_PI - 1e-12))
    dc = _quad(w, 0.0, TWO_PI, points=pts) / TWO_PI
    ms = _quad(lambda t: w(t) ** 2, 0.0, TWO_PI, points=pts) / TWO_PI
    piv = {"half-wave": Vm, "bridge": Vm - Von, "center-tapped": 2.0 * Vm - Von}[circuit]
    return {"Vp_out": Vm - V, "Vdc": dc, "Vrms": math.sqrt(ms), "ripple_factor": math.sqrt(ms / dc ** 2 - 1.0), "PIV": piv,
            "conduction_deg": math.degrees(math.pi - 2.0 * t1)}


def _wrong_rectifier(Vm, circuit, Von):
    r = rectifier(Vm, circuit, Von)
    V = 2.0 * Von if circuit == "bridge" else Von
    k = 1.0 if circuit == "half-wave" else 2.0
    return dict(r, Vdc=_r(k * (Vm - V) / math.pi))           # the ideal-rectifier average applied to the reduced peak


_add("elx_rectifier_averages",
     name="Rectifier output: average, RMS, ripple factor and PIV", topic="diodes / rectifiers",
     purpose="The DC (average) and RMS output voltage, ripple factor, peak inverse voltage and conduction angle of an unfiltered half-wave, bridge or centre-tapped full-wave rectifier with a constant diode drop (0 V for ideal diodes).",
     inputs=[_phys("Vm", "voltage", "peak AC voltage of the (secondary) winding"),
             _choice("circuit", _RECT, "half-wave, bridge or center-tapped (Vm per half of the winding)"),
             _phys("Von", "voltage", "forward drop of one diode (0 V for ideal diodes)")],
     outputs=[_o("Vp_out", "voltage", "peak output voltage"), _o("Vdc", "voltage", "average output voltage"),
              _o("Vrms", "voltage", "RMS output voltage"), _o("ripple_factor", "number", "ripple factor (AC rms / DC)"),
              _o("PIV", "voltage", "peak inverse voltage of a diode"), _o("conduction_deg", "number", "conduction angle per half cycle, degrees")],
     primary="Vdc", conditions=["diode drops below Vm", "resistive load, no filter"],
     method="output max(0, Vm|sin| - V) with V = Von (2 Von for a bridge); averages integrated exactly; PIV Vm, Vm - Von, 2Vm - Von",
     latex=r"V_{dc} = \frac{k}{2\pi}\left[2V_m\cos\theta_1 - V(\pi - 2\theta_1)\right],\ \theta_1 = \arcsin\frac{V}{V_m}",
     run=rectifier, reference=_ref_rectifier, wrong=_wrong_rectifier,
     rand=lambda rng: {"Vm": _sig(rng.uniform(3.0, 50.0), 3), "circuit": rng.choice(_RECT), "Von": _sig(rng.uniform(0.3, 1.0), 2)},
     phrases=("Rectifier output", "Rectifier averages"),
     examples=[_ex({"Vm": 10.0, "circuit": "half-wave", "Von": 0.0}), _ex({"Vm": 17.0, "circuit": "bridge", "Von": 0.7})],
     level="beginner",
     extra_q=[{"q": "Rectifier output: Vm = 10 V, circuit = half wave, Von = 0 V", "like": 0}],
     weak="PIV is the published table for each circuit (the averages and angle are integrated independently)")


def filter_ripple(Vp, f, RL, C, circuit):
    _pos("Vp, f, RL and C", Vp, f, RL, C)
    _need(circuit in ("half-wave", "full-wave"), "circuit: half-wave or full-wave")
    fr = f if circuit == "half-wave" else 2.0 * f
    Vr = Vp / (fr * RL * C)
    _need(Vr < Vp, "the ripple would exceed the peak: the small-ripple formula does not apply")
    Vdc = Vp - Vr / 2.0
    rms = Vr / (2.0 * math.sqrt(3.0))
    return _out(f_ripple=fr, Vr=Vr, Vdc=Vdc, Vr_rms=rms, ripple_factor=rms / Vdc)


def _ref_filter_ripple(Vp, f, RL, C, circuit):
    fr = f if circuit == "half-wave" else 2.0 * f
    T = 1.0 / fr
    Vr = _quad(lambda t: Vp / RL / C, 0.0, T)              # constant load current discharging C for a ripple period
    saw = lambda t: Vr * (t / T - 0.5)                      # sawtooth ripple about its mean
    rms = math.sqrt(_quad(lambda t: saw(t) ** 2, 0.0, T) / T)
    Vdc = Vp - Vr / 2.0
    return {"f_ripple": fr, "Vr": Vr, "Vdc": Vdc, "Vr_rms": rms, "ripple_factor": rms / Vdc}


_add("elx_capacitor_filter_ripple",
     name="Capacitor-input filter: ripple voltage and DC output", topic="diodes / rectifier filters",
     purpose="The peak-to-peak ripple, the DC output, the RMS ripple and the ripple factor of a rectifier with a smoothing capacitor and resistive load (small-ripple approximation).",
     inputs=[_phys("Vp", "voltage", "peak rectified voltage (after the diode drops)"), _phys("f", "frequency", "line frequency"),
             _phys("RL", "resistance", "load resistance"), _phys("C", "capacitance", "filter capacitance", show=("uF", 1e6)),
             _choice("circuit", ("half-wave", "full-wave"), "half-wave or full-wave rectification")],
     outputs=[_o("f_ripple", "frequency", "ripple frequency"), _o("Vr", "voltage", "peak-to-peak ripple"),
              _o("Vdc", "voltage", "DC output voltage"), _o("Vr_rms", "voltage", "RMS ripple"), _o("ripple_factor", "number", "ripple factor")],
     primary="Vr", conditions=["Vr < Vp (small ripple)"],
     method="Vr = Vp/(f_r RL C); Vdc = Vp - Vr/2; Vr,rms = Vr/(2 sqrt 3)",
     latex=r"V_r = \frac{V_p}{f_rR_LC},\quad V_{dc} = V_p - \frac{V_r}{2}",
     run=filter_ripple, reference=_ref_filter_ripple,
     wrong=lambda Vp, f, RL, C, circuit: filter_ripple(Vp, f, RL, C, "half-wave"),
     rand=lambda rng: {"Vp": _sig(rng.uniform(5.0, 40.0), 3), "f": rng.choice([50.0, 60.0]), "RL": _sig(_lu(rng, 200.0, 5000.0), 3),
                       "C": _sig(_lu(rng, 2e-4, 5e-3), 3), "circuit": "full-wave" if rng.random() < 0.8 else "half-wave"},
     phrases=("Capacitor filter ripple", "Rectifier ripple voltage"),
     examples=[_ex({"Vp": 16.3, "f": 60.0, "RL": 500.0, "C": 1000e-6, "circuit": "full-wave"}),
               _ex({"Vp": 12.0, "f": 50.0, "RL": 1000.0, "C": 470e-6, "circuit": "half-wave"})],
     level="intermediate",
     weak="the ripple as the integral of a constant discharge current and the RMS of the sawtooth by quadrature (the same small-ripple model)")


def zener_regulator(Vin, Rs, Vz, RL, rz=0.0):
    _pos("Vin, Rs, Vz and RL", Vin, Rs, Vz, RL)
    _nonneg("the Zener resistance rz", rz)
    _need(Vin * RL / (Rs + RL) > Vz, "the Zener is not in breakdown: the open-circuit voltage across the load is below Vz")
    if rz == 0:
        V = Vz
        Iz = (Vin - Vz) / Rs - Vz / RL
    else:
        V = (Vin / Rs + Vz / rz) / (1.0 / Rs + 1.0 / RL + 1.0 / rz)
        Iz = (V - Vz) / rz
    return _out(Vout=V, I_s=(Vin - V) / Rs, I_L=V / RL, I_z=Iz, P_z=V * Iz)


def _ref_zener_regulator(Vin, Rs, Vz, RL, rz=0.0):
    if rz == 0:
        el = [("V", 1, 0, Vin), ("R", 1, 2, Rs), ("R", 2, 0, RL), ("V", 2, 0, Vz)]
        v, cur = _mna(2, el)
        V, Iz = v[2].real, cur[3].real
    else:
        el = [("V", 1, 0, Vin), ("R", 1, 2, Rs), ("R", 2, 0, RL), ("R", 2, 3, rz), ("V", 3, 0, Vz)]
        v, cur = _mna(3, el)
        V, Iz = v[2].real, cur[4].real
    return {"Vout": V, "I_s": (Vin - V) / Rs, "I_L": V / RL, "I_z": Iz, "P_z": V * Iz}


def _rand_zener(rng):
    Vz = _sig(rng.uniform(3.0, 15.0), 3)
    d = {"Vin": _sig(Vz * rng.uniform(1.3, 3.0), 3), "Rs": _sig(_lu(rng, 50.0, 1000.0), 3), "Vz": Vz, "RL": _sig(_lu(rng, 500.0, 10000.0), 3)}
    if rng.random() < 0.5:
        d["rz"] = _sig(rng.uniform(1.0, 20.0), 3)
    return d


_add("elx_zener_regulator",
     name="Zener shunt regulator: load voltage and currents", topic="diodes / Zener regulators",
     purpose="The output voltage and the source, load and Zener currents and the Zener power of a Zener shunt regulator (series resistor, Zener across the load), with an optional Zener resistance.",
     inputs=[_phys("Vin", "voltage", "input voltage"), _phys("Rs", "resistance", "series resistance"),
             _phys("Vz", "voltage", "Zener (breakdown) voltage"), _phys("RL", "resistance", "load resistance"),
             _phys("rz", "resistance", "Zener resistance (0 for an ideal Zener)", required=False)],
     outputs=[_o("Vout", "voltage", "load voltage"), _o("I_s", "current", "current in the series resistor"),
              _o("I_L", "current", "load current"), _o("I_z", "current", "Zener current"), _o("P_z", "power", "Zener power")],
     primary="I_z", conditions=["the Zener must be in breakdown: Vin RL/(Rs + RL) > Vz"],
     method="ideal: Vout = Vz, Iz = (Vin - Vz)/Rs - Vz/RL; with rz: node equation at the load",
     latex=r"I_Z = \frac{V_{in} - V_Z}{R_S} - \frac{V_Z}{R_L}",
     run=zener_regulator, reference=_ref_zener_regulator,
     wrong=lambda Vin, Rs, Vz, RL, rz=0.0: (lambda r: dict(r, I_z=r["I_s"], P_z=_r(r["Vout"] * r["I_s"])))(zener_regulator(Vin, Rs, Vz, RL, rz)),
     rand=_rand_zener,
     phrases=("Zener regulator", "Zener shunt regulator"),
     examples=[_ex({"Vin": 20.0, "Rs": 220.0, "Vz": 10.0, "RL": 1000.0}), _ex({"Vin": 15.0, "Rs": 100.0, "Vz": 5.1, "RL": 470.0, "rz": 7.0})],
     level="intermediate",
     extra_q=[{"q": "Zener regulator: Vin = 12 V, Rs = 1 kohm, Vz = 9 V, RL = 1 kohm", "outcome": "INVALID_INPUT"}])


def zener_design(Vin_min, Vin_max, Vz, IL_min, IL_max, Iz_min):
    _pos("Vin_min, Vin_max, Vz and Iz_min", Vin_min, Vin_max, Vz, Iz_min)
    _nonneg("the load currents", IL_min, IL_max)
    _need(Vin_min > Vz, "Vin_min must exceed Vz")
    _need(Vin_max >= Vin_min and IL_max >= IL_min, "the maxima must not be below the minima")
    Rs = (Vin_min - Vz) / (Iz_min + IL_max)
    Izmax = (Vin_max - Vz) / Rs - IL_min
    return _out(Rs_max=Rs, Iz_max=Izmax, Pz_max=Vz * Izmax, PRs_max=(Vin_max - Vz) ** 2 / Rs)


def _ref_zener_design(Vin_min, Vin_max, Vz, IL_min, IL_max, Iz_min):
    # the largest Rs keeps the worst case (lowest input, largest load) at Iz_min: found by root finding
    iz = lambda R, vin, il: (vin - Vz) / R - il
    Rs = _brentq(lambda R: iz(R, Vin_min, IL_max) - Iz_min, 1e-9, 1e12)
    Izmax = iz(Rs, Vin_max, IL_min)
    return {"Rs_max": Rs, "Iz_max": Izmax, "Pz_max": Vz * Izmax, "PRs_max": (Vin_max - Vz) ** 2 / Rs}


def _rand_zener_design(rng):
    Vz = _sig(rng.uniform(3.0, 15.0), 3)
    vmin = _sig(Vz * rng.uniform(1.2, 2.0), 3)
    ilmax = _sig(rng.uniform(0.01, 0.2), 3)
    return {"Vin_min": vmin, "Vin_max": _sig(vmin * rng.uniform(1.05, 1.5), 3), "Vz": Vz, "IL_min": _sig(ilmax * rng.uniform(0.0, 0.6), 3),
            "IL_max": ilmax, "Iz_min": _sig(rng.uniform(0.001, 0.02), 3)}


_add("elx_zener_series_resistor_design",
     name="Zener regulator design: series resistor and Zener rating", topic="diodes / Zener regulators",
     purpose="The largest series resistance that keeps a Zener regulator in regulation at the lowest input and largest load, and the resulting worst-case Zener current and power (highest input, smallest load) and resistor power.",
     inputs=[_phys("Vin_min", "voltage", "lowest input voltage"), _phys("Vin_max", "voltage", "highest input voltage"),
             _phys("Vz", "voltage", "Zener voltage"), _phys("IL_min", "current", "smallest load current", show=("mA", 1e3)),
             _phys("IL_max", "current", "largest load current", show=("mA", 1e3)),
             _phys("Iz_min", "current", "minimum Zener current for regulation", show=("mA", 1e3))],
     outputs=[_o("Rs_max", "resistance", "largest series resistance"), _o("Iz_max", "current", "worst-case Zener current"),
              _o("Pz_max", "power", "worst-case Zener power"), _o("PRs_max", "power", "worst-case resistor power")],
     primary="Rs_max", conditions=["Vin_min > Vz", "Vin_max >= Vin_min, IL_max >= IL_min >= 0"],
     method="Rs = (Vin,min - Vz)/(Iz,min + IL,max); Iz,max = (Vin,max - Vz)/Rs - IL,min; Pz = Vz Iz,max",
     latex=r"R_S = \frac{V_{in,min} - V_Z}{I_{Z,min} + I_{L,max}}",
     run=zener_design, reference=_ref_zener_design,
     wrong=lambda Vin_min, Vin_max, Vz, IL_min, IL_max, Iz_min: zener_design(Vin_min, Vin_max, Vz, IL_min, IL_min, Iz_min),
     rand=_rand_zener_design,
     phrases=("Zener regulator design", "Zener series resistor design"),
     examples=[_ex({"Vin_min": 12.0, "Vin_max": 15.0, "Vz": 9.1, "IL_min": 0.0, "IL_max": 0.05, "Iz_min": 0.005}),
               _ex({"Vin_min": 9.0, "Vin_max": 11.0, "Vz": 5.6, "IL_min": 0.01, "IL_max": 0.1, "Iz_min": 0.002})],
     level="advanced",
     weak="the resistor found by root finding on the worst-case Zener current; the powers then follow by the same products")


def led_resistor(Vs, Vf, If, n=1):
    _pos("Vs, Vf and If", Vs, Vf, If)
    _need(isinstance(n, int) and n >= 1, "the number of LEDs in series must be a whole number of at least 1")
    _need(Vs > n * Vf, "the supply must exceed the total forward voltage")
    R = (Vs - n * Vf) / If
    return _out(R=R, P_R=If * If * R, P_LED=n * Vf * If)


def _ref_led_resistor(Vs, Vf, If, n=1):
    def current(R):
        el = [("V", 1, 0, Vs), ("R", 1, 2, R)]
        node = 2
        for i in range(n):                                  # each LED a forward-voltage source
            el.append(("V", node + i, node + i + 1 if i < n - 1 else 0, Vf))
        v, cur = _mna(1 + n, el)
        return -cur[0].real
    R = _brentq(lambda R: current(R) - If, 1e-6, 1e9)
    return {"R": R, "P_R": If * If * R, "P_LED": n * Vf * If}


_add("elx_led_series_resistor",
     name="LED series (current-limiting) resistor", topic="diodes / LEDs",
     purpose="The current-limiting resistor for one LED or a string of LEDs in series at a chosen forward current, and the power in the resistor and in the LEDs.",
     inputs=[_phys("Vs", "voltage", "supply voltage"), _phys("Vf", "voltage", "forward voltage of one LED"),
             _phys("If", "current", "LED forward current", show=("mA", 1e3)), _intin("n", "number of LEDs in series (default 1)", 1, 50, required=False)],
     outputs=[_o("R", "resistance", "series resistance"), _o("P_R", "power", "power in the resistor"), _o("P_LED", "power", "power in the LEDs")],
     primary="R", conditions=["Vs > n Vf", "If > 0"],
     method="R = (Vs - n Vf)/If; P_R = If^2 R",
     latex=r"R = \frac{V_S - nV_F}{I_F}",
     run=led_resistor, reference=_ref_led_resistor,
     wrong=lambda Vs, Vf, If, n=1: _out(R=Vs / If, P_R=Vs * If, P_LED=n * Vf * If),
     rand=lambda rng: (lambda n, vf: {"Vs": _sig(n * vf * rng.uniform(1.2, 3.0), 3), "Vf": vf, "If": _sig(rng.uniform(0.002, 0.03), 3), "n": n})(
         rng.randint(1, 4), _sig(rng.uniform(1.8, 3.4), 3)),
     phrases=("LED series resistor", "LED current-limiting resistor"),
     examples=[_ex({"Vs": 5.0, "Vf": 2.0, "If": 0.02}), _ex({"Vs": 12.0, "Vf": 3.1, "If": 0.015, "n": 3})],
     level="beginner",
     weak="the resistor found by root finding on the current of the netlist (supply, resistor, LED sources)")


# ================================================================================================ BJT: DC bias
def _active(VCE):
    _need(VCE >= 0.2, "the transistor is saturated (VCE below 0.2 V): the active-region bias equations do not apply")


def _beta(beta):
    _pos("beta", beta)


def bjt_currents(IB, beta):
    _pos("the base current IB", IB)
    _beta(beta)
    return _out(IC=beta * IB, IE=(beta + 1.0) * IB, alpha=beta / (beta + 1.0))


def _ref_bjt_currents(IB, beta):
    # a transistor model in a netlist: base fed by a current source, collector held by a supply
    el = [("I", 0, 1, IB), ("V", 1, 0, 0.7), ("F", 2, 0, 1, beta), ("V", 2, 0, 10.0)]
    v, cur = _mna(2, el)
    IC = -cur[3].real
    IE = cur[1].real + IC
    return {"IC": IC, "IE": IE, "alpha": IC / IE}


_add("elx_bjt_current_relations",
     name="BJT terminal currents from beta", topic="BJT / current relations",
     purpose="The collector and emitter currents and the common-base current gain alpha of a bipolar transistor in the active region from its base current and beta.",
     inputs=[_phys("IB", "current", "base current", show=("uA", 1e6)), _numin("beta", "common-emitter current gain (hFE)")],
     outputs=[_o("IC", "current", "collector current"), _o("IE", "current", "emitter current"), _o("alpha", "number", "alpha = beta/(beta + 1)")],
     primary="IC", conditions=["IB > 0, beta > 0", "active region"],
     method="IC = beta IB; IE = (beta + 1) IB; alpha = beta/(beta + 1)",
     latex=r"I_C = \beta I_B,\quad I_E = (\beta+1)I_B,\quad \alpha = \frac{\beta}{\beta+1}",
     run=bjt_currents, reference=_ref_bjt_currents,
     wrong=lambda IB, beta: _out(IC=beta * IB, IE=beta * IB, alpha=1.0),
     rand=lambda rng: {"IB": _sig(_lu(rng, 1e-6, 1e-3), 3), "beta": _sig(rng.uniform(20.0, 400.0), 3)},
     phrases=("BJT currents", "Transistor currents from beta"),
     examples=[_ex({"IB": 20e-6, "beta": 100.0}, {"IC": 0.002, "IE": 0.00202}), _ex({"IB": 50e-6, "beta": 150.0})],
     level="beginner",
     weak="the currents read from a netlist with a transistor model (the model itself is IC = beta IB)")


def _bjt(el, B, C, E, VBE, beta):
    """Reference helper: an active BJT in a netlist (VBE source base -> emitter, collector source beta * IB)."""
    k = len(el)
    el.append(("V", B, E, VBE))
    el.append(("F", C, E, k, beta))
    return k


def fixed_bias(VCC, RB, RC, beta, VBE):
    _pos("VCC, RB, RC and VBE", VCC, RB, RC, VBE)
    _beta(beta)
    _need(VCC > VBE, "VCC must exceed VBE")
    IB = (VCC - VBE) / RB
    IC = beta * IB
    VCE = VCC - IC * RC
    _active(VCE)
    return _out(IB=IB, IC=IC, IE=IB + IC, VCE=VCE)


def _ref_fixed_bias(VCC, RB, RC, beta, VBE):
    el = [("V", 1, 0, VCC), ("R", 1, 2, RB), ("R", 1, 3, RC)]
    k = _bjt(el, 2, 3, 0, VBE, beta)
    v, cur = _mna(3, el)
    IB = cur[k].real
    return {"IB": IB, "IC": beta * IB, "IE": (beta + 1.0) * IB, "VCE": v[3].real}


def _rand_fixed(rng):
    VCC, VBE, beta = _sig(rng.uniform(5.0, 24.0), 3), _sig(rng.uniform(0.6, 0.75), 2), _sig(rng.uniform(50.0, 300.0), 3)
    RB = _sig(_lu(rng, 1e5, 2e6), 3)
    IC = beta * (VCC - VBE) / RB
    return {"VCC": VCC, "RB": RB, "RC": _sig(rng.uniform(0.2, 0.9) * (VCC - 0.3) / IC, 3), "beta": beta, "VBE": VBE}


_add("elx_bjt_fixed_bias",
     name="BJT fixed (base-resistor) bias: Q-point", topic="BJT / DC bias",
     purpose="The quiescent base and collector currents and the collector-emitter voltage of a common-emitter stage biased by a single base resistor from the supply.",
     inputs=[_phys("VCC", "voltage", "supply voltage"), _phys("RB", "resistance", "base resistor"), _phys("RC", "resistance", "collector resistor"),
             _numin("beta", "current gain"), _phys("VBE", "voltage", "base-emitter voltage (about 0.7 V)")],
     outputs=[_o("IB", "current", "base current"), _o("IC", "current", "collector current"), _o("IE", "current", "emitter current"),
              _o("VCE", "voltage", "collector-emitter voltage")],
     primary="IC", conditions=["active region: VCE >= 0.2 V (otherwise refused as saturated)"],
     method="IB = (VCC - VBE)/RB; IC = beta IB; VCE = VCC - IC RC",
     latex=r"I_B = \frac{V_{CC} - V_{BE}}{R_B},\quad V_{CE} = V_{CC} - \beta I_BR_C",
     run=fixed_bias, reference=_ref_fixed_bias,
     wrong=lambda VCC, RB, RC, beta, VBE: (lambda IB: _out(IB=IB, IC=beta * IB, IE=(beta + 1) * IB, VCE=VCC - beta * IB * RC))(VCC / RB),
     rand=_rand_fixed,
     phrases=("BJT fixed bias", "Fixed bias Q-point"),
     examples=[_ex({"VCC": 12.0, "RB": 470e3, "RC": 2200.0, "beta": 100.0, "VBE": 0.7}),
               _ex({"VCC": 9.0, "RB": 820e3, "RC": 2200.0, "beta": 200.0, "VBE": 0.65})],
     level="beginner",
     extra_q=[{"q": "BJT fixed bias: VCC = 12 V, RB = 100 kohm, RC = 4.7 kohm, beta = 100, VBE = 0.7 V", "outcome": "INVALID_INPUT"}])


def divider_bias(VCC, R1, R2, RC, RE, beta, VBE):
    _pos("VCC, R1, R2, RC, RE and VBE", VCC, R1, R2, RC, RE, VBE)
    _beta(beta)
    VTH = VCC * R2 / (R1 + R2)
    RTH = R1 * R2 / (R1 + R2)
    _need(VTH > VBE, "the divider voltage is below VBE: the transistor is cut off")
    IB = (VTH - VBE) / (RTH + (beta + 1.0) * RE)
    IC, IE = beta * IB, (beta + 1.0) * IB
    VE = IE * RE
    VC = VCC - IC * RC
    _active(VC - VE)
    return _out(VTH=VTH, RTH=RTH, IB=IB, IC=IC, IE=IE, VB=VE + VBE, VE=VE, VC=VC, VCE=VC - VE)


def _ref_divider_bias(VCC, R1, R2, RC, RE, beta, VBE):
    el = [("V", 1, 0, VCC), ("R", 1, 2, R1), ("R", 2, 0, R2), ("R", 1, 3, RC), ("R", 4, 0, RE)]
    k = _bjt(el, 2, 3, 4, VBE, beta)
    v, cur = _mna(4, el)
    IB = cur[k].real
    vo, _ = _mna(2, [("V", 1, 0, VCC), ("R", 1, 2, R1), ("R", 2, 0, R2)])            # open-circuit divider
    vr, _ = _mna(1, [("R", 1, 0, R1), ("R", 1, 0, R2), ("I", 0, 1, 1.0)])            # its resistance by a test current
    VB, VC, VE = _re(v)[2], _re(v)[3], _re(v)[4]
    return {"VTH": vo[2].real, "RTH": vr[1].real, "IB": IB, "IC": beta * IB, "IE": (beta + 1.0) * IB, "VB": VB, "VE": VE, "VC": VC,
            "VCE": VC - VE}


def _wrong_divider(VCC, R1, R2, RC, RE, beta, VBE):
    VTH = VCC * R2 / (R1 + R2)
    IE = (VTH - VBE) / RE                                    # the approximate analysis (base current neglected)
    IC = IE * beta / (beta + 1.0)
    return _out(VTH=VTH, RTH=R1 * R2 / (R1 + R2), IB=IE / (beta + 1.0), IC=IC, IE=IE, VB=VTH, VE=IE * RE, VC=VCC - IC * RC,
                VCE=VCC - IC * RC - IE * RE)


def _rand_divider(rng):
    VCC, VBE, beta = _sig(rng.uniform(9.0, 24.0), 3), _sig(rng.uniform(0.6, 0.75), 2), _sig(rng.uniform(50.0, 300.0), 3)
    R2 = _sig(_lu(rng, 2e3, 5e4), 3)
    R1 = _sig(R2 * rng.uniform(2.0, 8.0), 3)
    RE = _sig(_lu(rng, 200.0, 3000.0), 3)
    VTH, RTH = VCC * R2 / (R1 + R2), R1 * R2 / (R1 + R2)
    IB = (VTH - VBE) / (RTH + (beta + 1) * RE)
    IC, VE = beta * IB, (beta + 1) * IB * RE
    RC = _sig(rng.uniform(0.2, 0.85) * (VCC - VE - 0.3) / IC, 3) if VCC - VE > 0.5 and IC > 0 else 1000.0
    return {"VCC": VCC, "R1": R1, "R2": R2, "RC": RC, "RE": RE, "beta": beta, "VBE": VBE}


_add("elx_bjt_voltage_divider_bias",
     name="BJT voltage-divider bias: exact Q-point", topic="BJT / DC bias",
     purpose="The Q-point (base, collector and emitter currents, node voltages and VCE) of a common-emitter stage with a base voltage divider and an emitter resistor, by the exact Thevenin analysis (base current included).",
     inputs=[_phys("VCC", "voltage", "supply voltage"), _phys("R1", "resistance", "upper divider resistor (supply to base)"),
             _phys("R2", "resistance", "lower divider resistor (base to ground)"), _phys("RC", "resistance", "collector resistor"),
             _phys("RE", "resistance", "emitter resistor"), _numin("beta", "current gain"), _phys("VBE", "voltage", "base-emitter voltage")],
     outputs=[_o("VTH", "voltage", "Thevenin voltage of the divider"), _o("RTH", "resistance", "Thevenin resistance of the divider"),
              _o("IB", "current", "base current"), _o("IC", "current", "collector current"), _o("IE", "current", "emitter current"),
              _o("VB", "voltage", "base voltage"), _o("VE", "voltage", "emitter voltage"), _o("VC", "voltage", "collector voltage"),
              _o("VCE", "voltage", "collector-emitter voltage")],
     primary="IC", conditions=["VTH > VBE", "active region: VCE >= 0.2 V"],
     method="VTH = VCC R2/(R1 + R2); RTH = R1 || R2; IB = (VTH - VBE)/(RTH + (beta + 1) RE); VCE = VCC - IC RC - IE RE",
     latex=r"I_B = \frac{V_{TH} - V_{BE}}{R_{TH} + (\beta+1)R_E}",
     run=divider_bias, reference=_ref_divider_bias, wrong=_wrong_divider, rand=_rand_divider,
     phrases=("BJT voltage-divider bias", "Voltage divider bias Q-point"),
     examples=[_ex({"VCC": 12.0, "R1": 47e3, "R2": 10e3, "RC": 2200.0, "RE": 470.0, "beta": 150.0, "VBE": 0.7}),
               _ex({"VCC": 15.0, "R1": 33e3, "R2": 6800.0, "RC": 3300.0, "RE": 1000.0, "beta": 100.0, "VBE": 0.7})],
     level="intermediate")


def emitter_bias(VCC, VEE, RB, RC, RE, beta, VBE):
    _pos("VCC, VEE, RB, RC, RE and VBE", VCC, VEE, RB, RC, RE, VBE)
    _beta(beta)
    _need(VEE > VBE, "VEE must exceed VBE")
    IE = (VEE - VBE) / (RE + RB / (beta + 1.0))
    IB = IE / (beta + 1.0)
    IC = beta * IB
    VB = -IB * RB
    VE = VB - VBE
    VC = VCC - IC * RC
    _active(VC - VE)
    return _out(IB=IB, IC=IC, IE=IE, VB=VB, VE=VE, VC=VC, VCE=VC - VE)


def _ref_emitter_bias(VCC, VEE, RB, RC, RE, beta, VBE):
    el = [("V", 1, 0, VCC), ("V", 5, 0, -VEE), ("R", 2, 0, RB), ("R", 1, 3, RC), ("R", 4, 5, RE)]
    k = _bjt(el, 2, 3, 4, VBE, beta)
    v, cur = _mna(5, el)
    IB = cur[k].real
    vv = _re(v)
    return {"IB": IB, "IC": beta * IB, "IE": (beta + 1.0) * IB, "VB": vv[2], "VE": vv[4], "VC": vv[3], "VCE": vv[3] - vv[4]}


def _rand_emitter(rng):
    VCC, VEE, VBE, beta = _sig(rng.uniform(6.0, 15.0), 3), _sig(rng.uniform(5.0, 15.0), 3), _sig(rng.uniform(0.6, 0.75), 2), _sig(rng.uniform(50.0, 300.0), 3)
    RE, RB = _sig(_lu(rng, 1e3, 2e4), 3), _sig(_lu(rng, 1e3, 1e5), 3)
    IE = (VEE - VBE) / (RE + RB / (beta + 1))
    IC = IE * beta / (beta + 1)
    VE = -IE / (beta + 1) * RB - VBE
    return {"VCC": VCC, "VEE": VEE, "RB": RB, "RC": _sig(rng.uniform(0.2, 0.85) * (VCC - VE - 0.3) / IC, 3), "RE": RE, "beta": beta, "VBE": VBE}


_add("elx_bjt_emitter_bias",
     name="BJT dual-supply emitter bias: Q-point", topic="BJT / DC bias",
     purpose="The Q-point of a transistor with its base returned to ground through RB and its emitter returned to a negative supply -VEE through RE (emitter bias).",
     inputs=[_phys("VCC", "voltage", "positive supply"), _phys("VEE", "voltage", "magnitude of the negative supply"),
             _phys("RB", "resistance", "base resistor to ground"), _phys("RC", "resistance", "collector resistor"),
             _phys("RE", "resistance", "emitter resistor to -VEE"), _numin("beta", "current gain"), _phys("VBE", "voltage", "base-emitter voltage")],
     outputs=[_o("IB", "current", "base current"), _o("IC", "current", "collector current"), _o("IE", "current", "emitter current"),
              _o("VB", "voltage", "base voltage"), _o("VE", "voltage", "emitter voltage"), _o("VC", "voltage", "collector voltage"),
              _o("VCE", "voltage", "collector-emitter voltage")],
     primary="IE", conditions=["VEE > VBE", "active region: VCE >= 0.2 V"],
     method="IE = (VEE - VBE)/(RE + RB/(beta + 1)); VB = -IB RB; VE = VB - VBE; VC = VCC - IC RC",
     latex=r"I_E = \frac{V_{EE} - V_{BE}}{R_E + R_B/(\beta+1)}",
     run=emitter_bias, reference=_ref_emitter_bias,
     wrong=lambda VCC, VEE, RB, RC, RE, beta, VBE: (lambda IE: _out(IB=IE / (beta + 1), IC=IE * beta / (beta + 1), IE=IE, VB=0.0, VE=-VBE,
                                                                   VC=VCC - IE * beta / (beta + 1) * RC, VCE=VCC - IE * beta / (beta + 1) * RC + VBE))((VEE - VBE) / RE),
     rand=_rand_emitter,
     phrases=("BJT emitter bias", "Dual-supply emitter bias"),
     examples=[_ex({"VCC": 10.0, "VEE": 10.0, "RB": 10e3, "RC": 4700.0, "RE": 10e3, "beta": 100.0, "VBE": 0.7}),
               _ex({"VCC": 12.0, "VEE": 9.0, "RB": 47e3, "RC": 5600.0, "RE": 8200.0, "beta": 200.0, "VBE": 0.65})],
     level="intermediate")


def collector_feedback(VCC, RC, RF, beta, VBE, RE=0.0):
    _pos("VCC, RC, RF and VBE", VCC, RC, RF, VBE)
    _nonneg("RE", RE)
    _beta(beta)
    _need(VCC > VBE, "VCC must exceed VBE")
    IB = (VCC - VBE) / (RF + (beta + 1.0) * (RC + RE))
    IE = (beta + 1.0) * IB
    VC = VCC - IE * RC
    VE = IE * RE
    _active(VC - VE)
    return _out(IB=IB, IC=beta * IB, IE=IE, VC=VC, VCE=VC - VE)


def _ref_collector_feedback(VCC, RC, RF, beta, VBE, RE=0.0):
    el = [("V", 1, 0, VCC), ("R", 1, 3, RC), ("R", 3, 2, RF)]
    if RE > 0:
        el.append(("R", 4, 0, RE))
        k = _bjt(el, 2, 3, 4, VBE, beta)
        v, cur = _mna(4, el)
    else:
        k = _bjt(el, 2, 3, 0, VBE, beta)
        v, cur = _mna(3, el)
    IB = cur[k].real
    vv = _re(v)
    VE = vv[4] if RE > 0 else 0.0
    return {"IB": IB, "IC": beta * IB, "IE": (beta + 1.0) * IB, "VC": vv[3], "VCE": vv[3] - VE}


def _rand_cfb(rng):
    VCC, VBE, beta = _sig(rng.uniform(5.0, 20.0), 3), _sig(rng.uniform(0.6, 0.75), 2), _sig(rng.uniform(30.0, 300.0), 3)
    RC = _sig(_lu(rng, 1e3, 1e4), 3)
    d = {"VCC": VCC, "RC": RC, "RF": _sig(RC * beta * rng.uniform(0.3, 2.0), 3), "beta": beta, "VBE": VBE}
    if rng.random() < 0.5:
        d["RE"] = _sig(RC * rng.uniform(0.05, 0.3), 3)
    return d


_add("elx_bjt_collector_feedback_bias",
     name="BJT collector-feedback bias: Q-point", topic="BJT / DC bias",
     purpose="The Q-point of a transistor biased by a resistor from collector to base (collector feedback), with an optional emitter resistor.",
     inputs=[_phys("VCC", "voltage", "supply voltage"), _phys("RC", "resistance", "collector resistor (carries IC + IB)"),
             _phys("RF", "resistance", "feedback resistor, collector to base"), _numin("beta", "current gain"),
             _phys("VBE", "voltage", "base-emitter voltage"), _phys("RE", "resistance", "emitter resistor (default 0)", required=False)],
     outputs=[_o("IB", "current", "base current"), _o("IC", "current", "collector current"), _o("IE", "current", "emitter current"),
              _o("VC", "voltage", "collector voltage"), _o("VCE", "voltage", "collector-emitter voltage")],
     primary="IC", conditions=["active region: VCE >= 0.2 V"],
     method="IB = (VCC - VBE)/(RF + (beta + 1)(RC + RE)); VC = VCC - IE RC",
     latex=r"I_B = \frac{V_{CC} - V_{BE}}{R_F + (\beta+1)(R_C + R_E)}",
     run=collector_feedback, reference=_ref_collector_feedback,
     wrong=lambda VCC, RC, RF, beta, VBE, RE=0.0: (lambda IB: _out(IB=IB, IC=beta * IB, IE=(beta + 1) * IB, VC=VCC - beta * IB * RC,
                                                                   VCE=VCC - beta * IB * RC - (beta + 1) * IB * RE))((VCC - VBE) / (RF + beta * (RC + RE))),
     rand=_rand_cfb,
     phrases=("BJT collector-feedback bias", "Collector feedback bias Q-point"),
     examples=[_ex({"VCC": 10.0, "RC": 4700.0, "RF": 250e3, "beta": 90.0, "VBE": 0.7}),
               _ex({"VCC": 12.0, "RC": 3300.0, "RF": 330e3, "beta": 120.0, "VBE": 0.7, "RE": 470.0})],
     level="intermediate")


# ================================================================================================ BJT: small signal
def bjt_small_signal(IC, beta, VT, VA):
    _pos("IC, beta, VT and VA", IC, beta, VT, VA)
    gm = IC / VT
    return _out(gm=gm, r_pi=beta / gm, r_e=VT * beta / ((beta + 1.0) * IC), r_o=VA / IC, A0=VA / VT)


_SS = {}


def _ref_bjt_small_signal(IC, beta, VT, VA):
    # derivatives of the large-signal model IC(VBE, VCE) = I0 exp((VBE - V0)/VT) (1 + VCE/VA), at VBE = V0, VCE = 0
    if "f" not in _SS:
        import sympy as sp
        i0, vbe, vce, v0, vt, va, b = sp.symbols("i0 vbe vce v0 vt va b", positive=True)
        ic = i0 * sp.exp((vbe - v0) / vt) * (1 + vce / va)
        ib, ie = ic / b, ic * (b + 1) / b
        at = {vbe: v0, vce: 0}
        _SS["f"] = sp.lambdify((i0, v0, vt, va, b), [sp.diff(ic, vbe).subs(at), 1 / sp.diff(ib, vbe).subs(at),
                                                      1 / sp.diff(ie, vbe).subs(at), 1 / sp.diff(ic, vce).subs(at)])
    gm, rpi, re_, ro = [float(x) for x in _SS["f"](IC, 0.7, VT, VA, beta)]
    return {"gm": gm, "r_pi": rpi, "r_e": re_, "r_o": ro, "A0": gm * ro}


_add("elx_bjt_small_signal_parameters",
     name="BJT small-signal (hybrid-pi) parameters", topic="BJT / small-signal model",
     purpose="The transconductance gm, the input resistance r_pi, the emitter resistance r_e, the output resistance r_o (Early voltage) and the intrinsic gain of a bipolar transistor at its bias current.",
     inputs=[_phys("IC", "current", "collector bias current", show=("mA", 1e3)), _numin("beta", "current gain"),
             _phys("VT", "voltage", "thermal voltage (25 or 26 mV)", show=("mV", 1e3)), _phys("VA", "voltage", "Early voltage")],
     outputs=[_o("gm", "conductance", "transconductance"), _o("r_pi", "resistance", "base input resistance r_pi"),
              _o("r_e", "resistance", "emitter resistance r_e"), _o("r_o", "resistance", "output resistance"),
              _o("A0", "number", "intrinsic voltage gain gm r_o")],
     primary="gm", conditions=["IC, beta, VT, VA > 0", "active region"],
     method="gm = IC/VT; r_pi = beta/gm; r_e = alpha/gm; r_o = VA/IC; A0 = VA/VT",
     latex=r"g_m = \frac{I_C}{V_T},\quad r_\pi = \frac{\beta}{g_m},\quad r_e = \frac{\alpha}{g_m},\quad r_o = \frac{V_A}{I_C}",
     run=bjt_small_signal, reference=_ref_bjt_small_signal,
     wrong=lambda IC, beta, VT, VA: (lambda gm: _out(gm=gm, r_pi=(beta + 1.0) / gm, r_e=1.0 / gm, r_o=VA / IC, A0=VA / VT))(IC / VT),
     rand=lambda rng: {"IC": _sig(_lu(rng, 1e-5, 1e-2), 3), "beta": _sig(rng.uniform(30.0, 400.0), 3), "VT": rng.choice([0.025, 0.026, 0.02585]),
                       "VA": _sig(rng.uniform(20.0, 200.0), 3)},
     phrases=("BJT small-signal parameters", "Hybrid-pi parameters"),
     examples=[_ex({"IC": 1e-3, "beta": 100.0, "VT": 0.025, "VA": 100.0}, {"gm": 0.04, "r_pi": 2500.0, "r_o": 100000.0, "A0": 4000.0}),
               _ex({"IC": 2.5e-3, "beta": 150.0, "VT": 0.026, "VA": 75.0})],
     level="intermediate")


def _hybrid(IC, beta, VT):
    _pos("IC, beta and VT", IC, beta, VT)
    gm = IC / VT
    return gm, beta / gm


def ce_amplifier(IC, beta, VT, RC, RL, RE1, RB, Rsig):
    _pos("RC, RL, RB and Rsig", RC, RL, RB, Rsig)
    _nonneg("the unbypassed emitter resistance RE1", RE1)
    gm, rpi = _hybrid(IC, beta, VT)
    Rl = _par(RC, RL)
    rin_b = rpi + (beta + 1.0) * RE1
    Av = -beta * Rl / rin_b
    Rin = _par(RB, rin_b)
    return _out(gm=gm, r_pi=rpi, Av=Av, Rin=Rin, Rout=RC, Gv=Av * Rin / (Rin + Rsig))


def _ref_ce_amplifier(IC, beta, VT, RC, RL, RE1, RB, Rsig):
    gm, rpi = IC / VT, beta * VT / IC
    E = 4 if RE1 > 0 else 0
    el = [("V", 1, 0, 1.0), ("R", 1, 2, Rsig), ("R", 2, 0, RB), ("R", 2, E, rpi), ("Gm", 3, E, 2, E, gm), ("R", 3, 0, RC), ("R", 3, 0, RL)]
    if RE1 > 0:
        el.append(("R", 4, 0, RE1))
    v, _ = _mna(4 if RE1 > 0 else 3, el)
    vb, vo = v[2].real, v[3].real
    Rin = vb / ((1.0 - vb) / Rsig)
    t = [("R", 1, 0, 1.0), ("R", 2, 0, Rsig), ("R", 2, 0, RB), ("R", 2, E, rpi), ("Gm", 3, E, 2, E, gm), ("R", 3, 0, RC), ("I", 0, 3, 1.0)]
    if RE1 > 0:
        t.append(("R", 4, 0, RE1))
    vt, _ = _mna(4 if RE1 > 0 else 3, t)
    return {"gm": gm, "r_pi": rpi, "Av": vo / vb, "Rin": Rin, "Rout": vt[3].real, "Gv": vo}


def _rand_amp(rng):
    return {"IC": _sig(_lu(rng, 1e-4, 1e-2), 3), "beta": _sig(rng.uniform(50.0, 300.0), 3), "VT": rng.choice([0.025, 0.026]),
            "RC": _sig(_lu(rng, 1e3, 1e4), 3), "RL": _sig(_lu(rng, 1e3, 1e5), 3), "RE1": _sig(_lu(rng, 20.0, 1000.0), 3) if rng.random() < 0.85 else 0.0,
            "RB": _sig(_lu(rng, 5e3, 2e5), 3), "Rsig": _sig(_lu(rng, 50.0, 1e4), 3)}


_add("elx_common_emitter_amplifier",
     name="Common-emitter amplifier: gain, input and output resistance", topic="BJT / amplifiers",
     purpose="The voltage gain (base to collector and source to load), the input and output resistances of a common-emitter stage with an optional unbypassed emitter resistance, from the bias current (hybrid-pi model, r_o neglected).",
     inputs=[_phys("IC", "current", "collector bias current", show=("mA", 1e3)), _numin("beta", "current gain"),
             _phys("VT", "voltage", "thermal voltage", show=("mV", 1e3)), _phys("RC", "resistance", "collector resistor"),
             _phys("RL", "resistance", "load resistor"), _phys("RE1", "resistance", "unbypassed emitter resistance (0 if fully bypassed)"),
             _phys("RB", "resistance", "bias resistance seen at the base (R1 || R2)"), _phys("Rsig", "resistance", "source resistance")],
     outputs=[_o("gm", "conductance", "transconductance"), _o("r_pi", "resistance", "r_pi"), _o("Av", "number", "voltage gain base to output"),
              _o("Rin", "resistance", "input resistance (including RB)"), _o("Rout", "resistance", "output resistance (RC)"),
              _o("Gv", "number", "overall gain from the source voltage")],
     primary="Av", conditions=["r_o neglected", "coupling and bypass capacitors short at signal frequency"],
     method="Av = -beta (RC || RL)/(r_pi + (beta + 1) RE1); Rin = RB || (r_pi + (beta + 1) RE1); Gv = Av Rin/(Rin + Rsig)",
     latex=r"A_v = -\frac{\beta\,(R_C\parallel R_L)}{r_\pi + (\beta+1)R_{E1}}",
     run=ce_amplifier, reference=_ref_ce_amplifier,
     wrong=lambda IC, beta, VT, RC, RL, RE1, RB, Rsig: (lambda r: dict(r, Av=_r(-IC / VT * _par(RC, RL))))(ce_amplifier(IC, beta, VT, RC, RL, RE1, RB, Rsig)),
     rand=_rand_amp,
     phrases=("Common-emitter amplifier", "CE amplifier"),
     examples=[_ex({"IC": 1e-3, "beta": 100.0, "VT": 0.025, "RC": 5000.0, "RL": 10e3, "RE1": 0.0, "RB": 20e3, "Rsig": 1000.0}),
               _ex({"IC": 2e-3, "beta": 150.0, "VT": 0.025, "RC": 3300.0, "RL": 4700.0, "RE1": 220.0, "RB": 30e3, "Rsig": 600.0})],
     level="advanced")


def cc_amplifier(IC, beta, VT, RE, RL, RB, Rsig):
    _pos("RE, RL, RB and Rsig", RE, RL, RB, Rsig)
    gm, rpi = _hybrid(IC, beta, VT)
    Re = _par(RE, RL)
    rin_b = rpi + (beta + 1.0) * Re
    Av = (beta + 1.0) * Re / rin_b
    Rin = _par(RB, rin_b)
    Rout = _par(RE, (rpi + _par(Rsig, RB)) / (beta + 1.0))
    return _out(Av=Av, Rin=Rin, Rout=Rout, Gv=Av * Rin / (Rin + Rsig))


def _ref_cc_amplifier(IC, beta, VT, RE, RL, RB, Rsig):
    gm, rpi = IC / VT, beta * VT / IC
    el = [("V", 1, 0, 1.0), ("R", 1, 2, Rsig), ("R", 2, 0, RB), ("R", 2, 3, rpi), ("Gm", 0, 3, 2, 3, gm), ("R", 3, 0, RE), ("R", 3, 0, RL)]
    v, _ = _mna(3, el)
    vb, vo = v[2].real, v[3].real
    t = [("R", 1, 0, 1.0), ("R", 2, 0, Rsig), ("R", 2, 0, RB), ("R", 2, 3, rpi), ("Gm", 0, 3, 2, 3, gm), ("R", 3, 0, RE), ("I", 0, 3, 1.0)]
    vt, _ = _mna(3, t)
    Rin = vb / ((1.0 - vb) / Rsig)
    return {"Av": vo / vb, "Rin": Rin, "Rout": vt[3].real, "Gv": vo}


_add("elx_common_collector_amplifier",
     name="Common-collector (emitter-follower) amplifier", topic="BJT / amplifiers",
     purpose="The voltage gain, input resistance and output resistance of an emitter follower, including the source resistance reflected into the emitter (hybrid-pi model, r_o neglected).",
     inputs=[_phys("IC", "current", "collector bias current", show=("mA", 1e3)), _numin("beta", "current gain"),
             _phys("VT", "voltage", "thermal voltage", show=("mV", 1e3)), _phys("RE", "resistance", "emitter resistor"),
             _phys("RL", "resistance", "load resistor"), _phys("RB", "resistance", "bias resistance at the base"),
             _phys("Rsig", "resistance", "source resistance")],
     outputs=[_o("Av", "number", "voltage gain base to output"), _o("Rin", "resistance", "input resistance"),
              _o("Rout", "resistance", "output resistance"), _o("Gv", "number", "overall gain from the source")],
     primary="Av", conditions=["r_o neglected"],
     method="Av = (beta + 1)(RE || RL)/(r_pi + (beta + 1)(RE || RL)); Rout = RE || (r_pi + Rsig || RB)/(beta + 1)",
     latex=r"A_v = \frac{(\beta+1)R_E'}{r_\pi + (\beta+1)R_E'},\quad R_{out} = R_E\parallel\frac{r_\pi + R_{sig}\parallel R_B}{\beta+1}",
     run=cc_amplifier, reference=_ref_cc_amplifier,
     wrong=lambda IC, beta, VT, RE, RL, RB, Rsig: dict(cc_amplifier(IC, beta, VT, RE, RL, RB, Rsig), Rout=_r(_par(RE, beta * VT / IC / (beta + 1.0)))),
     rand=lambda rng: {"IC": _sig(_lu(rng, 1e-4, 1e-2), 3), "beta": _sig(rng.uniform(50.0, 300.0), 3), "VT": rng.choice([0.025, 0.026]),
                       "RE": _sig(_lu(rng, 300.0, 1e4), 3), "RL": _sig(_lu(rng, 100.0, 1e4), 3), "RB": _sig(_lu(rng, 1e4, 3e5), 3),
                       "Rsig": _sig(_lu(rng, 500.0, 5e4), 3)},
     phrases=("Common-collector amplifier", "Emitter follower"),
     examples=[_ex({"IC": 5e-3, "beta": 100.0, "VT": 0.025, "RE": 1000.0, "RL": 1000.0, "RB": 100e3, "Rsig": 10e3}),
               _ex({"IC": 1e-3, "beta": 200.0, "VT": 0.026, "RE": 4700.0, "RL": 2000.0, "RB": 47e3, "Rsig": 5000.0})],
     level="advanced")


def cb_amplifier(IC, beta, VT, RC, RL, RE, Rsig):
    _pos("RC, RL, RE and Rsig", RC, RL, RE, Rsig)
    gm, rpi = _hybrid(IC, beta, VT)
    re_ = beta / ((beta + 1.0) * gm)
    Rin = _par(RE, re_)
    Av = gm * _par(RC, RL)
    return _out(Av=Av, Rin=Rin, Rout=RC, Gv=Av * Rin / (Rin + Rsig))


def _ref_cb_amplifier(IC, beta, VT, RC, RL, RE, Rsig):
    gm, rpi = IC / VT, beta * VT / IC
    el = [("V", 1, 0, 1.0), ("R", 1, 2, Rsig), ("R", 2, 0, RE), ("R", 2, 0, rpi), ("Gm", 3, 2, 0, 2, gm), ("R", 3, 0, RC), ("R", 3, 0, RL)]
    v, _ = _mna(3, el)
    ve, vo = v[2].real, v[3].real
    t = [("R", 1, 0, 1.0), ("R", 2, 0, Rsig), ("R", 2, 0, RE), ("R", 2, 0, rpi), ("Gm", 3, 2, 0, 2, gm), ("R", 3, 0, RC), ("I", 0, 3, 1.0)]
    vt, _ = _mna(3, t)
    return {"Av": vo / ve, "Rin": ve / ((1.0 - ve) / Rsig), "Rout": vt[3].real, "Gv": vo}


_add("elx_common_base_amplifier",
     name="Common-base amplifier", topic="BJT / amplifiers",
     purpose="The voltage gain (emitter to collector and source to load), the low input resistance and the output resistance of a common-base stage (hybrid-pi model, r_o neglected).",
     inputs=[_phys("IC", "current", "collector bias current", show=("mA", 1e3)), _numin("beta", "current gain"),
             _phys("VT", "voltage", "thermal voltage", show=("mV", 1e3)), _phys("RC", "resistance", "collector resistor"),
             _phys("RL", "resistance", "load resistor"), _phys("RE", "resistance", "emitter bias resistor"),
             _phys("Rsig", "resistance", "source resistance")],
     outputs=[_o("Av", "number", "voltage gain emitter to output"), _o("Rin", "resistance", "input resistance (RE || r_e)"),
              _o("Rout", "resistance", "output resistance (RC)"), _o("Gv", "number", "overall gain from the source")],
     primary="Av", conditions=["r_o neglected", "base at signal ground"],
     method="Av = gm (RC || RL); Rin = RE || r_e with r_e = alpha/gm; Gv = Av Rin/(Rin + Rsig)",
     latex=r"A_v = g_m(R_C\parallel R_L),\quad R_{in} = R_E\parallel r_e",
     run=cb_amplifier, reference=_ref_cb_amplifier,
     wrong=lambda IC, beta, VT, RC, RL, RE, Rsig: (lambda Rin: (lambda Av: _out(Av=Av, Rin=Rin, Rout=RC, Gv=Av * Rin / (Rin + Rsig)))(IC / VT * _par(RC, RL)))(_par(RE, VT / IC)),
     rand=lambda rng: {"IC": _sig(_lu(rng, 1e-4, 1e-2), 3), "beta": _sig(rng.uniform(30.0, 300.0), 3), "VT": rng.choice([0.025, 0.026]),
                       "RC": _sig(_lu(rng, 1e3, 1e4), 3), "RL": _sig(_lu(rng, 1e3, 1e5), 3), "RE": _sig(_lu(rng, 1e3, 2e4), 3),
                       "Rsig": _sig(_lu(rng, 10.0, 1000.0), 3)},
     phrases=("Common-base amplifier", "CB amplifier"),
     examples=[_ex({"IC": 1e-3, "beta": 100.0, "VT": 0.025, "RC": 5000.0, "RL": 5000.0, "RE": 10e3, "Rsig": 50.0}),
               _ex({"IC": 0.5e-3, "beta": 120.0, "VT": 0.026, "RC": 10e3, "RL": 20e3, "RE": 4700.0, "Rsig": 100.0})],
     level="advanced")


def transition_frequency(gm, beta, Cpi, Cmu):
    _pos("gm, beta, Cpi and Cmu", gm, beta, Cpi, Cmu)
    rpi = beta / gm
    fb = 1.0 / (TWO_PI * rpi * (Cpi + Cmu))
    return _out(r_pi=rpi, f_beta=fb, f_T=beta * fb)


def _ref_transition_frequency(gm, beta, Cpi, Cmu):
    rpi = beta / gm
    # short-circuit current gain h_fe(jw) = beta/(1 + jw r_pi (Cpi + Cmu)): its -3 dB frequency, then fT = beta f_beta
    h = lambda f: abs(beta / complex(1.0, TWO_PI * f * rpi * (Cpi + Cmu)))
    fb = _brentq(lambda f: h(f) - beta / math.sqrt(2.0), 0.0, 1e15)
    return {"r_pi": rpi, "f_beta": fb, "f_T": beta * fb}


_add("elx_transition_frequency",
     name="BJT beta cutoff and transition frequency fT", topic="BJT / frequency response",
     purpose="The beta cutoff frequency f_beta and the unity-current-gain (transition) frequency fT = gm/(2 pi (Cpi + Cmu)) of a bipolar transistor from its hybrid-pi capacitances.",
     inputs=[_phys("gm", "conductance", "transconductance"), _numin("beta", "low-frequency current gain"),
             _phys("Cpi", "capacitance", "base-emitter capacitance", show=("pF", 1e12)), _phys("Cmu", "capacitance", "base-collector capacitance", show=("pF", 1e12))],
     outputs=[_o("r_pi", "resistance", "r_pi = beta/gm"), _o("f_beta", "frequency", "beta cutoff frequency"),
              _o("f_T", "frequency", "transition frequency")],
     primary="f_T", conditions=["single-pole h_fe (the feed-forward zero through Cmu neglected)"],
     method="f_beta = 1/(2 pi r_pi (Cpi + Cmu)); fT = beta f_beta = gm/(2 pi (Cpi + Cmu))",
     latex=r"f_T = \frac{g_m}{2\pi(C_\pi + C_\mu)},\quad f_\beta = \frac{f_T}{\beta}",
     run=transition_frequency, reference=_ref_transition_frequency,
     wrong=lambda gm, beta, Cpi, Cmu: transition_frequency(gm, beta, Cpi, Cmu * 1e-12),
     rand=lambda rng: {"gm": _sig(_lu(rng, 1e-3, 0.4), 3), "beta": _sig(rng.uniform(50.0, 300.0), 3), "Cpi": _sig(_lu(rng, 1e-12, 50e-12), 3),
                       "Cmu": _sig(_lu(rng, 0.3e-12, 5e-12), 3)},
     phrases=("Transition frequency", "BJT transition frequency"),
     examples=[_ex({"gm": 0.04, "beta": 100.0, "Cpi": 10e-12, "Cmu": 2e-12}), _ex({"gm": 0.1, "beta": 150.0, "Cpi": 20e-12, "Cmu": 1e-12})],
     level="advanced",
     weak="the -3 dB point of the single-pole current gain found by root finding (the same model)")


def miller_octc(gm, r_pi, Cpi, Cmu, RLp, Rsigp):
    _pos("gm, r_pi, Cpi, Cmu, RLp and Rsigp", gm, r_pi, Cpi, Cmu, RLp, Rsigp)
    A = -gm * RLp
    CM = Cmu * (1.0 - A)
    Rp = _par(r_pi, Rsigp)
    fm = 1.0 / (TWO_PI * Rp * (Cpi + CM))
    tau = Cpi * Rp + Cmu * (Rp + RLp + gm * Rp * RLp)
    return _out(A_mid=A, C_M=CM, R_pi_eq=Rp, f_H_miller=fm, tau_oc=tau, f_H_octc=1.0 / (TWO_PI * tau))


def _ref_miller_octc(gm, r_pi, Cpi, Cmu, RLp, Rsigp):
    net = [("R", 1, 0, Rsigp), ("R", 1, 0, r_pi), ("Gm", 2, 0, 1, 0, gm), ("R", 2, 0, RLp)]
    v1, _ = _mna(2, net + [("I", 0, 1, 1.0)])                       # resistance seen by Cpi (Cmu open)
    R1 = v1[1].real
    v2, _ = _mna(2, net + [("I", 2, 1, 1.0)])                       # resistance seen by Cmu (between nodes 1 and 2)
    R2 = (v2[1] - v2[2]).real
    va, _ = _mna(2, [("V", 1, 0, 1.0), ("Gm", 2, 0, 1, 0, gm), ("R", 2, 0, RLp)])   # gain across Cmu
    A = va[2].real
    CM = Cmu * (1.0 - A)
    tau = Cpi * R1 + Cmu * R2
    return {"A_mid": A, "C_M": CM, "R_pi_eq": R1, "f_H_miller": 1.0 / (TWO_PI * R1 * (Cpi + CM)), "tau_oc": tau, "f_H_octc": 1.0 / (TWO_PI * tau)}


_add("elx_miller_high_cutoff",
     name="Common-emitter high-frequency cutoff: Miller effect and open-circuit time constants", topic="BJT / frequency response",
     purpose="The Miller capacitance, the upper -3 dB frequency of a common-emitter (or common-source) stage by the Miller approximation, and the open-circuit time-constant estimate that also counts the output side of Cmu.",
     inputs=[_phys("gm", "conductance", "transconductance"), _phys("r_pi", "resistance", "r_pi (use a very large value for a MOSFET)"),
             _phys("Cpi", "capacitance", "input capacitance Cpi (Cgs)", show=("pF", 1e12)),
             _phys("Cmu", "capacitance", "feedback capacitance Cmu (Cgd)", show=("pF", 1e12)),
             _phys("RLp", "resistance", "total load at the collector (RC || RL || r_o)"),
             _phys("Rsigp", "resistance", "source resistance seen at the base (Rsig || RB)")],
     outputs=[_o("A_mid", "number", "gain across Cmu, -gm RL'"), _o("C_M", "capacitance", "Miller capacitance"),
              _o("R_pi_eq", "resistance", "resistance seen by Cpi (r_pi || Rsig')"), _o("f_H_miller", "frequency", "upper cutoff, Miller approximation"),
              _o("tau_oc", "time", "sum of open-circuit time constants"), _o("f_H_octc", "frequency", "upper cutoff, open-circuit time constants")],
     primary="f_H_octc", conditions=["mid-band gain unchanged by the capacitors", "dominant-pole estimate"],
     method="C_M = Cmu (1 + gm RL'); f_H = 1/(2 pi (r_pi || Rsig')(Cpi + C_M)); tau = Cpi R' + Cmu (R' + RL' + gm R' RL')",
     latex=r"C_M = C_\mu(1 + g_mR_L'),\quad \tau_{oc} = C_\pi R_\pi' + C_\mu\left(R_\pi' + R_L' + g_mR_\pi'R_L'\right)",
     run=miller_octc, reference=_ref_miller_octc,
     wrong=lambda gm, r_pi, Cpi, Cmu, RLp, Rsigp: (lambda r: dict(r, C_M=_r(Cmu * gm * RLp), f_H_miller=_r(1.0 / (TWO_PI * _par(r_pi, Rsigp) * (Cpi + Cmu * gm * RLp)))))(
         miller_octc(gm, r_pi, Cpi, Cmu, RLp, Rsigp)),
     rand=lambda rng: (lambda gm: {"gm": gm, "r_pi": _sig(rng.uniform(50.0, 300.0) / gm, 3), "Cpi": _sig(_lu(rng, 2e-12, 40e-12), 3),
                                   "Cmu": _sig(_lu(rng, 0.3e-12, 5e-12), 3), "RLp": _sig(_lu(rng, 100.0, 2e4), 3) if True else 0.0,
                                   "Rsigp": _sig(_lu(rng, 50.0, 2e4), 3)})(_sig(_lu(rng, 1e-3, 0.2), 3)),
     phrases=("Miller high-frequency cutoff", "Common-emitter high cutoff"),
     examples=[_ex({"gm": 0.04, "r_pi": 2500.0, "Cpi": 10e-12, "Cmu": 2e-12, "RLp": 2500.0, "Rsigp": 1000.0}),
               _ex({"gm": 0.1, "r_pi": 1500.0, "Cpi": 20e-12, "Cmu": 1e-12, "RLp": 1000.0, "Rsigp": 500.0})],
     level="expert")


def sctc_low_cutoff(Rsig, RB, r_pi, beta, RE, RC, RL, C1, C2, CE):
    _pos("Rsig, RB, r_pi, beta, RE, RC, RL, C1, C2 and CE", Rsig, RB, r_pi, beta, RE, RC, RL, C1, C2, CE)
    R1 = Rsig + _par(RB, r_pi)
    RCE = _par(RE, (r_pi + _par(Rsig, RB)) / (beta + 1.0))
    R2 = RC + RL
    f1, fe, f2 = 1.0 / (TWO_PI * R1 * C1), 1.0 / (TWO_PI * RCE * CE), 1.0 / (TWO_PI * R2 * C2)
    return _out(R_C1=R1, R_CE=RCE, R_C2=R2, f_C1=f1, f_CE=fe, f_C2=f2, f_L=f1 + fe + f2)


def _ref_sctc_low_cutoff(Rsig, RB, r_pi, beta, RE, RC, RL, C1, C2, CE):
    gm = beta / r_pi
    # nodes: 1 source side of C1, 2 base, 3 emitter, 4 collector, 5 load side of C2; each capacitor sees a test source
    # with the other two capacitors shorted (their two nodes merged)
    def resistance(which):
        merge = {}
        if which != "C1":
            merge[1] = 2
        if which != "CE":
            merge[3] = 0
        if which != "C2":
            merge[5] = 4
        used = sorted(set(merge.get(x, x) for x in (1, 2, 3, 4, 5)) - {0})
        num = {old: i + 1 for i, old in enumerate(used)}
        m = lambda x: 0 if merge.get(x, x) == 0 else num[merge.get(x, x)]
        el = [("R", m(1), 0, Rsig), ("R", m(2), 0, RB), ("R", m(2), m(3), r_pi), ("Gm", m(4), m(3), m(2), m(3), gm), ("R", m(3), 0, RE),
              ("R", m(4), 0, RC), ("R", m(5), 0, RL)]
        a, b = {"C1": (1, 2), "CE": (3, 0), "C2": (4, 5)}[which]
        el.append(("I", m(b) if b else 0, m(a), 1.0))
        v, _ = _mna(len(used), el)
        return (v[m(a)] - (v[m(b)] if b else 0.0)).real
    R1, RCE, R2 = resistance("C1"), resistance("CE"), resistance("C2")
    f1, fe, f2 = 1.0 / (TWO_PI * R1 * C1), 1.0 / (TWO_PI * RCE * CE), 1.0 / (TWO_PI * R2 * C2)
    return {"R_C1": R1, "R_CE": RCE, "R_C2": R2, "f_C1": f1, "f_CE": fe, "f_C2": f2, "f_L": f1 + fe + f2}


_add("elx_low_frequency_cutoff",
     name="Common-emitter low-frequency cutoff by short-circuit time constants", topic="BJT / frequency response",
     purpose="The resistance seen by each coupling and bypass capacitor of a common-emitter amplifier and the lower -3 dB frequency estimated as the sum of the short-circuit pole frequencies (r_o neglected).",
     inputs=[_phys("Rsig", "resistance", "source resistance"), _phys("RB", "resistance", "bias resistance at the base (R1 || R2)"),
             _phys("r_pi", "resistance", "r_pi"), _numin("beta", "current gain"), _phys("RE", "resistance", "emitter resistor (bypassed by CE)"),
             _phys("RC", "resistance", "collector resistor"), _phys("RL", "resistance", "load resistor"),
             _phys("C1", "capacitance", "input coupling capacitor", show=("uF", 1e6)), _phys("C2", "capacitance", "output coupling capacitor", show=("uF", 1e6)),
             _phys("CE", "capacitance", "emitter bypass capacitor", show=("uF", 1e6))],
     outputs=[_o("R_C1", "resistance", "resistance seen by C1"), _o("R_CE", "resistance", "resistance seen by CE"),
              _o("R_C2", "resistance", "resistance seen by C2"), _o("f_C1", "frequency", "pole of C1"), _o("f_CE", "frequency", "pole of CE"),
              _o("f_C2", "frequency", "pole of C2"), _o("f_L", "frequency", "estimated lower cutoff")],
     primary="f_L", conditions=["short-circuit time-constant estimate", "r_o neglected"],
     method="R_C1 = Rsig + RB || r_pi; R_CE = RE || (r_pi + Rsig || RB)/(beta + 1); R_C2 = RC + RL; f_L = sum 1/(2 pi R C)",
     latex=r"f_L \approx \sum_i \frac{1}{2\pi R_iC_i}",
     run=sctc_low_cutoff, reference=_ref_sctc_low_cutoff,
     wrong=lambda Rsig, RB, r_pi, beta, RE, RC, RL, C1, C2, CE: (lambda RCE: (lambda r: dict(r, R_CE=_r(RCE), f_CE=_r(1 / (TWO_PI * RCE * CE)),
                                                                                         f_L=_r(r["f_C1"] + r["f_C2"] + 1 / (TWO_PI * RCE * CE))))(
         sctc_low_cutoff(Rsig, RB, r_pi, beta, RE, RC, RL, C1, C2, CE)))(_par(RE, r_pi / (beta + 1.0))),
     rand=lambda rng: {"Rsig": _sig(_lu(rng, 100.0, 1e4), 3), "RB": _sig(_lu(rng, 5e3, 1e5), 3), "r_pi": _sig(_lu(rng, 500.0, 2e4), 3),
                       "beta": _sig(rng.uniform(50.0, 300.0), 3), "RE": _sig(_lu(rng, 200.0, 5e3), 3), "RC": _sig(_lu(rng, 1e3, 1e4), 3),
                       "RL": _sig(_lu(rng, 1e3, 1e5), 3), "C1": _sig(_lu(rng, 1e-7, 1e-4), 3), "C2": _sig(_lu(rng, 1e-7, 1e-4), 3),
                       "CE": _sig(_lu(rng, 1e-6, 1e-3), 3)},
     phrases=("Low-frequency cutoff", "Common-emitter low cutoff"),
     examples=[_ex({"Rsig": 1000.0, "RB": 20e3, "r_pi": 2500.0, "beta": 100.0, "RE": 1000.0, "RC": 5000.0, "RL": 10e3,
                    "C1": 10e-6, "C2": 10e-6, "CE": 100e-6}),
               _ex({"Rsig": 600.0, "RB": 30e3, "r_pi": 1875.0, "beta": 150.0, "RE": 470.0, "RC": 3300.0, "RL": 4700.0,
                    "C1": 4.7e-6, "C2": 1e-6, "CE": 220e-6})],
     level="expert")


def current_mirror(VCC, R, VBE, beta):
    _pos("VCC, R and VBE", VCC, R, VBE)
    _beta(beta)
    _need(VCC > VBE, "VCC must exceed VBE")
    Iref = (VCC - VBE) / R
    Iout = Iref * beta / (beta + 2.0)
    return _out(Iref=Iref, Iout=Iout, ratio=beta / (beta + 2.0))


def _ref_current_mirror(VCC, R, VBE, beta):
    # matched pair, joined bases: one VBE source carries both base currents, each collector beta/2 of that total
    el = [("V", 1, 0, VCC), ("R", 1, 2, R), ("V", 2, 0, VBE), ("F", 2, 0, 2, beta / 2.0), ("F", 3, 0, 2, beta / 2.0), ("V", 3, 0, VCC)]
    v, cur = _mna(3, el)
    Iref = (VCC - v[2].real) / R
    Iout = -cur[5].real
    return {"Iref": Iref, "Iout": Iout, "ratio": Iout / Iref}


_add("elx_current_mirror",
     name="BJT current mirror with finite beta", topic="BJT / current sources",
     purpose="The reference current and the output current of a basic two-transistor current mirror with a resistor to the supply, including the base-current error Iout = Iref beta/(beta + 2).",
     inputs=[_phys("VCC", "voltage", "supply voltage"), _phys("R", "resistance", "reference resistor"),
             _phys("VBE", "voltage", "base-emitter voltage"), _numin("beta", "current gain of each (matched) transistor")],
     outputs=[_o("Iref", "current", "current in the reference resistor"), _o("Iout", "current", "output (mirrored) current"),
              _o("ratio", "number", "Iout/Iref")],
     primary="Iout", conditions=["matched transistors, Early effect neglected"],
     method="Iref = (VCC - VBE)/R; Iout = Iref beta/(beta + 2)",
     latex=r"I_{out} = \frac{\beta}{\beta+2}I_{ref}",
     run=current_mirror, reference=_ref_current_mirror,
     wrong=lambda VCC, R, VBE, beta: (lambda I: _out(Iref=I, Iout=I * beta / (beta + 1.0), ratio=beta / (beta + 1.0)))((VCC - VBE) / R),
     rand=lambda rng: {"VCC": _sig(rng.uniform(3.0, 15.0), 3), "R": _sig(_lu(rng, 1e3, 1e5), 3), "VBE": _sig(rng.uniform(0.6, 0.75), 2),
                       "beta": _sig(rng.uniform(20.0, 300.0), 3)},
     phrases=("Current mirror", "BJT current mirror"),
     examples=[_ex({"VCC": 10.0, "R": 9300.0, "VBE": 0.7, "beta": 100.0}), _ex({"VCC": 5.0, "R": 4300.0, "VBE": 0.7, "beta": 50.0})],
     level="intermediate")


def wilson_mirror(Iref, beta):
    _pos("Iref", Iref)
    _beta(beta)
    ratio = 1.0 - 2.0 / (beta * beta + 2.0 * beta + 2.0)
    return _out(Iout=Iref * ratio, ratio=ratio, error=1.0 - ratio)


def _ref_wilson_mirror(Iref, beta):
    # nodes: 1 = collector of Q1 / base of Q3, 2 = emitter of Q3 / joined bases of Q1, Q2 / collector of Q2, 3 = output
    el = [("I", 0, 1, Iref), ("V", 2, 0, 0.7), ("F", 1, 0, 1, beta / 2.0), ("F", 2, 0, 1, beta / 2.0),
          ("V", 1, 2, 0.7), ("F", 3, 2, 4, beta), ("V", 3, 0, 5.0)]
    v, cur = _mna(3, el)
    Iout = -cur[6].real
    return {"Iout": Iout, "ratio": Iout / Iref, "error": 1.0 - Iout / Iref}


_add("elx_wilson_current_mirror",
     name="Wilson current mirror output current", topic="BJT / current sources",
     purpose="The output current of a three-transistor Wilson current mirror and its small base-current error, 2/(beta^2 + 2 beta + 2).",
     inputs=[_phys("Iref", "current", "reference current", show=("mA", 1e3)), _numin("beta", "current gain of each matched transistor")],
     outputs=[_o("Iout", "current", "output current"), _o("ratio", "number", "Iout/Iref"), _o("error", "number", "relative error 1 - Iout/Iref")],
     primary="Iout", conditions=["matched transistors, Early effect neglected"],
     method="Iout = Iref (1 - 2/(beta^2 + 2 beta + 2))",
     latex=r"I_{out} = I_{ref}\left(1 - \frac{2}{\beta^2 + 2\beta + 2}\right)",
     run=wilson_mirror, reference=_ref_wilson_mirror,
     wrong=lambda Iref, beta: _out(Iout=Iref * beta / (beta + 2.0), ratio=beta / (beta + 2.0), error=2.0 / (beta + 2.0)),
     rand=lambda rng: {"Iref": _sig(_lu(rng, 1e-5, 1e-2), 3), "beta": _sig(rng.uniform(10.0, 300.0), 3)},
     phrases=("Wilson current mirror", "Wilson mirror"),
     examples=[_ex({"Iref": 1e-3, "beta": 100.0}), _ex({"Iref": 0.2e-3, "beta": 20.0})],
     level="expert")


def diff_pair(I_tail, beta, VT, RC, REE):
    _pos("I_tail, beta, VT, RC and REE", I_tail, beta, VT, RC, REE)
    IC = beta / (beta + 1.0) * I_tail / 2.0
    gm = IC / VT
    rpi = beta / gm
    Ad_se = -gm * RC / 2.0
    Acm = -gm * RC / (1.0 + 2.0 * REE * (beta + 1.0) / rpi)
    cmrr = abs(Ad_se / Acm)
    return _out(IC=IC, gm=gm, Ad_diff=-gm * RC, Ad_se=Ad_se, Acm=Acm, CMRR=cmrr, CMRR_dB=_db(cmrr))


def _ref_diff_pair(I_tail, beta, VT, RC, REE):
    IC = beta / (beta + 1.0) * I_tail / 2.0
    gm = IC / VT
    rpi = beta / gm

    def solve(v1, v2):
        el = [("V", 1, 0, v1), ("V", 2, 0, v2), ("R", 1, 5, rpi), ("R", 2, 5, rpi), ("Gm", 3, 5, 1, 5, gm), ("Gm", 4, 5, 2, 5, gm),
              ("R", 3, 0, RC), ("R", 4, 0, RC), ("R", 5, 0, REE)]
        v, _ = _mna(5, el)
        return v[3].real, v[4].real
    a3, a4 = solve(0.5, -0.5)
    c3, _ = solve(1.0, 1.0)
    cmrr = abs(a3 / c3)
    return {"IC": IC, "gm": gm, "Ad_diff": a3 - a4, "Ad_se": a3, "Acm": c3, "CMRR": cmrr, "CMRR_dB": 20.0 * math.log10(cmrr)}


_add("elx_differential_pair",
     name="BJT differential pair: differential and common-mode gain, CMRR", topic="BJT / differential amplifiers",
     purpose="The differential gain (differential and single-ended output), the common-mode gain and the common-mode rejection ratio of a resistively loaded BJT differential pair with a tail current source of finite resistance.",
     inputs=[_phys("I_tail", "current", "tail (bias) current", show=("mA", 1e3)), _numin("beta", "current gain"),
             _phys("VT", "voltage", "thermal voltage", show=("mV", 1e3)), _phys("RC", "resistance", "collector resistor of each side"),
             _phys("REE", "resistance", "output resistance of the tail current source")],
     outputs=[_o("IC", "current", "collector current of each transistor"), _o("gm", "conductance", "transconductance of each"),
              _o("Ad_diff", "number", "differential gain, differential output"), _o("Ad_se", "number", "differential gain, single-ended output"),
              _o("Acm", "number", "common-mode gain, single-ended output"), _o("CMRR", "number", "|Ad_se/Acm|"), _o("CMRR_dB", "number", "CMRR in dB")],
     primary="CMRR", conditions=["matched transistors, r_o neglected"],
     method="IC = alpha I/2; gm = IC/VT; Ad = -gm RC (/2 single-ended); Acm = -gm RC/(1 + 2 REE (beta + 1)/r_pi); CMRR = |Ad,se/Acm|",
     latex=r"A_{cm} = -\frac{g_mR_C}{1 + 2R_{EE}(\beta+1)/r_\pi},\quad \text{CMRR} = \frac{1 + 2R_{EE}(\beta+1)/r_\pi}{2}",
     run=diff_pair, reference=_ref_diff_pair,
     wrong=lambda I_tail, beta, VT, RC, REE: (lambda r: dict(r, CMRR=_r(r["gm"] * REE), CMRR_dB=_r(_db(r["gm"] * REE))))(diff_pair(I_tail, beta, VT, RC, REE)),
     rand=lambda rng: {"I_tail": _sig(_lu(rng, 1e-4, 1e-2), 3), "beta": _sig(rng.uniform(30.0, 300.0), 3), "VT": rng.choice([0.025, 0.026]),
                       "RC": _sig(_lu(rng, 1e3, 2e4), 3), "REE": _sig(_lu(rng, 1e4, 1e6), 3)},
     phrases=("Differential pair", "BJT differential amplifier"),
     examples=[_ex({"I_tail": 1e-3, "beta": 100.0, "VT": 0.025, "RC": 10e3, "REE": 100e3}),
               _ex({"I_tail": 2e-3, "beta": 200.0, "VT": 0.026, "RC": 5000.0, "REE": 50e3})],
     level="expert")


def cascode(IC, beta, VT, VA, RL):
    _pos("IC, beta, VT, VA and RL", IC, beta, VT, VA, RL)
    gm = IC / VT
    rpi, ro = beta / gm, VA / IC
    Rout = ro + (1.0 + gm * ro) * _par(ro, rpi)
    Gm = gm * (gm + 1.0 / ro) / (gm + 2.0 / ro + 1.0 / rpi)
    return _out(gm=gm, r_o=ro, Rout=Rout, Gm=Gm, Av=-Gm * _par(Rout, RL))


def _ref_cascode(IC, beta, VT, VA, RL):
    gm, ro = IC / VT, VA / IC
    rpi = beta / gm
    core = [("Gm", 2, 0, 1, 0, gm), ("R", 2, 0, ro), ("R", 2, 0, rpi), ("Gm", 3, 2, 0, 2, gm), ("R", 3, 2, ro)]
    v, _ = _mna(3, [("V", 1, 0, 1.0)] + core + [("R", 3, 0, RL)])
    vt, _ = _mna(3, [("V", 1, 0, 0.0)] + core + [("I", 0, 3, 1.0)])
    vs, cur = _mna(3, [("V", 1, 0, 1.0)] + core + [("V", 3, 0, 0.0)])          # output shorted: transconductance
    return {"gm": gm, "r_o": ro, "Rout": vt[3].real, "Gm": -cur[6].real, "Av": v[3].real}


_add("elx_bjt_cascode",
     name="BJT cascode: output resistance, transconductance and gain", topic="BJT / amplifiers",
     purpose="The output resistance, the short-circuit transconductance and the voltage gain into a load of a cascode (common emitter driving common base) of two identical transistors at the same bias current.",
     inputs=[_phys("IC", "current", "bias current of both transistors", show=("mA", 1e3)), _numin("beta", "current gain"),
             _phys("VT", "voltage", "thermal voltage", show=("mV", 1e3)), _phys("VA", "voltage", "Early voltage"),
             _phys("RL", "resistance", "load resistance")],
     outputs=[_o("gm", "conductance", "transconductance of each transistor"), _o("r_o", "resistance", "output resistance of each"),
              _o("Rout", "resistance", "output resistance of the cascode"), _o("Gm", "conductance", "short-circuit transconductance"),
              _o("Av", "number", "voltage gain into RL")],
     primary="Rout", conditions=["identical transistors, base of the upper one at signal ground"],
     method="Rout = r_o + (1 + gm r_o)(r_o || r_pi); Gm = gm (gm + 1/r_o)/(gm + 2/r_o + 1/r_pi); Av = -Gm (Rout || RL)",
     latex=r"R_{out} = r_o + (1 + g_mr_o)(r_o\parallel r_\pi),\quad A_v = -G_m(R_{out}\parallel R_L)",
     run=cascode, reference=_ref_cascode,
     wrong=lambda IC, beta, VT, VA, RL: (lambda r: dict(r, Rout=_r(IC / VT * (VA / IC) ** 2), Av=_r(-r["Gm"] * _par(IC / VT * (VA / IC) ** 2, RL))))(cascode(IC, beta, VT, VA, RL)),
     rand=lambda rng: {"IC": _sig(_lu(rng, 1e-5, 1e-2), 3), "beta": _sig(rng.uniform(50.0, 300.0), 3), "VT": rng.choice([0.025, 0.026]),
                       "VA": _sig(rng.uniform(20.0, 150.0), 3), "RL": _sig(_lu(rng, 1e4, 1e8), 3)},
     phrases=("BJT cascode", "Cascode amplifier"),
     examples=[_ex({"IC": 1e-3, "beta": 100.0, "VT": 0.025, "VA": 100.0, "RL": 1e6}), _ex({"IC": 0.1e-3, "beta": 200.0, "VT": 0.026, "VA": 50.0, "RL": 10e6})],
     level="expert")


# ================================================================================================ MOSFET and CMOS
def mosfet_id(kn, Vt, VGS, VDS, lam=0.0):
    _pos("kn and Vt", kn, Vt)
    _fin(VGS)
    _nonneg("VDS and lambda", VDS, lam)
    VOV = VGS - Vt
    if VOV <= 0:
        return _out(region="cutoff", ID=0.0, VOV=VOV, VDS_sat=0.0)
    if VDS < VOV:
        return _out(region="triode", ID=kn * (VOV * VDS - VDS * VDS / 2.0), VOV=VOV, VDS_sat=VOV)
    return _out(region="saturation", ID=kn / 2.0 * VOV * VOV * (1.0 + lam * VDS), VOV=VOV, VDS_sat=VOV)


def _ref_mosfet_id(kn, Vt, VGS, VDS, lam=0.0):
    # gradual-channel approximation: ID = kn * integral of the channel charge (VOV - V) along the channel up to the
    # pinch-off voltage, times (1 + lambda VDS) beyond pinch-off
    VOV = VGS - Vt
    if VOV <= 0:
        return {"region": "cutoff", "ID": 0.0, "VOV": VOV, "VDS_sat": 0.0}
    top = min(VDS, VOV)
    I = kn * _quad(lambda v: VOV - v, 0.0, top)
    sat = VDS >= VOV
    return {"region": "saturation" if sat else "triode", "ID": I * (1.0 + lam * VDS) if sat else I, "VOV": VOV, "VDS_sat": VOV}


def _rand_mosfet_id(rng):
    Vt = _sig(rng.uniform(0.3, 1.5), 3)
    VGS = _sig(Vt + (rng.uniform(0.05, 3.0) if rng.random() < 0.92 else -rng.uniform(0.0, 0.3)), 3)
    VOV = VGS - Vt
    VDS = _sig(VOV * rng.uniform(1.0, 4.0), 3) if rng.random() < 0.85 else _sig(max(VOV, 0.1) * rng.uniform(0.05, 0.95), 3)
    d = {"kn": _sig(_lu(rng, 1e-5, 1e-1), 3), "Vt": Vt, "VGS": VGS, "VDS": abs(VDS)}
    if rng.random() < 0.6:
        d["lam"] = _sig(rng.uniform(0.0, 0.1), 2)
    return d


_add("elx_mosfet_drain_current",
     name="MOSFET drain current (cutoff, triode, saturation)", topic="MOSFET / drain current",
     purpose="The operating region and drain current of an enhancement NMOS transistor by the square law: cutoff, triode or saturation with optional channel-length modulation (use magnitudes for PMOS).",
     inputs=[_phys("kn", "transconductance_parameter", "process transconductance times aspect ratio, mu Cox W/L"),
             _phys("Vt", "voltage", "threshold voltage"), _phys("VGS", "voltage", "gate-source voltage"),
             _phys("VDS", "voltage", "drain-source voltage"), _phys("lam", "inverse_voltage", "channel-length modulation lambda (default 0)", required=False)],
     outputs=[_o("region", "number", "cutoff, triode or saturation"), _o("ID", "current", "drain current"),
              _o("VOV", "voltage", "overdrive VGS - Vt"), _o("VDS_sat", "voltage", "saturation voltage")],
     primary="ID", conditions=["kn, Vt > 0; VDS >= 0", "long-channel square law"],
     method="cutoff VOV <= 0; triode VDS < VOV: ID = kn (VOV VDS - VDS^2/2); saturation: ID = (kn/2) VOV^2 (1 + lambda VDS)",
     latex=r"I_D = \begin{cases} k_n\left(V_{OV}V_{DS} - \tfrac{1}{2}V_{DS}^2\right) & V_{DS} < V_{OV}\\ \tfrac{1}{2}k_nV_{OV}^2(1+\lambda V_{DS}) & V_{DS}\ge V_{OV}\end{cases}",
     run=mosfet_id, reference=_ref_mosfet_id,
     wrong=lambda kn, Vt, VGS, VDS, lam=0.0: (lambda r: dict(r, ID=_r(2.0 * r["ID"])) if r["region"] == "saturation" else r)(mosfet_id(kn, Vt, VGS, VDS, lam)),
     rand=_rand_mosfet_id,
     phrases=("MOSFET drain current", "NMOS drain current"),
     examples=[_ex({"kn": 2e-3, "Vt": 1.0, "VGS": 3.0, "VDS": 5.0}, {"region": "saturation", "ID": 0.004}),
               _ex({"kn": 1e-3, "Vt": 0.7, "VGS": 2.0, "VDS": 0.5, "lam": 0.02}, {"region": "triode", "ID": 0.000525})],
     level="beginner")


def mosfet_gm_ro(kn, ID, lam, Vt):
    _pos("kn, ID, lambda and Vt", kn, ID, lam, Vt)
    VOV = math.sqrt(2.0 * ID / kn)
    gm = math.sqrt(2.0 * kn * ID)
    ro = 1.0 / (lam * ID)
    return _out(VOV=VOV, VGS=Vt + VOV, gm=gm, r_o=ro, A0=gm * ro)


_MG = {}


def _ref_mosfet_gm_ro(kn, ID, lam, Vt):
    if "f" not in _MG:
        import sympy as sp
        k, vgs, vds, vt, l = sp.symbols("k vgs vds vt l", positive=True)
        i = k / 2 * (vgs - vt) ** 2 * (1 + l * vds)
        _MG["f"] = sp.lambdify((k, vgs, vds, vt, l), [sp.diff(i, vgs), 1 / sp.diff(i, vds)])
    VOV = _brentq(lambda v: kn / 2.0 * v * v - ID, 0.0, 1e3)
    gm, ro = [float(x) for x in _MG["f"](kn, Vt + VOV, 0.0, Vt, lam)]
    return {"VOV": VOV, "VGS": Vt + VOV, "gm": gm, "r_o": ro, "A0": gm * ro}


_add("elx_mosfet_gm_ro",
     name="MOSFET small-signal gm, ro and intrinsic gain", topic="MOSFET / small-signal model",
     purpose="The overdrive voltage, gate-source voltage, transconductance gm = sqrt(2 kn ID), output resistance ro = 1/(lambda ID) and intrinsic gain of a MOSFET in saturation at a drain current.",
     inputs=[_phys("kn", "transconductance_parameter", "mu Cox W/L"), _phys("ID", "current", "drain bias current", show=("mA", 1e3)),
             _phys("lam", "inverse_voltage", "channel-length modulation lambda"), _phys("Vt", "voltage", "threshold voltage")],
     outputs=[_o("VOV", "voltage", "overdrive voltage"), _o("VGS", "voltage", "gate-source voltage"), _o("gm", "conductance", "transconductance"),
              _o("r_o", "resistance", "output resistance"), _o("A0", "number", "intrinsic gain gm r_o")],
     primary="gm", conditions=["saturation", "ID taken without the (1 + lambda VDS) factor"],
     method="VOV = sqrt(2 ID/kn); gm = sqrt(2 kn ID) = 2 ID/VOV; ro = 1/(lambda ID)",
     latex=r"g_m = \sqrt{2k_nI_D} = \frac{2I_D}{V_{OV}},\quad r_o = \frac{1}{\lambda I_D}",
     run=mosfet_gm_ro, reference=_ref_mosfet_gm_ro,
     wrong=lambda kn, ID, lam, Vt: (lambda r: dict(r, gm=_r(math.sqrt(kn * ID)), A0=_r(math.sqrt(kn * ID) / (lam * ID))))(mosfet_gm_ro(kn, ID, lam, Vt)),
     rand=lambda rng: {"kn": _sig(_lu(rng, 1e-4, 5e-2), 3), "ID": _sig(_lu(rng, 1e-5, 1e-2), 3), "lam": _sig(rng.uniform(0.005, 0.1), 2),
                       "Vt": _sig(rng.uniform(0.3, 1.2), 3)},
     phrases=("MOSFET small-signal parameters", "MOSFET gm and ro"),
     examples=[_ex({"kn": 2e-3, "ID": 1e-3, "lam": 0.01, "Vt": 0.7}, {"VOV": 1.0, "gm": 0.002, "r_o": 100000.0, "A0": 200.0}),
               _ex({"kn": 5e-3, "ID": 0.4e-3, "lam": 0.05, "Vt": 0.5})],
     level="intermediate")


def mosfet_divider_bias(VDD, R1, R2, RD, RS, kn, Vt):
    _pos("VDD, R1, R2, RD, kn and Vt", VDD, R1, R2, RD, kn, Vt)
    _nonneg("RS", RS)
    VG = VDD * R2 / (R1 + R2)
    _need(VG > Vt, "the gate voltage is below the threshold: the transistor is off")
    a = VG - Vt
    VOV = 2.0 * a / (1.0 + math.sqrt(1.0 + 2.0 * kn * RS * a))
    ID = kn / 2.0 * VOV * VOV
    VS = ID * RS
    VD = VDD - ID * RD
    _need(VD - VS >= VOV, "the transistor is in the triode region (VDS < VOV): the saturation bias equations do not apply")
    return _out(VG=VG, VGS=Vt + VOV, VOV=VOV, ID=ID, VS=VS, VD=VD, VDS=VD - VS)


def _ref_mosfet_divider_bias(VDD, R1, R2, RD, RS, kn, Vt):
    vo, _ = _mna(2, [("V", 1, 0, VDD), ("R", 1, 2, R1), ("R", 2, 0, R2)])
    VG = vo[2].real
    if RS > 0:
        ID = _brentq(lambda i: i - kn / 2.0 * (VG - i * RS - Vt) ** 2, 0.0, (VG - Vt) / RS)
    else:
        ID = kn / 2.0 * (VG - Vt) ** 2
    VS, VD = ID * RS, VDD - ID * RD
    return {"VG": VG, "VGS": VG - VS, "VOV": VG - VS - Vt, "ID": ID, "VS": VS, "VD": VD, "VDS": VD - VS}


def _wrong_mos_div(VDD, R1, R2, RD, RS, kn, Vt):
    VG = VDD * R2 / (R1 + R2)
    a = VG - Vt
    VOV = (-1.0 - math.sqrt(1.0 + 2.0 * kn * RS * a)) / (kn * RS)            # the other root of the quadratic
    ID = kn / 2.0 * VOV * VOV
    return _out(VG=VG, VGS=Vt + VOV, VOV=VOV, ID=ID, VS=ID * RS, VD=VDD - ID * RD, VDS=VDD - ID * RD - ID * RS)


def _rand_mos_div(rng):
    VDD, Vt, kn = _sig(rng.uniform(5.0, 20.0), 3), _sig(rng.uniform(0.5, 2.0), 3), _sig(_lu(rng, 2e-4, 2e-2), 3)
    R2 = _sig(_lu(rng, 1e4, 1e6), 3)
    R1 = _sig(R2 * rng.uniform(0.8, 4.0), 3)
    RS = _sig(_lu(rng, 100.0, 5000.0), 3)
    VG = VDD * R2 / (R1 + R2)
    if VG <= Vt:
        return {"VDD": VDD, "R1": R1, "R2": R2, "RD": 1000.0, "RS": RS, "kn": kn, "Vt": Vt}
    a = VG - Vt
    VOV = 2 * a / (1 + math.sqrt(1 + 2 * kn * RS * a))
    ID = kn / 2 * VOV * VOV
    room = VDD - ID * RS - VOV
    return {"VDD": VDD, "R1": R1, "R2": R2, "RD": _sig(rng.uniform(0.1, 0.9) * room / ID, 3) if room > 0 else 1000.0, "RS": RS, "kn": kn, "Vt": Vt}


_add("elx_mosfet_divider_bias",
     name="MOSFET voltage-divider bias with source resistor: Q-point", topic="MOSFET / DC bias",
     purpose="The Q-point (gate voltage, VGS, drain current, node voltages and VDS) of an NMOS stage with a gate divider and a source resistor, solving the square law and the gate loop (the physical root), with a saturation check.",
     inputs=[_phys("VDD", "voltage", "supply voltage"), _phys("R1", "resistance", "upper gate resistor"), _phys("R2", "resistance", "lower gate resistor"),
             _phys("RD", "resistance", "drain resistor"), _phys("RS", "resistance", "source resistor (0 allowed)"),
             _phys("kn", "transconductance_parameter", "mu Cox W/L"), _phys("Vt", "voltage", "threshold voltage")],
     outputs=[_o("VG", "voltage", "gate voltage"), _o("VGS", "voltage", "gate-source voltage"), _o("VOV", "voltage", "overdrive"),
              _o("ID", "current", "drain current"), _o("VS", "voltage", "source voltage"), _o("VD", "voltage", "drain voltage"),
              _o("VDS", "voltage", "drain-source voltage")],
     primary="ID", conditions=["VG > Vt", "saturation: VDS >= VOV (otherwise refused)", "lambda neglected"],
     method="VG = VDD R2/(R1 + R2); (kn RS/2) VOV^2 + VOV - (VG - Vt) = 0, positive root; ID = (kn/2) VOV^2",
     latex=r"V_{OV} = \frac{2(V_G - V_t)}{1 + \sqrt{1 + 2k_nR_S(V_G - V_t)}},\quad I_D = \tfrac{1}{2}k_nV_{OV}^2",
     run=mosfet_divider_bias, reference=_ref_mosfet_divider_bias, wrong=_wrong_mos_div, rand=_rand_mos_div,
     phrases=("MOSFET divider bias", "MOSFET voltage-divider bias"),
     examples=[_ex({"VDD": 15.0, "R1": 1e6, "R2": 500e3, "RD": 3300.0, "RS": 1000.0, "kn": 2e-3, "Vt": 1.5}),
               _ex({"VDD": 12.0, "R1": 680e3, "R2": 320e3, "RD": 2200.0, "RS": 0.0, "kn": 1e-3, "Vt": 1.0})],
     level="intermediate")


def cs_amplifier(gm, ro, RD, RL, RS, RG, Rsig):
    _pos("gm, ro, RD, RL, RG and Rsig", gm, ro, RD, RL, RG, Rsig)
    _nonneg("the unbypassed source resistance RS", RS)
    Rd = _par(RD, RL)
    Av = -gm * ro * Rd / (Rd + ro + RS * (1.0 + gm * ro))
    Rout = _par(RD, ro + RS * (1.0 + gm * ro))
    return _out(Av=Av, Rin=RG, Rout=Rout, Gv=Av * RG / (RG + Rsig))


def _ref_cs_amplifier(gm, ro, RD, RL, RS, RG, Rsig):
    S = 4 if RS > 0 else 0
    n = 4 if RS > 0 else 3
    core = [("R", 2, 0, RG), ("Gm", 3, S, 2, S, gm), ("R", 3, S, ro), ("R", 3, 0, RD)] + ([("R", 4, 0, RS)] if RS > 0 else [])
    v, _ = _mna(n, [("V", 1, 0, 1.0), ("R", 1, 2, Rsig), ("R", 3, 0, RL)] + core)
    vt, _ = _mna(n, [("V", 1, 0, 0.0), ("R", 1, 2, Rsig), ("I", 0, 3, 1.0)] + core)
    vi, cur = _mna(n, [("V", 1, 0, 1.0), ("R", 1, 2, Rsig), ("R", 3, 0, RL)] + core)
    vg = v[2].real
    return {"Av": v[3].real / vg, "Rin": vg / ((1.0 - vg) / Rsig), "Rout": vt[3].real, "Gv": v[3].real}


_add("elx_common_source_amplifier",
     name="Common-source amplifier with source degeneration and ro", topic="MOSFET / amplifiers",
     purpose="The voltage gain, input and output resistances and overall gain of a common-source stage, exact with the transistor's output resistance ro and an optional unbypassed source resistor.",
     inputs=[_phys("gm", "conductance", "transconductance"), _phys("ro", "resistance", "transistor output resistance"),
             _phys("RD", "resistance", "drain resistor"), _phys("RL", "resistance", "load resistor"),
             _phys("RS", "resistance", "unbypassed source resistance (0 if bypassed)"), _phys("RG", "resistance", "gate bias resistance"),
             _phys("Rsig", "resistance", "source resistance")],
     outputs=[_o("Av", "number", "voltage gain gate to output"), _o("Rin", "resistance", "input resistance"),
              _o("Rout", "resistance", "output resistance (excluding RL)"), _o("Gv", "number", "overall gain from the source")],
     primary="Av", conditions=["gm, ro, RD, RL, RG, Rsig > 0; RS >= 0"],
     method="Av = -gm ro RD'/(RD' + ro + RS (1 + gm ro)) with RD' = RD || RL; Rout = RD || (ro + RS(1 + gm ro))",
     latex=r"A_v = -\frac{g_mr_oR_D'}{R_D' + r_o + R_S(1 + g_mr_o)}",
     run=cs_amplifier, reference=_ref_cs_amplifier,
     wrong=lambda gm, ro, RD, RL, RS, RG, Rsig: (lambda Av: _out(Av=Av, Rin=RG, Rout=RD, Gv=Av * RG / (RG + Rsig)))(-gm * _par(RD, RL) / (1.0 + gm * RS)),
     rand=lambda rng: {"gm": _sig(_lu(rng, 5e-4, 2e-2), 3), "ro": _sig(_lu(rng, 1e4, 3e5), 3), "RD": _sig(_lu(rng, 1e3, 2e4), 3),
                       "RL": _sig(_lu(rng, 1e3, 1e5), 3), "RS": _sig(_lu(rng, 50.0, 2000.0), 3) if rng.random() < 0.7 else 0.0,
                       "RG": _sig(_lu(rng, 1e5, 1e7), 3), "Rsig": _sig(_lu(rng, 1e3, 1e5), 3)},
     phrases=("Common-source amplifier", "CS amplifier"),
     examples=[_ex({"gm": 2e-3, "ro": 100e3, "RD": 10e3, "RL": 10e3, "RS": 0.0, "RG": 1e6, "Rsig": 10e3}),
               _ex({"gm": 4e-3, "ro": 50e3, "RD": 4700.0, "RL": 22e3, "RS": 500.0, "RG": 2.2e6, "Rsig": 50e3})],
     level="advanced")


def source_follower(gm, ro, RS, RL, RG, Rsig):
    _pos("gm, ro, RS, RL, RG and Rsig", gm, ro, RS, RL, RG, Rsig)
    R = _par(RS, RL, ro)
    Av = gm * R / (1.0 + gm * R)
    return _out(Av=Av, Rin=RG, Rout=_par(RS, ro, 1.0 / gm), Gv=Av * RG / (RG + Rsig))


def _ref_source_follower(gm, ro, RS, RL, RG, Rsig):
    core = [("R", 2, 0, RG), ("Gm", 0, 3, 2, 3, gm), ("R", 3, 0, ro), ("R", 3, 0, RS)]
    v, _ = _mna(3, [("V", 1, 0, 1.0), ("R", 1, 2, Rsig), ("R", 3, 0, RL)] + core)
    vt, _ = _mna(3, [("V", 1, 0, 0.0), ("R", 1, 2, Rsig), ("I", 0, 3, 1.0)] + core)
    vg = v[2].real
    return {"Av": v[3].real / vg, "Rin": vg / ((1.0 - vg) / Rsig), "Rout": vt[3].real, "Gv": v[3].real}


_add("elx_source_follower",
     name="Source follower (common drain)", topic="MOSFET / amplifiers",
     purpose="The voltage gain, input resistance, output resistance and overall gain of a MOSFET source follower, including ro.",
     inputs=[_phys("gm", "conductance", "transconductance"), _phys("ro", "resistance", "transistor output resistance"),
             _phys("RS", "resistance", "source resistor"), _phys("RL", "resistance", "load resistor"),
             _phys("RG", "resistance", "gate bias resistance"), _phys("Rsig", "resistance", "source resistance")],
     outputs=[_o("Av", "number", "voltage gain gate to output"), _o("Rin", "resistance", "input resistance"),
              _o("Rout", "resistance", "output resistance"), _o("Gv", "number", "overall gain")],
     primary="Av", conditions=["drain at signal ground"],
     method="Av = gm R/(1 + gm R) with R = RS || RL || ro; Rout = RS || ro || 1/gm",
     latex=r"A_v = \frac{g_mR}{1 + g_mR},\quad R_{out} = R_S\parallel r_o\parallel\frac{1}{g_m}",
     run=source_follower, reference=_ref_source_follower,
     wrong=lambda gm, ro, RS, RL, RG, Rsig: dict(source_follower(gm, ro, RS, RL, RG, Rsig), Rout=_r(_par(RS, 1.0 / gm))),
     rand=lambda rng: {"gm": _sig(_lu(rng, 5e-4, 2e-2), 3), "ro": _sig(_lu(rng, 5e3, 1e5), 3), "RS": _sig(_lu(rng, 500.0, 2e4), 3),
                       "RL": _sig(_lu(rng, 500.0, 1e5), 3), "RG": _sig(_lu(rng, 1e5, 1e7), 3), "Rsig": _sig(_lu(rng, 1e3, 1e5), 3)},
     phrases=("Source follower", "Common-drain amplifier"),
     examples=[_ex({"gm": 2e-3, "ro": 50e3, "RS": 2000.0, "RL": 2000.0, "RG": 1e6, "Rsig": 10e3}),
               _ex({"gm": 5e-3, "ro": 20e3, "RS": 1000.0, "RL": 10e3, "RG": 470e3, "Rsig": 1000.0})],
     level="advanced")


def body_effect(Vt0, gamma, phi2F, VSB):
    _pos("Vt0 and 2 phi_F", Vt0, phi2F)
    _nonneg("gamma and VSB", gamma, VSB)
    d = gamma * (math.sqrt(phi2F + VSB) - math.sqrt(phi2F))
    return _out(Vt=Vt0 + d, dVt=d)


def _ref_body_effect(Vt0, gamma, phi2F, VSB):
    # through the depletion charge: gamma = sqrt(2 q eps NA)/Cox, Qd = sqrt(2 q eps NA (2 phiF + VSB)); a unit Cox
    # and the doping that gives this gamma (silicon permittivity)
    import numpy as np
    eps = 11.7 * EPS0
    Cox = 1e-3
    NA = (gamma * Cox) ** 2 / (2.0 * Q_E * eps)
    Qd = lambda v: np.sqrt(2.0 * Q_E * eps * NA * (phi2F + v))
    d = float((Qd(VSB) - Qd(0.0)) / Cox)
    return {"Vt": Vt0 + d, "dVt": d}


_add("elx_mosfet_body_effect",
     name="MOSFET threshold voltage with body effect", topic="MOSFET / threshold voltage",
     purpose="The threshold voltage of a MOSFET whose source is above its body, Vt = Vt0 + gamma (sqrt(2 phiF + VSB) - sqrt(2 phiF)).",
     inputs=[_phys("Vt0", "voltage", "zero-bias threshold voltage"), _numin("gamma", "body-effect coefficient in V^0.5", words=("V^0.5", "V^(1/2)", "sqrt(V)")),
             _phys("phi2F", "voltage", "surface potential 2 phi_F (about 0.6 to 0.9 V)"), _phys("VSB", "voltage", "source-body voltage")],
     outputs=[_o("Vt", "voltage", "threshold voltage"), _o("dVt", "voltage", "threshold increase")],
     primary="Vt", conditions=["VSB >= 0", "uniform substrate doping"],
     method="Vt = Vt0 + gamma (sqrt(2 phiF + VSB) - sqrt(2 phiF))",
     latex=r"V_t = V_{t0} + \gamma\left(\sqrt{2\phi_F + V_{SB}} - \sqrt{2\phi_F}\right)",
     run=body_effect, reference=_ref_body_effect,
     wrong=lambda Vt0, gamma, phi2F, VSB: (lambda d: _out(Vt=Vt0 + d, dVt=d))(gamma * math.sqrt(VSB)),
     rand=lambda rng: {"Vt0": _sig(rng.uniform(0.3, 1.0), 3), "gamma": _sig(rng.uniform(0.2, 0.9), 3), "phi2F": _sig(rng.uniform(0.6, 0.9), 3),
                       "VSB": _sig(rng.uniform(0.1, 5.0), 3)},
     phrases=("Body effect threshold voltage", "MOSFET body effect"),
     examples=[_ex({"Vt0": 0.7, "gamma": 0.4, "phi2F": 0.7, "VSB": 3.0}), _ex({"Vt0": 0.5, "gamma": 0.5, "phi2F": 0.8, "VSB": 1.0})],
     level="advanced",
     extra_q=[{"q": "Body effect threshold voltage: Vt0 = 0.7 V, gamma = 0.4 V^0.5, phi2F = 0.7 V, VSB = 3 V", "like": 0}],
     weak="the same model written through the depletion charge with numpy")


def cmos_threshold(VDD, Vtn, Vtp, kn, kp):
    _pos("VDD, Vtn, |Vtp|, kn and kp", VDD, Vtn, Vtp, kn, kp)
    _need(Vtn + Vtp < VDD, "VDD must exceed Vtn + |Vtp|")
    r = math.sqrt(kp / kn)
    return _out(r=r, VM=(Vtn + r * (VDD - Vtp)) / (1.0 + r))


def _ref_cmos_threshold(VDD, Vtn, Vtp, kn, kp):
    # both transistors saturated with equal currents at the switching threshold
    VM = _brentq(lambda v: kn / 2.0 * (v - Vtn) ** 2 - kp / 2.0 * (VDD - v - Vtp) ** 2, Vtn, VDD - Vtp)
    return {"r": float(np_sqrt(kp / kn)), "VM": VM}


def np_sqrt(x):
    import numpy as np
    return np.sqrt(x)


_add("elx_cmos_switching_threshold",
     name="CMOS inverter switching threshold VM", topic="CMOS / inverter",
     purpose="The switching threshold of a CMOS inverter (input = output, both transistors saturated) from the supply, the two threshold voltages and the two transconductance parameters.",
     inputs=[_phys("VDD", "voltage", "supply voltage"), _phys("Vtn", "voltage", "NMOS threshold"), _phys("Vtp", "voltage", "magnitude of the PMOS threshold"),
             _phys("kn", "transconductance_parameter", "NMOS mu_n Cox W/L"), _phys("kp", "transconductance_parameter", "PMOS mu_p Cox W/L")],
     outputs=[_o("r", "number", "sqrt(kp/kn)"), _o("VM", "voltage", "switching threshold")],
     primary="VM", conditions=["Vtn + |Vtp| < VDD", "lambda neglected"],
     method="VM = (Vtn + r (VDD - |Vtp|))/(1 + r), r = sqrt(kp/kn)",
     latex=r"V_M = \frac{V_{tn} + r\,(V_{DD} - |V_{tp}|)}{1 + r},\quad r = \sqrt{k_p/k_n}",
     run=cmos_threshold, reference=_ref_cmos_threshold,
     wrong=lambda VDD, Vtn, Vtp, kn, kp: (lambda r: _out(r=r, VM=(Vtn + r * (VDD - Vtp)) / (1 + r)))(kp / kn),
     rand=lambda rng: {"VDD": _sig(rng.uniform(1.0, 5.0), 3), "Vtn": _sig(rng.uniform(0.2, 0.45), 3), "Vtp": _sig(rng.uniform(0.2, 0.45), 3),
                       "kn": _sig(_lu(rng, 1e-5, 1e-3), 3), "kp": _sig(_lu(rng, 1e-5, 1e-3), 3)},
     phrases=("CMOS switching threshold", "CMOS inverter switching threshold"),
     examples=[_ex({"VDD": 2.5, "Vtn": 0.4, "Vtp": 0.4, "kn": 100e-6, "kp": 100e-6}, {"VM": 1.25}),
               _ex({"VDD": 3.3, "Vtn": 0.5, "Vtp": 0.6, "kn": 200e-6, "kp": 80e-6})],
     level="advanced")


def cmos_delay(Rn, Rp, CL):
    _pos("Rn, Rp and CL", Rn, Rp, CL)
    hl, lh = LN2 * Rn * CL, LN2 * Rp * CL
    return _out(tpHL=hl, tpLH=lh, tp=(hl + lh) / 2.0)


def _ref_cmos_delay(Rn, Rp, CL):
    # the 50 % crossing of the RC discharge (pull-down) and charge (pull-up) found by root finding
    t50 = lambda R: _brentq(lambda t: math.exp(-t / (R * CL)) - 0.5, 0.0, 10.0 * R * CL)
    hl, lh = t50(Rn), t50(Rp)
    return {"tpHL": hl, "tpLH": lh, "tp": (hl + lh) / 2.0}


_add("elx_cmos_propagation_delay",
     name="CMOS gate propagation delay (switch-resistor model)", topic="CMOS / timing",
     purpose="The high-to-low and low-to-high propagation delays (50 % points) and their average for a CMOS gate modelled as switched equivalent resistances driving a load capacitance.",
     inputs=[_phys("Rn", "resistance", "equivalent on-resistance of the pull-down network"), _phys("Rp", "resistance", "equivalent on-resistance of the pull-up network"),
             _phys("CL", "capacitance", "load capacitance", show=("fF", 1e15))],
     outputs=[_o("tpHL", "time", "high-to-low delay"), _o("tpLH", "time", "low-to-high delay"), _o("tp", "time", "average propagation delay")],
     primary="tp", conditions=["step input, RC switch model"],
     method="tpHL = ln 2 Rn CL; tpLH = ln 2 Rp CL; tp = (tpHL + tpLH)/2",
     latex=r"t_{pHL} = \ln 2\,R_nC_L,\quad t_{pLH} = \ln 2\,R_pC_L",
     run=cmos_delay, reference=_ref_cmos_delay,
     wrong=lambda Rn, Rp, CL: _out(tpHL=Rn * CL, tpLH=Rp * CL, tp=(Rn + Rp) * CL / 2.0),
     rand=lambda rng: {"Rn": _sig(_lu(rng, 1e3, 5e4), 3), "Rp": _sig(_lu(rng, 1e3, 1e5), 3), "CL": _sig(_lu(rng, 1e-15, 1e-12), 3)},
     phrases=("CMOS propagation delay", "Gate propagation delay"),
     examples=[_ex({"Rn": 10e3, "Rp": 20e3, "CL": 100e-15}), _ex({"Rn": 5e3, "Rp": 12e3, "CL": 50e-15})],
     level="intermediate")


# ================================================================================================ OP-AMPS
def inverting_amp(Rf, Rin, Vin):
    _pos("Rf and Rin", Rf, Rin)
    _fin(Vin)
    G = -Rf / Rin
    return _out(G=G, Vout=G * Vin, Zin=Rin)


def _ref_inverting_amp(Rf, Rin, Vin):
    el = lambda u: [("V", 1, 0, u), ("R", 1, 2, Rin), ("R", 2, 3, Rf), ("OA", 3, 0, 2)]
    v, cur = _mna(3, el(1.0))
    vv, _ = _mna(3, el(Vin))
    return {"G": v[3].real, "Vout": vv[3].real, "Zin": 1.0 / -cur[0].real}


_add("elx_inverting_amplifier",
     name="Inverting op-amp amplifier", topic="op-amps / basic amplifiers",
     purpose="The closed-loop gain, output voltage and input resistance of an ideal inverting amplifier.",
     inputs=[_phys("Rf", "resistance", "feedback resistor"), _phys("Rin", "resistance", "input resistor"), _phys("Vin", "voltage", "input voltage")],
     outputs=[_o("G", "number", "closed-loop gain -Rf/Rin"), _o("Vout", "voltage", "output voltage"), _o("Zin", "resistance", "input resistance")],
     primary="Vout", conditions=["ideal op-amp, output not saturated"],
     method="G = -Rf/Rin; Vout = G Vin; Zin = Rin",
     latex=r"V_{out} = -\frac{R_f}{R_{in}}V_{in}",
     run=inverting_amp, reference=_ref_inverting_amp,
     wrong=lambda Rf, Rin, Vin: _out(G=-(1 + Rf / Rin), Vout=-(1 + Rf / Rin) * Vin, Zin=Rin),
     rand=lambda rng: {"Rf": _sig(_lu(rng, 1e3, 1e6), 3), "Rin": _sig(_lu(rng, 1e3, 1e5), 3), "Vin": _sig(rng.uniform(-2.0, 2.0), 3) or 0.1},
     phrases=("Inverting amplifier", "Inverting op-amp amplifier"),
     examples=[_ex({"Rf": 100e3, "Rin": 10e3, "Vin": 0.5}, {"G": -10.0, "Vout": -5.0, "Zin": 10000.0}), _ex({"Rf": 47e3, "Rin": 4700.0, "Vin": -0.2})],
     level="beginner")


def noninverting_amp(Rf, Rg, Vin):
    _pos("Rf and Rg", Rf, Rg)
    _fin(Vin)
    G = 1.0 + Rf / Rg
    return _out(G=G, Vout=G * Vin)


def _ref_noninverting_amp(Rf, Rg, Vin):
    el = lambda u: [("V", 1, 0, u), ("R", 2, 0, Rg), ("R", 2, 3, Rf), ("OA", 3, 1, 2)]
    v, _ = _mna(3, el(1.0))
    vv, _ = _mna(3, el(Vin))
    return {"G": v[3].real, "Vout": vv[3].real}


_add("elx_noninverting_amplifier",
     name="Non-inverting op-amp amplifier", topic="op-amps / basic amplifiers",
     purpose="The closed-loop gain 1 + Rf/Rg and the output voltage of an ideal non-inverting amplifier.",
     inputs=[_phys("Rf", "resistance", "feedback resistor"), _phys("Rg", "resistance", "resistor from the inverting input to ground"),
             _phys("Vin", "voltage", "input voltage")],
     outputs=[_o("G", "number", "closed-loop gain"), _o("Vout", "voltage", "output voltage")],
     primary="Vout", conditions=["ideal op-amp"],
     method="G = 1 + Rf/Rg; Vout = G Vin",
     latex=r"V_{out} = \left(1 + \frac{R_f}{R_g}\right)V_{in}",
     run=noninverting_amp, reference=_ref_noninverting_amp,
     wrong=lambda Rf, Rg, Vin: _out(G=Rf / Rg, Vout=Rf / Rg * Vin),
     rand=lambda rng: {"Rf": _sig(_lu(rng, 1e3, 1e6), 3), "Rg": _sig(_lu(rng, 1e3, 1e5), 3), "Vin": _sig(rng.uniform(-2.0, 2.0), 3) or 0.1},
     phrases=("Non-inverting amplifier", "Noninverting amplifier"),
     examples=[_ex({"Rf": 90e3, "Rg": 10e3, "Vin": 0.3}, {"G": 10.0, "Vout": 3.0}), _ex({"Rf": 22e3, "Rg": 4700.0, "Vin": 1.2})],
     level="beginner")


def summing_amp(V, R, Rf):
    _pos("Rf", Rf)
    _need(len(V) == len(R) and len(V) >= 2, "give one resistor per input voltage (at least two inputs)")
    s = 0.0
    for v, r in zip(V, R):
        _fin(v)
        _pos("every input resistor", r)
        s += v / (r * 1e3)
    return _out(Vout=-Rf * s, I_f=s)


def _ref_summing_amp(V, R, Rf):
    n = len(V)
    el = []
    for i, (v, r) in enumerate(zip(V, R)):
        el.append(("V", i + 1, 0, v))
        el.append(("R", i + 1, n + 1, r * 1e3))
    el += [("R", n + 1, n + 2, Rf), ("OA", n + 2, 0, n + 1)]
    vv, _ = _mna(n + 2, el)
    return {"Vout": vv[n + 2].real, "I_f": -vv[n + 2].real / Rf}


_add("elx_summing_amplifier",
     name="Inverting summing amplifier", topic="op-amps / basic amplifiers",
     purpose="The output of an ideal inverting summing amplifier with several inputs, each through its own resistor, and the feedback current.",
     inputs=[_vecin("V", "input voltages in volts", 2, 16), _vecin("R", "input resistors in kilohms, one per input", 2, 16),
             _phys("Rf", "resistance", "feedback resistor")],
     outputs=[_o("Vout", "voltage", "output voltage"), _o("I_f", "current", "current through the feedback resistor")],
     primary="Vout", conditions=["one resistor per input", "ideal op-amp"],
     method="Vout = -Rf sum(Vi/Ri)",
     latex=r"V_{out} = -R_f\sum_i\frac{V_i}{R_i}",
     run=summing_amp, reference=_ref_summing_amp,
     wrong=lambda V, R, Rf: (lambda s: _out(Vout=Rf * s, I_f=s))(sum(v / (r * 1e3) for v, r in zip(V, R))),
     rand=lambda rng: (lambda n: {"V": [_sig(rng.uniform(-3.0, 3.0), 3) for _ in range(n)], "R": [_sig(_lu(rng, 1.0, 100.0), 3) for _ in range(n)],
                                  "Rf": _sig(_lu(rng, 1e3, 1e5), 3)})(rng.randint(2, 5)),
     phrases=("Summing amplifier", "Inverting summing amplifier"),
     examples=[_ex({"V": [1.0, 2.0, -0.5], "R": [10.0, 20.0, 5.0], "Rf": 10e3}, {"Vout": -1.0}),
               _ex({"V": [0.2, 0.4], "R": [1.0, 1.0], "Rf": 4700.0})],
     level="beginner")


def difference_amp(R1, R2, R3, R4, V1, V2):
    _pos("R1, R2, R3 and R4", R1, R2, R3, R4)
    _fin(V1, V2)
    a1 = -R2 / R1
    a2 = R4 / (R3 + R4) * (1.0 + R2 / R1)
    Ad = (a2 - a1) / 2.0
    Acm = a1 + a2
    if abs(Acm) < 1e-12 * abs(Ad):
        Acm = 0.0
    return _out(Vout=a1 * V1 + a2 * V2, Ad=Ad, Acm=Acm)


def _ref_difference_amp(R1, R2, R3, R4, V1, V2):
    def out(v1, v2):
        el = [("V", 1, 0, v1), ("R", 1, 2, R1), ("R", 2, 3, R2), ("V", 4, 0, v2), ("R", 4, 5, R3), ("R", 5, 0, R4), ("OA", 3, 5, 2)]
        v, _ = _mna(5, el)
        return v[3].real
    return {"Vout": out(V1, V2), "Ad": out(-0.5, 0.5), "Acm": out(1.0, 1.0)}


_add("elx_difference_amplifier",
     name="Op-amp difference amplifier (with resistor mismatch)", topic="op-amps / difference amplifiers",
     purpose="The output of a one-op-amp difference amplifier with any four resistors, and its differential and common-mode gains (the common-mode gain is zero only when R4/R3 = R2/R1).",
     inputs=[_phys("R1", "resistance", "resistor from V1 to the inverting input"), _phys("R2", "resistance", "feedback resistor"),
             _phys("R3", "resistance", "resistor from V2 to the non-inverting input"), _phys("R4", "resistance", "resistor from the non-inverting input to ground"),
             _phys("V1", "voltage", "voltage at the inverting side"), _phys("V2", "voltage", "voltage at the non-inverting side")],
     outputs=[_o("Vout", "voltage", "output voltage"), _o("Ad", "number", "differential gain (per volt of V2 - V1)"),
              _o("Acm", "number", "common-mode gain")],
     primary="Vout", conditions=["ideal op-amp"],
     method="Vout = -(R2/R1) V1 + (R4/(R3 + R4))(1 + R2/R1) V2; Ad = (a2 - a1)/2; Acm = a1 + a2",
     latex=r"V_{out} = \frac{R_4}{R_3+R_4}\left(1+\frac{R_2}{R_1}\right)V_2 - \frac{R_2}{R_1}V_1",
     run=difference_amp, reference=_ref_difference_amp,
     wrong=lambda R1, R2, R3, R4, V1, V2: _out(Vout=R2 / R1 * (V2 - V1), Ad=R2 / R1, Acm=0.0),
     rand=lambda rng: (lambda R1, R2: {"R1": R1, "R2": R2, "R3": _sig(R1 * rng.uniform(0.9, 1.1), 3), "R4": _sig(R2 * rng.uniform(0.9, 1.1), 3),
                                       "V1": _sig(rng.uniform(-2.0, 2.0), 3), "V2": _sig(rng.uniform(-2.0, 2.0), 3)})(_sig(_lu(rng, 1e3, 1e5), 3), _sig(_lu(rng, 1e3, 1e6), 3)),
     phrases=("Difference amplifier", "Op-amp difference amplifier"),
     examples=[_ex({"R1": 10e3, "R2": 100e3, "R3": 10e3, "R4": 100e3, "V1": 1.0, "V2": 1.2}, {"Vout": 2.0, "Ad": 10.0, "Acm": 0.0}),
               _ex({"R1": 10e3, "R2": 50e3, "R3": 10.5e3, "R4": 49e3, "V1": 2.0, "V2": 2.1})],
     level="intermediate")


def inamp(R1, RG, R2, R3, V1, V2):
    _pos("R1, RG, R2 and R3", R1, RG, R2, R3)
    _fin(V1, V2)
    G = (1.0 + 2.0 * R1 / RG) * (R3 / R2)
    return _out(G=G, G_stage1=1.0 + 2.0 * R1 / RG, Vout=G * (V2 - V1))


def _ref_inamp(R1, RG, R2, R3, V1, V2):
    def out(v1, v2):
        el = [("V", 1, 0, v1), ("V", 2, 0, v2), ("OA", 3, 1, 5), ("R", 3, 5, R1), ("R", 5, 6, RG), ("R", 6, 4, R1), ("OA", 4, 2, 6),
              ("R", 3, 7, R2), ("R", 7, 9, R3), ("R", 4, 8, R2), ("R", 8, 0, R3), ("OA", 9, 8, 7)]
        v, _ = _mna(9, el)
        return v, v[9].real
    v, G = out(0.0, 1.0)
    _, Vo = out(V1, V2)
    return {"G": G, "G_stage1": (v[4] - v[3]).real, "Vout": Vo}


_add("elx_instrumentation_amplifier",
     name="Three-op-amp instrumentation amplifier", topic="op-amps / difference amplifiers",
     purpose="The gain and output of the classic three-op-amp instrumentation amplifier: input stage gain 1 + 2 R1/RG times the difference stage gain R3/R2.",
     inputs=[_phys("R1", "resistance", "feedback resistor of each input buffer"), _phys("RG", "resistance", "gain-setting resistor between the buffers"),
             _phys("R2", "resistance", "input resistors of the difference stage"), _phys("R3", "resistance", "feedback and ground resistors of the difference stage"),
             _phys("V1", "voltage", "input at the inverting side"), _phys("V2", "voltage", "input at the non-inverting side")],
     outputs=[_o("G", "number", "differential gain"), _o("G_stage1", "number", "gain of the input stage"), _o("Vout", "voltage", "output voltage")],
     primary="Vout", conditions=["ideal op-amps, matched resistors"],
     method="G = (1 + 2 R1/RG)(R3/R2); Vout = G (V2 - V1)",
     latex=r"V_{out} = \left(1 + \frac{2R_1}{R_G}\right)\frac{R_3}{R_2}(V_2 - V_1)",
     run=inamp, reference=_ref_inamp,
     wrong=lambda R1, RG, R2, R3, V1, V2: (lambda G: _out(G=G, G_stage1=1 + R1 / RG, Vout=G * (V2 - V1)))((1 + R1 / RG) * R3 / R2),
     rand=lambda rng: {"R1": _sig(_lu(rng, 1e3, 1e5), 3), "RG": _sig(_lu(rng, 100.0, 1e5), 3), "R2": _sig(_lu(rng, 1e3, 1e5), 3),
                       "R3": _sig(_lu(rng, 1e3, 1e5), 3), "V1": _sig(rng.uniform(-1.0, 1.0), 3), "V2": _sig(rng.uniform(-1.0, 1.0), 3)},
     phrases=("Instrumentation amplifier", "Three-op-amp instrumentation amplifier"),
     examples=[_ex({"R1": 25e3, "RG": 1000.0, "R2": 10e3, "R3": 10e3, "V1": 1.000, "V2": 1.010}, {"G": 51.0}),
               _ex({"R1": 49.4e3, "RG": 494.0, "R2": 10e3, "R3": 20e3, "V1": 0.5, "V2": 0.502})],
     level="intermediate")


def finite_gain_inverting(Rf, R1, A0, Vin):
    _pos("Rf, R1 and A0", Rf, R1, A0)
    _fin(Vin)
    Gi = -Rf / R1
    G = Gi / (1.0 + (1.0 + Rf / R1) / A0)
    return _out(G=G, G_ideal=Gi, Vout=G * Vin, error=1.0 - G / Gi)


def _ref_finite_gain_inverting(Rf, R1, A0, Vin):
    el = lambda u: [("V", 1, 0, u), ("R", 1, 2, R1), ("R", 2, 3, Rf), ("E", 3, 0, 0, 2, A0)]
    v, _ = _mna(3, el(1.0))
    vv, _ = _mna(3, el(Vin))
    vi, _ = _mna(3, [("V", 1, 0, 1.0), ("R", 1, 2, R1), ("R", 2, 3, Rf), ("OA", 3, 0, 2)])
    G, Gi = v[3].real, vi[3].real
    return {"G": G, "G_ideal": Gi, "Vout": vv[3].real, "error": (Gi - G) / Gi}


_add("elx_inverting_amplifier_finite_gain",
     name="Inverting amplifier with finite open-loop gain", topic="op-amps / non-ideal",
     purpose="The exact closed-loop gain of an inverting amplifier built with an op-amp of finite open-loop gain A0, its relative gain error and output.",
     inputs=[_phys("Rf", "resistance", "feedback resistor"), _phys("R1", "resistance", "input resistor"),
             _numin("A0", "open-loop voltage gain (a ratio, not dB)"), _phys("Vin", "voltage", "input voltage")],
     outputs=[_o("G", "number", "closed-loop gain"), _o("G_ideal", "number", "ideal gain -Rf/R1"), _o("Vout", "voltage", "output voltage"),
              _o("error", "number", "relative gain error 1 - G/G_ideal")],
     primary="G", conditions=["A0 > 0 (ratio)", "infinite input resistance, zero output resistance"],
     method="G = -(Rf/R1)/(1 + (1 + Rf/R1)/A0)",
     latex=r"G = \frac{-R_f/R_1}{1 + \dfrac{1 + R_f/R_1}{A_0}}",
     run=finite_gain_inverting, reference=_ref_finite_gain_inverting,
     wrong=lambda Rf, R1, A0, Vin: (lambda G: _out(G=G, G_ideal=-Rf / R1, Vout=G * Vin, error=1 - G / (-Rf / R1)))(-Rf / R1 / (1 + 1 / A0)),
     rand=lambda rng: {"Rf": _sig(_lu(rng, 1e4, 1e6), 3), "R1": _sig(_lu(rng, 1e3, 1e4), 3), "A0": _sig(_lu(rng, 1e2, 1e5), 3),
                       "Vin": _sig(rng.uniform(-1.0, 1.0), 3) or 0.1},
     phrases=("Inverting amplifier with finite gain", "Finite open-loop gain inverting amplifier"),
     examples=[_ex({"Rf": 100e3, "R1": 1000.0, "A0": 1e4, "Vin": 0.01}), _ex({"Rf": 10e3, "R1": 1000.0, "A0": 1000.0, "Vin": 0.1})],
     level="intermediate")


def integrator(R, C, Vin, t, V0=0.0):
    _pos("R, C and t", R, C, t)
    _fin(Vin, V0)
    slope = -Vin / (R * C)
    return _out(Vout=V0 + slope * t, slope=slope, f_unity=1.0 / (TWO_PI * R * C))


def _ref_integrator(R, C, Vin, t, V0=0.0):
    # the capacitor current Vin/R integrated over time; the unity-gain frequency where |1/(j w R C)| = 1
    q = _quad(lambda s: Vin / R, 0.0, t)
    fu = _brentq(lambda f: abs(1.0 / complex(0.0, TWO_PI * f * R * C)) - 1.0, 1e-12, 1e15)
    return {"Vout": V0 - q / C, "slope": -Vin / (R * C), "f_unity": fu}


_add("elx_opamp_integrator",
     name="Op-amp integrator: output ramp and unity-gain frequency", topic="op-amps / integrator and differentiator",
     purpose="The output of an ideal inverting integrator after a time with a constant input, its ramp rate, and the frequency at which its gain is one.",
     inputs=[_phys("R", "resistance", "input resistor"), _phys("C", "capacitance", "feedback capacitor", show=("uF", 1e6)),
             _phys("Vin", "voltage", "constant input voltage"), _phys("t", "time", "time since the start", show=("ms", 1e3)),
             _phys("V0", "voltage", "initial output voltage (default 0)", required=False)],
     outputs=[_o("Vout", "voltage", "output voltage at t"), _o("slope", "voltage_slew_rate", "output ramp rate"),
              _o("f_unity", "frequency", "unity-gain frequency 1/(2 pi R C)")],
     primary="Vout", conditions=["ideal op-amp, output not saturated"],
     method="Vout = V0 - Vin t/(R C); f = 1/(2 pi R C)",
     latex=r"V_{out}(t) = V_0 - \frac{1}{RC}\int_0^t V_{in}\,dt",
     run=integrator, reference=_ref_integrator,
     wrong=lambda R, C, Vin, t, V0=0.0: _out(Vout=V0 + Vin * t / (R * C), slope=Vin / (R * C), f_unity=1 / (TWO_PI * R * C)),
     rand=lambda rng: {"R": _sig(_lu(rng, 1e3, 1e6), 3), "C": _sig(_lu(rng, 1e-9, 1e-5), 3), "Vin": _sig(rng.uniform(-2.0, 2.0), 3) or 0.5,
                       "t": _sig(_lu(rng, 1e-5, 1e-1), 3), **({"V0": _sig(rng.uniform(-2.0, 2.0), 3)} if rng.random() < 0.5 else {})},
     phrases=("Op-amp integrator", "Integrator output"),
     examples=[_ex({"R": 10e3, "C": 1e-6, "Vin": 0.5, "t": 2e-3}, {"Vout": -0.1, "slope": -50.0}),
               _ex({"R": 100e3, "C": 0.1e-6, "Vin": -1.0, "t": 5e-3, "V0": -2.0})],
     level="intermediate",
     weak="the capacitor current integrated by quadrature (the same ideal model)")


def differentiator(R, C, A, f):
    _pos("R, C, A and f", R, C, A, f)
    g = TWO_PI * f * R * C
    return _out(gain=g, Vout_peak=g * A, gain_dB=_db(g))


def _ref_differentiator(R, C, A, f):
    s = complex(0.0, TWO_PI * f)
    v, _ = _mna(3, [("V", 1, 0, 1.0), ("C", 1, 2, C), ("R", 2, 3, R), ("OA", 3, 0, 2)], s)
    g = abs(v[3])
    return {"gain": g, "Vout_peak": g * A, "gain_dB": 20.0 * math.log10(g)}


_add("elx_opamp_differentiator",
     name="Op-amp differentiator: gain and output amplitude", topic="op-amps / integrator and differentiator",
     purpose="The gain |H| = 2 pi f R C of an ideal inverting differentiator at a frequency and the peak output for a sinusoidal input of given amplitude.",
     inputs=[_phys("R", "resistance", "feedback resistor"), _phys("C", "capacitance", "input capacitor", show=("nF", 1e9)),
             _phys("A", "voltage", "peak input amplitude"), _phys("f", "frequency", "input frequency")],
     outputs=[_o("gain", "number", "gain magnitude"), _o("Vout_peak", "voltage", "peak output voltage"), _o("gain_dB", "number", "gain in dB")],
     primary="Vout_peak", conditions=["ideal op-amp (no high-frequency limiting)"],
     method="|H| = 2 pi f R C; Vout,peak = |H| A",
     latex=r"|H(j\omega)| = \omega RC",
     run=differentiator, reference=_ref_differentiator,
     wrong=lambda R, C, A, f: (lambda g: _out(gain=g, Vout_peak=g * A, gain_dB=_db(g)))(1.0 / (TWO_PI * f * R * C)),
     rand=lambda rng: {"R": _sig(_lu(rng, 1e3, 1e6), 3), "C": _sig(_lu(rng, 1e-10, 1e-6), 3), "A": _sig(rng.uniform(0.01, 2.0), 3),
                       "f": _sig(_lu(rng, 1.0, 1e5), 3)},
     phrases=("Op-amp differentiator", "Differentiator output"),
     examples=[_ex({"R": 10e3, "C": 10e-9, "A": 1.0, "f": 1000.0}), _ex({"R": 100e3, "C": 1e-9, "A": 0.5, "f": 500.0})],
     level="intermediate")


def closed_loop_bw(GBW, config, Rf, R1):
    _pos("GBW, Rf and R1", GBW, Rf, R1)
    _need(config in ("inverting", "non-inverting"), "config: inverting or non-inverting")
    NG = 1.0 + Rf / R1
    G0 = -Rf / R1 if config == "inverting" else NG
    return _out(G0=G0, noise_gain=NG, f_3dB=GBW / NG)


def _ref_closed_loop_bw(GBW, config, Rf, R1):
    # single-pole op-amp A(s) = 2 pi GBW / s (constant gain-bandwidth product) in the netlist; the -3 dB point
    def H(f):
        s = complex(0.0, TWO_PI * f)
        A = TWO_PI * GBW / s
        if config == "inverting":
            el = [("V", 1, 0, 1.0), ("R", 1, 2, R1), ("R", 2, 3, Rf), ("E", 3, 0, 0, 2, A)]
        else:
            el = [("V", 1, 0, 1.0), ("R", 2, 0, R1), ("R", 2, 3, Rf), ("E", 3, 0, 1, 2, A)]
        v, _ = _mna(3, el, s)
        return v[3]
    if config == "inverting":
        vi, _ = _mna(3, [("V", 1, 0, 1.0), ("R", 1, 2, R1), ("R", 2, 3, Rf), ("OA", 3, 0, 2)])
    else:
        vi, _ = _mna(3, [("V", 1, 0, 1.0), ("R", 2, 0, R1), ("R", 2, 3, Rf), ("OA", 3, 1, 2)])
    G0 = vi[3].real
    f3 = _brentq(lambda f: abs(H(f)) - abs(G0) / math.sqrt(2.0), GBW * 1e-12, GBW * 10.0)
    return {"G0": G0, "noise_gain": 1.0 + Rf / R1, "f_3dB": f3}


_add("elx_closed_loop_bandwidth",
     name="Closed-loop bandwidth from the gain-bandwidth product", topic="op-amps / frequency limits",
     purpose="The closed-loop gain, the noise gain 1 + Rf/R1 and the -3 dB bandwidth GBW/(noise gain) of an inverting or non-inverting amplifier built with a single-pole op-amp.",
     inputs=[_phys("GBW", "frequency", "gain-bandwidth product (unity-gain frequency)"),
             _choice("config", ("inverting", "non-inverting"), "inverting or non-inverting amplifier"),
             _phys("Rf", "resistance", "feedback resistor"), _phys("R1", "resistance", "input (or ground) resistor")],
     outputs=[_o("G0", "number", "low-frequency closed-loop gain"), _o("noise_gain", "number", "noise gain 1 + Rf/R1"),
              _o("f_3dB", "frequency", "closed-loop -3 dB bandwidth")],
     primary="f_3dB", conditions=["single-pole op-amp with open-loop gain much larger than the closed-loop gain"],
     method="f_3dB = GBW/(1 + Rf/R1) for both configurations",
     latex=r"f_{3dB} = \frac{\text{GBW}}{1 + R_f/R_1}",
     run=closed_loop_bw, reference=_ref_closed_loop_bw,
     wrong=lambda GBW, config, Rf, R1: (lambda r: dict(r, f_3dB=_r(GBW / abs(r["G0"]))))(closed_loop_bw(GBW, config, Rf, R1)),
     rand=lambda rng: {"GBW": _sig(_lu(rng, 1e5, 1e8), 3), "config": "inverting" if rng.random() < 0.8 else "non-inverting",
                       "Rf": _sig(_lu(rng, 1e3, 1e6), 3), "R1": _sig(_lu(rng, 1e3, 1e5), 3)},
     phrases=("Closed-loop bandwidth", "Op-amp closed-loop bandwidth"),
     examples=[_ex({"GBW": 1e6, "config": "non-inverting", "Rf": 90e3, "R1": 10e3}, {"G0": 10.0, "f_3dB": 100000.0}),
               _ex({"GBW": 10e6, "config": "inverting", "Rf": 100e3, "R1": 10e3})],
     level="intermediate",
     extra_q=[{"q": "Closed-loop bandwidth: GBW = 1 MHz, config = non-inverting, Rf = 90 kohm, R1 = 10 kohm", "like": 0}])


def slew_rate(SR, Vp, f):
    _pos("SR, Vp and f", SR, Vp, f)
    need = TWO_PI * f * Vp
    return _out(f_max=SR / (TWO_PI * Vp), Vp_max=SR / (TWO_PI * f), slope_needed=need,
                limited="slew-limited" if need > SR else "not slew-limited")


def _ref_slew_rate(SR, Vp, f):
    # the steepest slope of Vp sin(2 pi f t) (at the zero crossing, by a complex-step derivative); the frequency or
    # amplitude at which it equals SR by root finding
    slope = lambda ff, vp: (vp * complex(math.sin(0.0), 0.0) + 0j).real + (vp * __import__("cmath").sin(TWO_PI * ff * 1e-30j)).imag / 1e-30
    fmax = _brentq(lambda ff: slope(ff, Vp) - SR, 0.0, 1e3 * SR / Vp)
    vmax = _brentq(lambda vp: slope(f, vp) - SR, 0.0, 1e3 * SR / f)
    need = slope(f, Vp)
    return {"f_max": fmax, "Vp_max": vmax, "slope_needed": need, "limited": "slew-limited" if need > SR else "not slew-limited"}


_add("elx_slew_rate_limit",
     name="Slew-rate limit: full-power bandwidth", topic="op-amps / frequency limits",
     purpose="The full-power bandwidth SR/(2 pi Vp) of an op-amp, the largest undistorted amplitude at a frequency, the slope a sine wave needs and whether it is slew-limited.",
     inputs=[_phys("SR", "voltage_slew_rate", "slew rate"), _phys("Vp", "voltage", "peak output amplitude"), _phys("f", "frequency", "signal frequency")],
     outputs=[_o("f_max", "frequency", "full-power bandwidth at Vp"), _o("Vp_max", "voltage", "largest undistorted amplitude at f"),
              _o("slope_needed", "voltage_slew_rate", "maximum slope of the sine, 2 pi f Vp"), _o("limited", "number", "slew-limited or not slew-limited")],
     primary="f_max", conditions=["sinusoidal output"],
     method="f_max = SR/(2 pi Vp); Vp,max = SR/(2 pi f); slope = 2 pi f Vp",
     latex=r"f_{max} = \frac{SR}{2\pi V_p}",
     run=slew_rate, reference=_ref_slew_rate,
     wrong=lambda SR, Vp, f: dict(slew_rate(SR, Vp, f), f_max=_r(SR / Vp)),
     rand=lambda rng: {"SR": _sig(_lu(rng, 1e5, 1e8), 3), "Vp": _sig(rng.uniform(0.5, 15.0), 3), "f": _sig(_lu(rng, 1e3, 1e7), 3)},
     phrases=("Slew-rate limit", "Full-power bandwidth"),
     examples=[_ex({"SR": 0.5e6, "Vp": 10.0, "f": 20e3}, {"limited": "slew-limited"}), _ex({"SR": 13e6, "Vp": 5.0, "f": 100e3}, {"limited": "not slew-limited"})],
     level="beginner",
     weak="the sine's steepest slope by a complex-step derivative and the limits by root finding")


def opamp_offset(Vos, IB, Ios, Rf, R1, Rp):
    _fin(Vos, IB, Ios)
    _pos("Rf and R1", Rf, R1)
    _nonneg("Rp", Rp)
    NG = 1.0 + Rf / R1
    IB1, IB2 = IB + Ios / 2.0, IB - Ios / 2.0
    vv = Vos * NG
    vb = IB1 * Rf - IB2 * Rp * NG
    return _out(Vout_Vos=vv, Vout_bias=vb, Vout_total=vv + vb, Rp_optimal=_par(R1, Rf))


def _ref_opamp_offset(Vos, IB, Ios, Rf, R1, Rp):
    def out(vos, i1, i2):
        el = [("R", 1, 0, Rp) if Rp > 0 else ("V", 1, 0, 0.0), ("V", 4, 1, vos), ("OA", 3, 4, 2), ("R", 2, 0, R1), ("R", 2, 3, Rf),
              ("I", 1, 0, i2), ("I", 2, 0, i1)]
        v, _ = _mna(4, el)
        return v[3].real
    IB1, IB2 = IB + Ios / 2.0, IB - Ios / 2.0
    vt, _ = _mna(1, [("R", 1, 0, R1), ("R", 1, 0, Rf), ("I", 0, 1, 1.0)])
    return {"Vout_Vos": out(Vos, 0.0, 0.0), "Vout_bias": out(0.0, IB1, IB2), "Vout_total": out(Vos, IB1, IB2), "Rp_optimal": vt[1].real}


_add("elx_opamp_dc_offset",
     name="Op-amp output offset from input offset voltage and bias currents", topic="op-amps / non-ideal",
     purpose="The DC output error of an inverting or non-inverting amplifier (input grounded) caused by the input offset voltage and the input bias and offset currents, and the compensating resistor that cancels the bias-current error.",
     inputs=[_phys("Vos", "voltage", "input offset voltage (in series with the + input)", show=("mV", 1e3)),
             _phys("IB", "current", "average input bias current", show=("nA", 1e9)), _phys("Ios", "current", "input offset current IB- minus IB+", show=("nA", 1e9)),
             _phys("Rf", "resistance", "feedback resistor"), _phys("R1", "resistance", "resistor from the inverting input to ground"),
             _phys("Rp", "resistance", "resistor from the + input to ground (0 if none)")],
     outputs=[_o("Vout_Vos", "voltage", "output error from Vos"), _o("Vout_bias", "voltage", "output error from the bias currents"),
              _o("Vout_total", "voltage", "total output error"), _o("Rp_optimal", "resistance", "bias-cancelling Rp = R1 || Rf")],
     primary="Vout_total", conditions=["currents flow into the inputs; IB- = IB + Ios/2, IB+ = IB - Ios/2"],
     method="Vout = Vos (1 + Rf/R1) + IB- Rf - IB+ Rp (1 + Rf/R1)",
     latex=r"V_{out} = \left(V_{os} - I_{B+}R_p\right)\left(1 + \frac{R_f}{R_1}\right) + I_{B-}R_f",
     run=opamp_offset, reference=_ref_opamp_offset,
     wrong=lambda Vos, IB, Ios, Rf, R1, Rp: (lambda r: dict(r, Vout_bias=_r((IB + Ios / 2) * Rf), Vout_total=_r(r["Vout_Vos"] + (IB + Ios / 2) * Rf)))(
         opamp_offset(Vos, IB, Ios, Rf, R1, Rp)),
     rand=lambda rng: {"Vos": _sig(rng.uniform(-5e-3, 5e-3), 3), "IB": _sig(_lu(rng, 1e-9, 1e-6), 3), "Ios": _sig(rng.uniform(-0.2, 0.2), 3) * 1e-8,
                       "Rf": _sig(_lu(rng, 1e4, 1e6), 3), "R1": _sig(_lu(rng, 1e3, 1e5), 3), "Rp": _sig(_lu(rng, 1e3, 1e5), 3)},
     phrases=("Op-amp output offset", "Output offset voltage"),
     examples=[_ex({"Vos": 2e-3, "IB": 100e-9, "Ios": 10e-9, "Rf": 100e3, "R1": 10e3, "Rp": 0.0}),
               _ex({"Vos": 1e-3, "IB": 80e-9, "Ios": 20e-9, "Rf": 100e3, "R1": 10e3, "Rp": 9100.0})],
     level="advanced")


def schmitt(Rf, R1, VsatH, VsatL, Vref):
    _pos("Rf and R1", Rf, R1)
    _fin(VsatH, VsatL, Vref)
    _need(VsatH > VsatL, "VsatH must exceed VsatL")
    th = (VsatH * R1 + Vref * Rf) / (R1 + Rf)
    tl = (VsatL * R1 + Vref * Rf) / (R1 + Rf)
    return _out(V_TH=th, V_TL=tl, hysteresis=th - tl, center=(th + tl) / 2.0)


def _ref_schmitt(Rf, R1, VsatH, VsatL, Vref):
    vp = lambda vo: _mna(3, [("V", 3, 0, vo), ("R", 3, 2, Rf), ("R", 2, 1, R1), ("V", 1, 0, Vref)])[0][2].real
    th, tl = vp(VsatH), vp(VsatL)
    return {"V_TH": th, "V_TL": tl, "hysteresis": th - tl, "center": (th + tl) / 2.0}


_add("elx_schmitt_trigger",
     name="Inverting Schmitt trigger thresholds", topic="op-amps / comparators",
     purpose="The upper and lower switching thresholds, the hysteresis width and the centre of an inverting op-amp Schmitt trigger whose positive feedback divider (Rf from the output, R1 to a reference) sets the + input.",
     inputs=[_phys("Rf", "resistance", "resistor from the output to the + input"), _phys("R1", "resistance", "resistor from the + input to Vref"),
             _phys("VsatH", "voltage", "high output saturation voltage"), _phys("VsatL", "voltage", "low output saturation voltage (negative for a split supply)"),
             _phys("Vref", "voltage", "reference voltage (0 V for ground)")],
     outputs=[_o("V_TH", "voltage", "upper threshold"), _o("V_TL", "voltage", "lower threshold"), _o("hysteresis", "voltage", "hysteresis width"),
              _o("center", "voltage", "centre of the hysteresis band")],
     primary="hysteresis", conditions=["VsatH > VsatL", "input on the inverting terminal"],
     method="V_T = (Vsat R1 + Vref Rf)/(R1 + Rf); hysteresis = (VsatH - VsatL) R1/(R1 + Rf)",
     latex=r"V_{TH,TL} = \frac{V_{sat}R_1 + V_{ref}R_f}{R_1 + R_f}",
     run=schmitt, reference=_ref_schmitt,
     wrong=lambda Rf, R1, VsatH, VsatL, Vref: schmitt(R1, Rf, VsatH, VsatL, Vref),
     rand=lambda rng: (lambda Rf, R1: {"Rf": Rf, "R1": _sig(Rf * rng.uniform(0.05, 0.6), 3) if rng.random() < 0.9 else R1, "VsatH": _sig(rng.uniform(3.0, 15.0), 3),
                                       "VsatL": _sig(-rng.uniform(0.0, 15.0), 3), "Vref": _sig(rng.uniform(-2.0, 3.0), 3)})(_sig(_lu(rng, 1e4, 1e6), 3), _sig(_lu(rng, 1e3, 1e5), 3)),
     phrases=("Schmitt trigger thresholds", "Inverting Schmitt trigger"),
     examples=[_ex({"Rf": 90e3, "R1": 10e3, "VsatH": 12.0, "VsatL": -12.0, "Vref": 0.0}, {"V_TH": 1.2, "V_TL": -1.2, "hysteresis": 2.4}),
               _ex({"Rf": 100e3, "R1": 22e3, "VsatH": 4.8, "VsatL": 0.1, "Vref": 2.5})],
     level="intermediate")


def neg_feedback(A, beta, fH):
    _pos("A, beta and fH", A, beta, fH)
    L = A * beta
    D = 1.0 + L
    return _out(Af=A / D, loop_gain=L, desensitivity=D, desensitivity_dB=_db(D), fH_closed=fH * D)


def _ref_neg_feedback(A, beta, fH):
    Acl = lambda f: abs((A / complex(1.0, f / fH)) / (1.0 + beta * A / complex(1.0, f / fH)))
    Af = Acl(0.0)
    f3 = _brentq(lambda f: Acl(f) - Af / math.sqrt(2.0), 0.0, fH * (1.0 + A * beta) * 10.0)
    return {"Af": Af, "loop_gain": A * beta, "desensitivity": A / Af, "desensitivity_dB": 20.0 * math.log10(A / Af), "fH_closed": f3}


_add("elx_negative_feedback",
     name="Negative feedback: closed-loop gain and bandwidth extension", topic="op-amps / feedback",
     purpose="The closed-loop gain A/(1 + A beta), the loop gain, the desensitivity factor (also in dB) and the extended bandwidth of a single-pole amplifier with negative feedback.",
     inputs=[_numin("A", "open-loop gain (a ratio)"), _numin("beta", "feedback factor (a ratio)"), _phys("fH", "frequency", "open-loop -3 dB bandwidth")],
     outputs=[_o("Af", "number", "closed-loop gain"), _o("loop_gain", "number", "loop gain A beta"), _o("desensitivity", "number", "1 + A beta"),
              _o("desensitivity_dB", "number", "amount of feedback in dB"), _o("fH_closed", "frequency", "closed-loop bandwidth")],
     primary="Af", conditions=["single-pole open-loop response", "frequency-independent beta"],
     method="Af = A/(1 + A beta); fH,f = fH (1 + A beta)",
     latex=r"A_f = \frac{A}{1 + A\beta},\quad f_{H,f} = f_H(1 + A\beta)",
     run=neg_feedback, reference=_ref_neg_feedback,
     wrong=lambda A, beta, fH: (lambda D: _out(Af=A / D, loop_gain=A * beta, desensitivity=D, desensitivity_dB=_db(D), fH_closed=fH * D))(1.0 + beta),
     rand=lambda rng: {"A": _sig(_lu(rng, 1e2, 1e6), 3), "beta": _sig(_lu(rng, 1e-3, 1.0), 3), "fH": _sig(_lu(rng, 1.0, 1e5), 3)},
     phrases=("Negative feedback amplifier", "Feedback amplifier gain"),
     examples=[_ex({"A": 1e4, "beta": 0.01, "fH": 1000.0}, {"loop_gain": 100.0, "desensitivity": 101.0, "fH_closed": 101000.0}),
               _ex({"A": 2e5, "beta": 0.1, "fH": 10.0})],
     level="intermediate")


def phase_margin(A0, f1, f2, beta):
    _pos("A0, f1, f2 and beta", A0, f1, f2, beta)
    K = A0 * beta
    _need(K > 1.0, "the loop gain A0 beta must exceed 1 for a crossover to exist")
    a = 1.0 / (f1 * f1 * f2 * f2)
    b = 1.0 / (f1 * f1) + 1.0 / (f2 * f2)
    u = 2.0 * (K * K - 1.0) / (b + math.sqrt(b * b + 4.0 * a * (K * K - 1.0)))
    fc = math.sqrt(u)
    pm = 180.0 - math.degrees(math.atan(fc / f1)) - math.degrees(math.atan(fc / f2))
    return _out(f_c=fc, PM_deg=pm, Af=A0 / (1.0 + K))


def _ref_phase_margin(A0, f1, f2, beta):
    from scipy import signal
    import numpy as np
    w1, w2 = TWO_PI * f1, TWO_PI * f2
    sys_ = signal.lti([A0 * beta], np.polymul([1.0 / w1, 1.0], [1.0 / w2, 1.0]))
    mag = lambda f: abs(signal.freqresp(sys_, [TWO_PI * f])[1][0])
    fc = _brentq(lambda f: math.log(mag(f)), 0.0, A0 * beta * max(f1, f2) * 2.0)
    h = signal.freqresp(sys_, [TWO_PI * fc])[1][0]
    ph = math.degrees(math.atan2(h.imag, h.real))
    ph = ph - 360.0 if ph > 0 else ph
    return {"f_c": fc, "PM_deg": 180.0 + ph, "Af": A0 / (1.0 + A0 * beta)}


_add("elx_two_pole_phase_margin",
     name="Phase margin of a two-pole amplifier with feedback", topic="op-amps / stability",
     purpose="The loop-gain crossover frequency and the phase margin of an amplifier with two real poles and a frequency-independent feedback factor, and its closed-loop DC gain.",
     inputs=[_numin("A0", "DC open-loop gain (a ratio)"), _phys("f1", "frequency", "first pole frequency"),
             _phys("f2", "frequency", "second pole frequency"), _numin("beta", "feedback factor")],
     outputs=[_o("f_c", "frequency", "loop-gain crossover frequency"), _o("PM_deg", "number", "phase margin in degrees"),
              _o("Af", "number", "closed-loop DC gain")],
     primary="PM_deg", conditions=["A0 beta > 1", "two real left-half-plane poles"],
     method="|A0 beta|^2 = (1 + (fc/f1)^2)(1 + (fc/f2)^2) solved for fc^2; PM = 180 - atan(fc/f1) - atan(fc/f2)",
     latex=r"PM = 180^\circ - \arctan\frac{f_c}{f_1} - \arctan\frac{f_c}{f_2}",
     run=phase_margin, reference=_ref_phase_margin,
     wrong=lambda A0, f1, f2, beta: (lambda r: dict(r, PM_deg=_r(180.0 - math.degrees(math.atan(r["f_c"] / f2)))))(phase_margin(A0, f1, f2, beta)),
     rand=lambda rng: (lambda f1: {"A0": _sig(_lu(rng, 1e2, 1e5), 3), "f1": f1, "f2": _sig(f1 * _lu(rng, 10.0, 1e5), 3), "beta": _sig(_lu(rng, 0.01, 1.0), 3)})(
         _sig(_lu(rng, 1.0, 1e4), 3)),
     phrases=("Two-pole phase margin", "Phase margin with feedback"),
     examples=[_ex({"A0": 1e5, "f1": 10.0, "f2": 1e6, "beta": 0.1}), _ex({"A0": 2e4, "f1": 100.0, "f2": 2e5, "beta": 1.0})],
     level="expert")


# ================================================================================================ ACTIVE FILTERS
def _lp_coeffs(H, w, K):
    """Reference helper: from a second-order low-pass response H(jw) = K/(1 - a w^2 + j b w), the coefficients a, b."""
    q = K / H
    return (1.0 - q.real) / (w * w), q.imag / w


def first_order_lp(R1, Rf, C, f):
    _pos("R1, Rf, C and f", R1, Rf, C, f)
    H0 = -Rf / R1
    fc = 1.0 / (TWO_PI * Rf * C)
    g = abs(H0) / math.sqrt(1.0 + (f / fc) ** 2)
    return _out(H0=H0, fc=fc, gain_f=g, gain_dB=_db(g))


def _ref_first_order_lp(R1, Rf, C, f):
    H = lambda ff: _mna(3, [("V", 1, 0, 1.0), ("R", 1, 2, R1), ("R", 2, 3, Rf), ("C", 2, 3, C), ("OA", 3, 0, 2)], complex(0.0, TWO_PI * ff))[0][3]
    H0 = H(0.0).real
    fc = _brentq(lambda ff: abs(H(ff)) - abs(H0) / math.sqrt(2.0), 0.0, 1e3 / (Rf * C))
    g = abs(H(f))
    return {"H0": H0, "fc": fc, "gain_f": g, "gain_dB": 20.0 * math.log10(g)}


_add("elx_first_order_active_lowpass",
     name="First-order active low-pass filter (inverting)", topic="active filters / first order",
     purpose="The passband gain -Rf/R1, the cutoff frequency 1/(2 pi Rf C) and the gain at a frequency of an inverting op-amp low-pass filter with C across the feedback resistor.",
     inputs=[_phys("R1", "resistance", "input resistor"), _phys("Rf", "resistance", "feedback resistor"),
             _phys("C", "capacitance", "capacitor across Rf", show=("nF", 1e9)), _phys("f", "frequency", "frequency of interest")],
     outputs=[_o("H0", "number", "passband gain"), _o("fc", "frequency", "cutoff frequency"), _o("gain_f", "number", "gain magnitude at f"),
              _o("gain_dB", "number", "gain at f in dB")],
     primary="fc", conditions=["ideal op-amp"],
     method="H(s) = -(Rf/R1)/(1 + s Rf C); fc = 1/(2 pi Rf C)",
     latex=r"H(j\omega) = -\frac{R_f/R_1}{1 + j\omega R_fC},\quad f_c = \frac{1}{2\pi R_fC}",
     run=first_order_lp, reference=_ref_first_order_lp,
     wrong=lambda R1, Rf, C, f: (lambda fc: _out(H0=-Rf / R1, fc=fc, gain_f=Rf / R1 / math.sqrt(1 + (f / fc) ** 2), gain_dB=_db(Rf / R1 / math.sqrt(1 + (f / fc) ** 2))))(1.0 / (Rf * C)),
     rand=lambda rng: {"R1": _sig(_lu(rng, 1e3, 1e5), 3), "Rf": _sig(_lu(rng, 1e3, 1e6), 3), "C": _sig(_lu(rng, 1e-10, 1e-6), 3), "f": _sig(_lu(rng, 10.0, 1e5), 3)},
     phrases=("First-order active low-pass filter", "Active low-pass filter"),
     examples=[_ex({"R1": 10e3, "Rf": 100e3, "C": 1.59e-9, "f": 1000.0}), _ex({"R1": 4700.0, "Rf": 47e3, "C": 10e-9, "f": 500.0})],
     level="beginner")


def sk_lowpass(R1, R2, C1, C2, K):
    _pos("R1, R2, C1, C2 and K", R1, R2, C1, C2, K)
    a = R1 * R2 * C1 * C2
    b = R1 * C2 + R2 * C2 + R1 * C1 * (1.0 - K)
    _need(b > 0, "the gain K makes the filter unstable (the s coefficient is not positive)")
    return _out(f0=1.0 / (TWO_PI * math.sqrt(a)), Q=math.sqrt(a) / b, H0=K, peak_gain=K * math.sqrt(a) / b)


def _ref_sk_lowpass(R1, R2, C1, C2, K):
    w = 0.7 / math.sqrt(R1 * R2 * C1 * C2)
    el = [("V", 1, 0, 1.0), ("R", 1, 2, R1), ("R", 2, 3, R2), ("C", 3, 0, C2), ("C", 2, 4, C1), ("E", 4, 0, 3, 0, K)]
    H = _mna(4, el, complex(0.0, w))[0][4]
    H0 = _mna(4, el, 0.0)[0][4].real
    a, b = _lp_coeffs(H, w, H0)
    return {"f0": 1.0 / (TWO_PI * math.sqrt(a)), "Q": math.sqrt(a) / b, "H0": H0, "peak_gain": H0 * math.sqrt(a) / b}


_add("elx_sallen_key_lowpass",
     name="Sallen-Key low-pass filter: f0, Q and gain", topic="active filters / Sallen-Key",
     purpose="The natural frequency, quality factor, passband gain and the gain at f0 (K Q) of a second-order Sallen-Key low-pass filter with any components and amplifier gain K.",
     inputs=[_phys("R1", "resistance", "first series resistor"), _phys("R2", "resistance", "second series resistor"),
             _phys("C1", "capacitance", "capacitor from the middle node to the output (feedback)", show=("nF", 1e9)),
             _phys("C2", "capacitance", "capacitor from the amplifier input to ground", show=("nF", 1e9)),
             _numin("K", "amplifier gain (1 for a unity-gain buffer)")],
     outputs=[_o("f0", "frequency", "natural frequency"), _o("Q", "number", "quality factor"), _o("H0", "number", "passband gain"),
              _o("peak_gain", "number", "gain at f0, K Q")],
     primary="Q", conditions=["stable: R1 C2 + R2 C2 + R1 C1 (1 - K) > 0", "ideal amplifier"],
     method="w0 = 1/sqrt(R1 R2 C1 C2); Q = sqrt(R1 R2 C1 C2)/(R1 C2 + R2 C2 + R1 C1 (1 - K))",
     latex=r"\omega_0 = \frac{1}{\sqrt{R_1R_2C_1C_2}},\quad Q = \frac{\sqrt{R_1R_2C_1C_2}}{R_1C_2 + R_2C_2 + R_1C_1(1-K)}",
     run=sk_lowpass, reference=_ref_sk_lowpass,
     wrong=lambda R1, R2, C1, C2, K: (lambda a, b: _out(f0=1 / (TWO_PI * math.sqrt(a)), Q=math.sqrt(a) / b, H0=K, peak_gain=K * math.sqrt(a) / b))(
         R1 * R2 * C1 * C2, R1 * C2 + R2 * C2),
     rand=lambda rng: {"R1": _sig(_lu(rng, 1e3, 1e5), 3), "R2": _sig(_lu(rng, 1e3, 1e5), 3), "C1": _sig(_lu(rng, 1e-10, 1e-6), 3),
                       "C2": _sig(_lu(rng, 1e-10, 1e-6), 3), "K": 1.0 if rng.random() < 0.3 else _sig(rng.uniform(1.0, 2.5), 3)},
     phrases=("Sallen-Key low-pass filter", "Sallen-Key lowpass"),
     examples=[_ex({"R1": 10e3, "R2": 10e3, "C1": 20e-9, "C2": 10e-9, "K": 1.0}), _ex({"R1": 15e3, "R2": 15e3, "C1": 10e-9, "C2": 10e-9, "K": 1.586})],
     level="advanced")


def sk_highpass(C1, C2, R1, R2, K):
    _pos("C1, C2, R1, R2 and K", C1, C2, R1, R2, K)
    a = R1 * R2 * C1 * C2
    b = R1 * (C1 + C2) + R2 * C2 * (1.0 - K)
    _need(b > 0, "the gain K makes the filter unstable (the s coefficient is not positive)")
    return _out(f0=1.0 / (TWO_PI * math.sqrt(a)), Q=math.sqrt(a) / b, H_inf=K)


def _ref_sk_highpass(C1, C2, R1, R2, K):
    w = 0.7 / math.sqrt(R1 * R2 * C1 * C2)
    el = [("V", 1, 0, 1.0), ("C", 1, 2, C1), ("C", 2, 3, C2), ("R", 3, 0, R2), ("R", 2, 4, R1), ("E", 4, 0, 3, 0, K)]
    H = _mna(4, el, complex(0.0, w))[0][4]
    q = K / H                                              # K/H = 1 - 1/(a w^2) - j b/(a w)
    a = 1.0 / ((1.0 - q.real) * w * w)
    b = -q.imag * a * w
    Hinf = abs(_mna(4, el, complex(0.0, 1e7 / math.sqrt(a)))[0][4])
    return {"f0": 1.0 / (TWO_PI * math.sqrt(a)), "Q": math.sqrt(a) / b, "H_inf": Hinf}


_add("elx_sallen_key_highpass",
     name="Sallen-Key high-pass filter: f0, Q and gain", topic="active filters / Sallen-Key",
     purpose="The natural frequency, quality factor and high-frequency gain of a second-order Sallen-Key high-pass filter with any components and amplifier gain K.",
     inputs=[_phys("C1", "capacitance", "first series capacitor", show=("nF", 1e9)), _phys("C2", "capacitance", "second series capacitor", show=("nF", 1e9)),
             _phys("R1", "resistance", "resistor from the middle node to the output (feedback)"),
             _phys("R2", "resistance", "resistor from the amplifier input to ground"), _numin("K", "amplifier gain")],
     outputs=[_o("f0", "frequency", "natural frequency"), _o("Q", "number", "quality factor"), _o("H_inf", "number", "high-frequency gain")],
     primary="Q", conditions=["stable: R1 (C1 + C2) + R2 C2 (1 - K) > 0"],
     method="w0 = 1/sqrt(R1 R2 C1 C2); Q = sqrt(R1 R2 C1 C2)/(R1 (C1 + C2) + R2 C2 (1 - K))",
     latex=r"Q = \frac{\sqrt{R_1R_2C_1C_2}}{R_1(C_1+C_2) + R_2C_2(1-K)}",
     run=sk_highpass, reference=_ref_sk_highpass,
     wrong=lambda C1, C2, R1, R2, K: (lambda a, b: _out(f0=1 / (TWO_PI * math.sqrt(a)), Q=math.sqrt(a) / b, H_inf=K))(R1 * R2 * C1 * C2, R2 * (C1 + C2) + R1 * C2 * (1.0 - K)),
     rand=lambda rng: {"C1": _sig(_lu(rng, 1e-10, 1e-6), 3), "C2": _sig(_lu(rng, 1e-10, 1e-6), 3), "R1": _sig(_lu(rng, 1e3, 1e5), 3),
                       "R2": _sig(_lu(rng, 1e3, 1e5), 3), "K": 1.0 if rng.random() < 0.3 else _sig(rng.uniform(1.0, 2.5), 3)},
     phrases=("Sallen-Key high-pass filter", "Sallen-Key highpass"),
     examples=[_ex({"C1": 10e-9, "C2": 10e-9, "R1": 11.25e3, "R2": 22.5e3, "K": 1.0}), _ex({"C1": 22e-9, "C2": 22e-9, "R1": 10e3, "R2": 10e3, "K": 2.0})],
     level="advanced")


def mfb_lowpass(R1, R2, R3, C1, C2):
    _pos("R1, R2, R3, C1 and C2", R1, R2, R3, C1, C2)
    a = R2 * R3 * C1 * C2
    b = C2 * (R2 + R3 + R2 * R3 / R1)
    return _out(f0=1.0 / (TWO_PI * math.sqrt(a)), Q=math.sqrt(a) / b, H0=-R2 / R1)


def _ref_mfb_lowpass(R1, R2, R3, C1, C2):
    w = 0.7 / math.sqrt(R2 * R3 * C1 * C2)
    el = [("V", 1, 0, 1.0), ("R", 1, 2, R1), ("R", 2, 4, R2), ("R", 2, 3, R3), ("C", 2, 0, C1), ("C", 3, 4, C2), ("OA", 4, 0, 3)]
    H0 = _mna(4, el, 0.0)[0][4].real
    a, b = _lp_coeffs(_mna(4, el, complex(0.0, w))[0][4], w, H0)
    return {"f0": 1.0 / (TWO_PI * math.sqrt(a)), "Q": math.sqrt(a) / b, "H0": H0}


_add("elx_mfb_lowpass",
     name="Multiple-feedback (MFB) low-pass filter", topic="active filters / multiple feedback",
     purpose="The natural frequency, quality factor and inverting passband gain -R2/R1 of a second-order multiple-feedback low-pass filter.",
     inputs=[_phys("R1", "resistance", "input resistor"), _phys("R2", "resistance", "feedback resistor (middle node to output)"),
             _phys("R3", "resistance", "resistor from the middle node to the inverting input"),
             _phys("C1", "capacitance", "capacitor from the middle node to ground", show=("nF", 1e9)),
             _phys("C2", "capacitance", "capacitor from the inverting input to the output", show=("nF", 1e9))],
     outputs=[_o("f0", "frequency", "natural frequency"), _o("Q", "number", "quality factor"), _o("H0", "number", "passband gain")],
     primary="Q", conditions=["ideal op-amp"],
     method="w0 = 1/sqrt(R2 R3 C1 C2); Q = sqrt(R2 R3 C1 C2)/(C2 (R2 + R3 + R2 R3/R1)); H0 = -R2/R1",
     latex=r"Q = \frac{\sqrt{R_2R_3C_1C_2}}{C_2\left(R_2 + R_3 + R_2R_3/R_1\right)},\quad H_0 = -\frac{R_2}{R_1}",
     run=mfb_lowpass, reference=_ref_mfb_lowpass,
     wrong=lambda R1, R2, R3, C1, C2: (lambda a: _out(f0=1 / (TWO_PI * math.sqrt(a)), Q=math.sqrt(a) / (C2 * (R2 + R3)), H0=-R2 / R1))(R2 * R3 * C1 * C2),
     rand=lambda rng: {"R1": _sig(_lu(rng, 1e3, 1e5), 3), "R2": _sig(_lu(rng, 1e3, 1e5), 3), "R3": _sig(_lu(rng, 1e3, 1e5), 3),
                       "C1": _sig(_lu(rng, 1e-9, 1e-6), 3), "C2": _sig(_lu(rng, 1e-10, 1e-7), 3)},
     phrases=("MFB low-pass filter", "Multiple-feedback low-pass filter"),
     examples=[_ex({"R1": 10e3, "R2": 10e3, "R3": 5000.0, "C1": 47e-9, "C2": 4.7e-9}), _ex({"R1": 4700.0, "R2": 47e3, "R3": 10e3, "C1": 100e-9, "C2": 1e-9})],
     level="advanced")


def mfb_bandpass(R1, R2, R3, C1, C2):
    _pos("R1, R2, R3, C1 and C2", R1, R2, R3, C1, C2)
    w0 = math.sqrt((R1 + R2) / (R1 * R2 * R3 * C1 * C2))
    Q = w0 * C1 * C2 * R3 / (C1 + C2)
    f0 = w0 / TWO_PI
    return _out(f0=f0, Q=Q, BW=f0 / Q, H_peak=-C1 * R3 / (R1 * (C1 + C2)))


def _ref_mfb_bandpass(R1, R2, R3, C1, C2):
    el = [("V", 1, 0, 1.0), ("R", 1, 2, R1), ("R", 2, 0, R2), ("C", 2, 3, C1), ("C", 2, 4, C2), ("R", 3, 4, R3), ("OA", 4, 0, 3)]
    H = lambda f: _mna(4, el, complex(0.0, TWO_PI * f))[0][4]
    f0 = _brentq(lambda f: H(f).imag, 1e-6, 1e12)                  # the response is real (180 degrees) at the centre
    hp = H(f0).real
    f1 = _brentq(lambda f: abs(H(f)) - abs(hp) / math.sqrt(2.0), 1e-6, f0)
    f2 = _brentq(lambda f: abs(H(f)) - abs(hp) / math.sqrt(2.0), f0, 1e12)
    return {"f0": f0, "Q": f0 / (f2 - f1), "BW": f2 - f1, "H_peak": hp}


_add("elx_mfb_bandpass",
     name="Multiple-feedback (MFB) band-pass filter", topic="active filters / multiple feedback",
     purpose="The centre frequency, quality factor, -3 dB bandwidth and peak (inverting) gain of a multiple-feedback band-pass filter.",
     inputs=[_phys("R1", "resistance", "input resistor"), _phys("R2", "resistance", "resistor from the middle node to ground"),
             _phys("R3", "resistance", "feedback resistor (inverting input to output)"),
             _phys("C1", "capacitance", "capacitor from the middle node to the inverting input", show=("nF", 1e9)),
             _phys("C2", "capacitance", "capacitor from the middle node to the output", show=("nF", 1e9))],
     outputs=[_o("f0", "frequency", "centre frequency"), _o("Q", "number", "quality factor"), _o("BW", "frequency", "-3 dB bandwidth"),
              _o("H_peak", "number", "gain at the centre frequency")],
     primary="f0", conditions=["ideal op-amp"],
     method="w0 = sqrt((R1 + R2)/(R1 R2 R3 C1 C2)); Q = w0 C1 C2 R3/(C1 + C2); H = -C1 R3/(R1 (C1 + C2))",
     latex=r"\omega_0 = \sqrt{\frac{R_1+R_2}{R_1R_2R_3C_1C_2}},\quad Q = \frac{\omega_0C_1C_2R_3}{C_1+C_2}",
     run=mfb_bandpass, reference=_ref_mfb_bandpass,
     wrong=lambda R1, R2, R3, C1, C2: (lambda w0: (lambda Q: _out(f0=w0 / TWO_PI, Q=Q, BW=w0 / TWO_PI / Q, H_peak=-C1 * R3 / (R1 * (C1 + C2))))(w0 * C1 * C2 * R3 / (C1 + C2)))(
         math.sqrt(1.0 / (R1 * R3 * C1 * C2))),
     rand=lambda rng: {"R1": _sig(_lu(rng, 1e3, 1e5), 3), "R2": _sig(_lu(rng, 100.0, 1e4), 3), "R3": _sig(_lu(rng, 1e4, 1e6), 3),
                       "C1": _sig(_lu(rng, 1e-9, 1e-6), 3), "C2": _sig(_lu(rng, 1e-9, 1e-6), 3)},
     phrases=("MFB band-pass filter", "Multiple-feedback band-pass filter"),
     examples=[_ex({"R1": 15.9e3, "R2": 1.33e3, "R3": 31.8e3, "C1": 10e-9, "C2": 10e-9}), _ex({"R1": 10e3, "R2": 470.0, "R3": 100e3, "C1": 22e-9, "C2": 22e-9})],
     level="advanced")


def sk_equal_design(fc, Q, C):
    _pos("fc, Q and C", fc, Q, C)
    _need(0.5 <= Q <= 20.0, "the equal-component design needs 0.5 <= Q <= 20 (gain K = 3 - 1/Q between 1 and 3)")
    R = 1.0 / (TWO_PI * fc * C)
    K = 3.0 - 1.0 / Q
    return _out(R=R, K=K, Rf_over_Rg=K - 1.0)


def _ref_sk_equal_design(fc, Q, C):
    def f0_q(R, K):
        r = _ref_sk_lowpass(R, R, C, C, K)
        return r["f0"], r["Q"]
    R = _brentq(lambda R: f0_q(R, 1.0)[0] - fc, 1e-3 / (fc * C), 1e3 / (fc * C))
    K = _brentq(lambda K: f0_q(R, K)[1] - Q, 0.5, 2.999999)
    return {"R": R, "K": K, "Rf_over_Rg": K - 1.0}


_add("elx_sallen_key_equal_component_design",
     name="Equal-component Sallen-Key low-pass design", topic="active filters / Sallen-Key",
     purpose="The resistor value and amplifier gain of an equal-component (R1 = R2 = R, C1 = C2 = C) Sallen-Key low-pass filter for a cutoff frequency and Q, with the non-inverting gain ratio Rf/Rg = K - 1.",
     inputs=[_phys("fc", "frequency", "natural (cutoff) frequency"), _numin("Q", "quality factor (0.7071 for Butterworth)"),
             _phys("C", "capacitance", "chosen capacitor value", show=("nF", 1e9))],
     outputs=[_o("R", "resistance", "resistor value"), _o("K", "number", "amplifier gain"), _o("Rf_over_Rg", "number", "gain resistor ratio Rf/Rg")],
     primary="R", conditions=["0.5 <= Q <= 20"],
     method="R = 1/(2 pi fc C); K = 3 - 1/Q; Rf/Rg = K - 1",
     latex=r"R = \frac{1}{2\pi f_cC},\quad K = 3 - \frac{1}{Q}",
     run=sk_equal_design, reference=_ref_sk_equal_design,
     wrong=lambda fc, Q, C: _out(R=1.0 / (TWO_PI * fc * C), K=3.0 - Q, Rf_over_Rg=2.0 - Q),
     rand=lambda rng: {"fc": _sig(_lu(rng, 10.0, 1e5), 3), "Q": _sig(rng.uniform(0.5, 5.0), 3), "C": _sig(_lu(rng, 1e-10, 1e-6), 3)},
     phrases=("Equal-component Sallen-Key design", "Sallen-Key design"),
     examples=[_ex({"fc": 1000.0, "Q": 0.7071, "C": 10e-9}), _ex({"fc": 5000.0, "Q": 2.0, "C": 2.2e-9})],
     level="advanced",
     weak="the resistor and gain found by root finding on the netlist's f0 and Q")


def filter_order(ftype, fp, fs, Ap, As):
    _pos("fp, fs, Ap and As", fp, fs, Ap, As)
    _need(ftype in ("Butterworth", "Chebyshev"), "type: butterworth or chebyshev")
    _need(fs > fp, "the stopband edge must be above the passband edge (low-pass)")
    _need(As > Ap, "the stopband attenuation must exceed the passband ripple")
    d = (10.0 ** (As / 10.0) - 1.0) / (10.0 ** (Ap / 10.0) - 1.0)
    if ftype == "Butterworth":
        x = math.log10(d) / (2.0 * math.log10(fs / fp))
    else:
        x = math.acosh(math.sqrt(d)) / math.acosh(fs / fp)
    return _out(n=int(math.ceil(x)))


def _ref_filter_order(ftype, fp, fs, Ap, As):
    from scipy import signal
    if ftype == "Butterworth":
        n, _ = signal.buttord(TWO_PI * fp, TWO_PI * fs, Ap, As, analog=True)
    else:
        n, _ = signal.cheb1ord(TWO_PI * fp, TWO_PI * fs, Ap, As, analog=True)
    return {"n": int(n)}


_add("elx_filter_order",
     name="Butterworth or Chebyshev low-pass filter order", topic="active filters / approximations",
     purpose="The minimum order of a Butterworth or Chebyshev type I low-pass filter meeting a passband edge with at most Ap dB attenuation and a stopband edge with at least As dB.",
     inputs=[_choice("ftype", ("Butterworth", "Chebyshev"), "butterworth or chebyshev (type I)"), _phys("fp", "frequency", "passband edge"),
             _phys("fs", "frequency", "stopband edge"), _numin("Ap", "maximum passband attenuation (ripple) in dB", words=("dB",), unit_required=True),
             _numin("As", "minimum stopband attenuation in dB", words=("dB",), unit_required=True)],
     outputs=[_o("n", "number", "minimum filter order")],
     primary="n", conditions=["fs > fp", "As > Ap > 0"],
     method="Butterworth: n >= log10(D)/(2 log10(fs/fp)); Chebyshev: n >= acosh(sqrt D)/acosh(fs/fp), D = (10^(As/10) - 1)/(10^(Ap/10) - 1)",
     latex=r"n \ge \frac{\log_{10}D}{2\log_{10}(f_s/f_p)},\quad n \ge \frac{\cosh^{-1}\sqrt{D}}{\cosh^{-1}(f_s/f_p)}",
     run=filter_order, reference=_ref_filter_order,
     wrong=lambda ftype, fp, fs, Ap, As: _out(n=int(math.ceil(math.log10((10 ** (As / 10) - 1) / (10 ** (Ap / 10) - 1)) / math.log10(fs / fp)))),
     rand=lambda rng: (lambda fp: {"ftype": rng.choice(["Butterworth", "Chebyshev"]), "fp": fp, "fs": _sig(fp * rng.uniform(1.3, 6.0), 3),
                                   "Ap": _sig(rng.uniform(0.1, 3.0), 2), "As": _sig(rng.uniform(20.0, 80.0), 3)})(_sig(_lu(rng, 10.0, 1e5), 3)),
     phrases=("Filter order", "Low-pass filter order"),
     aliases={"type": "ftype"},
     examples=[_ex({"ftype": "Butterworth", "fp": 1000.0, "fs": 2500.0, "Ap": 1.0, "As": 40.0}, {"n": 6}),
               _ex({"ftype": "Chebyshev", "fp": 1000.0, "fs": 2500.0, "Ap": 1.0, "As": 40.0}, {"n": 4})],
     level="intermediate",
     extra_q=[{"q": "Filter order: type = Butterworth, fp = 1 kHz, fs = 2.5 kHz, Ap = 1 dB, As = 40 dB", "like": 0},
              {"q": "Filter order: ftype = butterworth, fp = 1 kHz, fs = 2.5 kHz, Ap = 1, As = 40 dB", "outcome": "UNRECOGNIZED_FORMAT"}])


def butterworth_sections(n):
    _need(isinstance(n, int) and 2 <= n <= 20, "the order must be a whole number from 2 to 20")
    qs = []
    k = n // 2
    while k >= 1:
        qs.append(1.0 / (2.0 * math.sin((2 * k - 1) * math.pi / (2 * n))))
        k -= 1
    return _out(Q_max=qs[-1], Q=qs, sections=n // 2, first_order=n % 2)


def _ref_butterworth_sections(n):
    from scipy import signal
    _, p, _ = signal.buttap(n)
    qs = sorted(float(abs(z) / (-2.0 * z.real)) for z in p if z.imag > 1e-12)
    return {"Q_max": qs[-1], "Q": qs, "sections": n // 2, "first_order": n % 2}


_add("elx_butterworth_section_q",
     name="Butterworth filter: Q of each second-order section", topic="active filters / approximations",
     purpose="The quality factors of the cascaded second-order sections of an n-th order Butterworth filter (all at the cutoff frequency), and whether a first-order section is needed.",
     inputs=[_intin("n", "filter order", 2, 20)],
     outputs=[_o("Q_max", "number", "largest section Q"), _o("Q", "number", "section Q values, ascending"),
              _o("sections", "number", "number of second-order sections"), _o("first_order", "number", "1 if a first-order section is needed")],
     primary="Q_max", conditions=["2 <= n <= 20"],
     method="Q_k = 1/(2 sin((2k - 1) pi/(2n))), k = 1 .. floor(n/2)",
     latex=r"Q_k = \frac{1}{2\sin\left(\frac{(2k-1)\pi}{2n}\right)}",
     run=butterworth_sections, reference=_ref_butterworth_sections,
     wrong=lambda n: (lambda qs: _out(Q_max=max(qs), Q=sorted(qs), sections=n // 2, first_order=n % 2))([1.0 / (2.0 * math.sin(k * math.pi / (2 * n))) for k in range(1, n // 2 + 1)]),
     rand=lambda rng: {"n": rng.randint(2, 12)},
     phrases=("Butterworth section Q", "Butterworth filter sections"),
     examples=[_ex({"n": 4}), _ex({"n": 5})],
     level="advanced")


def chebyshev_sections(n, ripple):
    _need(isinstance(n, int) and 2 <= n <= 16, "the order must be a whole number from 2 to 16")
    _need(0.001 <= ripple <= 6.0, "the passband ripple must be between 0.001 and 6 dB")
    eps = math.sqrt(10.0 ** (ripple / 10.0) - 1.0)
    a = math.asinh(1.0 / eps) / n
    pairs = []
    for k in range(1, n // 2 + 1):
        th = (2 * k - 1) * math.pi / (2 * n)
        s = math.sinh(a) * math.sin(th)
        w = math.cosh(a) * math.cos(th)
        w0 = math.sqrt(s * s + w * w)
        pairs.append((w0 / (2.0 * s), w0))
    pairs.sort()
    qs = [p[0] for p in pairs]
    f0 = [p[1] for p in pairs]
    return _out(Q_max=qs[-1], Q=qs, f0_ratio=f0, sections=n // 2, real_pole=math.sinh(a) if n % 2 else 0.0)


def _ref_chebyshev_sections(n, ripple):
    from scipy import signal
    _, p, _ = signal.cheb1ap(n, ripple)
    pairs = sorted((float(abs(z) / (-2.0 * z.real)), float(abs(z))) for z in p if z.imag > 1e-12)
    real = [float(-z.real) for z in p if abs(z.imag) <= 1e-12]
    return {"Q_max": pairs[-1][0], "Q": [x[0] for x in pairs], "f0_ratio": [x[1] for x in pairs], "sections": n // 2,
            "real_pole": real[0] if real else 0.0}


_add("elx_chebyshev_sections",
     name="Chebyshev type I filter: section Q and frequencies", topic="active filters / approximations",
     purpose="The quality factors and natural frequencies (relative to the passband edge) of the second-order sections of an n-th order Chebyshev type I low-pass filter with a given passband ripple, and the real pole of an odd order.",
     inputs=[_intin("n", "filter order", 2, 16), _numin("ripple", "passband ripple in dB", words=("dB",), unit_required=True)],
     outputs=[_o("Q_max", "number", "largest section Q"), _o("Q", "number", "section Q values, ascending"),
              _o("f0_ratio", "number", "section natural frequencies / passband edge, in the same order"),
              _o("sections", "number", "number of second-order sections"), _o("real_pole", "number", "real pole / passband edge (0 for even n)")],
     primary="Q_max", conditions=["2 <= n <= 16", "0.001 dB <= ripple <= 6 dB"],
     method="eps = sqrt(10^(r/10) - 1); a = asinh(1/eps)/n; poles -sinh a sin th_k + j cosh a cos th_k; Q = |p|/(2 |Re p|)",
     latex=r"p_k = -\sinh a\sin\theta_k + j\cosh a\cos\theta_k,\ a = \frac{1}{n}\sinh^{-1}\frac{1}{\varepsilon},\ Q_k = \frac{|p_k|}{2|\Re p_k|}",
     run=chebyshev_sections, reference=_ref_chebyshev_sections,
     wrong=lambda n, ripple: (lambda a: _out(Q_max=0.0, Q=[], f0_ratio=[], sections=n // 2, real_pole=a))(math.asinh(math.sqrt(10 ** (ripple / 10) - 1)) / n),
     rand=lambda rng: {"n": rng.randint(2, 10), "ripple": rng.choice([0.01, 0.1, 0.25, 0.5, 1.0, 2.0, 3.0])},
     phrases=("Chebyshev filter sections", "Chebyshev section Q"),
     examples=[_ex({"n": 4, "ripple": 0.5}), _ex({"n": 3, "ripple": 1.0})],
     level="expert")


# ================================================================================================ OSCILLATORS
def rc_phase_shift(R, C, ladder):
    _pos("R and C", R, C)
    _need(ladder in ("high-pass", "low-pass"), "ladder: highpass (series C, shunt R) or lowpass (series R, shunt C)")
    f = 1.0 / (TWO_PI * math.sqrt(6.0) * R * C) if ladder == "high-pass" else math.sqrt(6.0) / (TWO_PI * R * C)
    return _out(f0=f, A_min=29.0, beta=1.0 / 29.0)


def _ref_rc_phase_shift(R, C, ladder):
    def H(f):
        s = complex(0.0, TWO_PI * f)
        if ladder == "high-pass":
            el = [("V", 1, 0, 1.0), ("C", 1, 2, C), ("R", 2, 0, R), ("C", 2, 3, C), ("R", 3, 0, R), ("C", 3, 4, C), ("R", 4, 0, R)]
        else:
            el = [("V", 1, 0, 1.0), ("R", 1, 2, R), ("C", 2, 0, C), ("R", 2, 3, R), ("C", 3, 0, C), ("R", 3, 4, R), ("C", 4, 0, C)]
        return _mna(4, el, s)[0][4]
    sc = 1.0 / (TWO_PI * R * C)
    f0 = _brentq(lambda f: H(f).imag, sc * 0.05, sc * 20.0)             # 180 degrees: the response is real
    b = abs(H(f0))
    return {"f0": f0, "A_min": 1.0 / b, "beta": b}


_add("elx_rc_phase_shift_oscillator",
     name="RC phase-shift oscillator frequency and gain", topic="oscillators / RC",
     purpose="The oscillation frequency of a three-section RC phase-shift oscillator (high-pass ladder: series C, shunt R; or low-pass ladder) and the amplifier gain of 29 it needs.",
     inputs=[_phys("R", "resistance", "resistor of each section"), _phys("C", "capacitance", "capacitor of each section", show=("nF", 1e9)),
             _choice("ladder", ("high-pass", "low-pass"), "highpass (series C, shunt R) or lowpass (series R, shunt C)")],
     outputs=[_o("f0", "frequency", "oscillation frequency"), _o("A_min", "number", "minimum inverting amplifier gain"),
              _o("beta", "number", "feedback network attenuation at f0")],
     primary="f0", conditions=["unloaded ladder (driven by a voltage source, feeding a high impedance)"],
     method="high-pass ladder f = 1/(2 pi sqrt(6) R C); low-pass ladder f = sqrt(6)/(2 pi R C); |beta| = 1/29",
     latex=r"f_0 = \frac{1}{2\pi\sqrt{6}\,RC},\quad |A| \ge 29",
     run=rc_phase_shift, reference=_ref_rc_phase_shift,
     wrong=lambda R, C, ladder: _out(f0=1.0 / (TWO_PI * R * C), A_min=29.0, beta=1.0 / 29.0),
     rand=lambda rng: {"R": _sig(_lu(rng, 1e3, 1e5), 3), "C": _sig(_lu(rng, 1e-10, 1e-6), 3), "ladder": rng.choice(["high-pass", "low-pass"])},
     phrases=("RC phase-shift oscillator", "Phase-shift oscillator"),
     examples=[_ex({"R": 10e3, "C": 10e-9, "ladder": "high-pass"}), _ex({"R": 4700.0, "C": 47e-9, "ladder": "low-pass"})],
     level="intermediate")


def wien_bridge(R1, C1, R2, C2):
    _pos("R1, C1, R2 and C2", R1, C1, R2, C2)
    f0 = 1.0 / (TWO_PI * math.sqrt(R1 * R2 * C1 * C2))
    A = 1.0 + R1 / R2 + C2 / C1
    return _out(f0=f0, beta=1.0 / A, A_min=A, Rf_over_Rg=A - 1.0)


def _ref_wien_bridge(R1, C1, R2, C2):
    H = lambda f: _mna(3, [("V", 1, 0, 1.0), ("R", 1, 2, R1), ("C", 2, 3, C1), ("R", 3, 0, R2), ("C", 3, 0, C2)], complex(0.0, TWO_PI * f))[0][3]
    sc = 1.0 / (TWO_PI * min(R1, R2) * min(C1, C2))
    f0 = _brentq(lambda f: H(f).imag, sc * 1e-6, sc * 1e3)
    b = H(f0).real
    return {"f0": f0, "beta": b, "A_min": 1.0 / b, "Rf_over_Rg": 1.0 / b - 1.0}


_add("elx_wien_bridge_oscillator",
     name="Wien-bridge oscillator frequency and required gain", topic="oscillators / RC",
     purpose="The oscillation frequency of a Wien-bridge oscillator (series R1 C1, parallel R2 C2) and the non-inverting gain it needs (3 for equal components).",
     inputs=[_phys("R1", "resistance", "series-arm resistor"), _phys("C1", "capacitance", "series-arm capacitor", show=("nF", 1e9)),
             _phys("R2", "resistance", "parallel-arm resistor"), _phys("C2", "capacitance", "parallel-arm capacitor", show=("nF", 1e9))],
     outputs=[_o("f0", "frequency", "oscillation frequency"), _o("beta", "number", "feedback fraction at f0"),
              _o("A_min", "number", "required amplifier gain"), _o("Rf_over_Rg", "number", "gain resistor ratio")],
     primary="f0", conditions=["ideal amplifier"],
     method="f0 = 1/(2 pi sqrt(R1 R2 C1 C2)); A = 1 + R1/R2 + C2/C1",
     latex=r"f_0 = \frac{1}{2\pi\sqrt{R_1R_2C_1C_2}},\quad A = 1 + \frac{R_1}{R_2} + \frac{C_2}{C_1}",
     run=wien_bridge, reference=_ref_wien_bridge,
     wrong=lambda R1, C1, R2, C2: _out(f0=1.0 / (TWO_PI * math.sqrt(R1 * R2 * C1 * C2)), beta=1.0 / 3.0, A_min=3.0, Rf_over_Rg=2.0),
     rand=lambda rng: {"R1": _sig(_lu(rng, 1e3, 1e5), 3), "C1": _sig(_lu(rng, 1e-9, 1e-6), 3), "R2": _sig(_lu(rng, 1e3, 1e5), 3), "C2": _sig(_lu(rng, 1e-9, 1e-6), 3)},
     phrases=("Wien-bridge oscillator", "Wien bridge oscillator"),
     examples=[_ex({"R1": 10e3, "C1": 16e-9, "R2": 10e3, "C2": 16e-9}, {"A_min": 3.0}), _ex({"R1": 22e3, "C1": 10e-9, "R2": 10e3, "C2": 22e-9})],
     level="intermediate")


def colpitts(L, C1, C2):
    _pos("L, C1 and C2", L, C1, C2)
    Ceq = C1 * C2 / (C1 + C2)
    return _out(C_eq=Ceq, f0=1.0 / (TWO_PI * math.sqrt(L * Ceq)), beta=C1 / C2, A_min=C2 / C1)


def _ref_colpitts(L, C1, C2):
    # admittance of the tank seen across C1 (C1 in parallel with L in series with C2) is zero at resonance
    def tank(f):
        v, cur = _mna(2, [("V", 1, 0, 1.0), ("C", 1, 0, C1), ("L", 1, 2, L), ("C", 2, 0, C2)], complex(0.0, TWO_PI * f))
        return -cur[0], v[2]
    sc = 1.0 / (TWO_PI * math.sqrt(L * min(C1, C2)))
    fb = 1.0 / (TWO_PI * math.sqrt(L * C2))                          # series resonance of the L-C2 branch (a pole)
    f0 = _brentq(lambda f: tank(f)[0].imag, fb * (1.0 + 1e-9), sc * 1.5)
    ratio = abs(tank(f0)[1])
    return {"C_eq": 1.0 / (1.0 / C1 + 1.0 / C2), "f0": f0, "beta": ratio, "A_min": 1.0 / ratio}


_add("elx_colpitts_oscillator",
     name="Colpitts oscillator frequency and feedback ratio", topic="oscillators / LC",
     purpose="The oscillation frequency of a Colpitts oscillator from its inductor and the series combination of its two tank capacitors, the feedback fraction C1/C2 and the minimum amplifier gain C2/C1.",
     inputs=[_phys("L", "inductance", "tank inductance", show=("uH", 1e6)),
             _phys("C1", "capacitance", "capacitor across the amplifier output", show=("nF", 1e9)),
             _phys("C2", "capacitance", "capacitor across the amplifier input (feedback side)", show=("nF", 1e9))],
     outputs=[_o("C_eq", "capacitance", "series equivalent capacitance"), _o("f0", "frequency", "oscillation frequency"),
              _o("beta", "number", "feedback fraction C1/C2"), _o("A_min", "number", "minimum amplifier gain C2/C1")],
     primary="f0", conditions=["lossless tank, amplifier does not load the tank"],
     method="C_eq = C1 C2/(C1 + C2); f0 = 1/(2 pi sqrt(L C_eq)); beta = C1/C2",
     latex=r"f_0 = \frac{1}{2\pi\sqrt{L\,\frac{C_1C_2}{C_1+C_2}}},\quad A_{min} = \frac{C_2}{C_1}",
     run=colpitts, reference=_ref_colpitts,
     wrong=lambda L, C1, C2: _out(C_eq=C1 + C2, f0=1.0 / (TWO_PI * math.sqrt(L * (C1 + C2))), beta=C1 / C2, A_min=C2 / C1),
     rand=lambda rng: {"L": _sig(_lu(rng, 1e-7, 1e-2), 3), "C1": _sig(_lu(rng, 1e-11, 1e-7), 3), "C2": _sig(_lu(rng, 1e-11, 1e-7), 3)},
     phrases=("Colpitts oscillator", "Colpitts oscillator frequency"),
     examples=[_ex({"L": 100e-6, "C1": 1e-9, "C2": 10e-9}), _ex({"L": 10e-6, "C1": 470e-12, "C2": 4.7e-9})],
     level="intermediate")


def hartley(L1, L2, M, C):
    _pos("L1, L2 and C", L1, L2, C)
    _nonneg("the mutual inductance M", M)
    _need(M <= math.sqrt(L1 * L2), "M cannot exceed sqrt(L1 L2)")
    Leq = L1 + L2 + 2.0 * M
    return _out(L_eq=Leq, f0=1.0 / (TWO_PI * math.sqrt(Leq * C)), A_min=(L1 + M) / (L2 + M))


def _ref_hartley(L1, L2, M, C):
    import numpy as np
    Lm = np.array([[L1, M], [M, L2]])
    Leq = float(np.ones(2) @ Lm @ np.ones(2))                 # series-aiding coils: same current through both
    sc = 1.0 / (TWO_PI * math.sqrt(Leq * C))
    f0 = _brentq(lambda f: 1.0 - (TWO_PI * f) ** 2 * Leq * C, sc * 1e-3, sc * 1e3)
    # voltages across the coils at the same current: v = j w L i, the ratio of the output-side to the feedback-side coil
    v = Lm @ np.ones(2)
    return {"L_eq": Leq, "f0": f0, "A_min": float(v[0] / v[1])}


_add("elx_hartley_oscillator",
     name="Hartley oscillator frequency", topic="oscillators / LC",
     purpose="The oscillation frequency of a Hartley oscillator with a tapped (or two coupled) inductors and a tank capacitor, and the gain ratio (L1 + M)/(L2 + M).",
     inputs=[_phys("L1", "inductance", "inductance of the output-side coil", show=("uH", 1e6)),
             _phys("L2", "inductance", "inductance of the feedback-side coil", show=("uH", 1e6)),
             _phys("M", "inductance", "mutual inductance (0 for uncoupled coils)", show=("uH", 1e6)), _phys("C", "capacitance", "tank capacitance", show=("nF", 1e9))],
     outputs=[_o("L_eq", "inductance", "total inductance L1 + L2 + 2M"), _o("f0", "frequency", "oscillation frequency"),
              _o("A_min", "number", "minimum amplifier gain (L1 + M)/(L2 + M)")],
     primary="f0", conditions=["series-aiding coils, lossless tank", "M <= sqrt(L1 L2)"],
     method="L_eq = L1 + L2 + 2M; f0 = 1/(2 pi sqrt(L_eq C))",
     latex=r"f_0 = \frac{1}{2\pi\sqrt{(L_1 + L_2 + 2M)C}}",
     run=hartley, reference=_ref_hartley,
     wrong=lambda L1, L2, M, C: _out(L_eq=L1 + L2, f0=1.0 / (TWO_PI * math.sqrt((L1 + L2) * C)), A_min=L1 / L2),
     rand=lambda rng: (lambda L1, L2: {"L1": L1, "L2": L2, "M": _sig(math.sqrt(L1 * L2) * rng.uniform(0.05, 0.9), 3), "C": _sig(_lu(rng, 1e-11, 1e-7), 3)})(
         _sig(_lu(rng, 1e-7, 1e-3), 3), _sig(_lu(rng, 1e-7, 1e-3), 3)),
     phrases=("Hartley oscillator", "Hartley oscillator frequency"),
     examples=[_ex({"L1": 100e-6, "L2": 20e-6, "M": 0.0, "C": 1e-9}), _ex({"L1": 50e-6, "L2": 10e-6, "M": 5e-6, "C": 470e-12})],
     level="intermediate",
     weak="the coil matrix summed as a quadratic form and the resonance by root finding (the same lumped model)")


def crystal(Lm, Cm, Rm, C0):
    _pos("Lm, Cm, Rm and C0", Lm, Cm, Rm, C0)
    fs = 1.0 / (TWO_PI * math.sqrt(Lm * Cm))
    fp = fs * math.sqrt(1.0 + Cm / C0)
    return _out(fs=fs, fp=fp, delta_f=fs * (math.sqrt(1.0 + Cm / C0) - 1.0), Q=math.sqrt(Lm / Cm) / Rm)


def _ref_crystal(Lm, Cm, Rm, C0):
    xs = lambda f: TWO_PI * f * Lm - 1.0 / (TWO_PI * f * Cm)          # reactance of the motional arm
    sc = 1.0 / (TWO_PI * math.sqrt(Lm * Cm))
    fs = _brentq(xs, sc * 0.5, sc * 2.0)
    yp = lambda f: TWO_PI * f * C0 - 1.0 / xs(f)                      # susceptance with C0 in parallel
    fp = _brentq(yp, fs * (1.0 + 1e-12), fs * (1.0 + Cm / C0))
    return {"fs": fs, "fp": fp, "delta_f": fp - fs, "Q": TWO_PI * fs * Lm / Rm}


_add("elx_crystal_oscillator",
     name="Quartz crystal: series and parallel resonance, Q", topic="oscillators / crystal",
     purpose="The series and parallel (anti-) resonant frequencies, their spacing and the quality factor of a quartz crystal from its Butterworth-Van Dyke equivalent circuit.",
     inputs=[_phys("Lm", "inductance", "motional inductance"), _phys("Cm", "capacitance", "motional capacitance", show=("fF", 1e15)),
             _phys("Rm", "resistance", "motional resistance"), _phys("C0", "capacitance", "shunt (holder) capacitance", show=("pF", 1e12))],
     outputs=[_o("fs", "frequency", "series resonance"), _o("fp", "frequency", "parallel resonance"), _o("delta_f", "frequency", "fp - fs"),
              _o("Q", "number", "quality factor")],
     primary="fs", conditions=["resonances of the lossless circuit; Q from Rm"],
     method="fs = 1/(2 pi sqrt(Lm Cm)); fp = fs sqrt(1 + Cm/C0); Q = sqrt(Lm/Cm)/Rm",
     latex=r"f_s = \frac{1}{2\pi\sqrt{L_mC_m}},\quad f_p = f_s\sqrt{1 + \frac{C_m}{C_0}}",
     run=crystal, reference=_ref_crystal,
     wrong=lambda Lm, Cm, Rm, C0: (lambda fs: _out(fs=fs, fp=fs * (1 + Cm / C0), delta_f=fs * Cm / C0, Q=math.sqrt(Lm / Cm) / Rm))(1 / (TWO_PI * math.sqrt(Lm * Cm))),
     rand=lambda rng: {"Lm": _sig(_lu(rng, 1e-3, 1.0), 3), "Cm": _sig(_lu(rng, 5e-15, 5e-14), 3), "Rm": _sig(_lu(rng, 5.0, 200.0), 3),
                       "C0": _sig(_lu(rng, 2e-12, 10e-12), 3)},
     phrases=("Crystal resonance", "Quartz crystal resonance"),
     examples=[_ex({"Lm": 0.05, "Cm": 20e-15, "Rm": 20.0, "C0": 5e-12}), _ex({"Lm": 0.26, "Cm": 25e-15, "Rm": 50.0, "C0": 6e-12})],
     level="advanced")


def relaxation_oscillator(R, C, R1, R2):
    _pos("R, C, R1 and R2", R, C, R1, R2)
    b = R1 / (R1 + R2)
    T = 2.0 * R * C * math.log((1.0 + b) / (1.0 - b))
    return _out(beta=b, T=T, f=1.0 / T)


def _ref_relaxation_oscillator(R, C, R1, R2):
    b = R1 / (R1 + R2)
    # with the output at +1 the capacitor charges from -b towards +1: time to reach +b, twice per period
    th = _brentq(lambda t: 1.0 - (1.0 + b) * math.exp(-t / (R * C)) - b, 0.0, 100.0 * R * C)
    return {"beta": b, "T": 2.0 * th, "f": 1.0 / (2.0 * th)}


_add("elx_opamp_relaxation_oscillator",
     name="Op-amp relaxation (astable) oscillator", topic="oscillators / relaxation",
     purpose="The period and frequency of a square-wave op-amp astable: an RC timing network on the inverting input and positive feedback R1/(R1 + R2) to the non-inverting input, symmetric supplies.",
     inputs=[_phys("R", "resistance", "timing resistor (output to inverting input)"), _phys("C", "capacitance", "timing capacitor", show=("nF", 1e9)),
             _phys("R1", "resistance", "resistor from the + input to ground"), _phys("R2", "resistance", "resistor from the output to the + input")],
     outputs=[_o("beta", "number", "feedback fraction R1/(R1 + R2)"), _o("T", "time", "period"), _o("f", "frequency", "frequency")],
     primary="f", conditions=["symmetric output levels", "ideal comparator action"],
     method="T = 2 R C ln((1 + beta)/(1 - beta)) = 2 R C ln(1 + 2 R1/R2)",
     latex=r"T = 2RC\ln\frac{1+\beta}{1-\beta},\quad \beta = \frac{R_1}{R_1+R_2}",
     run=relaxation_oscillator, reference=_ref_relaxation_oscillator,
     wrong=lambda R, C, R1, R2: (lambda b: (lambda T: _out(beta=b, T=T, f=1 / T))(R * C * math.log((1 + b) / (1 - b))))(R1 / (R1 + R2)),
     rand=lambda rng: {"R": _sig(_lu(rng, 1e3, 1e6), 3), "C": _sig(_lu(rng, 1e-9, 1e-5), 3), "R1": _sig(_lu(rng, 1e3, 1e5), 3), "R2": _sig(_lu(rng, 1e3, 1e5), 3)},
     phrases=("Op-amp relaxation oscillator", "Op-amp astable"),
     examples=[_ex({"R": 10e3, "C": 100e-9, "R1": 10e3, "R2": 10e3}), _ex({"R": 47e3, "C": 10e-9, "R1": 8600.0, "R2": 10e3})],
     level="advanced")


def timer_astable(RA, RB, C):
    _pos("RA, RB and C", RA, RB, C)
    tH = LN2 * (RA + RB) * C
    tL = LN2 * RB * C
    return _out(t_high=tH, t_low=tL, T=tH + tL, f=1.0 / (tH + tL), duty=tH / (tH + tL))


def _ref_timer_astable(RA, RB, C):
    # charging through RA + RB from Vcc/3 towards Vcc until 2Vcc/3; discharging through RB from 2Vcc/3 to Vcc/3
    tH = _brentq(lambda t: 1.0 - (2.0 / 3.0) * math.exp(-t / ((RA + RB) * C)) - 2.0 / 3.0, 0.0, 10.0 * (RA + RB) * C)
    tL = _brentq(lambda t: (2.0 / 3.0) * math.exp(-t / (RB * C)) - 1.0 / 3.0, 0.0, 10.0 * RB * C)
    return {"t_high": tH, "t_low": tL, "T": tH + tL, "f": 1.0 / (tH + tL), "duty": tH / (tH + tL)}


_add("elx_555_astable",
     name="555 timer astable: frequency and duty cycle", topic="oscillators / 555 timer",
     purpose="The high and low times, period, frequency and duty cycle of a 555 timer in astable mode with resistors RA and RB and timing capacitor C.",
     inputs=[_phys("RA", "resistance", "resistor from Vcc to the discharge pin"), _phys("RB", "resistance", "resistor from discharge to threshold/trigger"),
             _phys("C", "capacitance", "timing capacitor", show=("uF", 1e6))],
     outputs=[_o("t_high", "time", "output high time"), _o("t_low", "time", "output low time"), _o("T", "time", "period"),
              _o("f", "frequency", "frequency"), _o("duty", "number", "duty cycle (fraction high)")],
     primary="f", conditions=["thresholds Vcc/3 and 2Vcc/3"],
     method="t_H = ln2 (RA + RB) C; t_L = ln2 RB C; f = 1/(t_H + t_L)",
     latex=r"t_H = \ln 2\,(R_A + R_B)C,\quad t_L = \ln 2\,R_BC,\quad f = \frac{1}{\ln 2\,(R_A + 2R_B)C}",
     run=timer_astable, reference=_ref_timer_astable,
     wrong=lambda RA, RB, C: (lambda t: _out(t_high=t, t_low=t, T=2 * t, f=1 / (2 * t), duty=0.5))(LN2 * (RA + RB) * C),
     rand=lambda rng: {"RA": _sig(_lu(rng, 1e3, 1e6), 3), "RB": _sig(_lu(rng, 1e3, 1e6), 3), "C": _sig(_lu(rng, 1e-9, 1e-4), 3)},
     phrases=("555 astable", "555 timer astable"),
     examples=[_ex({"RA": 1000.0, "RB": 10e3, "C": 10e-6}), _ex({"RA": 4700.0, "RB": 4700.0, "C": 100e-9})],
     level="beginner")


def timer_monostable(R, C):
    _pos("R and C", R, C)
    return _out(T=LN3 * R * C)


def _ref_timer_monostable(R, C):
    return {"T": _brentq(lambda t: 1.0 - math.exp(-t / (R * C)) - 2.0 / 3.0, 0.0, 20.0 * R * C)}


_add("elx_555_monostable",
     name="555 timer monostable pulse width", topic="oscillators / 555 timer",
     purpose="The output pulse width T = ln 3 R C (about 1.1 R C) of a 555 timer in monostable (one-shot) mode.",
     inputs=[_phys("R", "resistance", "timing resistor"), _phys("C", "capacitance", "timing capacitor", show=("uF", 1e6))],
     outputs=[_o("T", "time", "pulse width")],
     primary="T", conditions=["threshold at 2Vcc/3", "trigger pulse shorter than T"],
     method="T = ln 3 R C",
     latex=r"T = \ln 3\,RC \approx 1.1\,RC",
     run=timer_monostable, reference=_ref_timer_monostable,
     wrong=lambda R, C: _out(T=LN2 * R * C),
     rand=lambda rng: {"R": _sig(_lu(rng, 1e3, 1e6), 3), "C": _sig(_lu(rng, 1e-9, 1e-3), 3)},
     phrases=("555 monostable", "555 timer monostable"),
     examples=[_ex({"R": 100e3, "C": 10e-6}), _ex({"R": 47e3, "C": 1e-6})],
     level="beginner")


def pll(Kd, Ko, tau1, tau2):
    _pos("Kd, Ko, tau1 and tau2", Kd, Ko, tau1, tau2)
    wn = math.sqrt(Kd * Ko / tau1)
    return _out(wn=wn, fn=wn / TWO_PI, zeta=tau2 * wn / 2.0)


def _ref_pll(Kd, Ko, tau1, tau2):
    import numpy as np
    K = Kd * Ko
    p = np.roots([1.0, K * tau2 / tau1, K / tau1])           # closed-loop poles of the type-2 loop
    wn = math.sqrt(float((p[0] * p[1]).real))
    zeta = float(-(p[0] + p[1]).real) / (2.0 * wn)
    return {"wn": wn, "fn": wn / TWO_PI, "zeta": zeta}


_add("elx_pll_loop_dynamics",
     name="Type-2 PLL with active PI loop filter: natural frequency and damping", topic="oscillators / phase-locked loops",
     purpose="The natural frequency and damping factor of a second-order (type-2) phase-locked loop with phase-detector gain Kd, VCO gain Ko and an active PI loop filter F(s) = (1 + s tau2)/(s tau1).",
     inputs=[_numin("Kd", "phase-detector gain in V/rad", words=("V/rad",)), _numin("Ko", "VCO gain in rad/(s V)", words=("rad/(s*V)", "rad/s/V")),
             _phys("tau1", "time", "integrator time constant R1 C", show=("ms", 1e3)), _phys("tau2", "time", "zero time constant R2 C", show=("ms", 1e3))],
     outputs=[_o("wn", "angular_frequency", "natural frequency"), _o("fn", "frequency", "natural frequency in Hz"), _o("zeta", "number", "damping factor")],
     primary="zeta", conditions=["linear phase model, ideal integrator"],
     method="H(s) = K (1 + s tau2)/tau1 / (s^2 + s K tau2/tau1 + K/tau1), K = Kd Ko; wn = sqrt(K/tau1); zeta = tau2 wn/2",
     latex=r"\omega_n = \sqrt{\frac{K_dK_o}{\tau_1}},\quad \zeta = \frac{\tau_2\omega_n}{2}",
     run=pll, reference=_ref_pll,
     wrong=lambda Kd, Ko, tau1, tau2: (lambda wn: _out(wn=wn, fn=wn / TWO_PI, zeta=tau2 * wn))(math.sqrt(Kd * Ko / tau1)),
     rand=lambda rng: {"Kd": _sig(rng.uniform(0.1, 2.0), 3), "Ko": _sig(_lu(rng, 1e3, 1e6), 3), "tau1": _sig(_lu(rng, 1e-4, 1e-1), 3),
                       "tau2": _sig(_lu(rng, 1e-5, 1e-2), 3)},
     phrases=("PLL loop dynamics", "Phase-locked loop natural frequency"),
     examples=[_ex({"Kd": 0.5, "Ko": 2e4, "tau1": 0.01, "tau2": 1e-3}, {"wn": 1000.0, "zeta": 0.5}), _ex({"Kd": 1.0, "Ko": 1e5, "tau1": 0.1, "tau2": 2e-3})],
     level="expert")


# ================================================================================================ DATA CONVERTERS
def _bits(code, n):
    s = ""
    i = n - 1
    while i >= 0:
        s += "1" if (code // (2 ** i)) % 2 == 1 else "0"
        i -= 1
    return s


def adc_code(Vin, Vref, n):
    _pos("Vref", Vref)
    _fin(Vin)
    _need(isinstance(n, int) and 1 <= n <= 16, "the resolution must be a whole number of bits from 1 to 16")
    _need(0.0 <= Vin <= Vref, "the input must lie between 0 and Vref (unipolar converter)")
    lsb = Vref / 2 ** n
    code = int(math.floor(Vin * 2 ** n / Vref))
    if code > 2 ** n - 1:
        code = 2 ** n - 1
    return _out(code=code, bits=_bits(code, n), LSB=lsb, V_code=code * lsb, error=Vin - code * lsb)


def _ref_adc_code(Vin, Vref, n):
    import numpy as np
    edges = np.arange(1, 2 ** n) * (Vref / 2 ** n)              # the 2^n - 1 transition levels
    code = int(np.searchsorted(edges, Vin, side="right"))
    lsb = Vref / 2 ** n
    return {"code": code, "bits": np.binary_repr(code, n), "LSB": lsb, "V_code": code * lsb, "error": Vin - code * lsb}


_add("elx_adc_code",
     name="ADC output code, LSB and quantisation error", topic="data converters / ADC",
     purpose="The output code (decimal and binary) of an ideal unipolar truncating n-bit ADC for an input voltage, the LSB size, the voltage the code represents and the quantisation error.",
     inputs=[_phys("Vin", "voltage", "analog input"), _phys("Vref", "voltage", "reference (full-scale) voltage"), _intin("n", "resolution in bits", 1, 16)],
     outputs=[_o("code", "number", "output code (decimal)"), _o("bits", "number", "output code in binary"), _o("LSB", "voltage", "LSB = Vref/2^n"),
              _o("V_code", "voltage", "voltage of the code"), _o("error", "voltage", "quantisation error Vin - code LSB")],
     primary="code", conditions=["0 <= Vin <= Vref", "1 <= n <= 16"],
     method="code = floor(Vin 2^n/Vref), at most 2^n - 1; LSB = Vref/2^n",
     latex=r"D = \left\lfloor \frac{V_{in}}{V_{ref}}2^n\right\rfloor,\quad \text{LSB} = \frac{V_{ref}}{2^n}",
     run=adc_code, reference=_ref_adc_code,
     wrong=lambda Vin, Vref, n: (lambda lsb: (lambda c: _out(code=c, bits=_bits(c, n), LSB=lsb, V_code=c * lsb, error=Vin - c * lsb))(min(int(Vin / lsb), 2 ** n - 1)))(Vref / (2 ** n - 1)),
     rand=lambda rng: (lambda Vref, n: {"Vin": _sig(Vref * rng.uniform(0.0, 0.999), 8), "Vref": Vref, "n": n})(rng.choice([1.0, 2.048, 2.5, 3.3, 4.096, 5.0, 10.0]), rng.randint(4, 16)),
     phrases=("ADC output code", "ADC code"),
     examples=[_ex({"Vin": 3.2, "Vref": 5.0, "n": 8}, {"code": 163, "bits": "10100011"}), _ex({"Vin": 1.234, "Vref": 2.5, "n": 12})],
     level="beginner")


def adc_snr(n, OSR=1.0, A_frac=1.0):
    _need(isinstance(n, int) and 1 <= n <= 32, "the resolution must be a whole number of bits from 1 to 32")
    _need(OSR >= 1.0, "the oversampling ratio must be at least 1")
    _need(0.0 < A_frac <= 1.0, "the amplitude fraction must be in (0, 1] of full scale")
    snr = 20.0 * math.log10(A_frac * 2.0 ** n * math.sqrt(1.5)) + 10.0 * math.log10(OSR)
    return _out(SNR_dB=snr, noise_rms_lsb=1.0 / math.sqrt(12.0 * OSR), SNR_full_scale_dB=20.0 * math.log10(2.0 ** n * math.sqrt(1.5)))


def _ref_adc_snr(n, OSR=1.0, A_frac=1.0):
    # quantisation noise: uniform error over one LSB, its power by quadrature, spread over Nyquist (1/OSR in band);
    # signal: a sine of amplitude A_frac 2^(n-1) LSB, its mean square by quadrature over a period
    pn = _quad(lambda e: e * e, -0.5, 0.5)
    amp = A_frac * 2.0 ** (n - 1)
    ps = _quad(lambda t: (amp * math.sin(t)) ** 2, 0.0, TWO_PI) / TWO_PI
    ps_fs = _quad(lambda t: (2.0 ** (n - 1) * math.sin(t)) ** 2, 0.0, TWO_PI) / TWO_PI
    return {"SNR_dB": 10.0 * math.log10(ps / (pn / OSR)), "noise_rms_lsb": math.sqrt(pn / OSR), "SNR_full_scale_dB": 10.0 * math.log10(ps_fs / pn)}


_add("elx_adc_quantization_snr",
     name="ADC quantisation SNR with oversampling", topic="data converters / ADC",
     purpose="The signal-to-quantisation-noise ratio of an ideal n-bit converter for a sine input (6.02 n + 1.76 dB at full scale), with an oversampling ratio and a reduced amplitude, and the in-band noise in LSB.",
     inputs=[_intin("n", "resolution in bits", 1, 32), _numin("OSR", "oversampling ratio (default 1)", required=False),
             _numin("A_frac", "sine amplitude as a fraction of full scale (default 1)", required=False, pct=True)],
     outputs=[_o("SNR_dB", "number", "signal-to-quantisation-noise ratio, dB"), _o("noise_rms_lsb", "number", "in-band noise rms in LSB"),
              _o("SNR_full_scale_dB", "number", "full-scale SNR without oversampling, dB")],
     primary="SNR_dB", conditions=["uniform quantisation noise, white over Nyquist", "OSR >= 1"],
     method="SNR = 20 log10(A 2^n sqrt(1.5)) + 10 log10(OSR) = 6.02 n + 1.76 + 20 log10 A + 10 log10 OSR",
     latex=r"\text{SNR} = 6.02\,n + 1.76 + 10\log_{10}\text{OSR}\ \text{dB}",
     run=adc_snr, reference=_ref_adc_snr,
     wrong=lambda n, OSR=1.0, A_frac=1.0: _out(SNR_dB=20 * math.log10(A_frac * 2.0 ** n) + 10 * math.log10(OSR), noise_rms_lsb=1 / math.sqrt(12 * OSR),
                                             SNR_full_scale_dB=20 * math.log10(2.0 ** n)),
     rand=lambda rng: {"n": rng.randint(4, 24), **({"OSR": float(rng.choice([1, 2, 4, 16, 64, 256]))} if rng.random() < 0.6 else {}),
                       **({"A_frac": _sig(rng.uniform(0.05, 1.0), 3)} if rng.random() < 0.5 else {})},
     phrases=("ADC quantization SNR", "ADC quantisation SNR"),
     examples=[_ex({"n": 12}), _ex({"n": 10, "OSR": 16.0, "A_frac": 0.5})],
     level="intermediate")


def adc_enob(SINAD, n):
    _fin(SINAD)
    _need(isinstance(n, int) and 1 <= n <= 32, "the nominal resolution must be a whole number of bits from 1 to 32")
    e = (SINAD - 1.76) / 6.02
    return _out(ENOB=e, bits_lost=n - e)


def _ref_adc_enob(SINAD, n):
    import numpy as np
    e = float(np.polyval([1.0 / 6.02, -1.76 / 6.02], SINAD))
    return {"ENOB": e, "bits_lost": n - e}


_add("elx_adc_enob",
     name="Effective number of bits (ENOB) from SINAD", topic="data converters / ADC",
     purpose="The effective number of bits of a converter from its measured SINAD (IEEE 1241: ENOB = (SINAD - 1.76)/6.02) and the bits lost against its nominal resolution.",
     inputs=[_numin("SINAD", "signal to noise and distortion ratio in dB", words=("dB",), unit_required=True), _intin("n", "nominal resolution in bits", 1, 32)],
     outputs=[_o("ENOB", "number", "effective number of bits"), _o("bits_lost", "number", "n - ENOB")],
     primary="ENOB", conditions=["full-scale sine test"],
     method="ENOB = (SINAD - 1.76)/6.02",
     latex=r"\text{ENOB} = \frac{\text{SINAD} - 1.76}{6.02}",
     run=adc_enob, reference=_ref_adc_enob,
     wrong=lambda SINAD, n: _out(ENOB=SINAD / 6.02, bits_lost=n - SINAD / 6.02),
     rand=lambda rng: (lambda n: {"SINAD": _sig(6.02 * n + 1.76 - rng.uniform(0.5, 20.0), 4), "n": n})(rng.randint(6, 24)),
     phrases=("ADC ENOB", "Effective number of bits"),
     examples=[_ex({"SINAD": 74.0, "n": 14}, {"ENOB": 12.0, "bits_lost": 2.0}), _ex({"SINAD": 62.5, "n": 12})],
     level="intermediate",
     weak="the same standard expression evaluated as a numpy polynomial")


def jitter_snr(f, tj, n):
    _pos("f and tj", f, tj)
    _need(isinstance(n, int) and 1 <= n <= 32, "the resolution must be a whole number of bits from 1 to 32")
    sj = -20.0 * math.log10(TWO_PI * f * tj)
    sq = 20.0 * math.log10(2.0 ** n * math.sqrt(1.5))
    st = -10.0 * math.log10(10.0 ** (-sj / 10.0) + 10.0 ** (-sq / 10.0))
    return _out(SNR_jitter_dB=sj, SNR_quant_dB=sq, SNR_total_dB=st, ENOB_total=(st - 1.76) / 6.02)


def _ref_jitter_snr(f, tj, n):
    # jitter error e(t) = A w cos(w t) tj (slope times the timing error); its mean square against the signal's
    w = TWO_PI * f
    pe = _quad(lambda t: (w * math.cos(t) * tj) ** 2, 0.0, TWO_PI) / TWO_PI
    ps = _quad(lambda t: math.sin(t) ** 2, 0.0, TWO_PI) / TWO_PI
    sj = 10.0 * math.log10(ps / pe)
    sq = _ref_adc_snr(n)["SNR_dB"]
    pt = ps / (10.0 ** (sj / 10.0)) + ps / (10.0 ** (sq / 10.0))
    st = 10.0 * math.log10(ps / pt)
    return {"SNR_jitter_dB": sj, "SNR_quant_dB": sq, "SNR_total_dB": st, "ENOB_total": (st - 1.76) / 6.02}


_add("elx_aperture_jitter_snr",
     name="SNR limit from sampling-clock (aperture) jitter", topic="data converters / sampling",
     purpose="The SNR limit -20 log10(2 pi f tj) set by rms sampling-clock jitter for a full-scale sine, combined with the quantisation SNR of an n-bit converter into a total SNR and effective bits.",
     inputs=[_phys("f", "frequency", "input frequency"), _phys("tj", "time", "rms clock jitter", show=("ps", 1e12)), _intin("n", "converter resolution in bits", 1, 32)],
     outputs=[_o("SNR_jitter_dB", "number", "jitter-limited SNR, dB"), _o("SNR_quant_dB", "number", "quantisation SNR, dB"),
              _o("SNR_total_dB", "number", "combined SNR, dB"), _o("ENOB_total", "number", "effective bits of the combination")],
     primary="SNR_jitter_dB", conditions=["full-scale sine; jitter and quantisation noise uncorrelated"],
     method="SNR_j = -20 log10(2 pi f tj); SNR_q = 6.02 n + 1.76; noise powers add",
     latex=r"\text{SNR}_j = -20\log_{10}(2\pi f t_j)",
     run=jitter_snr, reference=_ref_jitter_snr,
     wrong=lambda f, tj, n: (lambda r: dict(r, SNR_jitter_dB=_r(-20 * math.log10(f * tj))))(jitter_snr(f, tj, n)),
     rand=lambda rng: {"f": _sig(_lu(rng, 1e5, 1e9), 3), "tj": _sig(_lu(rng, 1e-14, 1e-11), 3), "n": rng.randint(8, 20)},
     phrases=("Aperture jitter SNR", "Clock jitter SNR"),
     examples=[_ex({"f": 100e6, "tj": 1e-12, "n": 14}), _ex({"f": 10e6, "tj": 0.5e-12, "n": 16})],
     level="expert")


def dac_output(n, code, Vref):
    _need(isinstance(n, int) and 1 <= n <= 16, "the resolution must be a whole number of bits from 1 to 16")
    _need(isinstance(code, int) and 0 <= code <= 2 ** n - 1, "the code must be a whole number from 0 to 2^n - 1")
    _pos("Vref", Vref)
    lsb = Vref / 2 ** n
    return _out(Vout=code * lsb, LSB=lsb, V_full_scale=(2 ** n - 1) * lsb)


def _ref_dac_output(n, code, Vref):
    # an R-2R ladder (R = 1) solved as a whole: node k carries bit k-1 through a 2R leg to Vref or ground; 2R end
    # termination; the output is the unloaded MSB node
    el = [("V", n + 1, 0, Vref), ("R", 1, 0, 2.0)]
    for k in range(1, n + 1):
        bit = (code >> (k - 1)) & 1
        el.append(("R", k, n + 1 if bit else 0, 2.0))
        if k < n:
            el.append(("R", k, k + 1, 1.0))
    v, _ = _mna(n + 1, el)
    full = _mna(n + 1, [("V", n + 1, 0, Vref), ("R", 1, 0, 2.0)] + [("R", k, n + 1, 2.0) for k in range(1, n + 1)]
                + [("R", k, k + 1, 1.0) for k in range(1, n)])[0][n].real
    return {"Vout": v[n].real, "LSB": Vref / 2 ** n, "V_full_scale": full}


_add("elx_dac_output",
     name="DAC output voltage (R-2R or any binary DAC)", topic="data converters / DAC",
     purpose="The output voltage of an ideal n-bit unipolar DAC for a code, Vout = Vref D/2^n, its step size and its full-scale output.",
     inputs=[_intin("n", "resolution in bits", 1, 16), _intin("code", "input code (decimal)", 0, 65535), _phys("Vref", "voltage", "reference voltage")],
     outputs=[_o("Vout", "voltage", "output voltage"), _o("LSB", "voltage", "step size Vref/2^n"), _o("V_full_scale", "voltage", "output for the all-ones code")],
     primary="Vout", conditions=["0 <= code <= 2^n - 1"],
     method="Vout = Vref D/2^n; full scale Vref (2^n - 1)/2^n",
     latex=r"V_{out} = V_{ref}\frac{D}{2^n}",
     run=dac_output, reference=_ref_dac_output,
     wrong=lambda n, code, Vref: _out(Vout=Vref * code / (2 ** n - 1), LSB=Vref / (2 ** n - 1), V_full_scale=Vref),
     rand=lambda rng: (lambda n: {"n": n, "code": rng.randint(1, 2 ** n - 1), "Vref": rng.choice([1.0, 2.5, 3.3, 5.0, 10.0])})(rng.randint(2, 14)),
     phrases=("DAC output voltage", "DAC output"),
     examples=[_ex({"n": 8, "code": 200, "Vref": 5.0}, {"Vout": 3.90625, "LSB": 0.01953125}), _ex({"n": 4, "code": 11, "Vref": 10.0}, {"Vout": 6.875})],
     level="beginner")


def dac_binary_weighted(n, code, Vref, R, Rf):
    _need(isinstance(n, int) and 1 <= n <= 12, "the resolution must be a whole number of bits from 1 to 12")
    _need(isinstance(code, int) and 0 <= code <= 2 ** n - 1, "the code must be a whole number from 0 to 2^n - 1")
    _pos("Vref, R and Rf", Vref, R, Rf)
    I = Vref * code / (R * 2 ** (n - 1))
    return _out(Vout=-Rf * I, I_total=I, R_LSB=R * 2 ** (n - 1))


def _ref_dac_binary_weighted(n, code, Vref, R, Rf):
    # each bit k (k = 0 the MSB) switches a resistor R 2^k between Vref and the virtual ground; ideal op-amp
    el = [("V", 1, 0, Vref)]
    for k in range(n):
        bit = (code >> (n - 1 - k)) & 1
        el.append(("R", 1 if bit else 0, 2, R * 2 ** k))
    el += [("R", 2, 3, Rf), ("OA", 3, 0, 2)]
    v, _ = _mna(3, el)
    return {"Vout": v[3].real, "I_total": -v[3].real / Rf, "R_LSB": R * 2 ** (n - 1)}


_add("elx_dac_binary_weighted",
     name="Binary-weighted resistor DAC", topic="data converters / DAC",
     purpose="The output voltage and summing current of a binary-weighted resistor DAC with an inverting op-amp (MSB resistor R, each lower bit doubling it) and the largest resistor needed.",
     inputs=[_intin("n", "resolution in bits", 1, 12), _intin("code", "input code (decimal)", 0, 4095), _phys("Vref", "voltage", "reference voltage"),
             _phys("R", "resistance", "MSB resistor"), _phys("Rf", "resistance", "feedback resistor")],
     outputs=[_o("Vout", "voltage", "output voltage"), _o("I_total", "current", "total current into the summing node"),
              _o("R_LSB", "resistance", "LSB resistor R 2^(n-1)")],
     primary="Vout", conditions=["ideal switches and op-amp"],
     method="I = Vref D/(R 2^(n-1)); Vout = -Rf I",
     latex=r"V_{out} = -V_{ref}\frac{R_f}{R}\sum_{k=0}^{n-1}\frac{b_k}{2^k} = -V_{ref}\frac{R_f}{R}\frac{D}{2^{n-1}}",
     run=dac_binary_weighted, reference=_ref_dac_binary_weighted,
     wrong=lambda n, code, Vref, R, Rf: (lambda I: _out(Vout=-Rf * I, I_total=I, R_LSB=R * 2 ** (n - 1)))(Vref * code / (R * 2 ** n)),
     rand=lambda rng: (lambda n: {"n": n, "code": rng.randint(1, 2 ** n - 1), "Vref": rng.choice([1.0, 2.5, 5.0, 10.0]), "R": _sig(_lu(rng, 1e3, 1e5), 3),
                                  "Rf": _sig(_lu(rng, 1e3, 1e5), 3)})(rng.randint(2, 10)),
     phrases=("Binary-weighted DAC", "Binary weighted resistor DAC"),
     examples=[_ex({"n": 4, "code": 10, "Vref": 5.0, "R": 10e3, "Rf": 5000.0}, {"Vout": -3.125}), _ex({"n": 8, "code": 129, "Vref": 2.5, "R": 1000.0, "Rf": 1000.0})],
     level="intermediate")


def sar_conversion(Vin, Vref, n):
    _pos("Vref", Vref)
    _fin(Vin)
    _need(isinstance(n, int) and 1 <= n <= 16, "the resolution must be a whole number of bits from 1 to 16")
    _need(0.0 <= Vin < Vref, "the input must lie in [0, Vref)")
    lsb = Vref / 2 ** n
    code = 0
    trials = []
    b = n - 1
    while b >= 0:
        t = code + 2 ** b
        trials.append(t * lsb)
        if Vin >= t * lsb:
            code = t
        b -= 1
    return _out(code=code, bits=_bits(code, n), trials=trials, V_dac=code * lsb, cycles=n)


def _ref_sar_conversion(Vin, Vref, n):
    from fractions import Fraction
    code = int((Fraction(Vin) * 2 ** n) // Fraction(Vref))
    lsb = Vref / 2 ** n
    trials = [(((code >> (b + 1)) << (b + 1)) | (1 << b)) * lsb for b in range(n - 1, -1, -1)]
    return {"code": code, "bits": format(code, "0%db" % n), "trials": trials, "V_dac": code * lsb, "cycles": n}


_add("elx_sar_conversion",
     name="Successive-approximation (SAR) ADC conversion", topic="data converters / ADC",
     purpose="The bit-by-bit trial voltages, the final code (decimal and binary) and the DAC voltage of an ideal n-bit successive-approximation ADC converting an input.",
     inputs=[_phys("Vin", "voltage", "analog input"), _phys("Vref", "voltage", "reference voltage"), _intin("n", "resolution in bits", 1, 16)],
     outputs=[_o("code", "number", "final code"), _o("bits", "number", "final code in binary"), _o("trials", "number", "trial DAC voltages, MSB first (V)"),
              _o("V_dac", "voltage", "DAC voltage of the final code"), _o("cycles", "number", "comparison cycles")],
     primary="code", conditions=["0 <= Vin < Vref"],
     method="for each bit from the MSB: set it, keep it if Vin >= DAC(trial); LSB = Vref/2^n",
     latex=r"b_k = \left[V_{in} \ge V_{DAC}(D_{k} + 2^k)\right]",
     run=sar_conversion, reference=_ref_sar_conversion,
     wrong=lambda Vin, Vref, n: (lambda r: dict(r, trials=[_r(t * 2 ** n / (2 ** n - 1)) for t in r["trials"]]))(sar_conversion(Vin, Vref, n)),
     rand=lambda rng: (lambda Vref, n: {"Vin": _sig(Vref * rng.uniform(0.0, 0.999), 8), "Vref": Vref, "n": n})(rng.choice([1.0, 2.5, 3.3, 5.0]), rng.randint(3, 12)),
     phrases=("SAR ADC conversion", "Successive-approximation conversion"),
     examples=[_ex({"Vin": 3.2, "Vref": 5.0, "n": 4}, {"code": 10, "bits": "1010"}), _ex({"Vin": 1.7, "Vref": 3.3, "n": 8})],
     level="advanced")


def dual_slope(Vin, Vref, N1, fclk, R, C):
    _pos("Vin, Vref, fclk, R and C", Vin, Vref, fclk, R, C)
    _need(isinstance(N1, int) and N1 >= 1, "the fixed count N1 must be a whole number")
    _need(Vin <= Vref, "the input must not exceed the reference")
    T1 = N1 / fclk
    return _out(T1=T1, T2=T1 * Vin / Vref, N2=N1 * Vin / Vref, V_peak=Vin * T1 / (R * C))


def _ref_dual_slope(Vin, Vref, N1, fclk, R, C):
    T1 = N1 / fclk
    vp = _quad(lambda t: Vin / (R * C), 0.0, T1)                       # run-up: integrate the input for T1
    T2 = _brentq(lambda t: vp - Vref * t / (R * C), 0.0, 2.0 * T1)     # run-down with the reference to zero
    return {"T1": T1, "T2": T2, "N2": T2 * fclk, "V_peak": vp}


_add("elx_dual_slope_adc",
     name="Dual-slope integrating ADC", topic="data converters / ADC",
     purpose="The fixed integration time, the de-integration time and count, and the peak integrator voltage of a dual-slope ADC.",
     inputs=[_phys("Vin", "voltage", "input voltage (magnitude)"), _phys("Vref", "voltage", "reference voltage (magnitude)"),
             _intin("N1", "fixed count of the integration phase", 1, 10 ** 9), _phys("fclk", "frequency", "clock frequency"),
             _phys("R", "resistance", "integrator resistor"), _phys("C", "capacitance", "integrator capacitor", show=("uF", 1e6))],
     outputs=[_o("T1", "time", "integration time N1/fclk"), _o("T2", "time", "de-integration time"), _o("N2", "number", "count of the de-integration phase (unrounded)"),
              _o("V_peak", "voltage", "peak integrator output")],
     primary="T2", conditions=["0 < Vin <= Vref"],
     method="T1 = N1/fclk; T2 = T1 Vin/Vref; N2 = N1 Vin/Vref; V_peak = Vin T1/(R C)",
     latex=r"T_2 = T_1\frac{V_{in}}{V_{ref}},\quad N_2 = N_1\frac{V_{in}}{V_{ref}}",
     run=dual_slope, reference=_ref_dual_slope,
     wrong=lambda Vin, Vref, N1, fclk, R, C: (lambda T1: _out(T1=T1, T2=T1 * Vref / Vin, N2=N1 * Vref / Vin, V_peak=Vin * T1 / (R * C)))(N1 / fclk),
     rand=lambda rng: (lambda Vref: {"Vin": _sig(Vref * rng.uniform(0.05, 0.95), 4), "Vref": Vref, "N1": rng.choice([1000, 2000, 4096, 10000]),
                                     "fclk": _sig(_lu(rng, 1e4, 1e7), 3), "R": _sig(_lu(rng, 1e4, 1e6), 3), "C": _sig(_lu(rng, 1e-8, 1e-6), 3)})(rng.choice([1.0, 2.0, 2.5])),
     phrases=("Dual-slope ADC", "Dual slope integrating ADC"),
     examples=[_ex({"Vin": 0.5, "Vref": 1.0, "N1": 1000, "fclk": 100e3, "R": 100e3, "C": 0.1e-6}, {"T1": 0.01, "T2": 0.005, "N2": 500.0}),
               _ex({"Vin": 1.234, "Vref": 2.0, "N1": 2000, "fclk": 1e6, "R": 47e3, "C": 0.22e-6})],
     level="advanced")


def sample_hold(I_leak, C, t_hold, Vref, n):
    _pos("I_leak, C, t_hold and Vref", I_leak, C, t_hold, Vref)
    _need(isinstance(n, int) and 1 <= n <= 24, "the resolution must be a whole number of bits from 1 to 24")
    rate = I_leak / C
    lsb = Vref / 2 ** n
    return _out(droop_rate=rate, droop=rate * t_hold, droop_lsb=rate * t_hold / lsb, t_max_half_lsb=lsb / 2.0 / rate)


def _ref_sample_hold(I_leak, C, t_hold, Vref, n):
    lsb = Vref / 2 ** n
    dv = _quad(lambda t: I_leak / C, 0.0, t_hold)                         # dV/dt = I/C integrated
    tmax = _brentq(lambda t: I_leak * t / C - lsb / 2.0, 0.0, 1e3 * lsb * C / I_leak)
    return {"droop_rate": I_leak / C, "droop": dv, "droop_lsb": dv / lsb, "t_max_half_lsb": tmax}


_add("elx_sample_hold_droop",
     name="Sample-and-hold droop", topic="data converters / sampling",
     purpose="The droop rate I/C of a sample-and-hold capacitor discharged by a leakage current, the droop over a hold time (in volts and LSB) and the longest hold that keeps the droop within half an LSB of an n-bit converter.",
     inputs=[_phys("I_leak", "current", "leakage (bias) current", show=("nA", 1e9)), _phys("C", "capacitance", "hold capacitor", show=("nF", 1e9)),
             _phys("t_hold", "time", "hold time", show=("us", 1e6)), _phys("Vref", "voltage", "converter full scale"), _intin("n", "converter resolution in bits", 1, 24)],
     outputs=[_o("droop_rate", "voltage_slew_rate", "droop rate I/C"), _o("droop", "voltage", "droop over the hold time"),
              _o("droop_lsb", "number", "droop in LSB"), _o("t_max_half_lsb", "time", "longest hold for half an LSB")],
     primary="droop", conditions=["constant leakage current"],
     method="dV/dt = I/C; dV = I t/C; t_max = (Vref/2^(n+1)) C/I",
     latex=r"\frac{dV}{dt} = \frac{I_{leak}}{C}",
     run=sample_hold, reference=_ref_sample_hold,
     wrong=lambda I_leak, C, t_hold, Vref, n: (lambda r: dict(r, droop=_r(I_leak * C * t_hold)))(sample_hold(I_leak, C, t_hold, Vref, n)),
     rand=lambda rng: {"I_leak": _sig(_lu(rng, 1e-12, 1e-7), 3), "C": _sig(_lu(rng, 1e-11, 1e-7), 3), "t_hold": _sig(_lu(rng, 1e-7, 1e-3), 3),
                       "Vref": rng.choice([2.5, 5.0, 10.0]), "n": rng.randint(8, 18)},
     phrases=("Sample-and-hold droop", "Sample and hold droop"),
     examples=[_ex({"I_leak": 1e-9, "C": 1e-9, "t_hold": 10e-6, "Vref": 5.0, "n": 12}, {"droop_rate": 1.0, "droop": 1e-05}),
               _ex({"I_leak": 50e-12, "C": 100e-12, "t_hold": 20e-6, "Vref": 10.0, "n": 16})],
     level="intermediate",
     weak="the droop integrated by quadrature and the hold limit by root finding (the same constant-current model)")


# ================================================================================================ TIMING
def rc_timing(R, C):
    _pos("R and C", R, C)
    tau = R * C
    tr = math.log(9.0) * tau
    f3 = 1.0 / (TWO_PI * tau)
    return _out(tau=tau, t50=LN2 * tau, t_rise=tr, f_3dB=f3, tr_times_bw=tr * f3)


def _ref_rc_timing(R, C):
    tau = R * C
    cross = lambda lvl: _brentq(lambda t: 1.0 - math.exp(-t / tau) - lvl, 0.0, 50.0 * tau)
    t10, t50, t90 = cross(0.1), cross(0.5), cross(0.9)
    f3 = _brentq(lambda f: abs(1.0 / complex(1.0, TWO_PI * f * tau)) - 1.0 / math.sqrt(2.0), 0.0, 10.0 / tau)
    return {"tau": tau, "t50": t50, "t_rise": t90 - t10, "f_3dB": f3, "tr_times_bw": (t90 - t10) * f3}


_add("elx_rc_delay_rise_time",
     name="RC delay, 10-90 % rise time and bandwidth", topic="timing / RC",
     purpose="The time constant, the 50 % delay (ln 2 RC), the 10-90 % rise time (ln 9 RC = 2.2 RC), the -3 dB bandwidth and the rise-time-bandwidth product (0.35) of a first-order RC low-pass.",
     inputs=[_phys("R", "resistance", "resistance"), _phys("C", "capacitance", "capacitance", show=("pF", 1e12))],
     outputs=[_o("tau", "time", "time constant RC"), _o("t50", "time", "50 % delay"), _o("t_rise", "time", "10-90 % rise time"),
              _o("f_3dB", "frequency", "-3 dB bandwidth"), _o("tr_times_bw", "number", "rise time x bandwidth")],
     primary="t_rise", conditions=["step input, first-order RC"],
     method="t50 = ln 2 RC; tr = ln 9 RC; f = 1/(2 pi RC); tr f = ln 9/(2 pi)",
     latex=r"t_r = \ln 9\,RC \approx 2.2RC,\quad t_rf_{3dB} \approx 0.35",
     run=rc_timing, reference=_ref_rc_timing,
     wrong=lambda R, C: (lambda r: dict(r, t_rise=_r(math.log(10.0) * R * C), tr_times_bw=_r(math.log(10.0) / TWO_PI)))(rc_timing(R, C)),
     rand=lambda rng: {"R": _sig(_lu(rng, 10.0, 1e6), 3), "C": _sig(_lu(rng, 1e-13, 1e-5), 3)},
     phrases=("RC delay and rise time", "RC rise time"),
     examples=[_ex({"R": 1000.0, "C": 10e-12}), _ex({"R": 50.0, "C": 100e-12})],
     level="beginner")


def elmore(R, C):
    _need(len(R) == len(C) and len(R) >= 1, "give one capacitance per resistor segment")
    path = []
    s = 0.0
    for r in R:
        _pos("every resistance", r)
        s += r
        path.append(s)
    for c in C:
        _nonneg("every capacitance", c)
    delays = []
    for k in range(len(R)):
        d = 0.0
        for i in range(len(C)):
            d += C[i] * 1e-12 * path[min(i, k)]
        delays.append(d)
    return _out(tau_D=delays[-1], t50=LN2 * delays[-1], delays=delays)


def _ref_elmore(R, C):
    # the Elmore delay at node k is the first moment of its impulse response: (G^-1 C)_k for the ladder's
    # conductance matrix G (tridiagonal) and node capacitances C
    import numpy as np
    n = len(R)
    G = np.zeros((n, n))
    for k in range(n):
        g = 1.0 / R[k]
        G[k, k] += g
        if k > 0:
            G[k - 1, k - 1] += g
            G[k, k - 1] -= g
            G[k - 1, k] -= g
    d = np.linalg.solve(G, np.array(C) * 1e-12)
    delays = [float(x) for x in d]
    return {"tau_D": delays[-1], "t50": LN2 * delays[-1], "delays": delays}


_add("elx_elmore_delay",
     name="Elmore delay of an RC ladder (interconnect)", topic="timing / interconnect",
     purpose="The Elmore delay to every node of an RC ladder (a driver resistance and wire segments, each followed by a capacitance to ground), the delay at the far end and its 50 % estimate ln 2 tau.",
     inputs=[_vecin("R", "series resistances in ohms, from the driver outwards", 1, 64), _vecin("C", "node capacitances in picofarads, one after each resistor", 1, 64)],
     outputs=[_o("tau_D", "time", "Elmore delay at the far end"), _o("t50", "time", "50 % delay estimate ln 2 tau_D"),
              _o("delays", "number", "Elmore delay at each node, seconds")],
     primary="tau_D", conditions=["one capacitance per resistor", "step input"],
     method="tau_k = sum_i C_i R(shared path of nodes i and k)",
     latex=r"\tau_D = \sum_i C_i\sum_{j\le i}R_j",
     run=elmore, reference=_ref_elmore,
     wrong=lambda R, C: (lambda d: _out(tau_D=d, t50=LN2 * d, delays=[r * c * 1e-12 for r, c in zip(R, C)]))(sum(r * c * 1e-12 for r, c in zip(R, C))),
     rand=lambda rng: (lambda n: {"R": [_sig(_lu(rng, 1.0, 1e4), 3) for _ in range(n)], "C": [_sig(_lu(rng, 0.001, 10.0), 3) for _ in range(n)]})(rng.randint(2, 10)),
     phrases=("Elmore delay", "Elmore delay of an RC ladder"),
     examples=[_ex({"R": [100.0, 50.0, 50.0], "C": [0.1, 0.1, 0.2]}), _ex({"R": [1000.0, 200.0], "C": [0.05, 0.5]})],
     level="advanced")


def cmos_power(alpha, C, VDD, f):
    _pos("C, VDD and f", C, VDD, f)
    _need(0.0 <= alpha <= 1.0, "the activity factor must be between 0 and 1")
    P = alpha * C * VDD * VDD * f
    return _out(P=P, E_supply=C * VDD * VDD, E_stored=C * VDD * VDD / 2.0, I_avg=P / VDD)


def _ref_cmos_power(alpha, C, VDD, f):
    # energy drawn from the supply while charging C through any resistance: integral of VDD i(t)
    Rx = 1000.0
    tau = Rx * C
    E = _quad(lambda t: VDD * VDD / Rx * math.exp(-t / tau), 0.0, 80.0 * tau)
    Es = _quad(lambda t: VDD * VDD / Rx * math.exp(-t / tau) * (1.0 - math.exp(-t / tau)), 0.0, 80.0 * tau)
    P = alpha * E * f
    return {"P": P, "E_supply": E, "E_stored": Es, "I_avg": P / VDD}


_add("elx_cmos_dynamic_power",
     name="CMOS dynamic (switching) power", topic="timing / power",
     purpose="The dynamic power alpha C VDD^2 f of CMOS logic, the energy drawn from the supply per 0 -> 1 transition, the energy stored on the load and the average supply current.",
     inputs=[_numin("alpha", "activity factor: probability of a 0 -> 1 transition per clock cycle", pct=True),
             _phys("C", "capacitance", "switched capacitance", show=("pF", 1e12)), _phys("VDD", "voltage", "supply voltage"),
             _phys("f", "frequency", "clock frequency")],
     outputs=[_o("P", "power", "dynamic power"), _o("E_supply", "energy", "energy from the supply per transition, C VDD^2"),
              _o("E_stored", "energy", "energy stored on C, C VDD^2/2"), _o("I_avg", "current", "average supply current")],
     primary="P", conditions=["0 <= alpha <= 1", "full-swing switching, short-circuit and leakage power not included"],
     method="P = alpha C VDD^2 f",
     latex=r"P_{dyn} = \alpha\,C\,V_{DD}^2f",
     run=cmos_power, reference=_ref_cmos_power,
     wrong=lambda alpha, C, VDD, f: (lambda P: _out(P=P, E_supply=C * VDD, E_stored=C * VDD / 2, I_avg=P / VDD))(alpha * C * VDD * f),
     rand=lambda rng: {"alpha": _sig(rng.uniform(0.01, 1.0), 3), "C": _sig(_lu(rng, 1e-15, 1e-8), 3), "VDD": _sig(rng.uniform(0.6, 5.0), 3),
                       "f": _sig(_lu(rng, 1e3, 5e9), 3)},
     phrases=("CMOS dynamic power", "CMOS switching power"),
     examples=[_ex({"alpha": 0.1, "C": 1e-9, "VDD": 1.0, "f": 1e9}, {"P": 0.1}), _ex({"alpha": 0.25, "C": 50e-12, "VDD": 3.3, "f": 100e6})],
     level="beginner")


def setup_hold(T, tcq, tpd, tsu, tskew, tccq, tcd, th):
    _pos("the clock period T", T)
    _nonneg("tcq, tpd, tsu, tccq, tcd and th", tcq, tpd, tsu, tccq, tcd, th)
    _fin(tskew)
    ss = T + tskew - (tcq + tpd + tsu)
    hs = tccq + tcd - th - tskew
    need = tcq + tpd + tsu - tskew
    _need(need > 0, "the path needs no time: f_max is undefined")
    if ss >= 0 and hs >= 0:
        v = "meets timing"
    elif ss < 0 and hs < 0:
        v = "setup and hold violations"
    elif ss < 0:
        v = "setup violation"
    else:
        v = "hold violation"
    return _out(setup_slack=ss, hold_slack=hs, f_max=1.0 / need, verdict=v)


def _ref_setup_hold(T, tcq, tpd, tsu, tskew, tccq, tcd, th):
    # event times: launch edge at 0, capture edge at T + skew; data arrives at tcq + tpd and must settle tsu before
    # the capture edge; the earliest change (tccq + tcd) must come th after the capture clock's own edge (skew)
    arrive, required = tcq + tpd, T + tskew - tsu
    earliest, hold_req = tccq + tcd, tskew + th
    ss, hs = required - arrive, earliest - hold_req
    Tmin = _brentq(lambda P: (P + tskew - tsu) - arrive, -1e3, 1e3)
    v = "meets timing" if ss >= 0 and hs >= 0 else ("setup and hold violations" if ss < 0 and hs < 0 else ("setup violation" if ss < 0 else "hold violation"))
    return {"setup_slack": ss, "hold_slack": hs, "f_max": 1.0 / Tmin, "verdict": v}


_add("elx_setup_hold_timing",
     name="Flip-flop setup and hold slack, maximum clock frequency", topic="timing / synchronous logic",
     purpose="The setup slack, hold slack, maximum clock frequency and a verdict for a register-to-register path from clock-to-Q, logic delays, setup and hold times and clock skew (positive when the capture clock is late).",
     inputs=[_phys("T", "time", "clock period", show=("ns", 1e9)), _phys("tcq", "time", "maximum clock-to-Q delay", show=("ns", 1e9)),
             _phys("tpd", "time", "maximum logic delay", show=("ns", 1e9)), _phys("tsu", "time", "setup time", show=("ns", 1e9)),
             _phys("tskew", "time", "clock skew, capture minus launch", show=("ns", 1e9)), _phys("tccq", "time", "minimum (contamination) clock-to-Q", show=("ns", 1e9)),
             _phys("tcd", "time", "minimum (contamination) logic delay", show=("ns", 1e9)), _phys("th", "time", "hold time", show=("ns", 1e9))],
     outputs=[_o("setup_slack", "time", "setup slack"), _o("hold_slack", "time", "hold slack"), _o("f_max", "frequency", "maximum clock frequency"),
              _o("verdict", "number", "meets timing, setup violation, hold violation or both")],
     primary="setup_slack", conditions=["edge-triggered registers, single clock"],
     method="setup slack = T + skew - (tcq + tpd + tsu); hold slack = tccq + tcd - th - skew; f_max = 1/(tcq + tpd + tsu - skew)",
     latex=r"T \ge t_{cq} + t_{pd} + t_{su} - t_{skew},\quad t_{ccq} + t_{cd} \ge t_h + t_{skew}",
     run=setup_hold, reference=_ref_setup_hold,
     wrong=lambda T, tcq, tpd, tsu, tskew, tccq, tcd, th: (lambda r: dict(r, setup_slack=_r(T - tskew - (tcq + tpd + tsu)), hold_slack=_r(tccq + tcd - th + tskew)))(
         setup_hold(T, tcq, tpd, tsu, tskew, tccq, tcd, th)),
     rand=lambda rng: {"T": _sig(_lu(rng, 1e-9, 2e-8), 3), "tcq": _sig(_lu(rng, 5e-11, 5e-10), 3), "tpd": _sig(_lu(rng, 2e-10, 1e-8), 3),
                       "tsu": _sig(_lu(rng, 2e-11, 2e-10), 3), "tskew": _sig(rng.uniform(-3e-10, 3e-10), 3), "tccq": _sig(_lu(rng, 2e-11, 2e-10), 3),
                       "tcd": _sig(_lu(rng, 1e-11, 3e-10), 3), "th": _sig(_lu(rng, 1e-11, 1e-10), 3)},
     phrases=("Setup and hold timing", "Flip-flop timing check"),
     examples=[_ex({"T": 5e-9, "tcq": 0.3e-9, "tpd": 3.5e-9, "tsu": 0.2e-9, "tskew": 0.1e-9, "tccq": 0.1e-9, "tcd": 0.2e-9, "th": 0.1e-9}),
               _ex({"T": 2e-9, "tcq": 0.2e-9, "tpd": 1.9e-9, "tsu": 0.1e-9, "tskew": -0.05e-9, "tccq": 0.05e-9, "tcd": 0.02e-9, "th": 0.1e-9})],
     level="intermediate",
     weak="the same timing equations written as arrival and required event times (f_max by root finding)")


def mtbf(tr, tau, T0, fclk, fdata):
    _pos("tr, tau, T0, fclk and fdata", tr, tau, T0, fclk, fdata)
    _need(tr / tau <= 700.0, "tr/tau is beyond the range of a double-precision exponential")
    m = math.exp(tr / tau) / (T0 * fclk * fdata)
    return _out(MTBF_s=m, MTBF_years=m / (365.25 * 86400.0), log10_MTBF_s=tr / tau / math.log(10.0) - math.log10(T0 * fclk * fdata))


def _ref_mtbf(tr, tau, T0, fclk, fdata):
    import numpy as np
    lg = float(np.log10(np.e) * tr / tau - np.log10(T0) - np.log10(fclk) - np.log10(fdata))
    m = 10.0 ** lg
    return {"MTBF_s": m, "MTBF_years": m / (365.25 * 86400.0), "log10_MTBF_s": lg}


_add("elx_metastability_mtbf",
     name="Synchronizer metastability MTBF", topic="timing / synchronous logic",
     purpose="The mean time between synchronizer failures MTBF = exp(tr/tau)/(T0 fclk fdata) from the resolution time allowed, the flip-flop's time constant and window, and the clock and data rates.",
     inputs=[_phys("tr", "time", "resolution time available", show=("ns", 1e9)), _phys("tau", "time", "metastability time constant", show=("ps", 1e12)),
             _phys("T0", "time", "metastability window T0", show=("ps", 1e12)), _phys("fclk", "frequency", "clock frequency"),
             _phys("fdata", "frequency", "rate of asynchronous data changes")],
     outputs=[_o("MTBF_s", "time", "MTBF"), _o("MTBF_years", "number", "MTBF in years (365.25 days)"), _o("log10_MTBF_s", "number", "log10 of MTBF in seconds")],
     primary="log10_MTBF_s", conditions=["tr/tau <= 700"],
     method="MTBF = exp(tr/tau)/(T0 fclk fdata)",
     latex=r"\text{MTBF} = \frac{e^{t_r/\tau}}{T_0f_{clk}f_{data}}",
     run=mtbf, reference=_ref_mtbf,
     wrong=lambda tr, tau, T0, fclk, fdata: (lambda m: _out(MTBF_s=m, MTBF_years=m / (365.25 * 86400.0), log10_MTBF_s=math.log10(m)))(math.exp(tr / tau) / (fclk * fdata)),
     rand=lambda rng: (lambda tau: {"tr": _sig(tau * rng.uniform(5.0, 100.0), 3), "tau": tau, "T0": _sig(_lu(rng, 1e-11, 1e-9), 3),
                                    "fclk": _sig(_lu(rng, 1e7, 2e9), 3), "fdata": _sig(_lu(rng, 1e5, 1e8), 3)})(_sig(_lu(rng, 1e-11, 2e-10), 3)),
     phrases=("Metastability MTBF", "Synchronizer MTBF"),
     examples=[_ex({"tr": 2e-9, "tau": 50e-12, "T0": 100e-12, "fclk": 500e6, "fdata": 10e6}), _ex({"tr": 1e-9, "tau": 20e-12, "T0": 50e-12, "fclk": 1e9, "fdata": 50e6})],
     level="expert",
     weak="the same expression evaluated in base-10 logarithms with numpy")


# ================================================================================================ POWER SUPPLIES
def linear_regulator(Vin, Vout, Iload, Iq):
    _pos("Vin, Vout and Iload", Vin, Vout, Iload)
    _nonneg("the quiescent current Iq", Iq)
    _need(Vin > Vout, "the input must exceed the output")
    Pin = Vin * (Iload + Iq)
    Pout = Vout * Iload
    return _out(P_in=Pin, P_out=Pout, P_diss=Pin - Pout, efficiency=Pout / Pin, efficiency_pct=100.0 * Pout / Pin)


def _ref_linear_regulator(Vin, Vout, Iload, Iq):
    # the regulator as a netlist: the input source feeds the pass element (a voltage drop) and the ground pin (Iq)
    el = [("V", 1, 0, Vin), ("V", 1, 2, Vin - Vout), ("R", 2, 0, Vout / Iload), ("I", 1, 0, Iq)]
    v, cur = _mna(2, el)
    Pin = Vin * -cur[0].real
    Pout = v[2].real ** 2 / (Vout / Iload)
    return {"P_in": Pin, "P_out": Pout, "P_diss": Pin - Pout, "efficiency": Pout / Pin, "efficiency_pct": 100.0 * Pout / Pin}


_add("elx_linear_regulator_efficiency",
     name="Linear regulator dissipation and efficiency", topic="power supplies / linear regulators",
     purpose="The input and output power, the dissipation in the pass element and the efficiency of a linear (series) regulator including its quiescent current.",
     inputs=[_phys("Vin", "voltage", "input voltage"), _phys("Vout", "voltage", "regulated output voltage"),
             _phys("Iload", "current", "load current", show=("mA", 1e3)), _phys("Iq", "current", "quiescent (ground-pin) current", show=("mA", 1e3))],
     outputs=[_o("P_in", "power", "input power"), _o("P_out", "power", "output power"), _o("P_diss", "power", "dissipation in the regulator"),
              _o("efficiency", "number", "efficiency"), _o("efficiency_pct", "number", "efficiency in per cent")],
     primary="P_diss", conditions=["Vin > Vout"],
     method="P_in = Vin (IL + Iq); P_out = Vout IL; P_diss = P_in - P_out; eta = P_out/P_in",
     latex=r"P_D = (V_{in} - V_{out})I_L + V_{in}I_q,\quad \eta = \frac{V_{out}I_L}{V_{in}(I_L + I_q)}",
     run=linear_regulator, reference=_ref_linear_regulator,
     wrong=lambda Vin, Vout, Iload, Iq: _out(P_in=Vin * Iload, P_out=Vout * Iload, P_diss=(Vin - Vout) * Iload, efficiency=Vout / Vin, efficiency_pct=100 * Vout / Vin),
     rand=lambda rng: (lambda Vout: {"Vin": _sig(Vout + rng.uniform(0.3, 15.0), 3), "Vout": Vout, "Iload": _sig(_lu(rng, 1e-3, 3.0), 3),
                                     "Iq": _sig(_lu(rng, 1e-5, 1e-2), 3)})(_sig(rng.uniform(1.2, 15.0), 3)),
     phrases=("Linear regulator efficiency", "Linear regulator dissipation"),
     examples=[_ex({"Vin": 12.0, "Vout": 5.0, "Iload": 0.5, "Iq": 5e-3}), _ex({"Vin": 5.0, "Vout": 3.3, "Iload": 0.2, "Iq": 50e-6})],
     level="beginner",
     weak="the power balance read from a netlist of the regulator (the same model)")


def adj_regulator(R1, R2, Vref, Iadj=0.0):
    _pos("R1, R2 and Vref", R1, R2, Vref)
    _nonneg("Iadj", Iadj)
    return _out(Vout=Vref * (1.0 + R2 / R1) + Iadj * R2, I_R1=Vref / R1)


def _ref_adj_regulator(R1, R2, Vref, Iadj=0.0):
    # the regulator holds OUT at ADJ + Vref and sources its output current; ADJ draws Iadj into the divider
    el = [("R", 1, 2, R1), ("R", 2, 0, R2), ("I", 0, 2, Iadj), ("E", 3, 0, 2, 0, 1.0), ("V", 4, 3, Vref), ("OA", 1, 4, 1)]
    v, _ = _mna(4, el)
    return {"Vout": v[1].real, "I_R1": (v[1] - v[2]).real / R1}


_add("elx_adjustable_regulator_output",
     name="Adjustable three-terminal regulator output voltage", topic="power supplies / linear regulators",
     purpose="The output voltage of an adjustable three-terminal regulator (LM317 type: reference between OUT and ADJ, R1 from OUT to ADJ, R2 from ADJ to ground), including the adjust-pin current.",
     inputs=[_phys("R1", "resistance", "resistor from OUT to ADJ"), _phys("R2", "resistance", "resistor from ADJ to ground"),
             _phys("Vref", "voltage", "internal reference (1.25 V for the LM317 type)"),
             _phys("Iadj", "current", "adjust-pin current (default 0)", required=False, show=("uA", 1e6))],
     outputs=[_o("Vout", "voltage", "output voltage"), _o("I_R1", "current", "current through R1")],
     primary="Vout", conditions=["regulator in regulation (input high enough)"],
     method="Vout = Vref (1 + R2/R1) + Iadj R2",
     latex=r"V_{out} = V_{ref}\left(1 + \frac{R_2}{R_1}\right) + I_{adj}R_2",
     run=adj_regulator, reference=_ref_adj_regulator,
     wrong=lambda R1, R2, Vref, Iadj=0.0: _out(Vout=Vref * R2 / R1 + Iadj * R2, I_R1=Vref / R1),
     rand=lambda rng: {"R1": _sig(_lu(rng, 100.0, 1000.0), 3), "R2": _sig(_lu(rng, 100.0, 1e4), 3), "Vref": rng.choice([1.25, 1.24, 1.2]),
                       **({"Iadj": _sig(rng.uniform(10e-6, 100e-6), 2)} if rng.random() < 0.5 else {})},
     phrases=("Adjustable regulator output", "LM317 output voltage"),
     examples=[_ex({"R1": 240.0, "R2": 720.0, "Vref": 1.25}, {"Vout": 5.0}), _ex({"R1": 240.0, "R2": 2000.0, "Vref": 1.25, "Iadj": 50e-6})],
     level="beginner")


def series_pass(Vin, Vz, Rs, RL, beta, VBE):
    _pos("Vin, Vz, Rs, RL, beta and VBE", Vin, Vz, Rs, RL, beta, VBE)
    _need(Vin > Vz > VBE, "need Vin > Vz > VBE")
    Vo = Vz - VBE
    IL = Vo / RL
    IB = IL / (beta + 1.0)
    IRs = (Vin - Vz) / Rs
    Iz = IRs - IB
    _need(Iz > 0, "the Zener is starved (Rs too large for this load): no regulation")
    IC = beta * IB
    return _out(Vout=Vo, I_L=IL, I_B=IB, I_z=Iz, I_Rs=IRs, P_Q=(Vin - Vo) * IC)


def _ref_series_pass(Vin, Vz, Rs, RL, beta, VBE):
    el = [("V", 1, 0, Vin), ("R", 1, 2, Rs), ("V", 2, 0, Vz), ("R", 3, 0, RL)]
    k = _bjt(el, 2, 1, 3, VBE, beta)
    v, cur = _mna(3, el)
    IB = cur[k].real
    Vo = v[3].real
    return {"Vout": Vo, "I_L": Vo / RL, "I_B": IB, "I_z": cur[2].real, "I_Rs": (Vin - Vz) / Rs, "P_Q": (Vin - Vo) * beta * IB}


_add("elx_series_pass_regulator",
     name="Zener-referenced series-pass (emitter-follower) regulator", topic="power supplies / linear regulators",
     purpose="The output voltage, load, base, Zener and resistor currents and the pass-transistor dissipation of a regulator in which a Zener diode biased through Rs sets the base of an emitter-follower pass transistor.",
     inputs=[_phys("Vin", "voltage", "unregulated input"), _phys("Vz", "voltage", "Zener voltage"), _phys("Rs", "resistance", "Zener bias resistor"),
             _phys("RL", "resistance", "load resistance"), _numin("beta", "current gain of the pass transistor"), _phys("VBE", "voltage", "base-emitter voltage")],
     outputs=[_o("Vout", "voltage", "output voltage Vz - VBE"), _o("I_L", "current", "load current"), _o("I_B", "current", "base current"),
              _o("I_z", "current", "Zener current"), _o("I_Rs", "current", "current in Rs"), _o("P_Q", "power", "pass-transistor dissipation")],
     primary="Vout", conditions=["Vin > Vz > VBE", "the Zener stays in breakdown (Iz > 0)"],
     method="Vout = Vz - VBE; IB = IL/(beta + 1); Iz = (Vin - Vz)/Rs - IB; P = (Vin - Vout) IC",
     latex=r"V_{out} = V_Z - V_{BE},\quad I_Z = \frac{V_{in} - V_Z}{R_S} - \frac{I_L}{\beta+1}",
     run=series_pass, reference=_ref_series_pass,
     wrong=lambda Vin, Vz, Rs, RL, beta, VBE: (lambda r: dict(r, Vout=_r(Vz), I_L=_r(Vz / RL)))(series_pass(Vin, Vz, Rs, RL, beta, VBE)),
     rand=lambda rng: (lambda Vz, RL, beta: {"Vin": _sig(Vz + rng.uniform(2.0, 15.0), 3), "Vz": Vz, "Rs": _sig(rng.uniform(0.1, 0.8) * 2.0 * (beta + 1) * RL / max(Vz, 1.0), 3),
                                             "RL": RL, "beta": beta, "VBE": _sig(rng.uniform(0.6, 0.75), 2)})(
         _sig(rng.uniform(3.0, 15.0), 3), _sig(_lu(rng, 10.0, 1000.0), 3), _sig(rng.uniform(30.0, 200.0), 3)),
     phrases=("Series-pass regulator", "Zener series pass regulator"),
     examples=[_ex({"Vin": 15.0, "Vz": 10.0, "Rs": 1000.0, "RL": 100.0, "beta": 100.0, "VBE": 0.7}),
               _ex({"Vin": 20.0, "Vz": 12.7, "Rs": 470.0, "RL": 50.0, "beta": 60.0, "VBE": 0.7})],
     level="advanced")


def heat_sink(P, Ta, theta_jc, theta_cs, theta_sa):
    _pos("P, Ta and the thermal resistances", P, Ta, theta_jc, theta_cs, theta_sa)
    Ts = Ta + P * theta_sa
    Tc = Ts + P * theta_cs
    Tj = Tc + P * theta_jc
    return _out(Tj=Tj, Tc=Tc, Ts=Ts, Tj_C=Tj - T0C, theta_ja=theta_jc + theta_cs + theta_sa)


def _ref_heat_sink(P, Ta, theta_jc, theta_cs, theta_sa):
    # thermal network: power as a current source into the junction, resistances to the ambient (a voltage source)
    el = [("I", 0, 1, P), ("R", 1, 2, theta_jc), ("R", 2, 3, theta_cs), ("R", 3, 4, theta_sa), ("V", 4, 0, Ta)]
    v, _ = _mna(4, el)
    vv = _re(v)
    return {"Tj": vv[1], "Tc": vv[2], "Ts": vv[3], "Tj_C": vv[1] - T0C, "theta_ja": (vv[1] - Ta) / P}


_add("elx_heat_sink_temperatures",
     name="Heat-sink thermal path: junction, case and sink temperatures", topic="power supplies / thermal design",
     purpose="The junction, case and heat-sink temperatures of a power device dissipating P through junction-to-case, case-to-sink (interface) and sink-to-ambient thermal resistances.",
     inputs=[_phys("P", "power", "power dissipated"), _phys("Ta", "temperature", "ambient temperature", show=("degC", 1.0)),
             _phys("theta_jc", "thermal_resistance", "junction-to-case thermal resistance"), _phys("theta_cs", "thermal_resistance", "case-to-sink thermal resistance"),
             _phys("theta_sa", "thermal_resistance", "sink-to-ambient thermal resistance")],
     outputs=[_o("Tj", "temperature", "junction temperature"), _o("Tc", "temperature", "case temperature"), _o("Ts", "temperature", "sink temperature"),
              _o("Tj_C", "number", "junction temperature in degC"), _o("theta_ja", "thermal_resistance", "total junction-to-ambient resistance")],
     primary="Tj", conditions=["steady state, series thermal path"],
     method="Tj = Ta + P (theta_jc + theta_cs + theta_sa)",
     latex=r"T_j = T_a + P\,(\theta_{jc} + \theta_{cs} + \theta_{sa})",
     run=heat_sink, reference=_ref_heat_sink,
     wrong=lambda P, Ta, theta_jc, theta_cs, theta_sa: (lambda Tj: _out(Tj=Tj, Tc=Tj, Ts=Ta + P * theta_sa, Tj_C=Tj - T0C, theta_ja=theta_sa))(Ta + P * theta_sa),
     rand=lambda rng: {"P": _sig(_lu(rng, 0.5, 100.0), 3), "Ta": _sig(rng.uniform(273.15, 333.15), 5), "theta_jc": _sig(_lu(rng, 0.2, 5.0), 3),
                       "theta_cs": _sig(_lu(rng, 0.05, 1.0), 3), "theta_sa": _sig(_lu(rng, 0.5, 20.0), 3)},
     phrases=("Heat sink temperatures", "Junction temperature with heat sink"),
     examples=[_ex({"P": 15.0, "Ta": 298.15, "theta_jc": 1.5, "theta_cs": 0.5, "theta_sa": 3.0}, {"Tj_C": 100.0}),
               _ex({"P": 40.0, "Ta": 313.15, "theta_jc": 0.8, "theta_cs": 0.2, "theta_sa": 1.2})],
     level="intermediate",
     extra_q=[{"q": "Heat sink temperatures: P = 15 W, Ta = 25 degC, theta_jc = 1.5 degC/W, theta_cs = 0.5 degC/W, theta_sa = 3 degC/W", "like": 0}])


def heat_sink_required(P, Ta, Tj_max, theta_jc, theta_cs):
    _pos("P, Ta, Tj_max and the thermal resistances", P, Ta, Tj_max, theta_jc, theta_cs)
    tja = (Tj_max - Ta) / P
    tsa = tja - theta_jc - theta_cs
    _need(tsa > 0, "no heat sink can keep the junction below Tj_max at this power (the device and interface alone are too hot)")
    return _out(theta_ja_max=tja, theta_sa_max=tsa)


def _ref_heat_sink_required(P, Ta, Tj_max, theta_jc, theta_cs):
    tsa = _brentq(lambda t: _ref_heat_sink(P, Ta, theta_jc, theta_cs, t)["Tj"] - Tj_max, 1e-12, 1e6)
    return {"theta_ja_max": theta_jc + theta_cs + tsa, "theta_sa_max": tsa}


_add("elx_heat_sink_required",
     name="Required heat-sink thermal resistance", topic="power supplies / thermal design",
     purpose="The largest sink-to-ambient thermal resistance (and total junction-to-ambient resistance) that keeps a device dissipating P below its maximum junction temperature.",
     inputs=[_phys("P", "power", "power dissipated"), _phys("Ta", "temperature", "ambient temperature", show=("degC", 1.0)),
             _phys("Tj_max", "temperature", "maximum junction temperature", show=("degC", 1.0)),
             _phys("theta_jc", "thermal_resistance", "junction-to-case thermal resistance"), _phys("theta_cs", "thermal_resistance", "case-to-sink thermal resistance")],
     outputs=[_o("theta_ja_max", "thermal_resistance", "largest junction-to-ambient resistance"), _o("theta_sa_max", "thermal_resistance", "largest sink-to-ambient resistance")],
     primary="theta_sa_max", conditions=["a positive result (otherwise no heat sink suffices)"],
     method="theta_sa = (Tj,max - Ta)/P - theta_jc - theta_cs",
     latex=r"\theta_{sa} \le \frac{T_{j,max} - T_a}{P} - \theta_{jc} - \theta_{cs}",
     run=heat_sink_required, reference=_ref_heat_sink_required,
     wrong=lambda P, Ta, Tj_max, theta_jc, theta_cs: _out(theta_ja_max=(Tj_max - Ta) / P, theta_sa_max=(Tj_max - Ta) / P - theta_jc),
     rand=lambda rng: (lambda P, tjc, tcs: {"P": P, "Ta": _sig(rng.uniform(273.15, 323.15), 5), "Tj_max": rng.choice([398.15, 423.15, 448.15]),
                                            "theta_jc": tjc, "theta_cs": tcs})(_sig(_lu(rng, 0.5, 30.0), 3), _sig(_lu(rng, 0.2, 2.0), 3), _sig(_lu(rng, 0.05, 0.5), 3)),
     phrases=("Required heat sink", "Heat sink thermal resistance required"),
     examples=[_ex({"P": 20.0, "Ta": 313.15, "Tj_max": 423.15, "theta_jc": 1.0, "theta_cs": 0.5}, {"theta_ja_max": 5.5, "theta_sa_max": 4.0}),
               _ex({"P": 8.0, "Ta": 323.15, "Tj_max": 398.15, "theta_jc": 2.5, "theta_cs": 0.3})],
     level="advanced")


# the worked examples' outputs (computed by their references on the build machine): filled in below
_EXV: dict = {
    'elx_thermal_voltage': [{'VT': 0.0258519997864, 'VT_mV': 25.8519997864}, {'VT': 0.0301606664175, 'VT_mV': 30.1606664175}],
    'elx_intrinsic_carrier_concentration': [{'ni': 6675898719710000.0, 'ni_cm3': 6675898719.71, 'Ei_minus_midgap_eV': -0.0128018935425, 'Nc_T': 2.8e+25, 'Nv_T': 1.04e+25}, {'ni': 3165189926790000.0, 'ni_cm3': 3165189926.79, 'Ei_minus_midgap_eV': 0.0465496749635, 'Nc_T': 7.23612337384e+23, 'Nv_T': 1.07772050249e+25}],
    'elx_semiconductor_conductivity': [{'sigma': 216.29384559, 'rho': 0.00462334005515, 'rho_ohm_cm': 0.462334005515}, {'sigma': 0.000439797486033, 'rho': 2273.77379762, 'rho_ohm_cm': 227377.379762}],
    'elx_drift_current': [{'E': 5000.0, 'v_d': 675.0, 'J': 1081469.22795, 'I': 0.0108146922795}, {'E': 4000.0, 'v_d': 192.0, 'J': 153808.956864, 'I': 0.00038452239216}],
    'elx_einstein_diffusion': [{'D': 0.00349001997117, 'D_cm2': 34.9001997117, 'L_diff': 5.90763909796e-05, 'L_um': 59.0763909796}, {'D': 0.00124089598975, 'D_cm2': 12.4089598975, 'L_diff': 0.000111395511119, 'L_um': 111.395511119}],
    'elx_doped_carrier_concentrations': [{'n': 1e+22, 'p': 22499999999.9, 'n_cm3': 1e+16, 'p_cm3': 22499.9999999, 'type': 'n-type'}, {'n': 8027756377320000.0, 'p': 2.80277563773e+16, 'n_cm3': 8027756377.32, 'p_cm3': 28027756377.3, 'type': 'p-type'}],
    'elx_builtin_potential': [{'Vbi': 0.752879413544}, {'Vbi': 0.773843581323}],
    'elx_depletion_width': [{'W': 9.09099681381e-07, 'W_um': 0.909099681381, 'xn': 8.26454255801e-07, 'xp': 8.26454255801e-08, 'Emax': 12781876.661, 'Cj_area': 0.000113952297566}, {'W': 4.25491721388e-07, 'W_um': 0.425491721388, 'xn': 2.12745860694e-07, 'xp': 2.12745860694e-07, 'Emax': 3290310.78544, 'Cj_area': 0.000243468890703}],
    'elx_junction_capacitance': [{'Cj': 7.22315118515e-13, 'ratio': 0.361157559257}, {'Cj': 3.20259716869e-11, 'ratio': 0.640519433739}],
    'elx_shockley_diode_current': [{'ID': 0.000830843834767, 'VT': 0.0258519997864}, {'ID': -2e-12, 'VT': 0.0258519997864}],
    'elx_diode_voltage': [{'VD': 0.654790722656, 'VT': 0.0258519997864}, {'VD': 1.03645843989, 'VT': 0.0275754664389}],
    'elx_diode_operating_point': [{'ID': 0.00430745637251, 'VD': 0.692543627494, 'P_D': 0.00298310146149}, {'ID': 0.00513409435432, 'VD': 0.704992420488, 'P_D': 0.00361949760587}],
    'elx_rectifier_averages': [{'Vp_out': 10.0, 'Vdc': 3.18309886184, 'Vrms': 5.0, 'ripple_factor': 1.21136332298, 'PIV': 10.0, 'conduction_deg': 180.0}, {'Vp_out': 15.6, 'Vdc': 9.45925617126, 'Vrms': 10.776021277, 'ripple_factor': 0.545697230078, 'PIV': 16.3, 'conduction_deg': 170.552348482}],
    'elx_capacitor_filter_ripple': [{'f_ripple': 120.0, 'Vr': 0.271666666667, 'Vdc': 16.1641666667, 'Vr_rms': 0.0784234115649, 'ripple_factor': 0.00485168293437}, {'f_ripple': 50.0, 'Vr': 0.510638297872, 'Vdc': 11.7446808511, 'Vr_rms': 0.147408579368, 'ripple_factor': 0.0125510928085}],
    'elx_zener_regulator': [{'Vout': 10.0, 'I_s': 0.0454545454545, 'I_L': 0.01, 'I_z': 0.0354545454545, 'P_z': 0.354545454545}, {'Vout': 5.66875858011, 'I_s': 0.0933124141989, 'I_L': 0.0120611884683, 'I_z': 0.0812512257305, 'P_z': 0.460593583005}],
    'elx_zener_series_resistor_design': [{'Rs_max': 52.7272727273, 'Iz_max': 0.111896551724, 'Pz_max': 1.01825862069, 'PRs_max': 0.660189655172}, {'Rs_max': 33.3333333333, 'Iz_max': 0.152, 'Pz_max': 0.8512, 'PRs_max': 0.8748}],
    'elx_led_series_resistor': [{'R': 150.0, 'P_R': 0.06, 'P_LED': 0.04}, {'R': 180.0, 'P_R': 0.0405, 'P_LED': 0.1395}],
    'elx_bjt_current_relations': [{'IC': 0.002, 'IE': 0.00202, 'alpha': 0.990099009901}, {'IC': 0.0075, 'IE': 0.00755, 'alpha': 0.993377483444}],
    'elx_bjt_fixed_bias': [{'IB': 2.40425531915e-05, 'IC': 0.00240425531915, 'IE': 0.00242829787234, 'VCE': 6.71063829787}, {'IB': 1.01829268293e-05, 'IC': 0.00203658536585, 'IE': 0.00204676829268, 'VCE': 4.51951219512}],
    'elx_bjt_voltage_divider_bias': [{'VTH': 2.10526315789, 'RTH': 8245.61403509, 'IB': 1.77397243588e-05, 'IC': 0.00266095865382, 'IE': 0.00267869837818, 'VB': 1.95898823774, 'VE': 1.25898823774, 'VC': 6.1458909616, 'VCE': 4.88690272386}, {'VTH': 2.56281407035, 'RTH': 5638.19095477, 'IB': 1.74685453089e-05, 'IC': 0.00174685453089, 'IE': 0.0017643230762, 'VB': 2.4643230762, 'VE': 1.7643230762, 'VC': 9.23538004807, 'VCE': 7.47105697187}],
    'elx_bjt_emitter_bias': [{'IB': 9.11764705882e-06, 'IC': 0.000911764705882, 'IE': 0.000920882352941, 'VB': -0.0911764705882, 'VE': -0.791176470588, 'VC': 5.71470588235, 'VCE': 6.50588235294}, {'IB': 4.92567248702e-06, 'IC': 0.000985134497404, 'IE': 0.000990060169891, 'VB': -0.23150660689, 'VE': -0.88150660689, 'VC': 6.48324681454, 'VCE': 7.36475342143}],
    'elx_bjt_collector_feedback_bias': [{'IB': 1.37228862328e-05, 'IC': 0.00123505976096, 'IE': 0.00124878264719, 'VC': 4.13072155821, 'VCE': 4.13072155821}, {'IB': 1.43734815625e-05, 'IC': 0.0017248177875, 'IE': 0.00173919126906, 'VC': 6.26066881209, 'VCE': 5.44324891563}],
    'elx_bjt_small_signal_parameters': [{'gm': 0.04, 'r_pi': 2500.0, 'r_e': 24.7524752475, 'r_o': 100000.0, 'A0': 4000.0}, {'gm': 0.0961538461538, 'r_pi': 1560.0, 'r_e': 10.3311258278, 'r_o': 30000.0, 'A0': 2884.61538462}],
    'elx_common_emitter_amplifier': [{'gm': 0.04, 'r_pi': 2500.0, 'Av': -133.333333333, 'Rin': 2222.22222222, 'Rout': 5000.0, 'Gv': -91.9540229885}, {'gm': 0.08, 'r_pi': 1875.0, 'Av': -8.28643681436, 'Rin': 16174.0533067, 'Rout': 3300.0, 'Gv': -7.99003486561}],
    'elx_common_collector_amplifier': [{'Av': 0.990196078431, 'Rin': 33774.8344371, 'Rout': 86.7242087957, 'Gv': 0.763993948563}, {'Av': 0.981894150418, 'Rin': 40390.1855177, 'Rout': 47.8619700261, 'Gv': 0.873732645985}],
    'elx_common_base_amplifier': [{'Av': 100.0, 'Rin': 24.6913580247, 'Rout': 5000.0, 'Gv': 33.0578512397}, {'Av': 128.205128205, 'Rin': 51.0105402303, 'Rout': 10000.0, 'Gv': 43.3069959227}],
    'elx_transition_frequency': [{'r_pi': 2500.0, 'f_beta': 5305164.76973, 'f_T': 530516476.973}, {'r_pi': 1500.0, 'f_beta': 5052537.87593, 'f_T': 757880681.39}],
    'elx_miller_high_cutoff': [{'A_mid': -100.0, 'C_M': 2.02e-10, 'R_pi_eq': 714.285714286, 'f_H_miller': 1051023.2091, 'tau_oc': 1.56428571429e-07, 'f_H_octc': 1017428.85995}, {'A_mid': -100.0, 'C_M': 1.01e-10, 'R_pi_eq': 375.0, 'f_H_miller': 3507546.95519, 'tau_oc': 4.6375e-08, 'f_H_octc': 3431912.5195}],
    'elx_low_frequency_cutoff': [{'R_C1': 3222.22222222, 'R_CE': 33.0521996809, 'R_C2': 15000.0, 'f_C1': 4.93929133733, 'f_CE': 48.1526024375, 'f_C2': 1.06103295395, 'f_L': 54.1529267288}, {'R_C1': 2364.70588235, 'R_CE': 15.7656214328, 'R_C2': 8000.0, 'f_C1': 14.3200700358, 'f_CE': 45.8866504305, 'f_C2': 19.8943678865, 'f_L': 80.1010883528}],
    'elx_current_mirror': [{'Iref': 0.001, 'Iout': 0.000980392156863, 'ratio': 0.980392156863}, {'Iref': 0.001, 'Iout': 0.000961538461538, 'ratio': 0.961538461538}],
    'elx_wilson_current_mirror': [{'Iout': 0.000999803960008, 'ratio': 0.999803960008, 'error': 0.000196039992159}, {'Iout': 0.000199095022624, 'ratio': 0.995475113122, 'error': 0.00452488687783}],
    'elx_differential_pair': [{'IC': 0.00049504950495, 'gm': 0.019801980198, 'Ad_diff': -198.01980198, 'Ad_se': -99.0099009901, 'Acm': -0.0494925773507, 'CMRR': 2000.5, 'CMRR_dB': 66.0227711143}, {'IC': 0.000995024875622, 'gm': 0.0382701875239, 'Ad_diff': -191.35093762, 'Ad_se': -95.6754688098, 'Acm': -0.0497383118201, 'CMRR': 1923.57692308, 'CMRR_dB': 65.6821911651}],
    'elx_bjt_cascode': [{'gm': 0.04, 'r_o': 100000.0, 'Rout': 9858536.58537, 'Gm': 0.0395942602672, 'Av': -35947.8885894}, {'gm': 0.00384615384615, 'r_o': 500000.0, 'Rout': 91126811.5942, 'Gm': 0.00382505054089, 'Av': -34468.0757242}],
    'elx_mosfet_drain_current': [{'region': 'saturation', 'ID': 0.004, 'VOV': 2.0, 'VDS_sat': 2.0}, {'region': 'triode', 'ID': 0.000525, 'VOV': 1.3, 'VDS_sat': 1.3}],
    'elx_mosfet_gm_ro': [{'VOV': 1.0, 'VGS': 1.7, 'gm': 0.002, 'r_o': 100000.0, 'A0': 200.0}, {'VOV': 0.4, 'VGS': 0.9, 'gm': 0.002, 'r_o': 50000.0, 'A0': 100.0}],
    'elx_mosfet_divider_bias': [{'VG': 5.0, 'VGS': 2.9364916731, 'VOV': 1.4364916731, 'ID': 0.0020635083269, 'VS': 2.0635083269, 'VD': 8.19042252124, 'VDS': 6.12691419435}, {'VG': 3.84, 'VGS': 3.84, 'VOV': 2.84, 'ID': 0.0040328, 'VS': 0.0, 'VD': 3.12784, 'VDS': 3.12784}],
    'elx_common_source_amplifier': [{'Av': -9.52380952381, 'Rin': 1000000.0, 'Rout': 9090.90909091, 'Gv': -9.42951438001}, {'Av': -5.01728634682, 'Rin': 2200000.0, 'Rout': 4557.66752577, 'Gv': -4.90579109467}],
    'elx_source_follower': [{'Av': 0.662251655629, 'Rin': 1000000.0, 'Rout': 396.825396825, 'Gv': 0.655694708544}, {'Av': 0.813008130081, 'Rin': 470000.0, 'Rout': 165.289256198, 'Gv': 0.81128199817}],
    'elx_mosfet_body_effect': [{'Vt': 1.13475135185, 'dVt': 0.434751351853}, {'Vt': 0.72360679775, 'dVt': 0.22360679775}],
    'elx_cmos_switching_threshold': [{'r': 1.0, 'VM': 1.25}, {'r': 0.632455532034, 'VM': 1.35233695079}],
    'elx_cmos_propagation_delay': [{'tpHL': 6.9314718056e-10, 'tpLH': 1.38629436112e-09, 'tp': 1.03972077084e-09}, {'tpHL': 1.7328679514e-10, 'tpLH': 4.15888308336e-10, 'tp': 2.94587551738e-10}],
    'elx_inverting_amplifier': [{'G': -10.0, 'Vout': -5.0, 'Zin': 10000.0}, {'G': -10.0, 'Vout': 2.0, 'Zin': 4700.0}],
    'elx_noninverting_amplifier': [{'G': 10.0, 'Vout': 3.0}, {'G': 5.68085106383, 'Vout': 6.8170212766}],
    'elx_summing_amplifier': [{'Vout': -1.0, 'I_f': 0.0001}, {'Vout': -2.82, 'I_f': 0.0006}],
    'elx_difference_amplifier': [{'Vout': 2.0, 'Ad': 10.0, 'Acm': 1.69406589451e-16}, {'Vout': 0.376470588235, 'Ad': 4.97058823529, 'Acm': -0.0588235294118}],
    'elx_instrumentation_amplifier': [{'G': 51.0, 'G_stage1': 51.0, 'Vout': 0.51}, {'G': 402.0, 'G_stage1': 201.0, 'Vout': 0.804}],
    'elx_inverting_amplifier_finite_gain': [{'G': -99.0000990001, 'G_ideal': -100.0, 'Vout': -0.990000990001, 'error': 0.00999900999901}, {'G': -9.89119683482, 'G_ideal': -10.0, 'Vout': -0.989119683482, 'error': 0.0108803165183}],
    'elx_opamp_integrator': [{'Vout': -0.1, 'slope': -50.0, 'f_unity': 15.9154943092}, {'Vout': -1.5, 'slope': 100.0, 'f_unity': 15.9154943092}],
    'elx_opamp_differentiator': [{'gain': 0.628318530718, 'Vout_peak': 0.628318530718, 'gain_dB': -4.03640263284}, {'gain': 0.314159265359, 'Vout_peak': 0.157079632679, 'gain_dB': -10.0570025461}],
    'elx_closed_loop_bandwidth': [{'G0': 10.0, 'noise_gain': 10.0, 'f_3dB': 100000.0}, {'G0': -10.0, 'noise_gain': 11.0, 'f_3dB': 909090.909091}],
    'elx_slew_rate_limit': [{'f_max': 7957.74715459, 'Vp_max': 3.9788735773, 'slope_needed': 1256637.06144, 'limited': 'slew-limited'}, {'f_max': 413802.852039, 'Vp_max': 20.6901426019, 'slope_needed': 3141592.65359, 'limited': 'not slew-limited'}],
    'elx_opamp_dc_offset': [{'Vout_Vos': 0.022, 'Vout_bias': 0.0105, 'Vout_total': 0.0325, 'Rp_optimal': 9090.90909091}, {'Vout_Vos': 0.011, 'Vout_bias': 0.001993, 'Vout_total': 0.012993, 'Rp_optimal': 9090.90909091}],
    'elx_schmitt_trigger': [{'V_TH': 1.2, 'V_TL': -1.2, 'hysteresis': 2.4, 'center': 0.0}, {'V_TH': 2.91475409836, 'V_TL': 2.06721311475, 'hysteresis': 0.847540983607, 'center': 2.49098360656}],
    'elx_negative_feedback': [{'Af': 99.0099009901, 'loop_gain': 100.0, 'desensitivity': 101.0, 'desensitivity_dB': 40.0864274757, 'fH_closed': 101000.0}, {'Af': 9.999500025, 'loop_gain': 20000.0, 'desensitivity': 20001.0, 'desensitivity_dB': 86.0210341969, 'fH_closed': 200010.0}],
    'elx_two_pole_phase_margin': [{'f_c': 99508.5486792, 'PM_deg': 84.3230453856, 'Af': 9.99900009999}, {'f_c': 616846.563288, 'PM_deg': 17.9735245301, 'Af': 0.9999500025}],
    'elx_first_order_active_lowpass': [{'H0': -10.0, 'fc': 1000.97448485, 'gain_f': 7.07451061927, 'gain_dB': 16.9939280565}, {'H0': -10.0, 'fc': 338.627538493, 'gain_f': 5.60754907423, 'gain_dB': 14.9754616596}],
    'elx_sallen_key_lowpass': [{'f0': 1125.3953952, 'Q': 0.707106781187, 'H0': 1.0, 'peak_gain': 0.707106781187}, {'f0': 1061.03295395, 'Q': 0.707213578501, 'H0': 1.586, 'peak_gain': 1.1216407355}],
    'elx_sallen_key_highpass': [{'f0': 1000.3514624, 'Q': 0.707106781187, 'H_inf': 1.0}, {'f0': 723.431559509, 'Q': 1.0, 'H_inf': 2.0}],
    'elx_mfb_lowpass': [{'f0': 1514.38839025, 'Q': 1.11803398875, 'H0': -1.0}, {'f0': 734.127009572, 'Q': 1.38085881457, 'H0': -10.0}],
    'elx_mfb_bandpass': [{'f0': 2547.56238423, 'Q': 2.54508224013, 'BW': 1000.97448485, 'H_peak': -1.0}, {'f0': 1079.74664354, 'Q': 7.4626730708, 'BW': 144.686311902, 'H_peak': -5.0}],
    'elx_sallen_key_equal_component_design': [{'R': 15915.4943092, 'K': 1.58577287512, 'Rf_over_Rg': 0.585772875124}, {'R': 14468.6311902, 'K': 2.5, 'Rf_over_Rg': 1.5}],
    'elx_filter_order': [{'n': 6}, {'n': 4}],
    'elx_butterworth_section_q': [{'Q_max': 1.30656296488, 'Q': [0.541196100146, 1.30656296488], 'sections': 2, 'first_order': 0}, {'Q_max': 1.61803398875, 'Q': [0.61803398875, 1.61803398875], 'sections': 2, 'first_order': 1}],
    'elx_chebyshev_sections': [{'Q_max': 2.94055417439, 'Q': [0.705110236803, 2.94055417439], 'f0_ratio': [0.59700239512, 1.03127040148], 'sections': 2, 'real_pole': 0.0}, {'Q_max': 2.01772034366, 'Q': [2.01772034366], 'f0_ratio': [0.997098082833], 'sections': 1, 'real_pole': 0.494170604942}],
    'elx_rc_phase_shift_oscillator': [{'f0': 649.747334361, 'A_min': 29.0, 'beta': 0.0344827586207}, {'f0': 1764.81847269, 'A_min': 29.0, 'beta': 0.0344827586207}],
    'elx_wien_bridge_oscillator': [{'f0': 994.718394324, 'beta': 0.333333333333, 'A_min': 3.0, 'Rf_over_Rg': 2.0}, {'f0': 723.431559509, 'beta': 0.185185185185, 'A_min': 5.4, 'Rf_over_Rg': 4.4}],
    'elx_colpitts_oscillator': [{'C_eq': 9.09090909091e-10, 'f0': 527857.229766, 'beta': 0.1, 'A_min': 10.0}, {'C_eq': 4.27272727273e-10, 'f0': 2434823.83921, 'beta': 0.1, 'A_min': 10.0}],
    'elx_hartley_oscillator': [{'L_eq': 0.00012, 'f0': 459440.746185, 'A_min': 5.0}, {'L_eq': 7e-05, 'f0': 877449.604725, 'A_min': 3.66666666667}],
    'elx_crystal_oscillator': [{'fs': 5032921.21045, 'fp': 5042977.00711, 'delta_f': 10055.79666, 'Q': 79056.9415042}, {'fs': 1974074.1125, 'fp': 1978182.49178, 'delta_f': 4108.37928789, 'Q': 64498.0619864}],
    'elx_opamp_relaxation_oscillator': [{'beta': 0.5, 'T': 0.00219722457734, 'f': 455.119613313}, {'beta': 0.462365591398, 'T': 0.000940593967489, 'f': 1063.15799863}],
    'elx_555_astable': [{'t_high': 0.0762461898616, 't_low': 0.069314718056, 'T': 0.145560907918, 'f': 6.86997638519, 'duty': 0.52380952381}, {'t_high': 0.000651558349726, 't_low': 0.000325779174863, 'T': 0.00097733752459, 'f': 1023.18797226, 'duty': 0.666666666667}],
    'elx_555_monostable': [{'T': 1.09861228867}, {'T': 0.0516347775674}],
    'elx_pll_loop_dynamics': [{'wn': 1000.0, 'fn': 159.154943092, 'zeta': 0.5}, {'wn': 1000.0, 'fn': 159.154943092, 'zeta': 1.0}],
    'elx_adc_code': [{'code': 163, 'bits': '10100011', 'LSB': 0.01953125, 'V_code': 3.18359375, 'error': 0.01640625}, {'code': 2021, 'bits': '011111100101', 'LSB': 0.0006103515625, 'V_code': 1.23352050781, 'error': 0.0004794921875}],
    'elx_adc_quantization_snr': [{'SNR_dB': 74.0081115499, 'noise_rms_lsb': 0.288675134595, 'SNR_full_scale_dB': 74.0081115499}, {'SNR_dB': 67.9875116366, 'noise_rms_lsb': 0.0721687836487, 'SNR_full_scale_dB': 61.9669117234}],
    'elx_adc_enob': [{'ENOB': 12.0, 'bits_lost': 2.0}, {'ENOB': 10.0897009967, 'bits_lost': 1.91029900332}],
    'elx_aperture_jitter_snr': [{'SNR_jitter_dB': 64.0364026328, 'SNR_quant_dB': 86.0493113765, 'SNR_total_dB': 64.0091674081, 'ENOB_total': 10.3403932572}, {'SNR_jitter_dB': 90.0570025461, 'SNR_quant_dB': 98.090511203, 'SNR_total_dB': 89.4226511826, 'ENOB_total': 14.5619021898}],
    'elx_dac_output': [{'Vout': 3.90625, 'LSB': 0.01953125, 'V_full_scale': 4.98046875}, {'Vout': 6.875, 'LSB': 0.625, 'V_full_scale': 9.375}],
    'elx_dac_binary_weighted': [{'Vout': -3.125, 'I_total': 0.000625, 'R_LSB': 80000.0}, {'Vout': -2.51953125, 'I_total': 0.00251953125, 'R_LSB': 128000.0}],
    'elx_sar_conversion': [{'code': 10, 'bits': '1010', 'trials': [2.5, 3.75, 3.125, 3.4375], 'V_dac': 3.125, 'cycles': 4}, {'code': 131, 'bits': '10000011', 'trials': [1.65, 2.475, 2.0625, 1.85625, 1.753125, 1.7015625, 1.67578125, 1.688671875], 'V_dac': 1.688671875, 'cycles': 8}],
    'elx_dual_slope_adc': [{'T1': 0.01, 'T2': 0.005, 'N2': 500.0, 'V_peak': 0.5}, {'T1': 0.002, 'T2': 0.001234, 'N2': 1234.0, 'V_peak': 0.238684719536}],
    'elx_sample_hold_droop': [{'droop_rate': 1.0, 'droop': 1e-05, 'droop_lsb': 0.008192, 't_max_half_lsb': 0.0006103515625}, {'droop_rate': 0.5, 'droop': 1e-05, 'droop_lsb': 0.065536, 't_max_half_lsb': 0.000152587890625}],
    'elx_rc_delay_rise_time': [{'tau': 1e-08, 't50': 6.9314718056e-09, 't_rise': 2.19722457734e-08, 'f_3dB': 15915494.3092, 'tr_times_bw': 0.349699152566}, {'tau': 5e-09, 't50': 3.4657359028e-09, 't_rise': 1.09861228867e-08, 'f_3dB': 31830988.6184, 'tr_times_bw': 0.349699152566}],
    'elx_elmore_delay': [{'tau_D': 6.5e-11, 't50': 4.50545667364e-11, 'delays': [4e-11, 5.5e-11, 6.5e-11]}, {'tau_D': 6.5e-10, 't50': 4.50545667364e-10, 'delays': [5.5e-10, 6.5e-10]}],
    'elx_cmos_dynamic_power': [{'P': 0.1, 'E_supply': 1e-09, 'E_stored': 5e-10, 'I_avg': 0.1}, {'P': 0.0136125, 'E_supply': 5.445e-10, 'E_stored': 2.7225e-10, 'I_avg': 0.004125}],
    'elx_setup_hold_timing': [{'setup_slack': 1.1e-09, 'hold_slack': 1e-10, 'f_max': 256410256.41, 'verdict': 'meets timing'}, {'setup_slack': -2.5e-10, 'hold_slack': 2e-11, 'f_max': 444444444.444, 'verdict': 'setup violation'}],
    'elx_metastability_mtbf': [{'MTBF_s': 470770533674.0, 'MTBF_years': 14917.8180113, 'log10_MTBF_s': 11.6728092718}, {'MTBF_s': 2073882211430000.0, 'MTBF_years': 65717361.6319, 'log10_MTBF_s': 15.3167840865}],
    'elx_linear_regulator_efficiency': [{'P_in': 6.06, 'P_out': 2.5, 'P_diss': 3.56, 'efficiency': 0.412541254125, 'efficiency_pct': 41.2541254125}, {'P_in': 1.00025, 'P_out': 0.66, 'P_diss': 0.34025, 'efficiency': 0.65983504124, 'efficiency_pct': 65.983504124}],
    'elx_adjustable_regulator_output': [{'Vout': 5.0, 'I_R1': 0.00520833333333}, {'Vout': 11.7666666667, 'I_R1': 0.00520833333333}],
    'elx_series_pass_regulator': [{'Vout': 9.3, 'I_L': 0.093, 'I_B': 0.000920792079208, 'I_z': 0.00407920792079, 'I_Rs': 0.005, 'P_Q': 0.524851485149}, {'Vout': 12.0, 'I_L': 0.24, 'I_B': 0.00393442622951, 'I_z': 0.0115974886641, 'I_Rs': 0.0155319148936, 'P_Q': 1.88852459016}],
    'elx_heat_sink_temperatures': [{'Tj': 373.15, 'Tc': 350.65, 'Ts': 343.15, 'Tj_C': 100.0, 'theta_ja': 5.0}, {'Tj': 401.15, 'Tc': 369.15, 'Ts': 361.15, 'Tj_C': 128.0, 'theta_ja': 2.2}],
    'elx_heat_sink_required': [{'theta_ja_max': 5.5, 'theta_sa_max': 4.0}, {'theta_ja_max': 9.375, 'theta_sa_max': 6.575}],
}

for _k, _vals in _EXV.items():
    if _k in PROCEDURES:
        for _e, _v in zip(PROCEDURES[_k]["examples"], _vals):
            _e["outputs"].update(_v)
