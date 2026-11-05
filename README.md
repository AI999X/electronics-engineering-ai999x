# electronics-engineering-ai999x — offline, verified electronics engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/electronics-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/electronics-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/electronics-engineering-ai999x)](https://pypi.org/project/electronics-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/electronics-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/electronics-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/electronics-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/electronics-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/electronics-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.1.0

Field: **Engineering** · Tasks: Question Answering, Sentence Similarity, Feature Extraction

electronics-engineering-ai999x is an offline, deterministic Python solver that answers electronics engineering questions with verified results from 92 certified capabilities, gives 561 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**electronics-engineering-ai999x** is a compact, deterministic electronics engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified electronics engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** electronics-engineering-ai999x comes as the AI999X desktop app and as the command-line program `electronics-engineering` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Electronics Engineering desktop app** 1.4.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-electronics-engineering-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-electronics-engineering-1.4.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.4.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz) |
| **Command line** (`electronics-engineering`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/electronics-engineering-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install electronics-engineering-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Electronics Engineering** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Electronics Engineering among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-electronics-engineering-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-electronics-engineering-1.4.0-linux-x86_64.tar.gz
./ai999x-electronics-engineering-1.4.0-linux-x86_64/ai999x-electronics-engineering
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
electronics-engineering --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
electronics-engineering --version
```

Every command below was run as shown; the text under it is what the program printed.

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ electronics-engineering info elx_bjt_voltage_divider_bias
BJT voltage-divider bias: exact Q-point
  VTH = VCC R2/(R1 + R2); RTH = R1 || R2; IB = (VTH - VBE)/(RTH + (beta + 1) RE); VCE = VCC - IC RC - IE RE
  in   VCC      supply voltage [V]
  in   R1       upper divider resistor (supply to base) [ohm]
  in   R2       lower divider resistor (base to ground) [ohm]
  in   RC       collector resistor [ohm]
  in   RE       emitter resistor [ohm]
  in   beta     current gain [1]
  in   VBE      base-emitter voltage [V]
  out  VTH      Thevenin voltage of the divider [V]
  out  RTH      Thevenin resistance of the divider [ohm]
  out  IB       base current [A]
  out  IC       collector current [A]
  out  IE       emitter current [A]
  out  VB       base voltage [V]
  out  VE       emitter voltage [V]
  out  VC       collector voltage [V]
  out  VCE      collector-emitter voltage [V]
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ electronics-engineering solve elx_bjt_voltage_divider_bias "VCC=12 V" "R1=47 kohm" "R2=10 kohm" "RC=2.2 kohm" "RE=470 ohm" beta=150.0 "VBE=0.7 V"
VTH = 2.105263158 V
RTH = 8245.614035 ohm
IB = 1.773972436e-05 A
IC = 0.002660958654 A
IE = 0.002678698378 A
VB = 1.958988238 V
VE = 1.258988238 V
VC = 6.145890962 V
VCE = 4.886902724 V
  by: BJT voltage-divider bias: exact Q-point  (VTH = VCC R2/(R1 + R2); RTH = R1 || R2; IB = (VTH - VBE)/(RTH + (beta + 1) RE); VCE = VCC - IC RC - IE RE)
  VCC = 12 V
  R1 = 47000 ohm
  R2 = 10000 ohm
  RC = 2200 ohm
  RE = 470 ohm
  beta = 150
  VBE = 0.7 V
  assumes: VTH > VBE; active region: VCE >= 0.2 V
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | electronics-engineering batch

`cases.jsonl`:

```json
{"key": "elx_bjt_voltage_divider_bias", "inputs": {"VCC": "12 V", "R1": "47 kohm", "R2": "10 kohm", "RC": "2.2 kohm", "RE": "470 ohm", "beta": 150.0, "VBE": "0.7 V"}}
{"key": "elx_sallen_key_lowpass", "inputs": {"R1": "10 kohm", "R2": "10 kohm", "C1": "20 nF", "C2": "10 nF", "K": 1.0}}
```

```console
$ electronics-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.00266095865382,"unit":"A","solved_for":"IC","si":[["VCC",12.0],["R1",47000.0],["R2",10000.0],["RC",2200.0],["RE",470.0],["beta",150.0],["VBE",0.7]],"values":[["VTH",2.10526315789,"V"],["RTH",8245.61403509,"ohm"],["IB",0.0000177397243588,"A"],["IC",0.00266095865382,"A"],["IE",0.00267869837818,"A"],["VB",1.95898823774,"V"],["VE",1.25898823774,"V"],["VC",6.1458909616,"V"],["VCE",4.88690272386,"V"]],"target":"IC","phrase":"Thevenin voltage of the divider","assumptions":["VTH > VBE","active region: VCE >= 0.2 V"],"name":"BJT voltage-divider bias: exact Q-point","equation":"VTH = VCC R2/(R1 + R2); RTH = R1 || R2; IB = (VTH - VBE)/(RTH + (beta + 1) RE); VCE = VCC - IC RC - IE RE"}
{"outcome":"SOLVED","message":"","value":0.707106781187,"unit":"","solved_for":"Q","si":[["R1",10000.0],["R2",10000.0],["C1",2e-8],["C2",1e-8],["K",1.0]],"values":[["f0",1125.3953952,"Hz"],["Q",0.707106781187,""],["H0",1.0,""],["peak_gain",0.707106781187,""]],"target":"Q","phrase":"natural frequency","assumptions":["stable: R1 C2 + R2 C2 + R1 C1 (1 - K) > 0","ideal amplifier"],"name":"Sallen-Key low-pass filter: f0, Q and gain","equation":"w0 = 1/sqrt(R1 R2 C1 C2); Q = sqrt(R1 R2 C1 C2)/(R1 C2 + R2 C2 + R1 C1 (1 - K))"}
```

### In your browser

Open [Electronics Engineering on ai999x.com](https://www.ai999x.com/models/electronics-engineering-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `electronics_engineering_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install electronics-engineering-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Butterworth filter: Q of each second-order section

The quality factors of the cascaded second-order sections of an n-th order Butterworth filter (all at the cutoff frequency), and whether a first-order section is needed.

Command line:

```console
$ electronics-engineering solve elx_butterworth_section_q n=4
Q_max = 1.306562965
Q = [0.541196100146,1.30656296488]
sections = 2
first_order = 0
  by: Butterworth filter: Q of each second-order section  (Q_k = 1/(2 sin((2k - 1) pi/(2n))), k = 1 .. floor(n/2))
  assumes: 2 <= n <= 20
```

Python:

```python
import electronics_engineering_ai999x as m

a = m.solve("elx_butterworth_section_q", n="4")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/electronics-engineering-ai999x/#cap-elx_butterworth_section_q) and press **Compute in your browser**.

### Butterworth or Chebyshev low-pass filter order

The minimum order of a Butterworth or Chebyshev type I low-pass filter meeting a passband edge with at most Ap dB attenuation and a stopband edge with at least As dB.

Command line:

```console
$ electronics-engineering solve elx_filter_order ftype=Butterworth "fp=1000 Hz" "fs=2500 Hz" Ap=1 As=40
n = 6
  by: Butterworth or Chebyshev low-pass filter order  (Butterworth: n >= log10(D)/(2 log10(fs/fp)); Chebyshev: n >= acosh(sqrt D)/acosh(fs/fp), D = (10^(As/10) - 1)/(10^(Ap/10) - 1))
  fp = 1000 Hz
  fs = 2500 Hz
  Ap = 1
  As = 40
  ftype = Butterworth
  assumes: fs > fp; As > Ap > 0
```

Python:

```python
import electronics_engineering_ai999x as m

a = m.solve("elx_filter_order", ftype="Butterworth", fp="1000 Hz", fs="2500 Hz", Ap="1", As="40")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/electronics-engineering-ai999x/#cap-elx_filter_order) and press **Compute in your browser**.

### Chebyshev type I filter: section Q and frequencies

The quality factors and natural frequencies (relative to the passband edge) of the second-order sections of an n-th order Chebyshev type I low-pass filter with a given passband ripple, and the real pole of an odd order.

Command line:

```console
$ electronics-engineering solve elx_chebyshev_sections n=4 ripple=0.5
Q_max = 2.940554174
Q = [0.705110236803,2.94055417439]
f0_ratio = [0.59700239512,1.03127040148]
sections = 2
real_pole = 0
  by: Chebyshev type I filter: section Q and frequencies  (eps = sqrt(10^(r/10) - 1); a = asinh(1/eps)/n; poles -sinh a sin th_k + j cosh a cos th_k; Q = |p|/(2 |Re p|))
  ripple = 0.5
  assumes: 2 <= n <= 16; 0.001 dB <= ripple <= 6 dB
```

Python:

```python
import electronics_engineering_ai999x as m

a = m.solve("elx_chebyshev_sections", n="4", ripple="0.5")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/electronics-engineering-ai999x/#cap-elx_chebyshev_sections) and press **Compute in your browser**.

### Equal-component Sallen-Key low-pass design

The resistor value and amplifier gain of an equal-component (R1 = R2 = R, C1 = C2 = C) Sallen-Key low-pass filter for a cutoff frequency and Q, with the non-inverting gain ratio Rf/Rg = K - 1.

Command line:

```console
$ electronics-engineering solve elx_sallen_key_equal_component_design "fc=1000 Hz" Q=0.7071 "C=10 nF"
R = 15915.49431 ohm
K = 1.585772875
Rf_over_Rg = 0.5857728751
  by: Equal-component Sallen-Key low-pass design  (R = 1/(2 pi fc C); K = 3 - 1/Q; Rf/Rg = K - 1)
  fc = 1000 Hz
  Q = 0.7071
  C = 1e-08 F
  assumes: 0.5 <= Q <= 20
```

Python:

```python
import electronics_engineering_ai999x as m

a = m.solve("elx_sallen_key_equal_component_design", fc="1000 Hz", Q="0.7071", C="10 nF")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/electronics-engineering-ai999x/#cap-elx_sallen_key_equal_component_design) and press **Compute in your browser**.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/electronics-engineering-ai999x/blob/main/docs/electronics_engineering_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/electronics-engineering-ai999x/blob/main/docs/electronics_engineering_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/electronics-engineering-ai999x/blob/main/docs/electronics_engineering_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/electronics-engineering-ai999x/blob/main/docs/electronics_engineering_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/electronics-engineering-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified electronics engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 561 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `electronics-engineering` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** semiconductors, diode and transistor circuits, amplifiers, active filters, oscillators, data converters, timing and power supplies.

## Model overview

| Property | Value |
|---|---|
| Package | `electronics-engineering-ai999x` 1.1.0 (import `electronics_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic electronics engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 92 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 561 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `electronics-engineering-ai999x "<question>"`; for the native program use `electronics-engineering` (above). The outputs above were produced by running this exact code on electronics-engineering-ai999x 1.1.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `electronics-engineering-ai999x`; the answer is the one it returned, with the steps it took.

**BJT voltage-divider bias: exact Q-point** · _BJT_

> BJT voltage-divider bias: VCC = 12 V, R1 = 47 kohm, R2 = 10 kohm, RC = 2.2 kohm, RE = 470 ohm, beta = 150, VBE = 0.7 V

→ **VTH = 2.105263158 V, RTH = 8245.614035 ohm, IB = 1.773972436e-05 A, IC = 0.002660958654 A, IE = 0.002678698378 A, VB = 1.958988238 V, VE = 1.258988238 V, VC = 6.145890962 V, VCE = 4.886902724 V**

<details><summary>Steps</summary>

- BJT voltage-divider bias: exact Q-point: VCC = 12 V, R1 = 47 kohm, R2 = 10 kohm, RC = 2.2 kohm, RE = 470 ohm, beta = 150, VBE = 0.7 V → VTH = 2.105263158 V, RTH = 8245.614035 ohm, IB = 1.773972436e-05 A, IC = 0.002660958654 A, IE = 0.002678698378 A, VB = 1.958988238 V, VE = 1.258988238 V, VC = 6.145890962 V, VCE = 4.886902724 V

</details>

**Sallen-Key low-pass filter: f0, Q and gain** · _Active filters_

> Sallen-Key low-pass filter: R1 = 10 kohm, R2 = 10 kohm, C1 = 20 nF, C2 = 10 nF, K = 1

→ **f0 = 1125.395395 Hz, Q = 0.7071067812, H0 = 1, peak_gain = 0.7071067812**

<details><summary>Steps</summary>

- Sallen-Key low-pass filter: f0, Q and gain: R1 = 10 kohm, R2 = 10 kohm, C1 = 20 nF, C2 = 10 nF, K = 1 → f0 = 1125.395395 Hz, Q = 0.7071067812, H0 = 1, peak_gain = 0.7071067812

</details>

**Common-emitter amplifier: gain, input and output resistance** · _BJT_

`m.solve("elx_common_emitter_amplifier", IC=0.001, beta=100.0, VT=0.025, RC=5000.0, RL=10000.0, RE1=0.0, RB=20000.0, Rsig=1000.0)`

→ **gm = 0.04 S, r_pi = 2500 ohm, Av = -133.3333333, Rin = 2222.222222 ohm, Rout = 5000 ohm, Gv = -91.95402299**

<details><summary>Steps</summary>

- Common-emitter amplifier: gain, input and output resistance: IC = 0.001 A, beta = 100, VT = 0.025 V, RC = 5000 ohm, RL = 10000 ohm, RE1 = 0 ohm, RB = 20000 ohm, Rsig = 1000 ohm → gm = 0.04 S, r_pi = 2500 ohm, Av = -133.3333333, Rin = 2222.222222 ohm, Rout = 5000 ohm, Gv = -91.95402299

</details>

**555 timer astable: frequency and duty cycle** · _Oscillators_

> 555 astable: RA = 1 kohm, RB = 10 kohm, C = 10 uF

→ **t_high = 0.07624618986 s, t_low = 0.06931471806 s, T = 0.1455609079 s, f = 6.869976385 Hz, duty = 0.5238095238**

<details><summary>Steps</summary>

- 555 timer astable: frequency and duty cycle: RA = 1 kohm, RB = 10 kohm, C = 10 uF → t_high = 0.07624618986 s, t_low = 0.06931471806 s, T = 0.1455609079 s, f = 6.869976385 Hz, duty = 0.5238095238 Hz

</details>

**Operating point of a diode in series with a resistor (exact)** · _Diodes_

> Diode operating point: VS = 5 V, R = 1 kohm, Is = 1e-14 A, n = 1, T = 300 K

→ **ID = 0.004307456373 A, VD = 0.6925436275 V, P_D = 0.002983101461 W**

<details><summary>Steps</summary>

- Operating point of a diode in series with a resistor (exact): VS = 5 V, R = 1 kohm, Is = 1e-14 A, n = 1, T = 300 K → ID = 0.004307456373 A, VD = 0.6925436275 V, P_D = 0.002983101461 W

</details>

**CMOS gate propagation delay (switch-resistor model)** · _CMOS_

> CMOS propagation delay: Rn = 10000 ohm, Rp = 20000 ohm, CL = 100 fF

→ **tpHL = 6.931471806e-10 s, tpLH = 1.386294361e-09 s, tp = 1.039720771e-09 s**

<details><summary>Steps</summary>

- CMOS gate propagation delay (switch-resistor model): Rn = 10000 ohm, Rp = 20000 ohm, CL = 100 fF → tpHL = 6.931471806e-10 s, tpLH = 1.386294361e-09 s, tp = 1.039720771e-09 s

</details>

## Figures

Every point was computed by `electronics-engineering-ai999x`.

**Sallen-Key low-pass filter: f0, Q and gain**

![Sallen-Key low-pass filter: f0, Q and gain — computed by electronics-engineering-ai999x](https://huggingface.co/AI999X/electronics-engineering-ai999x/resolve/main/figures/elx-sallen-key-lowpass-q-vs-r1.svg)

_Q (quality factor) against R1 (first series resistor), with C1 = 2.00e-8 F, C2 = 1.00e-8 F, K = 1, R2 = 10000 ohm. 31 points computed by electronics-engineering-ai999x; marked: R1 = 10000, Q = 0.707107._

**ADC output code, LSB and quantisation error**

![ADC output code, LSB and quantisation error — computed by electronics-engineering-ai999x](https://huggingface.co/AI999X/electronics-engineering-ai999x/resolve/main/figures/elx-adc-code-code-vs-vin.svg)

_code (output code (decimal)) against Vin (analog input), with Vref = 5 V, n = 8. 31 points computed by electronics-engineering-ai999x; marked: Vin = 3.2, code = 163._

**Operating point of a diode in series with a resistor (exact)**

![Operating point of a diode in series with a resistor (exact) — computed by electronics-engineering-ai999x](https://huggingface.co/AI999X/electronics-engineering-ai999x/resolve/main/figures/elx-diode-operating-point-id-vs-vs.svg)

_ID (diode current) against VS (source voltage), with Is = 1.00e-14 A, R = 1000 ohm, T = 300 K, n = 1. 31 points computed by electronics-engineering-ai999x; marked: VS = 5, ID = 0.00430746 A._

**555 timer astable: frequency and duty cycle**

![555 timer astable: frequency and duty cycle — computed by electronics-engineering-ai999x](https://huggingface.co/AI999X/electronics-engineering-ai999x/resolve/main/figures/elx-555-astable-f-vs-ra.svg)

_f (frequency) against RA (resistor from Vcc to the discharge pin), with C = 1.00e-5 F, RB = 10000 ohm. 31 points computed by electronics-engineering-ai999x; marked: RA = 1000, f = 6.86998 Hz._

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
@software{electronics_engineering_ai999x_1_1_0,
  title  = {electronics-engineering-ai999x: an offline, verified electronics engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.1.0},
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
