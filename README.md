# Electronics Engineering (electronics-engineering-ai999x)

[![tests](https://github.com/AI999X/electronics-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/electronics-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/electronics-engineering-ai999x)](https://pypi.org/project/electronics-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/electronics-engineering-ai999x)

Electronics Engineering covers semiconductors and pn junctions, diode circuits and rectifiers, BJT and MOSFET biasing and amplifiers, op-amp circuits, active filters, oscillators and 555 timers, ADCs and DACs, digital timing and power supplies: 92 calculations in 11 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Electronics Engineering desktop app: BJT dual-supply emitter bias: Q-point, with its worked example and live scene](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/electronics-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `electronics-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/electronics-engineering-ai999x/#try) |
| **Python** 3.10+ | `pip install electronics-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import electronics_engineering_ai999x as electronics_engineering

a = electronics_engineering.ask(
    "Transition frequency: gm = 40 mS, beta = 100, Cpi = 10 pF, Cmu = "
    "2 pF"
)
print(a.outcome, a.values)
# SOLVED {'r_pi': 2500.0, 'f_beta': 5305164.76973, 'f_T': 530516476.973}

a = electronics_engineering.ask(
    "Closed-loop bandwidth: GBW = 1000000 Hz, config = non-inverting, "
    "Rf = 90000 ohm, R1 = 10000 ohm"
)
print(a.outcome, a.values)
# SOLVED {'G0': 10.0, 'noise_gain': 10.0, 'f_3dB': 100000.0}

a = electronics_engineering.ask(
    "Adjustable regulator output: R1 = 240 ohm, R2 = 720 ohm, Vref = "
    "1.25 V"
)
print(a.outcome, a.values)
# SOLVED {'Vout': 5.0, 'I_R1': 0.00520833333333}

a = electronics_engineering.ask(
    "Built-in potential: NA = 1e+17 cm^-3, ND = 1e+16 cm^-3, ni = "
    "1.5e+10 cm^-3, T = 300 K"
)
print(a.outcome, a.values)
# SOLVED {'Vbi': 0.752879413544}

a = electronics_engineering.ask(
    "Filter order: ftype = Butterworth, fp = 1000 Hz, fs = 2500 Hz, Ap "
    "= 1 dB, As = 40 dB"
)
print(a.outcome, a.values)
# SOLVED {'n': 6}

a = electronics_engineering.ask(
    "CMOS switching threshold: VDD = 2.5 V, Vtn = 0.4 V, Vtp = 0.4 V, "
    "kn = 0.1 mA/V^2, kp = 0.1 mA/V^2"
)
print(a.outcome, a.values)
# SOLVED {'r': 1.0, 'VM': 1.25}

a = electronics_engineering.ask(
    "Shockley diode current: Is = 1e-14 A, n = 1, VD = 0.65 V, T = 300 "
    "K"
)
print(a.outcome, a.values)
# SOLVED {'ID': 0.000830843834767, 'VT': 0.0258519997864}

a = electronics_engineering.ask(
    "MOSFET drain current: kn = 2 mA/V^2, Vt = 1 V, VGS = 3 V, VDS = 5 "
    "V"
)
print(a.outcome, a.values)
# SOLVED {'region': 'saturation', 'ID': 0.004, 'VOV': 2.0, 'VDS_sat': 2.0}

a = electronics_engineering.ask(
    "555 monostable: R = 100000 ohm, C = 10 uF"
)
print(a.outcome, a.values)
# SOLVED {'T': 1.09861228867}

a = electronics_engineering.ask(
    "Binary-weighted DAC: n = 4, code = 10, Vref = 5 V, R = 10000 ohm, "
    "Rf = 5000 ohm"
)
print(a.outcome, a.values)
# SOLVED {'Vout': -3.125, 'I_total': 0.000625, 'R_LSB': 80000.0}

a = electronics_engineering.ask(
    "Diode voltage for a current: ID = 0.001 A, Is = 1e-14 A, n = 1, T "
    "= 300 K"
)
print(a.outcome, a.values)
# SOLVED {'VD': 0.654790722656, 'VT': 0.0258519997864}

a = electronics_engineering.ask(
    "Inverting amplifier: Rf = 100000 ohm, Rin = 10000 ohm, Vin = 0.5 "
    "V"
)
print(a.outcome, a.values)
# SOLVED {'G': -10.0, 'Vout': -5.0, 'Zin': 10000.0}

a = electronics_engineering.ask(
    "DAC output voltage: n = 8, code = 200, Vref = 5 V"
)
print(a.outcome, a.values)
# SOLVED {'Vout': 3.90625, 'LSB': 0.01953125, 'V_full_scale': 4.98046875}

a = electronics_engineering.ask(
    "LED series resistor: Vs = 5 V, Vf = 2 V, If = 20 mA"
)
print(a.outcome, a.values)
# SOLVED {'R': 150.0, 'P_R': 0.06, 'P_LED': 0.04}

a = electronics_engineering.ask(
    "Body effect threshold voltage: Vt0 = 0.7 V, gamma = 0.4 V^0.5, "
    "phi2F = 0.7 V, VSB = 3 V"
)
print(a.outcome, a.values)
# SOLVED {'Vt': 1.13475135185, 'dVt': 0.434751351853}
```

## Screenshots

[![The Electronics Engineering desktop app: BJT dual-supply emitter bias: Q-point, with its worked example and live scene](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/hero-dark.webp)](https://www.ai999x.com/models/electronics-engineering-ai999x/)

[![Butterworth filter: Q of each second-order section: the live scene in the Electronics Engineering desktop app](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/see-1.webp)](https://www.ai999x.com/models/electronics-engineering-ai999x/)

*Butterworth filter: Q of each second-order section*

[![ADC output code, LSB and quantisation error: the live scene in the Electronics Engineering desktop app](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/see-2.webp)](https://www.ai999x.com/models/electronics-engineering-ai999x/)

*ADC output code, LSB and quantisation error*

[![Einstein relation and diffusion length: the live scene in the Electronics Engineering desktop app](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/see-3.webp)](https://www.ai999x.com/models/electronics-engineering-ai999x/)

*Einstein relation and diffusion length*

Every calculation, with its formula, a worked example and its figures: **[Electronics Engineering on ai999x.com](https://www.ai999x.com/models/electronics-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/electronics-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/electronics-engineering-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `electronics-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/electronics-engineering`) and its data (`rust/models/electronics_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p electronics-engineering-ai999x
./target/release/electronics-engineering --help
cargo test -p electronics-engineering-ai999x
```
