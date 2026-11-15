# Electronics Engineering (electronics-engineering-ai999x)

[![tests](https://github.com/AI999X/electronics-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/electronics-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/electronics-engineering-ai999x)](https://pypi.org/project/electronics-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/electronics-engineering-ai999x)

Electronics Engineering covers semiconductors and pn junctions, diode circuits and rectifiers, BJT and MOSFET biasing and amplifiers, op-amp circuits, active filters, oscillators and 555 timers, ADCs and DACs, digital timing and power supplies: 92 calculations in 11 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Electronics Engineering desktop app: BJT dual-supply emitter bias: Q-point, with its worked example and live scene](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/electronics-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-linux-x86_64.tar.gz) |
| **Command line** `electronics-engineering` (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-cli-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-cli-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/electronics-engineering-ai999x/#try) |
| **Python** 3.10+ | `pip install electronics-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Example

```python
import electronics_engineering_ai999x as electronics_engineering
a = electronics_engineering.ask("BJT voltage-divider bias: VCC = 12 V, R1 = 47 kohm, R2 = 10 kohm, RC = 2.2 kohm, RE = 470 ohm, beta = 150, VBE = 0.7 V")
print(a.outcome, a.value, a.unit)
```

Output:

```text
SOLVED 0.00266095865382 A
```

Every calculation, with its formula, a worked example and its figures: **[Electronics Engineering on ai999x.com](https://www.ai999x.com/models/electronics-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.3 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/electronics-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.3`; the [GitHub release](https://github.com/AI999X/electronics-engineering-ai999x/releases/tag/v1.6.3) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `electronics-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/electronics-engineering`) and its data (`rust/models/electronics_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p electronics-engineering-ai999x
./target/release/electronics-engineering --help
cargo test -p electronics-engineering-ai999x
```
