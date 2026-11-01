# electronics-engineering-ai999x — offline, verified electronics engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/electronics-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/electronics-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/electronics-engineering-ai999x)](https://pypi.org/project/electronics-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/electronics-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/electronics-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/electronics-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/electronics-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/electronics-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

electronics-engineering-ai999x is an offline, deterministic Python solver that answers electronics engineering questions with verified results from 92 certified capabilities, gives 561 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**electronics-engineering-ai999x** is a compact, deterministic electronics engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified electronics engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified electronics engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 561 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10–3.14).
- **Coverage:** semiconductors, diode and transistor circuits, amplifiers, active filters, oscillators, data converters, timing and power supplies.

## Model overview

| Property | Value |
|---|---|
| Package | `electronics-engineering-ai999x` 1.0.0 (import `electronics_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic electronics engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 92 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 561 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python ) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install electronics-engineering-ai999x
```

```python
import electronics_engineering_ai999x as electronics_engineering

electronics_engineering.ask("BJT voltage-divider bias: VCC = 12 V, R1 = 47 kohm, R2 = 10 kohm, RC = 2.2 kohm, RE = 470 ohm, beta = 150, VBE = 0.7 V").show()
# -> SOLVED — VTH = 2.10526, RTH = 8245.61, IB = 1.77397e-05, IC = 0.00266096  (elx_bjt_voltage_divider_bias)
electronics_engineering.ask("Sallen-Key low-pass filter: R1 = 10 kohm, R2 = 10 kohm, C1 = 20 nF, C2 = 10 nF, K = 1").show()
# -> SOLVED — f0 = 1125.4, Q = 0.707107, H0 = 1, peak_gain = 0.707107  (elx_sallen_key_lowpass)
electronics_engineering.solve("elx_common_emitter_amplifier", IC=0.001, beta=100.0, VT=0.025, RC=5000.0, RL=10000.0, RE1=0.0, RB=20000.0, Rsig=1000.0).show()
# -> SOLVED — gm = 0.04, r_pi = 2500, Av = -133.333, Rin = 2222.22  (elx_common_emitter_amplifier)
electronics_engineering.ask("555 astable: RA = 1 kohm, RB = 10 kohm, C = 10 uF").show()
# -> SOLVED — t_high = 0.0762462, t_low = 0.0693147, T = 0.145561, f = 6.86998  (elx_555_astable)
electronics_engineering.ask("Diode operating point: VS = 5 V, R = 1 kohm, Is = 1e-14 A, n = 1, T = 300 K").show()
# -> SOLVED — ID = 0.00430746, VD = 0.692544, P_D = 0.0029831  (elx_diode_operating_point)
electronics_engineering.ask("Who created you?").message
# -> electronics-engineering-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs its own command: `electronics-engineering-ai999x "<question>"`, `electronics-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on electronics-engineering-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

92 certified procedures. A few of them:

- **Procedures:** 555 timer astable: frequency and duty cycle; 555 timer monostable pulse width; ADC output code, LSB and quantisation error; ADC quantisation SNR with oversampling; Adjustable three-terminal regulator output voltage; BJT beta cutoff and transition frequency fT; BJT cascode: output resistance, transconductance and gain; BJT collector-feedback bias: Q-point; BJT current mirror with finite beta; BJT differential pair: differential and common-mode gain, CMRR; BJT dual-supply emitter bias: Q-point; BJT fixed (base-resistor) bias: Q-point; BJT small-signal (hybrid-pi) parameters; BJT terminal currents from beta; BJT voltage-divider bias: exact Q-point; Binary-weighted resistor DAC; Built-in potential of a pn junction; Butterworth filter: Q of each second-order section; Butterworth or Chebyshev low-pass filter order; CMOS dynamic (switching) power; CMOS gate propagation delay (switch-resistor model); CMOS inverter switching threshold VM; Capacitor-input filter: ripple voltage and DC output; Chebyshev type I filter: section Q and frequencies …

`electronics_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified electronics engineering capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (semiconductors, diode and transistor circuits, amplifiers, active filters, oscillators, data converters, timing and power supplies); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{electronics_engineering_ai999x_1_0_0,
  title  = {electronics-engineering-ai999x: an offline, verified electronics engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/electronics-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/electronics-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

electronics-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/electronics-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/electronics-engineering-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/electronics-engineering-ai999x/tree/main/onnx).
