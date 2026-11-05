# electronics-engineering: the Electronics Engineering program — user guide

**electronics-engineering-ai999x 1.1.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Electronics Engineering covers semiconductors and pn junctions, diode circuits and rectifiers, BJT and MOSFET biasing and amplifiers, op-amp circuits, active filters, oscillators and 555 timers, ADCs and DACs, digital timing and power supplies. This guide is about `electronics-engineering`, the model as a self-contained program for Windows and Linux: 92 capabilities in 11 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/electronics-engineering-ai999x/); the Python package (`pip install electronics-engineering-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.4.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Electronics Engineering | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-electronics-engineering-1.4.0-windows-x86_64.zip |
| Desktop app: Electronics Engineering | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-electronics-engineering-1.4.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz |

### Desktop app

Electronics Engineering in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-electronics-engineering-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-electronics-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-electronics-engineering-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-electronics-engineering-1.4.0-linux-x86_64.tar.gz
./ai999x-electronics-engineering-1.4.0-linux-x86_64/ai999x-electronics-engineering
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.4.0-linux-x86_64.tar.gz
./ai999x-1.4.0-linux-x86_64/ai999x
```

### Command line — Linux

The bundle holds one program per model; `electronics-engineering` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
electronics-engineering --version
```

### Command line — Windows (PowerShell)

The fourth line should print True.

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

### In your browser

Open https://www.ai999x.com/models/electronics-engineering-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install electronics-engineering-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ electronics-engineering --version
electronics-engineering-ai999x 1.0.0 (Rust build, ax-engine 0.1.0)
```

### 2.2 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ electronics-engineering search "bjt voltage-divider bias: exact q-point"
elx_bjt_voltage_divider_bias         BJT voltage-divider bias: exact Q-point  (BJT / DC bias)
elx_mosfet_divider_bias              MOSFET voltage-divider bias with source resistor: Q-point  (MOSFET / DC bias)
elx_bjt_collector_feedback_bias      BJT collector-feedback bias: Q-point  (BJT / DC bias)
elx_bjt_fixed_bias                   BJT fixed (base-resistor) bias: Q-point  (BJT / DC bias)
elx_bjt_emitter_bias                 BJT dual-supply emitter bias: Q-point  (BJT / DC bias)
elx_diode_operating_point            Operating point of a diode in series with a resistor (exact)  (diodes / operating point)
elx_bjt_cascode                      BJT cascode: output resistance, transconductance and gain  (BJT / amplifiers)
elx_differential_pair                BJT differential pair: differential and common-mode gain, CMRR  (BJT / differential amplifiers)
… 12 more
```

### 2.3 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ electronics-engineering list
elx_555_astable                          555 timer astable: frequency and duty cycle
elx_555_monostable                       555 timer monostable pulse width
elx_adc_code                             ADC output code, LSB and quantisation error
elx_adc_enob                             Effective number of bits (ENOB) from SINAD
elx_adc_quantization_snr                 ADC quantisation SNR with oversampling
elx_adjustable_regulator_output          Adjustable three-terminal regulator output voltage
elx_aperture_jitter_snr                  SNR limit from sampling-clock (aperture) jitter
elx_bjt_cascode                          BJT cascode: output resistance, transconductance and gain
… 84 more
```

### 2.4 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.5 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
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

### 2.6 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ electronics-engineering solve elx_bjt_voltage_divider_bias "VCC=12 V" "R1=47 kohm" "R2=10 kohm" "RC=2.2 kohm" "RE=470 ohm" beta=150.0 "VBE=0.7 V" --json
{"outcome":"SOLVED","message":"","value":0.00266095865382,"unit":"A","solved_for":"IC","si":[["VCC",12.0],["R1",47000.0],["R2",10000.0],["RC",2200.0],["RE",470.0],["beta",150.0],["VBE",0.7]],"values":[["VTH",2.10526315789,"V"],["RTH",8245.61403509,"ohm"],["IB",0.0000177397243588,"A"],["IC",0.00266095865382,"A"],["IE",0.00267869837818,"A"],["VB",1.95898823774,"V"],["VE",1.25898823774,"V"],["VC",6.1458909616,"V"],["VCE",4.88690272386,"V"]],"target":"IC","phrase":"Thevenin voltage of the divider","assumptions":["VTH > VBE","active region: VCE >= 0.2 V"],"name":"BJT voltage-divider bias: exact Q-point","equation":"VTH = VCC R2/(R1 + R2); RTH = R1 || R2; IB = (VTH - VBE)/(RTH + (beta + 1) RE); VCE = VCC - IC RC - IE RE"}
```

### 2.7 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ electronics-engineering solve elx_bjt_voltage_divider_bias "VCC=12 V" "R1=47 kohm" "R2=10 kohm" "RC=2.2 kohm" "RE=470 ohm" beta=150.0
NEEDS_INPUT: elx_bjt_voltage_divider_bias needs VBE (base-emitter voltage). Example: solve('elx_bjt_voltage_divider_bias', VCC='12 V', R1='47000 ohm', R2='10000 ohm', RC='2200 ohm', RE='470 ohm', beta=150, VBE='0.7 V').
```

### 2.8 Draw the figures

The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```shell
$ electronics-engineering figures elx_bjt_voltage_divider_bias "VCC=12 V" "R1=47 kohm" "R2=10 kohm" "RC=2.2 kohm" "RE=470 ohm" beta=150.0 "VBE=0.7 V" --out .
[
  {
    "slug": "elx-bjt-voltage-divider-bias-live-1",
    "title": "Collector current as emitter resistor varies",
    "caption": "Collector current as RE runs from 240.9 to 822.5 ohm, every other input as you entered it (34 solutions). Marked: your RE = 470 ohm gives IC = 0.002661. Your inputs: VCC = 12 V, R1 = 47 kohm, R2 = 10 kohm, RC = 2.2 kohm, RE = 470 ohm, beta = 150.0, VBE = 0.7 V.",
    "file": ".\\elx-bjt-voltage-divider-bias-live-1.svg"
  },
  {
    "slug": "elx-bjt-voltage-divider-bias-live-2",
    "title": "Collector current as lower divider resistor (base to ground) varies",
    "caption": "Collector current as R2 runs from 3.036 to 16.43 kohm, every other input as you entered it (26 solutions). Marked: your R2 = 10 kohm gives IC = 0.002661. Your inputs: VCC = 12 V, R1 = 47 kohm, R2 = 10 kohm, RC = 2.2 kohm, RE = 470 ohm, beta = 150.0, VBE = 0.7 V.",
    "file": ".\\elx-bjt-voltage-divider-bias-live-2.svg"
  }
]
```

### 2.9 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | electronics-engineering batch

`cases.jsonl`:

```json
{"key": "elx_bjt_voltage_divider_bias", "inputs": {"VCC": "12 V", "R1": "47 kohm", "R2": "10 kohm", "RC": "2.2 kohm", "RE": "470 ohm", "beta": 150.0, "VBE": "0.7 V"}}
{"key": "elx_sallen_key_lowpass", "inputs": {"R1": "10 kohm", "R2": "10 kohm", "C1": "20 nF", "C2": "10 nF", "K": 1.0}}
```

```shell
$ electronics-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.00266095865382,"unit":"A","solved_for":"IC","si":[["VCC",12.0],["R1",47000.0],["R2",10000.0],["RC",2200.0],["RE",470.0],["beta",150.0],["VBE",0.7]],"values":[["VTH",2.10526315789,"V"],["RTH",8245.61403509,"ohm"],["IB",0.0000177397243588,"A"],["IC",0.00266095865382,"A"],["IE",0.00267869837818,"A"],["VB",1.95898823774,"V"],["VE",1.25898823774,"V"],["VC",6.1458909616,"V"],["VCE",4.88690272386,"V"]],"target":"IC","phrase":"Thevenin voltage of the divider","assumptions":["VTH > VBE","active region: VCE >= 0.2 V"],"name":"BJT voltage-divider bias: exact Q-point","equation":"VTH = VCC R2/(R1 + R2); RTH = R1 || R2; IB = (VTH - VBE)/(RTH + (beta + 1) RE); VCE = VCC - IC RC - IE RE"}
{"outcome":"SOLVED","message":"","value":0.707106781187,"unit":"","solved_for":"Q","si":[["R1",10000.0],["R2",10000.0],["C1",2e-8],["C2",1e-8],["K",1.0]],"values":[["f0",1125.3953952,"Hz"],["Q",0.707106781187,""],["H0",1.0,""],["peak_gain",0.707106781187,""]],"target":"Q","phrase":"natural frequency","assumptions":["stable: R1 C2 + R2 C2 + R1 C1 (1 - K) > 0","ideal amplifier"],"name":"Sallen-Key low-pass filter: f0, Q and gain","equation":"w0 = 1/sqrt(R1 R2 C1 C2); Q = sqrt(R1 R2 C1 C2)/(R1 C2 + R2 C2 + R1 C1 (1 - K))"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `electronics-engineering info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`electronics-engineering solve ... --json` prints one answer as one line of JSON. `electronics-engineering batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | electronics-engineering batch`. `electronics-engineering catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["electronics-engineering", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`electronics-engineering figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

```shell
$ electronics-engineering figures elx_bjt_voltage_divider_bias "VCC=12 V" "R1=47 kohm" "R2=10 kohm" "RC=2.2 kohm" "RE=470 ohm" beta=150.0 "VBE=0.7 V" --out .
[
  {
    "slug": "elx-bjt-voltage-divider-bias-live-1",
    "title": "Collector current as emitter resistor varies",
    "caption": "Collector current as RE runs from 240.9 to 822.5 ohm, every other input as you entered it (34 solutions). Marked: your RE = 470 ohm gives IC = 0.002661. Your inputs: VCC = 12 V, R1 = 47 kohm, R2 = 10 kohm, RC = 2.2 kohm, RE = 470 ohm, beta = 150.0, VBE = 0.7 V.",
    "file": ".\\elx-bjt-voltage-divider-bias-live-1.svg"
  },
  {
    "slug": "elx-bjt-voltage-divider-bias-live-2",
    "title": "Collector current as lower divider resistor (base to ground) varies",
    "caption": "Collector current as R2 runs from 3.036 to 16.43 kohm, every other input as you entered it (26 solutions). Marked: your R2 = 10 kohm gives IC = 0.002661. Your inputs: VCC = 12 V, R1 = 47 kohm, R2 = 10 kohm, RC = 2.2 kohm, RE = 470 ohm, beta = 150.0, VBE = 0.7 V.",
    "file": ".\\elx-bjt-voltage-divider-bias-live-2.svg"
  }
]
```

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `electronics_engineering_ai999x.wasm` (https://www.ai999x.com/compute/rust/electronics_engineering_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("electronics_engineering_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `electronics-engineering info <id>` shows the same inputs and outputs in the terminal.

### Active filters (9)

<a id="elx_butterworth_section_q"></a>

#### Butterworth filter: Q of each second-order section — `elx_butterworth_section_q`

The quality factors of the cascaded second-order sections of an n-th order Butterworth filter (all at the cutoff frequency), and whether a first-order section is needed.

$$
Q_k = \frac{1}{2\sin\left(\frac{(2k-1)\pi}{2n}\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | filter order | — |
| `Q_max` | out | largest section Q | — |
| `Q` | out | section Q values, ascending | — |
| `sections` | out | number of second-order sections | — |
| `first_order` | out | 1 if a first-order section is needed | — |

**Applies when:** 2 <= n <= 20.

```shell
$ electronics-engineering solve elx_butterworth_section_q n=4
Q_max = 1.306562965
Q = [0.541196100146,1.30656296488]
sections = 2
first_order = 0
  by: Butterworth filter: Q of each second-order section  (Q_k = 1/(2 sin((2k - 1) pi/(2n))), k = 1 .. floor(n/2))
  assumes: 2 <= n <= 20
```

In the worked example, n = 4 go through the equation above and give Q_max = 1.306562965, Q = [0.5411961001, 1.306562965], sections = 2, first_order = 0. It applies only when 2 <= n <= 20: outside that it says what is wrong instead of returning a number.

<a id="elx_filter_order"></a>

#### Butterworth or Chebyshev low-pass filter order — `elx_filter_order`

The minimum order of a Butterworth or Chebyshev type I low-pass filter meeting a passband edge with at most Ap dB attenuation and a stopband edge with at least As dB.

$$
n \ge \frac{\log_{10}D}{2\log_{10}(f_s/f_p)},\quad n \ge \frac{\cosh^{-1}\sqrt{D}}{\cosh^{-1}(f_s/f_p)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ftype` | in | butterworth or chebyshev (type I) | — |
| `fp` | in | passband edge | Hz |
| `fs` | in | stopband edge | Hz |
| `Ap` | in | maximum passband attenuation (ripple) in dB | — |
| `As` | in | minimum stopband attenuation in dB | — |
| `n` | out | minimum filter order | — |

**Applies when:** fs > fp; As > Ap > 0.

```shell
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

In the worked example, ftype = Butterworth, fp = 1000 Hz, fs = 2500 Hz, Ap = 1, As = 40 go through the equation above and give n = 6. It applies only when fs > fp; As > Ap > 0: outside that it says what is wrong instead of returning a number.

<a id="elx_chebyshev_sections"></a>

#### Chebyshev type I filter: section Q and frequencies — `elx_chebyshev_sections`

The quality factors and natural frequencies (relative to the passband edge) of the second-order sections of an n-th order Chebyshev type I low-pass filter with a given passband ripple, and the real pole of an odd order.

$$
p_k = -\sinh a\sin\theta_k + j\cosh a\cos\theta_k,\ a = \frac{1}{n}\sinh^{-1}\frac{1}{\varepsilon},\ Q_k = \frac{|p_k|}{2|\Re p_k|}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | filter order | — |
| `ripple` | in | passband ripple in dB | — |
| `Q_max` | out | largest section Q | — |
| `Q` | out | section Q values, ascending | — |
| `f0_ratio` | out | section natural frequencies / passband edge, in the same order | — |
| `sections` | out | number of second-order sections | — |
| `real_pole` | out | real pole / passband edge (0 for even n) | — |

**Applies when:** 2 <= n <= 16; 0.001 dB <= ripple <= 6 dB.

```shell
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

In the worked example, n = 4, ripple = 0.5 go through the equation above and give Q_max = 2.940554174, Q = [0.7051102368, 2.940554174], f0_ratio = [0.5970023951, 1.031270401], sections = 2. It applies only when 2 <= n <= 16; 0.001 dB <= ripple <= 6 dB: outside that it says what is wrong instead of returning a number.

<a id="elx_sallen_key_equal_component_design"></a>

#### Equal-component Sallen-Key low-pass design — `elx_sallen_key_equal_component_design`

The resistor value and amplifier gain of an equal-component (R1 = R2 = R, C1 = C2 = C) Sallen-Key low-pass filter for a cutoff frequency and Q, with the non-inverting gain ratio Rf/Rg = K - 1.

$$
R = \frac{1}{2\pi f_cC},\quad K = 3 - \frac{1}{Q}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `fc` | in | natural (cutoff) frequency | Hz |
| `Q` | in | quality factor (0.7071 for Butterworth) | — |
| `C` | in | chosen capacitor value | F |
| `R` | out | resistor value | ohm |
| `K` | out | amplifier gain | — |
| `Rf_over_Rg` | out | gain resistor ratio Rf/Rg | — |

**Applies when:** 0.5 <= Q <= 20.

```shell
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

In the worked example, fc = 1000 Hz, Q = 0.7071, C = 10 nF go through the equation above and give R = 15915.49431 ohm, K = 1.585772875, Rf_over_Rg = 0.5857728751. It applies only when 0.5 <= Q <= 20: outside that it says what is wrong instead of returning a number.

<a id="elx_first_order_active_lowpass"></a>

#### First-order active low-pass filter (inverting) — `elx_first_order_active_lowpass`

The passband gain -Rf/R1, the cutoff frequency 1/(2 pi Rf C) and the gain at a frequency of an inverting op-amp low-pass filter with C across the feedback resistor.

$$
H(j\omega) = -\frac{R_f/R_1}{1 + j\omega R_fC},\quad f_c = \frac{1}{2\pi R_fC}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R1` | in | input resistor | ohm |
| `Rf` | in | feedback resistor | ohm |
| `C` | in | capacitor across Rf | F |
| `f` | in | frequency of interest | Hz |
| `H0` | out | passband gain | — |
| `fc` | out | cutoff frequency | Hz |
| `gain_f` | out | gain magnitude at f | — |
| `gain_dB` | out | gain at f in dB | — |

**Applies when:** ideal op-amp.

```shell
$ electronics-engineering solve elx_first_order_active_lowpass "R1=10000 ohm" "Rf=100000 ohm" "C=1.59 nF" "f=1000 Hz"
H0 = -10
fc = 1000.974485 Hz
gain_f = 7.074510619
gain_dB = 16.99392806
  by: First-order active low-pass filter (inverting)  (H(s) = -(Rf/R1)/(1 + s Rf C); fc = 1/(2 pi Rf C))
  R1 = 10000 ohm
  Rf = 100000 ohm
  C = 1.59e-09 F
  f = 1000 Hz
  assumes: ideal op-amp
```

In the worked example, R1 = 10000 ohm, Rf = 100000 ohm, C = 1.59 nF, f = 1000 Hz go through the equation above and give H0 = -10, fc = 1000.974485 Hz, gain_f = 7.074510619, gain_dB = 16.99392806. It applies only when ideal op-amp: outside that it says what is wrong instead of returning a number.

<a id="elx_mfb_bandpass"></a>

#### Multiple-feedback (MFB) band-pass filter — `elx_mfb_bandpass`

The centre frequency, quality factor, -3 dB bandwidth and peak (inverting) gain of a multiple-feedback band-pass filter.

$$
\omega_0 = \sqrt{\frac{R_1+R_2}{R_1R_2R_3C_1C_2}},\quad Q = \frac{\omega_0C_1C_2R_3}{C_1+C_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R1` | in | input resistor | ohm |
| `R2` | in | resistor from the middle node to ground | ohm |
| `R3` | in | feedback resistor (inverting input to output) | ohm |
| `C1` | in | capacitor from the middle node to the inverting input | F |
| `C2` | in | capacitor from the middle node to the output | F |
| `f0` | out | centre frequency | Hz |
| `Q` | out | quality factor | — |
| `BW` | out | -3 dB bandwidth | Hz |
| `H_peak` | out | gain at the centre frequency | — |

**Applies when:** ideal op-amp.

```shell
$ electronics-engineering solve elx_mfb_bandpass "R1=15900 ohm" "R2=1330 ohm" "R3=31800 ohm" "C1=10 nF" "C2=10 nF"
f0 = 2547.562384 Hz
Q = 2.54508224
BW = 1000.974485 Hz
H_peak = -1
  by: Multiple-feedback (MFB) band-pass filter  (w0 = sqrt((R1 + R2)/(R1 R2 R3 C1 C2)); Q = w0 C1 C2 R3/(C1 + C2); H = -C1 R3/(R1 (C1 + C2)))
  R1 = 15900 ohm
  R2 = 1330 ohm
  R3 = 31800 ohm
  C1 = 1e-08 F
  C2 = 1e-08 F
  assumes: ideal op-amp
```

In the worked example, R1 = 15900 ohm, R2 = 1330 ohm, R3 = 31800 ohm, C1 = 10 nF, C2 = 10 nF go through the equation above and give f0 = 2547.562384 Hz, Q = 2.54508224, BW = 1000.974485 Hz, H_peak = -1. It applies only when ideal op-amp: outside that it says what is wrong instead of returning a number.

<a id="elx_mfb_lowpass"></a>

#### Multiple-feedback (MFB) low-pass filter — `elx_mfb_lowpass`

The natural frequency, quality factor and inverting passband gain -R2/R1 of a second-order multiple-feedback low-pass filter.

$$
Q = \frac{\sqrt{R_2R_3C_1C_2}}{C_2\left(R_2 + R_3 + R_2R_3/R_1\right)},\quad H_0 = -\frac{R_2}{R_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R1` | in | input resistor | ohm |
| `R2` | in | feedback resistor (middle node to output) | ohm |
| `R3` | in | resistor from the middle node to the inverting input | ohm |
| `C1` | in | capacitor from the middle node to ground | F |
| `C2` | in | capacitor from the inverting input to the output | F |
| `f0` | out | natural frequency | Hz |
| `Q` | out | quality factor | — |
| `H0` | out | passband gain | — |

**Applies when:** ideal op-amp.

```shell
$ electronics-engineering solve elx_mfb_lowpass "R1=10000 ohm" "R2=10000 ohm" "R3=5000 ohm" "C1=47 nF" "C2=4.7 nF"
f0 = 1514.38839 Hz
Q = 1.118033989
H0 = -1
  by: Multiple-feedback (MFB) low-pass filter  (w0 = 1/sqrt(R2 R3 C1 C2); Q = sqrt(R2 R3 C1 C2)/(C2 (R2 + R3 + R2 R3/R1)); H0 = -R2/R1)
  R1 = 10000 ohm
  R2 = 10000 ohm
  R3 = 5000 ohm
  C1 = 4.7e-08 F
  C2 = 4.7e-09 F
  assumes: ideal op-amp
```

In the worked example, R1 = 10000 ohm, R2 = 10000 ohm, R3 = 5000 ohm, C1 = 47 nF, C2 = 4.7 nF go through the equation above and give f0 = 1514.38839 Hz, Q = 1.118033989, H0 = -1. It applies only when ideal op-amp: outside that it says what is wrong instead of returning a number.

<a id="elx_sallen_key_highpass"></a>

#### Sallen-Key high-pass filter: f0, Q and gain — `elx_sallen_key_highpass`

The natural frequency, quality factor and high-frequency gain of a second-order Sallen-Key high-pass filter with any components and amplifier gain K.

$$
Q = \frac{\sqrt{R_1R_2C_1C_2}}{R_1(C_1+C_2) + R_2C_2(1-K)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C1` | in | first series capacitor | F |
| `C2` | in | second series capacitor | F |
| `R1` | in | resistor from the middle node to the output (feedback) | ohm |
| `R2` | in | resistor from the amplifier input to ground | ohm |
| `K` | in | amplifier gain | — |
| `f0` | out | natural frequency | Hz |
| `Q` | out | quality factor | — |
| `H_inf` | out | high-frequency gain | — |

**Applies when:** stable: R1 (C1 + C2) + R2 C2 (1 - K) > 0.

```shell
$ electronics-engineering solve elx_sallen_key_highpass "C1=10 nF" "C2=10 nF" "R1=11250 ohm" "R2=22500 ohm" K=1
f0 = 1000.351462 Hz
Q = 0.7071067812
H_inf = 1
  by: Sallen-Key high-pass filter: f0, Q and gain  (w0 = 1/sqrt(R1 R2 C1 C2); Q = sqrt(R1 R2 C1 C2)/(R1 (C1 + C2) + R2 C2 (1 - K)))
  C1 = 1e-08 F
  C2 = 1e-08 F
  R1 = 11250 ohm
  R2 = 22500 ohm
  K = 1
  assumes: stable: R1 (C1 + C2) + R2 C2 (1 - K) > 0
```

In the worked example, C1 = 10 nF, C2 = 10 nF, R1 = 11250 ohm, R2 = 22500 ohm, K = 1 go through the equation above and give f0 = 1000.351462 Hz, Q = 0.7071067812, H_inf = 1. It applies only when stable: R1 (C1 + C2) + R2 C2 (1 - K) > 0: outside that it says what is wrong instead of returning a number.

<a id="elx_sallen_key_lowpass"></a>

#### Sallen-Key low-pass filter: f0, Q and gain — `elx_sallen_key_lowpass`

The natural frequency, quality factor, passband gain and the gain at f0 (K Q) of a second-order Sallen-Key low-pass filter with any components and amplifier gain K.

$$
\omega_0 = \frac{1}{\sqrt{R_1R_2C_1C_2}},\quad Q = \frac{\sqrt{R_1R_2C_1C_2}}{R_1C_2 + R_2C_2 + R_1C_1(1-K)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R1` | in | first series resistor | ohm |
| `R2` | in | second series resistor | ohm |
| `C1` | in | capacitor from the middle node to the output (feedback) | F |
| `C2` | in | capacitor from the amplifier input to ground | F |
| `K` | in | amplifier gain (1 for a unity-gain buffer) | — |
| `f0` | out | natural frequency | Hz |
| `Q` | out | quality factor | — |
| `H0` | out | passband gain | — |
| `peak_gain` | out | gain at f0, K Q | — |

**Applies when:** stable: R1 C2 + R2 C2 + R1 C1 (1 - K) > 0; ideal amplifier.

```shell
$ electronics-engineering solve elx_sallen_key_lowpass "R1=10000 ohm" "R2=10000 ohm" "C1=20 nF" "C2=10 nF" K=1
f0 = 1125.395395 Hz
Q = 0.7071067812
H0 = 1
peak_gain = 0.7071067812
  by: Sallen-Key low-pass filter: f0, Q and gain  (w0 = 1/sqrt(R1 R2 C1 C2); Q = sqrt(R1 R2 C1 C2)/(R1 C2 + R2 C2 + R1 C1 (1 - K)))
  R1 = 10000 ohm
  R2 = 10000 ohm
  C1 = 2e-08 F
  C2 = 1e-08 F
  K = 1
  assumes: stable: R1 C2 + R2 C2 + R1 C1 (1 - K) > 0; ideal amplifier
```

In the worked example, R1 = 10000 ohm, R2 = 10000 ohm, C1 = 20 nF, C2 = 10 nF, K = 1 go through the equation above and give f0 = 1125.395395 Hz, Q = 0.7071067812, H0 = 1, peak_gain = 0.7071067812. It applies only when stable: R1 C2 + R2 C2 + R1 C1 (1 - K) > 0; ideal amplifier: outside that it says what is wrong instead of returning a number.

### BJT (16)

<a id="elx_transition_frequency"></a>

#### BJT beta cutoff and transition frequency fT — `elx_transition_frequency`

The beta cutoff frequency f_beta and the unity-current-gain (transition) frequency fT = gm/(2 pi (Cpi + Cmu)) of a bipolar transistor from its hybrid-pi capacitances.

$$
f_T = \frac{g_m}{2\pi(C_\pi + C_\mu)},\quad f_\beta = \frac{f_T}{\beta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gm` | in | transconductance | S |
| `beta` | in | low-frequency current gain | — |
| `Cpi` | in | base-emitter capacitance | F |
| `Cmu` | in | base-collector capacitance | F |
| `r_pi` | out | r_pi = beta/gm | ohm |
| `f_beta` | out | beta cutoff frequency | Hz |
| `f_T` | out | transition frequency | Hz |

**Applies when:** single-pole h_fe (the feed-forward zero through Cmu neglected).

```shell
$ electronics-engineering solve elx_transition_frequency "gm=40 mS" beta=100 "Cpi=10 pF" "Cmu=2 pF"
r_pi = 2500 ohm
f_beta = 5305164.77 Hz
f_T = 530516477 Hz
  by: BJT beta cutoff and transition frequency fT  (f_beta = 1/(2 pi r_pi (Cpi + Cmu)); fT = beta f_beta = gm/(2 pi (Cpi + Cmu)))
  gm = 0.04 S
  beta = 100
  Cpi = 1e-11 F
  Cmu = 2e-12 F
  assumes: single-pole h_fe (the feed-forward zero through Cmu neglected)
```

In the worked example, gm = 40 mS, beta = 100, Cpi = 10 pF, Cmu = 2 pF go through the equation above and give r_pi = 2500 ohm, f_beta = 5305164.77 Hz, f_T = 530516477 Hz. It applies only when single-pole h_fe (the feed-forward zero through Cmu neglected): outside that it says what is wrong instead of returning a number.

<a id="elx_bjt_cascode"></a>

#### BJT cascode: output resistance, transconductance and gain — `elx_bjt_cascode`

The output resistance, the short-circuit transconductance and the voltage gain into a load of a cascode (common emitter driving common base) of two identical transistors at the same bias current.

$$
R_{out} = r_o + (1 + g_mr_o)(r_o\parallel r_\pi),\quad A_v = -G_m(R_{out}\parallel R_L)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `IC` | in | bias current of both transistors | A |
| `beta` | in | current gain | — |
| `VT` | in | thermal voltage | V |
| `VA` | in | Early voltage | V |
| `RL` | in | load resistance | ohm |
| `gm` | out | transconductance of each transistor | S |
| `r_o` | out | output resistance of each | ohm |
| `Rout` | out | output resistance of the cascode | ohm |
| `Gm` | out | short-circuit transconductance | S |
| `Av` | out | voltage gain into RL | — |

**Applies when:** identical transistors, base of the upper one at signal ground.

```shell
$ electronics-engineering solve elx_bjt_cascode "IC=1 mA" beta=100 "VT=25 mV" "VA=100 V" "RL=1000000 ohm"
gm = 0.04 S
r_o = 100000 ohm
Rout = 9858536.585 ohm
Gm = 0.03959426027 S
Av = -35947.88859
  by: BJT cascode: output resistance, transconductance and gain  (Rout = r_o + (1 + gm r_o)(r_o || r_pi); Gm = gm (gm + 1/r_o)/(gm + 2/r_o + 1/r_pi); Av = -Gm (Rout || RL))
  IC = 0.001 A
  beta = 100
  VT = 0.025 V
  VA = 100 V
  RL = 1000000 ohm
  assumes: identical transistors, base of the upper one at signal ground
```

In the worked example, IC = 1 mA, beta = 100, VT = 25 mV, VA = 100 V, RL = 1000000 ohm go through the equation above and give gm = 0.04 S, r_o = 100000 ohm, Rout = 9858536.585 ohm, Gm = 0.03959426027 S. It applies only when identical transistors, base of the upper one at signal ground: outside that it says what is wrong instead of returning a number.

<a id="elx_bjt_collector_feedback_bias"></a>

#### BJT collector-feedback bias: Q-point — `elx_bjt_collector_feedback_bias`

The Q-point of a transistor biased by a resistor from collector to base (collector feedback), with an optional emitter resistor.

$$
I_B = \frac{V_{CC} - V_{BE}}{R_F + (\beta+1)(R_C + R_E)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VCC` | in | supply voltage | V |
| `RC` | in | collector resistor (carries IC + IB) | ohm |
| `RF` | in | feedback resistor, collector to base | ohm |
| `beta` | in | current gain | — |
| `VBE` | in | base-emitter voltage | V |
| `RE` | in | emitter resistor (default 0) (optional) | ohm |
| `IB` | out | base current | A |
| `IC` | out | collector current | A |
| `IE` | out | emitter current | A |
| `VC` | out | collector voltage | V |
| `VCE` | out | collector-emitter voltage | V |

**Applies when:** active region: VCE >= 0.2 V.

```shell
$ electronics-engineering solve elx_bjt_collector_feedback_bias "VCC=10 V" "RC=4700 ohm" "RF=250000 ohm" beta=90 "VBE=0.7 V"
IB = 1.372288623e-05 A
IC = 0.001235059761 A
IE = 0.001248782647 A
VC = 4.130721558 V
VCE = 4.130721558 V
  by: BJT collector-feedback bias: Q-point  (IB = (VCC - VBE)/(RF + (beta + 1)(RC + RE)); VC = VCC - IE RC)
  VCC = 10 V
  RC = 4700 ohm
  RF = 250000 ohm
  beta = 90
  VBE = 0.7 V
  assumes: active region: VCE >= 0.2 V
```

In the worked example, VCC = 10 V, RC = 4700 ohm, RF = 250000 ohm, beta = 90, VBE = 0.7 V go through the equation above and give IB = 1.372288623e-05 A, IC = 0.001235059761 A, IE = 0.001248782647 A, VC = 4.130721558 V. It applies only when active region: VCE >= 0.2 V: outside that it says what is wrong instead of returning a number.

<a id="elx_current_mirror"></a>

#### BJT current mirror with finite beta — `elx_current_mirror`

The reference current and the output current of a basic two-transistor current mirror with a resistor to the supply, including the base-current error Iout = Iref beta/(beta + 2).

$$
I_{out} = \frac{\beta}{\beta+2}I_{ref}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VCC` | in | supply voltage | V |
| `R` | in | reference resistor | ohm |
| `VBE` | in | base-emitter voltage | V |
| `beta` | in | current gain of each (matched) transistor | — |
| `Iref` | out | current in the reference resistor | A |
| `Iout` | out | output (mirrored) current | A |
| `ratio` | out | Iout/Iref | — |

**Applies when:** matched transistors, Early effect neglected.

```shell
$ electronics-engineering solve elx_current_mirror "VCC=10 V" "R=9300 ohm" "VBE=0.7 V" beta=100
Iref = 0.001 A
Iout = 0.0009803921569 A
ratio = 0.9803921569
  by: BJT current mirror with finite beta  (Iref = (VCC - VBE)/R; Iout = Iref beta/(beta + 2))
  VCC = 10 V
  R = 9300 ohm
  VBE = 0.7 V
  beta = 100
  assumes: matched transistors, Early effect neglected
```

In the worked example, VCC = 10 V, R = 9300 ohm, VBE = 0.7 V, beta = 100 go through the equation above and give Iref = 0.001 A, Iout = 0.0009803921569 A, ratio = 0.9803921569. It applies only when matched transistors, Early effect neglected: outside that it says what is wrong instead of returning a number.

<a id="elx_differential_pair"></a>

#### BJT differential pair: differential and common-mode gain, CMRR — `elx_differential_pair`

The differential gain (differential and single-ended output), the common-mode gain and the common-mode rejection ratio of a resistively loaded BJT differential pair with a tail current source of finite resistance.

$$
A_{cm} = -\frac{g_mR_C}{1 + 2R_{EE}(\beta+1)/r_\pi},\quad \text{CMRR} = \frac{1 + 2R_{EE}(\beta+1)/r_\pi}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I_tail` | in | tail (bias) current | A |
| `beta` | in | current gain | — |
| `VT` | in | thermal voltage | V |
| `RC` | in | collector resistor of each side | ohm |
| `REE` | in | output resistance of the tail current source | ohm |
| `IC` | out | collector current of each transistor | A |
| `gm` | out | transconductance of each | S |
| `Ad_diff` | out | differential gain, differential output | — |
| `Ad_se` | out | differential gain, single-ended output | — |
| `Acm` | out | common-mode gain, single-ended output | — |
| `CMRR` | out | \|Ad_se/Acm\| | — |
| `CMRR_dB` | out | CMRR in dB | — |

**Applies when:** matched transistors, r_o neglected.

```shell
$ electronics-engineering solve elx_differential_pair "I_tail=1 mA" beta=100 "VT=25 mV" "RC=10000 ohm" "REE=100000 ohm"
IC = 0.000495049505 A
gm = 0.0198019802 S
Ad_diff = -198.019802
Ad_se = -99.00990099
Acm = -0.04949257735
CMRR = 2000.5
CMRR_dB = 66.02277111
  by: BJT differential pair: differential and common-mode gain, CMRR  (IC = alpha I/2; gm = IC/VT; Ad = -gm RC (/2 single-ended); Acm = -gm RC/(1 + 2 REE (beta + 1)/r_pi); CMRR = |Ad,se/Acm|)
  I_tail = 0.001 A
  beta = 100
  VT = 0.025 V
  RC = 10000 ohm
  REE = 100000 ohm
  assumes: matched transistors, r_o neglected
```

In the worked example, I_tail = 1 mA, beta = 100, VT = 25 mV, RC = 10000 ohm, REE = 100000 ohm go through the equation above and give IC = 0.000495049505 A, gm = 0.0198019802 S, Ad_diff = -198.019802, Ad_se = -99.00990099. It applies only when matched transistors, r_o neglected: outside that it says what is wrong instead of returning a number.

<a id="elx_bjt_emitter_bias"></a>

#### BJT dual-supply emitter bias: Q-point — `elx_bjt_emitter_bias`

The Q-point of a transistor with its base returned to ground through RB and its emitter returned to a negative supply -VEE through RE (emitter bias).

$$
I_E = \frac{V_{EE} - V_{BE}}{R_E + R_B/(\beta+1)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VCC` | in | positive supply | V |
| `VEE` | in | magnitude of the negative supply | V |
| `RB` | in | base resistor to ground | ohm |
| `RC` | in | collector resistor | ohm |
| `RE` | in | emitter resistor to -VEE | ohm |
| `beta` | in | current gain | — |
| `VBE` | in | base-emitter voltage | V |
| `IB` | out | base current | A |
| `IC` | out | collector current | A |
| `IE` | out | emitter current | A |
| `VB` | out | base voltage | V |
| `VE` | out | emitter voltage | V |
| `VC` | out | collector voltage | V |
| `VCE` | out | collector-emitter voltage | V |

**Applies when:** VEE > VBE; active region: VCE >= 0.2 V.

```shell
$ electronics-engineering solve elx_bjt_emitter_bias "VCC=10 V" "VEE=10 V" "RB=10000 ohm" "RC=4700 ohm" "RE=10000 ohm" beta=100 "VBE=0.7 V"
IB = 9.117647059e-06 A
IC = 0.0009117647059 A
IE = 0.0009208823529 A
VB = -0.09117647059 V
VE = -0.7911764706 V
VC = 5.714705882 V
VCE = 6.505882353 V
  by: BJT dual-supply emitter bias: Q-point  (IE = (VEE - VBE)/(RE + RB/(beta + 1)); VB = -IB RB; VE = VB - VBE; VC = VCC - IC RC)
  VCC = 10 V
  VEE = 10 V
  RB = 10000 ohm
  RC = 4700 ohm
  RE = 10000 ohm
  beta = 100
  VBE = 0.7 V
  assumes: VEE > VBE; active region: VCE >= 0.2 V
```

In the worked example, VCC = 10 V, VEE = 10 V, RB = 10000 ohm, RC = 4700 ohm, RE = 10000 ohm, beta = 100, VBE = 0.7 V go through the equation above and give IB = 9.117647059e-06 A, IC = 0.0009117647059 A, IE = 0.0009208823529 A, VB = -0.09117647059 V. It applies only when VEE > VBE; active region: VCE >= 0.2 V: outside that it says what is wrong instead of returning a number.

<a id="elx_bjt_fixed_bias"></a>

#### BJT fixed (base-resistor) bias: Q-point — `elx_bjt_fixed_bias`

The quiescent base and collector currents and the collector-emitter voltage of a common-emitter stage biased by a single base resistor from the supply.

$$
I_B = \frac{V_{CC} - V_{BE}}{R_B},\quad V_{CE} = V_{CC} - \beta I_BR_C
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VCC` | in | supply voltage | V |
| `RB` | in | base resistor | ohm |
| `RC` | in | collector resistor | ohm |
| `beta` | in | current gain | — |
| `VBE` | in | base-emitter voltage (about 0.7 V) | V |
| `IB` | out | base current | A |
| `IC` | out | collector current | A |
| `IE` | out | emitter current | A |
| `VCE` | out | collector-emitter voltage | V |

**Applies when:** active region: VCE >= 0.2 V (otherwise refused as saturated).

```shell
$ electronics-engineering solve elx_bjt_fixed_bias "VCC=12 V" "RB=470000 ohm" "RC=2200 ohm" beta=100 "VBE=0.7 V"
IB = 2.404255319e-05 A
IC = 0.002404255319 A
IE = 0.002428297872 A
VCE = 6.710638298 V
  by: BJT fixed (base-resistor) bias: Q-point  (IB = (VCC - VBE)/RB; IC = beta IB; VCE = VCC - IC RC)
  VCC = 12 V
  RB = 470000 ohm
  RC = 2200 ohm
  beta = 100
  VBE = 0.7 V
  assumes: active region: VCE >= 0.2 V (otherwise refused as saturated)
```

In the worked example, VCC = 12 V, RB = 470000 ohm, RC = 2200 ohm, beta = 100, VBE = 0.7 V go through the equation above and give IB = 2.404255319e-05 A, IC = 0.002404255319 A, IE = 0.002428297872 A, VCE = 6.710638298 V. It applies only when active region: VCE >= 0.2 V (otherwise refused as saturated): outside that it says what is wrong instead of returning a number.

<a id="elx_bjt_small_signal_parameters"></a>

#### BJT small-signal (hybrid-pi) parameters — `elx_bjt_small_signal_parameters`

The transconductance gm, the input resistance r_pi, the emitter resistance r_e, the output resistance r_o (Early voltage) and the intrinsic gain of a bipolar transistor at its bias current.

$$
g_m = \frac{I_C}{V_T},\quad r_\pi = \frac{\beta}{g_m},\quad r_e = \frac{\alpha}{g_m},\quad r_o = \frac{V_A}{I_C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `IC` | in | collector bias current | A |
| `beta` | in | current gain | — |
| `VT` | in | thermal voltage (25 or 26 mV) | V |
| `VA` | in | Early voltage | V |
| `gm` | out | transconductance | S |
| `r_pi` | out | base input resistance r_pi | ohm |
| `r_e` | out | emitter resistance r_e | ohm |
| `r_o` | out | output resistance | ohm |
| `A0` | out | intrinsic voltage gain gm r_o | — |

**Applies when:** IC, beta, VT, VA > 0; active region.

```shell
$ electronics-engineering solve elx_bjt_small_signal_parameters "IC=1 mA" beta=100 "VT=25 mV" "VA=100 V"
gm = 0.04 S
r_pi = 2500 ohm
r_e = 24.75247525 ohm
r_o = 100000 ohm
A0 = 4000
  by: BJT small-signal (hybrid-pi) parameters  (gm = IC/VT; r_pi = beta/gm; r_e = alpha/gm; r_o = VA/IC; A0 = VA/VT)
  IC = 0.001 A
  beta = 100
  VT = 0.025 V
  VA = 100 V
  assumes: IC, beta, VT, VA > 0; active region
```

In the worked example, IC = 1 mA, beta = 100, VT = 25 mV, VA = 100 V go through the equation above and give gm = 0.04 S, r_pi = 2500 ohm, r_e = 24.75247525 ohm, r_o = 100000 ohm. It applies only when IC, beta, VT, VA > 0; active region: outside that it says what is wrong instead of returning a number.

<a id="elx_bjt_current_relations"></a>

#### BJT terminal currents from beta — `elx_bjt_current_relations`

The collector and emitter currents and the common-base current gain alpha of a bipolar transistor in the active region from its base current and beta.

$$
I_C = \beta I_B,\quad I_E = (\beta+1)I_B,\quad \alpha = \frac{\beta}{\beta+1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `IB` | in | base current | A |
| `beta` | in | common-emitter current gain (hFE) | — |
| `IC` | out | collector current | A |
| `IE` | out | emitter current | A |
| `alpha` | out | alpha = beta/(beta + 1) | — |

**Applies when:** IB > 0, beta > 0; active region.

```shell
$ electronics-engineering solve elx_bjt_current_relations "IB=20 uA" beta=100
IC = 0.002 A
IE = 0.00202 A
alpha = 0.9900990099
  by: BJT terminal currents from beta  (IC = beta IB; IE = (beta + 1) IB; alpha = beta/(beta + 1))
  IB = 2e-05 A
  beta = 100
  assumes: IB > 0, beta > 0; active region
```

In the worked example, IB = 20 uA, beta = 100 go through the equation above and give IC = 0.002 A, IE = 0.00202 A, alpha = 0.9900990099. It applies only when IB > 0, beta > 0; active region: outside that it says what is wrong instead of returning a number.

<a id="elx_bjt_voltage_divider_bias"></a>

#### BJT voltage-divider bias: exact Q-point — `elx_bjt_voltage_divider_bias`

The Q-point (base, collector and emitter currents, node voltages and VCE) of a common-emitter stage with a base voltage divider and an emitter resistor, by the exact Thevenin analysis (base current included).

$$
I_B = \frac{V_{TH} - V_{BE}}{R_{TH} + (\beta+1)R_E}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VCC` | in | supply voltage | V |
| `R1` | in | upper divider resistor (supply to base) | ohm |
| `R2` | in | lower divider resistor (base to ground) | ohm |
| `RC` | in | collector resistor | ohm |
| `RE` | in | emitter resistor | ohm |
| `beta` | in | current gain | — |
| `VBE` | in | base-emitter voltage | V |
| `VTH` | out | Thevenin voltage of the divider | V |
| `RTH` | out | Thevenin resistance of the divider | ohm |
| `IB` | out | base current | A |
| `IC` | out | collector current | A |
| `IE` | out | emitter current | A |
| `VB` | out | base voltage | V |
| `VE` | out | emitter voltage | V |
| `VC` | out | collector voltage | V |
| `VCE` | out | collector-emitter voltage | V |

**Applies when:** VTH > VBE; active region: VCE >= 0.2 V.

```shell
$ electronics-engineering solve elx_bjt_voltage_divider_bias "VCC=12 V" "R1=47000 ohm" "R2=10000 ohm" "RC=2200 ohm" "RE=470 ohm" beta=150 "VBE=0.7 V"
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

In the worked example, VCC = 12 V, R1 = 47000 ohm, R2 = 10000 ohm, RC = 2200 ohm, RE = 470 ohm, beta = 150, VBE = 0.7 V go through the equation above and give VTH = 2.105263158 V, RTH = 8245.614035 ohm, IB = 1.773972436e-05 A, IC = 0.002660958654 A. It applies only when VTH > VBE; active region: VCE >= 0.2 V: outside that it says what is wrong instead of returning a number.

<a id="elx_common_base_amplifier"></a>

#### Common-base amplifier — `elx_common_base_amplifier`

The voltage gain (emitter to collector and source to load), the low input resistance and the output resistance of a common-base stage (hybrid-pi model, r_o neglected).

$$
A_v = g_m(R_C\parallel R_L),\quad R_{in} = R_E\parallel r_e
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `IC` | in | collector bias current | A |
| `beta` | in | current gain | — |
| `VT` | in | thermal voltage | V |
| `RC` | in | collector resistor | ohm |
| `RL` | in | load resistor | ohm |
| `RE` | in | emitter bias resistor | ohm |
| `Rsig` | in | source resistance | ohm |
| `Av` | out | voltage gain emitter to output | — |
| `Rin` | out | input resistance (RE \|\| r_e) | ohm |
| `Rout` | out | output resistance (RC) | ohm |
| `Gv` | out | overall gain from the source | — |

**Applies when:** r_o neglected; base at signal ground.

```shell
$ electronics-engineering solve elx_common_base_amplifier "IC=1 mA" beta=100 "VT=25 mV" "RC=5000 ohm" "RL=5000 ohm" "RE=10000 ohm" "Rsig=50 ohm"
Av = 100
Rin = 24.69135802 ohm
Rout = 5000 ohm
Gv = 33.05785124
  by: Common-base amplifier  (Av = gm (RC || RL); Rin = RE || r_e with r_e = alpha/gm; Gv = Av Rin/(Rin + Rsig))
  IC = 0.001 A
  beta = 100
  VT = 0.025 V
  RC = 5000 ohm
  RL = 5000 ohm
  RE = 10000 ohm
  Rsig = 50 ohm
  assumes: r_o neglected; base at signal ground
```

In the worked example, IC = 1 mA, beta = 100, VT = 25 mV, RC = 5000 ohm, RL = 5000 ohm, RE = 10000 ohm, Rsig = 50 ohm go through the equation above and give Av = 100, Rin = 24.69135802 ohm, Rout = 5000 ohm, Gv = 33.05785124. It applies only when r_o neglected; base at signal ground: outside that it says what is wrong instead of returning a number.

<a id="elx_common_collector_amplifier"></a>

#### Common-collector (emitter-follower) amplifier — `elx_common_collector_amplifier`

The voltage gain, input resistance and output resistance of an emitter follower, including the source resistance reflected into the emitter (hybrid-pi model, r_o neglected).

$$
A_v = \frac{(\beta+1)R_E'}{r_\pi + (\beta+1)R_E'},\quad R_{out} = R_E\parallel\frac{r_\pi + R_{sig}\parallel R_B}{\beta+1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `IC` | in | collector bias current | A |
| `beta` | in | current gain | — |
| `VT` | in | thermal voltage | V |
| `RE` | in | emitter resistor | ohm |
| `RL` | in | load resistor | ohm |
| `RB` | in | bias resistance at the base | ohm |
| `Rsig` | in | source resistance | ohm |
| `Av` | out | voltage gain base to output | — |
| `Rin` | out | input resistance | ohm |
| `Rout` | out | output resistance | ohm |
| `Gv` | out | overall gain from the source | — |

**Applies when:** r_o neglected.

```shell
$ electronics-engineering solve elx_common_collector_amplifier "IC=5 mA" beta=100 "VT=25 mV" "RE=1000 ohm" "RL=1000 ohm" "RB=100000 ohm" "Rsig=10000 ohm"
Av = 0.9901960784
Rin = 33774.83444 ohm
Rout = 86.7242088 ohm
Gv = 0.7639939486
  by: Common-collector (emitter-follower) amplifier  (Av = (beta + 1)(RE || RL)/(r_pi + (beta + 1)(RE || RL)); Rout = RE || (r_pi + Rsig || RB)/(beta + 1))
  IC = 0.005 A
  beta = 100
  VT = 0.025 V
  RE = 1000 ohm
  RL = 1000 ohm
  RB = 100000 ohm
  Rsig = 10000 ohm
  assumes: r_o neglected
```

In the worked example, IC = 5 mA, beta = 100, VT = 25 mV, RE = 1000 ohm, RL = 1000 ohm, RB = 100000 ohm, Rsig = 10000 ohm go through the equation above and give Av = 0.9901960784, Rin = 33774.83444 ohm, Rout = 86.7242088 ohm, Gv = 0.7639939486. It applies only when r_o neglected: outside that it says what is wrong instead of returning a number.

<a id="elx_common_emitter_amplifier"></a>

#### Common-emitter amplifier: gain, input and output resistance — `elx_common_emitter_amplifier`

The voltage gain (base to collector and source to load), the input and output resistances of a common-emitter stage with an optional unbypassed emitter resistance, from the bias current (hybrid-pi model, r_o neglected).

$$
A_v = -\frac{\beta\,(R_C\parallel R_L)}{r_\pi + (\beta+1)R_{E1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `IC` | in | collector bias current | A |
| `beta` | in | current gain | — |
| `VT` | in | thermal voltage | V |
| `RC` | in | collector resistor | ohm |
| `RL` | in | load resistor | ohm |
| `RE1` | in | unbypassed emitter resistance (0 if fully bypassed) | ohm |
| `RB` | in | bias resistance seen at the base (R1 \|\| R2) | ohm |
| `Rsig` | in | source resistance | ohm |
| `gm` | out | transconductance | S |
| `r_pi` | out | r_pi | ohm |
| `Av` | out | voltage gain base to output | — |
| `Rin` | out | input resistance (including RB) | ohm |
| `Rout` | out | output resistance (RC) | ohm |
| `Gv` | out | overall gain from the source voltage | — |

**Applies when:** r_o neglected; coupling and bypass capacitors short at signal frequency.

```shell
$ electronics-engineering solve elx_common_emitter_amplifier "IC=1 mA" beta=100 "VT=25 mV" "RC=5000 ohm" "RL=10000 ohm" "RE1=0 ohm" "RB=20000 ohm" "Rsig=1000 ohm"
gm = 0.04 S
r_pi = 2500 ohm
Av = -133.3333333
Rin = 2222.222222 ohm
Rout = 5000 ohm
Gv = -91.95402299
  by: Common-emitter amplifier: gain, input and output resistance  (Av = -beta (RC || RL)/(r_pi + (beta + 1) RE1); Rin = RB || (r_pi + (beta + 1) RE1); Gv = Av Rin/(Rin + Rsig))
  IC = 0.001 A
  beta = 100
  VT = 0.025 V
  RC = 5000 ohm
  RL = 10000 ohm
  RE1 = 0 ohm
  RB = 20000 ohm
  Rsig = 1000 ohm
  assumes: r_o neglected; coupling and bypass capacitors short at signal frequency
```

In the worked example, IC = 1 mA, beta = 100, VT = 25 mV, RC = 5000 ohm, RL = 10000 ohm, RE1 = 0 ohm, RB = 20000 ohm, Rsig = 1000 ohm go through the equation above and give gm = 0.04 S, r_pi = 2500 ohm, Av = -133.3333333, Rin = 2222.222222 ohm. It applies only when r_o neglected; coupling and bypass capacitors short at signal frequency: outside that it says what is wrong instead of returning a number.

<a id="elx_miller_high_cutoff"></a>

#### Common-emitter high-frequency cutoff: Miller effect and open-circuit time constants — `elx_miller_high_cutoff`

The Miller capacitance, the upper -3 dB frequency of a common-emitter (or common-source) stage by the Miller approximation, and the open-circuit time-constant estimate that also counts the output side of Cmu.

$$
C_M = C_\mu(1 + g_mR_L'),\quad \tau_{oc} = C_\pi R_\pi' + C_\mu\left(R_\pi' + R_L' + g_mR_\pi'R_L'\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gm` | in | transconductance | S |
| `r_pi` | in | r_pi (use a very large value for a MOSFET) | ohm |
| `Cpi` | in | input capacitance Cpi (Cgs) | F |
| `Cmu` | in | feedback capacitance Cmu (Cgd) | F |
| `RLp` | in | total load at the collector (RC \|\| RL \|\| r_o) | ohm |
| `Rsigp` | in | source resistance seen at the base (Rsig \|\| RB) | ohm |
| `A_mid` | out | gain across Cmu, -gm RL' | — |
| `C_M` | out | Miller capacitance | F |
| `R_pi_eq` | out | resistance seen by Cpi (r_pi \|\| Rsig') | ohm |
| `f_H_miller` | out | upper cutoff, Miller approximation | Hz |
| `tau_oc` | out | sum of open-circuit time constants | s |
| `f_H_octc` | out | upper cutoff, open-circuit time constants | Hz |

**Applies when:** mid-band gain unchanged by the capacitors; dominant-pole estimate.

```shell
$ electronics-engineering solve elx_miller_high_cutoff "gm=40 mS" "r_pi=2500 ohm" "Cpi=10 pF" "Cmu=2 pF" "RLp=2500 ohm" "Rsigp=1000 ohm"
A_mid = -100
C_M = 2.02e-10 F
R_pi_eq = 714.2857143 ohm
f_H_miller = 1051023.209 Hz
tau_oc = 1.564285714e-07 s
f_H_octc = 1017428.86 Hz
  by: Common-emitter high-frequency cutoff: Miller effect and open-circuit time constants  (C_M = Cmu (1 + gm RL'); f_H = 1/(2 pi (r_pi || Rsig')(Cpi + C_M)); tau = Cpi R' + Cmu (R' + RL' + gm R' RL'))
  gm = 0.04 S
  r_pi = 2500 ohm
  Cpi = 1e-11 F
  Cmu = 2e-12 F
  RLp = 2500 ohm
  Rsigp = 1000 ohm
  assumes: mid-band gain unchanged by the capacitors; dominant-pole estimate
```

In the worked example, gm = 40 mS, r_pi = 2500 ohm, Cpi = 10 pF, Cmu = 2 pF, RLp = 2500 ohm, Rsigp = 1000 ohm go through the equation above and give A_mid = -100, C_M = 2.02e-10 F, R_pi_eq = 714.2857143 ohm, f_H_miller = 1051023.209 Hz. It applies only when mid-band gain unchanged by the capacitors; dominant-pole estimate: outside that it says what is wrong instead of returning a number.

<a id="elx_low_frequency_cutoff"></a>

#### Common-emitter low-frequency cutoff by short-circuit time constants — `elx_low_frequency_cutoff`

The resistance seen by each coupling and bypass capacitor of a common-emitter amplifier and the lower -3 dB frequency estimated as the sum of the short-circuit pole frequencies (r_o neglected).

$$
f_L \approx \sum_i \frac{1}{2\pi R_iC_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rsig` | in | source resistance | ohm |
| `RB` | in | bias resistance at the base (R1 \|\| R2) | ohm |
| `r_pi` | in | r_pi | ohm |
| `beta` | in | current gain | — |
| `RE` | in | emitter resistor (bypassed by CE) | ohm |
| `RC` | in | collector resistor | ohm |
| `RL` | in | load resistor | ohm |
| `C1` | in | input coupling capacitor | F |
| `C2` | in | output coupling capacitor | F |
| `CE` | in | emitter bypass capacitor | F |
| `R_C1` | out | resistance seen by C1 | ohm |
| `R_CE` | out | resistance seen by CE | ohm |
| `R_C2` | out | resistance seen by C2 | ohm |
| `f_C1` | out | pole of C1 | Hz |
| `f_CE` | out | pole of CE | Hz |
| `f_C2` | out | pole of C2 | Hz |
| `f_L` | out | estimated lower cutoff | Hz |

**Applies when:** short-circuit time-constant estimate; r_o neglected.

```shell
$ electronics-engineering solve elx_low_frequency_cutoff "Rsig=1000 ohm" "RB=20000 ohm" "r_pi=2500 ohm" beta=100 "RE=1000 ohm" "RC=5000 ohm" "RL=10000 ohm" "C1=10 uF" "C2=10 uF" "CE=100 uF"
R_C1 = 3222.222222 ohm
R_CE = 33.05219968 ohm
R_C2 = 15000 ohm
f_C1 = 4.939291337 Hz
f_CE = 48.15260244 Hz
f_C2 = 1.061032954 Hz
f_L = 54.15292673 Hz
  by: Common-emitter low-frequency cutoff by short-circuit time constants  (R_C1 = Rsig + RB || r_pi; R_CE = RE || (r_pi + Rsig || RB)/(beta + 1); R_C2 = RC + RL; f_L = sum 1/(2 pi R C))
  Rsig = 1000 ohm
  RB = 20000 ohm
  r_pi = 2500 ohm
  beta = 100
  RE = 1000 ohm
  RC = 5000 ohm
  RL = 10000 ohm
  C1 = 1e-05 F
  C2 = 1e-05 F
  CE = 0.0001 F
  assumes: short-circuit time-constant estimate; r_o neglected
```

In the worked example, Rsig = 1000 ohm, RB = 20000 ohm, r_pi = 2500 ohm, beta = 100, RE = 1000 ohm, RC = 5000 ohm, RL = 10000 ohm, C1 = 10 uF, C2 = 10 uF, CE = 100 uF go through the equation above and give R_C1 = 3222.222222 ohm, R_CE = 33.05219968 ohm, R_C2 = 15000 ohm, f_C1 = 4.939291337 Hz. It applies only when short-circuit time-constant estimate; r_o neglected: outside that it says what is wrong instead of returning a number.

<a id="elx_wilson_current_mirror"></a>

#### Wilson current mirror output current — `elx_wilson_current_mirror`

The output current of a three-transistor Wilson current mirror and its small base-current error, 2/(beta^2 + 2 beta + 2).

$$
I_{out} = I_{ref}\left(1 - \frac{2}{\beta^2 + 2\beta + 2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Iref` | in | reference current | A |
| `beta` | in | current gain of each matched transistor | — |
| `Iout` | out | output current | A |
| `ratio` | out | Iout/Iref | — |
| `error` | out | relative error 1 - Iout/Iref | — |

**Applies when:** matched transistors, Early effect neglected.

```shell
$ electronics-engineering solve elx_wilson_current_mirror "Iref=1 mA" beta=100
Iout = 0.00099980396 A
ratio = 0.99980396
error = 0.0001960399922
  by: Wilson current mirror output current  (Iout = Iref (1 - 2/(beta^2 + 2 beta + 2)))
  Iref = 0.001 A
  beta = 100
  assumes: matched transistors, Early effect neglected
```

In the worked example, Iref = 1 mA, beta = 100 go through the equation above and give Iout = 0.00099980396 A, ratio = 0.99980396, error = 0.0001960399922. It applies only when matched transistors, Early effect neglected: outside that it says what is wrong instead of returning a number.

### CMOS (2)

<a id="elx_cmos_propagation_delay"></a>

#### CMOS gate propagation delay (switch-resistor model) — `elx_cmos_propagation_delay`

The high-to-low and low-to-high propagation delays (50 % points) and their average for a CMOS gate modelled as switched equivalent resistances driving a load capacitance.

$$
t_{pHL} = \ln 2\,R_nC_L,\quad t_{pLH} = \ln 2\,R_pC_L
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rn` | in | equivalent on-resistance of the pull-down network | ohm |
| `Rp` | in | equivalent on-resistance of the pull-up network | ohm |
| `CL` | in | load capacitance | F |
| `tpHL` | out | high-to-low delay | s |
| `tpLH` | out | low-to-high delay | s |
| `tp` | out | average propagation delay | s |

**Applies when:** step input, RC switch model.

```shell
$ electronics-engineering solve elx_cmos_propagation_delay "Rn=10000 ohm" "Rp=20000 ohm" "CL=100 fF"
tpHL = 6.931471806e-10 s
tpLH = 1.386294361e-09 s
tp = 1.039720771e-09 s
  by: CMOS gate propagation delay (switch-resistor model)  (tpHL = ln 2 Rn CL; tpLH = ln 2 Rp CL; tp = (tpHL + tpLH)/2)
  Rn = 10000 ohm
  Rp = 20000 ohm
  CL = 1e-13 F
  assumes: step input, RC switch model
```

In the worked example, Rn = 10000 ohm, Rp = 20000 ohm, CL = 100 fF go through the equation above and give tpHL = 6.931471806e-10 s, tpLH = 1.386294361e-09 s, tp = 1.039720771e-09 s. It applies only when step input, RC switch model: outside that it says what is wrong instead of returning a number.

<a id="elx_cmos_switching_threshold"></a>

#### CMOS inverter switching threshold VM — `elx_cmos_switching_threshold`

The switching threshold of a CMOS inverter (input = output, both transistors saturated) from the supply, the two threshold voltages and the two transconductance parameters.

$$
V_M = \frac{V_{tn} + r\,(V_{DD} - |V_{tp}|)}{1 + r},\quad r = \sqrt{k_p/k_n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VDD` | in | supply voltage | V |
| `Vtn` | in | NMOS threshold | V |
| `Vtp` | in | magnitude of the PMOS threshold | V |
| `kn` | in | NMOS mu_n Cox W/L | A/V^2 |
| `kp` | in | PMOS mu_p Cox W/L | A/V^2 |
| `r` | out | sqrt(kp/kn) | — |
| `VM` | out | switching threshold | V |

**Applies when:** Vtn + |Vtp| < VDD; lambda neglected.

```shell
$ electronics-engineering solve elx_cmos_switching_threshold "VDD=2.5 V" "Vtn=0.4 V" "Vtp=0.4 V" "kn=0.1 mA/V^2" "kp=0.1 mA/V^2"
r = 1
VM = 1.25 V
  by: CMOS inverter switching threshold VM  (VM = (Vtn + r (VDD - |Vtp|))/(1 + r), r = sqrt(kp/kn))
  VDD = 2.5 V
  Vtn = 0.4 V
  Vtp = 0.4 V
  kn = 0.0001 A/V^2
  kp = 0.0001 A/V^2
  assumes: Vtn + |Vtp| < VDD; lambda neglected
```

In the worked example, VDD = 2.5 V, Vtn = 0.4 V, Vtp = 0.4 V, kn = 0.1 mA/V^2, kp = 0.1 mA/V^2 go through the equation above and give r = 1, VM = 1.25 V. It applies only when Vtn + |Vtp| < VDD; lambda neglected: outside that it says what is wrong instead of returning a number.

### Data converters (9)

<a id="elx_adc_code"></a>

#### ADC output code, LSB and quantisation error — `elx_adc_code`

The output code (decimal and binary) of an ideal unipolar truncating n-bit ADC for an input voltage, the LSB size, the voltage the code represents and the quantisation error.

$$
D = \left\lfloor \frac{V_{in}}{V_{ref}}2^n\right\rfloor,\quad \text{LSB} = \frac{V_{ref}}{2^n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | analog input | V |
| `Vref` | in | reference (full-scale) voltage | V |
| `n` | in | resolution in bits | — |
| `code` | out | output code (decimal) | — |
| `bits` | out | output code in binary | — |
| `LSB` | out | LSB = Vref/2^n | V |
| `V_code` | out | voltage of the code | V |
| `error` | out | quantisation error Vin - code LSB | V |

**Applies when:** 0 <= Vin <= Vref; 1 <= n <= 16.

```shell
$ electronics-engineering solve elx_adc_code "Vin=3.2 V" "Vref=5 V" n=8
code = 163
bits = 10100011
LSB = 0.01953125 V
V_code = 3.18359375 V
error = 0.01640625 V
  by: ADC output code, LSB and quantisation error  (code = floor(Vin 2^n/Vref), at most 2^n - 1; LSB = Vref/2^n)
  Vin = 3.2 V
  Vref = 5 V
  assumes: 0 <= Vin <= Vref; 1 <= n <= 16
```

In the worked example, Vin = 3.2 V, Vref = 5 V, n = 8 go through the equation above and give code = 163, bits = 10100011, LSB = 0.01953125 V, V_code = 3.18359375 V. It applies only when 0 <= Vin <= Vref; 1 <= n <= 16: outside that it says what is wrong instead of returning a number.

<a id="elx_adc_quantization_snr"></a>

#### ADC quantisation SNR with oversampling — `elx_adc_quantization_snr`

The signal-to-quantisation-noise ratio of an ideal n-bit converter for a sine input (6.02 n + 1.76 dB at full scale), with an oversampling ratio and a reduced amplitude, and the in-band noise in LSB.

$$
\text{SNR} = 6.02\,n + 1.76 + 10\log_{10}\text{OSR}\ \text{dB}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | resolution in bits | — |
| `OSR` | in | oversampling ratio (default 1) (optional) | — |
| `A_frac` | in | sine amplitude as a fraction of full scale (default 1) (optional) | — |
| `SNR_dB` | out | signal-to-quantisation-noise ratio, dB | — |
| `noise_rms_lsb` | out | in-band noise rms in LSB | — |
| `SNR_full_scale_dB` | out | full-scale SNR without oversampling, dB | — |

**Applies when:** uniform quantisation noise, white over Nyquist; OSR >= 1.

```shell
$ electronics-engineering solve elx_adc_quantization_snr n=12
SNR_dB = 74.00811155
noise_rms_lsb = 0.2886751346
SNR_full_scale_dB = 74.00811155
  by: ADC quantisation SNR with oversampling  (SNR = 20 log10(A 2^n sqrt(1.5)) + 10 log10(OSR) = 6.02 n + 1.76 + 20 log10 A + 10 log10 OSR)
  assumes: uniform quantisation noise, white over Nyquist; OSR >= 1
```

In the worked example, n = 12 go through the equation above and give SNR_dB = 74.00811155, noise_rms_lsb = 0.2886751346, SNR_full_scale_dB = 74.00811155. It applies only when uniform quantisation noise, white over Nyquist; OSR >= 1: outside that it says what is wrong instead of returning a number.

<a id="elx_dac_binary_weighted"></a>

#### Binary-weighted resistor DAC — `elx_dac_binary_weighted`

The output voltage and summing current of a binary-weighted resistor DAC with an inverting op-amp (MSB resistor R, each lower bit doubling it) and the largest resistor needed.

$$
V_{out} = -V_{ref}\frac{R_f}{R}\sum_{k=0}^{n-1}\frac{b_k}{2^k} = -V_{ref}\frac{R_f}{R}\frac{D}{2^{n-1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | resolution in bits | — |
| `code` | in | input code (decimal) | — |
| `Vref` | in | reference voltage | V |
| `R` | in | MSB resistor | ohm |
| `Rf` | in | feedback resistor | ohm |
| `Vout` | out | output voltage | V |
| `I_total` | out | total current into the summing node | A |
| `R_LSB` | out | LSB resistor R 2^(n-1) | ohm |

**Applies when:** ideal switches and op-amp.

```shell
$ electronics-engineering solve elx_dac_binary_weighted n=4 code=10 "Vref=5 V" "R=10000 ohm" "Rf=5000 ohm"
Vout = -3.125 V
I_total = 0.000625 A
R_LSB = 80000 ohm
  by: Binary-weighted resistor DAC  (I = Vref D/(R 2^(n-1)); Vout = -Rf I)
  Vref = 5 V
  R = 10000 ohm
  Rf = 5000 ohm
  assumes: ideal switches and op-amp
```

In the worked example, n = 4, code = 10, Vref = 5 V, R = 10000 ohm, Rf = 5000 ohm go through the equation above and give Vout = -3.125 V, I_total = 0.000625 A, R_LSB = 80000 ohm. It applies only when ideal switches and op-amp: outside that it says what is wrong instead of returning a number.

<a id="elx_dac_output"></a>

#### DAC output voltage (R-2R or any binary DAC) — `elx_dac_output`

The output voltage of an ideal n-bit unipolar DAC for a code, Vout = Vref D/2^n, its step size and its full-scale output.

$$
V_{out} = V_{ref}\frac{D}{2^n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | resolution in bits | — |
| `code` | in | input code (decimal) | — |
| `Vref` | in | reference voltage | V |
| `Vout` | out | output voltage | V |
| `LSB` | out | step size Vref/2^n | V |
| `V_full_scale` | out | output for the all-ones code | V |

**Applies when:** 0 <= code <= 2^n - 1.

```shell
$ electronics-engineering solve elx_dac_output n=8 code=200 "Vref=5 V"
Vout = 3.90625 V
LSB = 0.01953125 V
V_full_scale = 4.98046875 V
  by: DAC output voltage (R-2R or any binary DAC)  (Vout = Vref D/2^n; full scale Vref (2^n - 1)/2^n)
  Vref = 5 V
  assumes: 0 <= code <= 2^n - 1
```

In the worked example, n = 8, code = 200, Vref = 5 V go through the equation above and give Vout = 3.90625 V, LSB = 0.01953125 V, V_full_scale = 4.98046875 V. It applies only when 0 <= code <= 2^n - 1: outside that it says what is wrong instead of returning a number.

<a id="elx_dual_slope_adc"></a>

#### Dual-slope integrating ADC — `elx_dual_slope_adc`

The fixed integration time, the de-integration time and count, and the peak integrator voltage of a dual-slope ADC.

$$
T_2 = T_1\frac{V_{in}}{V_{ref}},\quad N_2 = N_1\frac{V_{in}}{V_{ref}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage (magnitude) | V |
| `Vref` | in | reference voltage (magnitude) | V |
| `N1` | in | fixed count of the integration phase | — |
| `fclk` | in | clock frequency | Hz |
| `R` | in | integrator resistor | ohm |
| `C` | in | integrator capacitor | F |
| `T1` | out | integration time N1/fclk | s |
| `T2` | out | de-integration time | s |
| `N2` | out | count of the de-integration phase (unrounded) | — |
| `V_peak` | out | peak integrator output | V |

**Applies when:** 0 < Vin <= Vref.

```shell
$ electronics-engineering solve elx_dual_slope_adc "Vin=0.5 V" "Vref=1 V" N1=1000 "fclk=100000 Hz" "R=100000 ohm" "C=0.1 uF"
T1 = 0.01 s
T2 = 0.005 s
N2 = 500
V_peak = 0.5 V
  by: Dual-slope integrating ADC  (T1 = N1/fclk; T2 = T1 Vin/Vref; N2 = N1 Vin/Vref; V_peak = Vin T1/(R C))
  Vin = 0.5 V
  Vref = 1 V
  fclk = 100000 Hz
  R = 100000 ohm
  C = 1e-07 F
  assumes: 0 < Vin <= Vref
```

In the worked example, Vin = 0.5 V, Vref = 1 V, N1 = 1000, fclk = 100000 Hz, R = 100000 ohm, C = 0.1 uF go through the equation above and give T1 = 0.01 s, T2 = 0.005 s, N2 = 500, V_peak = 0.5 V. It applies only when 0 < Vin <= Vref: outside that it says what is wrong instead of returning a number.

<a id="elx_adc_enob"></a>

#### Effective number of bits (ENOB) from SINAD — `elx_adc_enob`

The effective number of bits of a converter from its measured SINAD (IEEE 1241: ENOB = (SINAD - 1.76)/6.02) and the bits lost against its nominal resolution.

$$
\text{ENOB} = \frac{\text{SINAD} - 1.76}{6.02}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `SINAD` | in | signal to noise and distortion ratio in dB | — |
| `n` | in | nominal resolution in bits | — |
| `ENOB` | out | effective number of bits | — |
| `bits_lost` | out | n - ENOB | — |

**Applies when:** full-scale sine test.

```shell
$ electronics-engineering solve elx_adc_enob SINAD=74 n=14
ENOB = 12
bits_lost = 2
  by: Effective number of bits (ENOB) from SINAD  (ENOB = (SINAD - 1.76)/6.02)
  SINAD = 74
  assumes: full-scale sine test
```

In the worked example, SINAD = 74, n = 14 go through the equation above and give ENOB = 12, bits_lost = 2. It applies only when full-scale sine test: outside that it says what is wrong instead of returning a number.

<a id="elx_sample_hold_droop"></a>

#### Sample-and-hold droop — `elx_sample_hold_droop`

The droop rate I/C of a sample-and-hold capacitor discharged by a leakage current, the droop over a hold time (in volts and LSB) and the longest hold that keeps the droop within half an LSB of an n-bit converter.

$$
\frac{dV}{dt} = \frac{I_{leak}}{C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I_leak` | in | leakage (bias) current | A |
| `C` | in | hold capacitor | F |
| `t_hold` | in | hold time | s |
| `Vref` | in | converter full scale | V |
| `n` | in | converter resolution in bits | — |
| `droop_rate` | out | droop rate I/C | V/s |
| `droop` | out | droop over the hold time | V |
| `droop_lsb` | out | droop in LSB | — |
| `t_max_half_lsb` | out | longest hold for half an LSB | s |

**Applies when:** constant leakage current.

```shell
$ electronics-engineering solve elx_sample_hold_droop "I_leak=1 nA" "C=1 nF" "t_hold=10 us" "Vref=5 V" n=12
droop_rate = 1 V/s
droop = 1e-05 V
droop_lsb = 0.008192
t_max_half_lsb = 0.0006103515625 s
  by: Sample-and-hold droop  (dV/dt = I/C; dV = I t/C; t_max = (Vref/2^(n+1)) C/I)
  I_leak = 1e-09 A
  C = 1e-09 F
  t_hold = 1e-05 s
  Vref = 5 V
  assumes: constant leakage current
```

In the worked example, I_leak = 1 nA, C = 1 nF, t_hold = 10 us, Vref = 5 V, n = 12 go through the equation above and give droop_rate = 1 V/s, droop = 1e-05 V, droop_lsb = 0.008192, t_max_half_lsb = 0.0006103515625 s. It applies only when constant leakage current: outside that it says what is wrong instead of returning a number.

<a id="elx_aperture_jitter_snr"></a>

#### SNR limit from sampling-clock (aperture) jitter — `elx_aperture_jitter_snr`

The SNR limit -20 log10(2 pi f tj) set by rms sampling-clock jitter for a full-scale sine, combined with the quantisation SNR of an n-bit converter into a total SNR and effective bits.

$$
\text{SNR}_j = -20\log_{10}(2\pi f t_j)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | input frequency | Hz |
| `tj` | in | rms clock jitter | s |
| `n` | in | converter resolution in bits | — |
| `SNR_jitter_dB` | out | jitter-limited SNR, dB | — |
| `SNR_quant_dB` | out | quantisation SNR, dB | — |
| `SNR_total_dB` | out | combined SNR, dB | — |
| `ENOB_total` | out | effective bits of the combination | — |

**Applies when:** full-scale sine; jitter and quantisation noise uncorrelated.

```shell
$ electronics-engineering solve elx_aperture_jitter_snr "f=100000000 Hz" "tj=1 ps" n=14
SNR_jitter_dB = 64.03640263
SNR_quant_dB = 86.04931138
SNR_total_dB = 64.00916741
ENOB_total = 10.34039326
  by: SNR limit from sampling-clock (aperture) jitter  (SNR_j = -20 log10(2 pi f tj); SNR_q = 6.02 n + 1.76; noise powers add)
  f = 100000000 Hz
  tj = 1e-12 s
  assumes: full-scale sine; jitter and quantisation noise uncorrelated
```

In the worked example, f = 100000000 Hz, tj = 1 ps, n = 14 go through the equation above and give SNR_jitter_dB = 64.03640263, SNR_quant_dB = 86.04931138, SNR_total_dB = 64.00916741, ENOB_total = 10.34039326. It applies only when full-scale sine; jitter and quantisation noise uncorrelated: outside that it says what is wrong instead of returning a number.

<a id="elx_sar_conversion"></a>

#### Successive-approximation (SAR) ADC conversion — `elx_sar_conversion`

The bit-by-bit trial voltages, the final code (decimal and binary) and the DAC voltage of an ideal n-bit successive-approximation ADC converting an input.

$$
b_k = \left[V_{in} \ge V_{DAC}(D_{k} + 2^k)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | analog input | V |
| `Vref` | in | reference voltage | V |
| `n` | in | resolution in bits | — |
| `code` | out | final code | — |
| `bits` | out | final code in binary | — |
| `trials` | out | trial DAC voltages, MSB first (V) | — |
| `V_dac` | out | DAC voltage of the final code | V |
| `cycles` | out | comparison cycles | — |

**Applies when:** 0 <= Vin < Vref.

```shell
$ electronics-engineering solve elx_sar_conversion "Vin=3.2 V" "Vref=5 V" n=4
code = 10
bits = 1010
trials = [2.5,3.75,3.125,3.4375]
V_dac = 3.125 V
cycles = 4
  by: Successive-approximation (SAR) ADC conversion  (for each bit from the MSB: set it, keep it if Vin >= DAC(trial); LSB = Vref/2^n)
  Vin = 3.2 V
  Vref = 5 V
  assumes: 0 <= Vin < Vref
```

In the worked example, Vin = 3.2 V, Vref = 5 V, n = 4 go through the equation above and give code = 10, bits = 1010, trials = [2.5, 3.75, 3.125, 3.4375], V_dac = 3.125 V. It applies only when 0 <= Vin < Vref: outside that it says what is wrong instead of returning a number.

### Diodes (8)

<a id="elx_capacitor_filter_ripple"></a>

#### Capacitor-input filter: ripple voltage and DC output — `elx_capacitor_filter_ripple`

The peak-to-peak ripple, the DC output, the RMS ripple and the ripple factor of a rectifier with a smoothing capacitor and resistive load (small-ripple approximation).

$$
V_r = \frac{V_p}{f_rR_LC},\quad V_{dc} = V_p - \frac{V_r}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vp` | in | peak rectified voltage (after the diode drops) | V |
| `f` | in | line frequency | Hz |
| `RL` | in | load resistance | ohm |
| `C` | in | filter capacitance | F |
| `circuit` | in | half-wave or full-wave rectification | — |
| `f_ripple` | out | ripple frequency | Hz |
| `Vr` | out | peak-to-peak ripple | V |
| `Vdc` | out | DC output voltage | V |
| `Vr_rms` | out | RMS ripple | V |
| `ripple_factor` | out | ripple factor | — |

**Applies when:** Vr < Vp (small ripple).

```shell
$ electronics-engineering solve elx_capacitor_filter_ripple "Vp=16.3 V" "f=60 Hz" "RL=500 ohm" "C=1000 uF" circuit=full-wave
f_ripple = 120 Hz
Vr = 0.2716666667 V
Vdc = 16.16416667 V
Vr_rms = 0.07842341156 V
ripple_factor = 0.004851682934
  by: Capacitor-input filter: ripple voltage and DC output  (Vr = Vp/(f_r RL C); Vdc = Vp - Vr/2; Vr,rms = Vr/(2 sqrt 3))
  Vp = 16.3 V
  f = 60 Hz
  RL = 500 ohm
  C = 0.001 F
  circuit = full-wave
  assumes: Vr < Vp (small ripple)
```

In the worked example, Vp = 16.3 V, f = 60 Hz, RL = 500 ohm, C = 1000 uF, circuit = full-wave go through the equation above and give f_ripple = 120 Hz, Vr = 0.2716666667 V, Vdc = 16.16416667 V, Vr_rms = 0.07842341156 V. It applies only when Vr < Vp (small ripple): outside that it says what is wrong instead of returning a number.

<a id="elx_shockley_diode_current"></a>

#### Diode current from the Shockley equation — `elx_shockley_diode_current`

The current of a pn-junction diode at a terminal voltage from its saturation current, ideality factor and temperature (forward or reverse).

$$
I_D = I_S\left(e^{V_D/nV_T} - 1\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Is` | in | saturation current | A |
| `n` | in | ideality (emission) factor, 1 to 2 | — |
| `VD` | in | diode voltage (negative for reverse bias) | V |
| `T` | in | junction temperature | K |
| `ID` | out | diode current | A |
| `VT` | out | thermal voltage | V |

**Applies when:** Is, n, T > 0.

```shell
$ electronics-engineering solve elx_shockley_diode_current "Is=1e-14 A" n=1 "VD=0.65 V" "T=300 K"
ID = 0.0008308438348 A
VT = 0.02585199979 V
  by: Diode current from the Shockley equation  (ID = Is (exp(VD/(n VT)) - 1), VT = kT/q)
  Is = 1e-14 A
  n = 1
  VD = 0.65 V
  T = 300 K
  assumes: Is, n, T > 0
```

In the worked example, Is = 1e-14 A, n = 1, VD = 0.65 V, T = 300 K go through the equation above and give ID = 0.0008308438348 A, VT = 0.02585199979 V. It applies only when Is, n, T > 0: outside that it says what is wrong instead of returning a number.

<a id="elx_diode_voltage"></a>

#### Diode voltage for a given current — `elx_diode_voltage`

The voltage across a pn-junction diode carrying a given current (the inverse of the Shockley equation).

$$
V_D = nV_T\ln\left(1 + \frac{I_D}{I_S}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ID` | in | diode current | A |
| `Is` | in | saturation current | A |
| `n` | in | ideality factor | — |
| `T` | in | junction temperature | K |
| `VD` | out | diode voltage | V |
| `VT` | out | thermal voltage | V |

**Applies when:** ID > -Is; Is, n, T > 0.

```shell
$ electronics-engineering solve elx_diode_voltage "ID=0.001 A" "Is=1e-14 A" n=1 "T=300 K"
VD = 0.6547907227 V
VT = 0.02585199979 V
  by: Diode voltage for a given current  (VD = n VT ln(1 + ID/Is))
  ID = 0.001 A
  Is = 1e-14 A
  n = 1
  T = 300 K
  assumes: ID > -Is; Is, n, T > 0
```

In the worked example, ID = 0.001 A, Is = 1e-14 A, n = 1, T = 300 K go through the equation above and give VD = 0.6547907227 V, VT = 0.02585199979 V. It applies only when ID > -Is; Is, n, T > 0: outside that it says what is wrong instead of returning a number.

<a id="elx_led_series_resistor"></a>

#### LED series (current-limiting) resistor — `elx_led_series_resistor`

The current-limiting resistor for one LED or a string of LEDs in series at a chosen forward current, and the power in the resistor and in the LEDs.

$$
R = \frac{V_S - nV_F}{I_F}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vs` | in | supply voltage | V |
| `Vf` | in | forward voltage of one LED | V |
| `If` | in | LED forward current | A |
| `n` | in | number of LEDs in series (default 1) (optional) | — |
| `R` | out | series resistance | ohm |
| `P_R` | out | power in the resistor | W |
| `P_LED` | out | power in the LEDs | W |

**Applies when:** Vs > n Vf; If > 0.

```shell
$ electronics-engineering solve elx_led_series_resistor "Vs=5 V" "Vf=2 V" "If=20 mA"
R = 150 ohm
P_R = 0.06 W
P_LED = 0.04 W
  by: LED series (current-limiting) resistor  (R = (Vs - n Vf)/If; P_R = If^2 R)
  Vs = 5 V
  Vf = 2 V
  If = 0.02 A
  assumes: Vs > n Vf; If > 0
```

In the worked example, Vs = 5 V, Vf = 2 V, If = 20 mA go through the equation above and give R = 150 ohm, P_R = 0.06 W, P_LED = 0.04 W. It applies only when Vs > n Vf; If > 0: outside that it says what is wrong instead of returning a number.

<a id="elx_diode_operating_point"></a>

#### Operating point of a diode in series with a resistor (exact) — `elx_diode_operating_point`

The current and voltage of a forward-biased diode fed from a DC source through a resistor, solving the Shockley equation and Kirchhoff's voltage law together (no constant-drop approximation).

$$
V_S = I_DR + nV_T\ln\left(1 + \frac{I_D}{I_S}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VS` | in | source voltage | V |
| `R` | in | series resistance | ohm |
| `Is` | in | saturation current | A |
| `n` | in | ideality factor | — |
| `T` | in | junction temperature | K |
| `ID` | out | diode current | A |
| `VD` | out | diode voltage | V |
| `P_D` | out | power in the diode | W |

**Applies when:** VS > 0 (forward bias); R, Is, n, T > 0.

```shell
$ electronics-engineering solve elx_diode_operating_point "VS=5 V" "R=1000 ohm" "Is=1e-14 A" n=1 "T=300 K"
ID = 0.004307456373 A
VD = 0.6925436275 V
P_D = 0.002983101461 W
  by: Operating point of a diode in series with a resistor (exact)  (solve VS = ID R + n VT ln(1 + ID/Is) (bisection on VD))
  VS = 5 V
  R = 1000 ohm
  Is = 1e-14 A
  n = 1
  T = 300 K
  assumes: VS > 0 (forward bias); R, Is, n, T > 0
```

In the worked example, VS = 5 V, R = 1000 ohm, Is = 1e-14 A, n = 1, T = 300 K go through the equation above and give ID = 0.004307456373 A, VD = 0.6925436275 V, P_D = 0.002983101461 W. It applies only when VS > 0 (forward bias); R, Is, n, T > 0: outside that it says what is wrong instead of returning a number.

<a id="elx_rectifier_averages"></a>

#### Rectifier output: average, RMS, ripple factor and PIV — `elx_rectifier_averages`

The DC (average) and RMS output voltage, ripple factor, peak inverse voltage and conduction angle of an unfiltered half-wave, bridge or centre-tapped full-wave rectifier with a constant diode drop (0 V for ideal diodes).

$$
V_{dc} = \frac{k}{2\pi}\left[2V_m\cos\theta_1 - V(\pi - 2\theta_1)\right],\ \theta_1 = \arcsin\frac{V}{V_m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vm` | in | peak AC voltage of the (secondary) winding | V |
| `circuit` | in | half-wave, bridge or center-tapped (Vm per half of the winding) | — |
| `Von` | in | forward drop of one diode (0 V for ideal diodes) | V |
| `Vp_out` | out | peak output voltage | V |
| `Vdc` | out | average output voltage | V |
| `Vrms` | out | RMS output voltage | V |
| `ripple_factor` | out | ripple factor (AC rms / DC) | — |
| `PIV` | out | peak inverse voltage of a diode | V |
| `conduction_deg` | out | conduction angle per half cycle, degrees | — |

**Applies when:** diode drops below Vm; resistive load, no filter.

```shell
$ electronics-engineering solve elx_rectifier_averages "Vm=10 V" circuit=half-wave "Von=0 V"
Vp_out = 10 V
Vdc = 3.183098862 V
Vrms = 5 V
ripple_factor = 1.211363323
PIV = 10 V
conduction_deg = 180
  by: Rectifier output: average, RMS, ripple factor and PIV  (output max(0, Vm|sin| - V) with V = Von (2 Von for a bridge); averages integrated exactly; PIV Vm, Vm - Von, 2Vm - Von)
  Vm = 10 V
  Von = 0 V
  circuit = half-wave
  assumes: diode drops below Vm; resistive load, no filter
```

In the worked example, Vm = 10 V, circuit = half-wave, Von = 0 V go through the equation above and give Vp_out = 10 V, Vdc = 3.183098862 V, Vrms = 5 V, ripple_factor = 1.211363323. It applies only when diode drops below Vm; resistive load, no filter: outside that it says what is wrong instead of returning a number.

<a id="elx_zener_series_resistor_design"></a>

#### Zener regulator design: series resistor and Zener rating — `elx_zener_series_resistor_design`

The largest series resistance that keeps a Zener regulator in regulation at the lowest input and largest load, and the resulting worst-case Zener current and power (highest input, smallest load) and resistor power.

$$
R_S = \frac{V_{in,min} - V_Z}{I_{Z,min} + I_{L,max}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin_min` | in | lowest input voltage | V |
| `Vin_max` | in | highest input voltage | V |
| `Vz` | in | Zener voltage | V |
| `IL_min` | in | smallest load current | A |
| `IL_max` | in | largest load current | A |
| `Iz_min` | in | minimum Zener current for regulation | A |
| `Rs_max` | out | largest series resistance | ohm |
| `Iz_max` | out | worst-case Zener current | A |
| `Pz_max` | out | worst-case Zener power | W |
| `PRs_max` | out | worst-case resistor power | W |

**Applies when:** Vin_min > Vz; Vin_max >= Vin_min, IL_max >= IL_min >= 0.

```shell
$ electronics-engineering solve elx_zener_series_resistor_design "Vin_min=12 V" "Vin_max=15 V" "Vz=9.1 V" "IL_min=0 mA" "IL_max=50 mA" "Iz_min=5 mA"
Rs_max = 52.72727273 ohm
Iz_max = 0.1118965517 A
Pz_max = 1.018258621 W
PRs_max = 0.6601896552 W
  by: Zener regulator design: series resistor and Zener rating  (Rs = (Vin,min - Vz)/(Iz,min + IL,max); Iz,max = (Vin,max - Vz)/Rs - IL,min; Pz = Vz Iz,max)
  Vin_min = 12 V
  Vin_max = 15 V
  Vz = 9.1 V
  IL_min = 0 A
  IL_max = 0.05 A
  Iz_min = 0.005 A
  assumes: Vin_min > Vz; Vin_max >= Vin_min, IL_max >= IL_min >= 0
```

In the worked example, Vin_min = 12 V, Vin_max = 15 V, Vz = 9.1 V, IL_min = 0 mA, IL_max = 50 mA, Iz_min = 5 mA go through the equation above and give Rs_max = 52.72727273 ohm, Iz_max = 0.1118965517 A, Pz_max = 1.018258621 W, PRs_max = 0.6601896552 W. It applies only when Vin_min > Vz; Vin_max >= Vin_min, IL_max >= IL_min >= 0: outside that it says what is wrong instead of returning a number.

<a id="elx_zener_regulator"></a>

#### Zener shunt regulator: load voltage and currents — `elx_zener_regulator`

The output voltage and the source, load and Zener currents and the Zener power of a Zener shunt regulator (series resistor, Zener across the load), with an optional Zener resistance.

$$
I_Z = \frac{V_{in} - V_Z}{R_S} - \frac{V_Z}{R_L}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Rs` | in | series resistance | ohm |
| `Vz` | in | Zener (breakdown) voltage | V |
| `RL` | in | load resistance | ohm |
| `rz` | in | Zener resistance (0 for an ideal Zener) (optional) | ohm |
| `Vout` | out | load voltage | V |
| `I_s` | out | current in the series resistor | A |
| `I_L` | out | load current | A |
| `I_z` | out | Zener current | A |
| `P_z` | out | Zener power | W |

**Applies when:** the Zener must be in breakdown: Vin RL/(Rs + RL) > Vz.

```shell
$ electronics-engineering solve elx_zener_regulator "Vin=20 V" "Rs=220 ohm" "Vz=10 V" "RL=1000 ohm"
Vout = 10 V
I_s = 0.04545454545 A
I_L = 0.01 A
I_z = 0.03545454545 A
P_z = 0.3545454545 W
  by: Zener shunt regulator: load voltage and currents  (ideal: Vout = Vz, Iz = (Vin - Vz)/Rs - Vz/RL; with rz: node equation at the load)
  Vin = 20 V
  Rs = 220 ohm
  Vz = 10 V
  RL = 1000 ohm
  assumes: the Zener must be in breakdown: Vin RL/(Rs + RL) > Vz
```

In the worked example, Vin = 20 V, Rs = 220 ohm, Vz = 10 V, RL = 1000 ohm go through the equation above and give Vout = 10 V, I_s = 0.04545454545 A, I_L = 0.01 A, I_z = 0.03545454545 A. It applies only when the Zener must be in breakdown: Vin RL/(Rs + RL) > Vz: outside that it says what is wrong instead of returning a number.

### MOSFET (6)

<a id="elx_common_source_amplifier"></a>

#### Common-source amplifier with source degeneration and ro — `elx_common_source_amplifier`

The voltage gain, input and output resistances and overall gain of a common-source stage, exact with the transistor's output resistance ro and an optional unbypassed source resistor.

$$
A_v = -\frac{g_mr_oR_D'}{R_D' + r_o + R_S(1 + g_mr_o)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gm` | in | transconductance | S |
| `ro` | in | transistor output resistance | ohm |
| `RD` | in | drain resistor | ohm |
| `RL` | in | load resistor | ohm |
| `RS` | in | unbypassed source resistance (0 if bypassed) | ohm |
| `RG` | in | gate bias resistance | ohm |
| `Rsig` | in | source resistance | ohm |
| `Av` | out | voltage gain gate to output | — |
| `Rin` | out | input resistance | ohm |
| `Rout` | out | output resistance (excluding RL) | ohm |
| `Gv` | out | overall gain from the source | — |

**Applies when:** gm, ro, RD, RL, RG, Rsig > 0; RS >= 0.

```shell
$ electronics-engineering solve elx_common_source_amplifier "gm=2 mS" "ro=100000 ohm" "RD=10000 ohm" "RL=10000 ohm" "RS=0 ohm" "RG=1000000 ohm" "Rsig=10000 ohm"
Av = -9.523809524
Rin = 1000000 ohm
Rout = 9090.909091 ohm
Gv = -9.42951438
  by: Common-source amplifier with source degeneration and ro  (Av = -gm ro RD'/(RD' + ro + RS (1 + gm ro)) with RD' = RD || RL; Rout = RD || (ro + RS(1 + gm ro)))
  gm = 0.002 S
  ro = 100000 ohm
  RD = 10000 ohm
  RL = 10000 ohm
  RS = 0 ohm
  RG = 1000000 ohm
  Rsig = 10000 ohm
  assumes: gm, ro, RD, RL, RG, Rsig > 0; RS >= 0
```

In the worked example, gm = 2 mS, ro = 100000 ohm, RD = 10000 ohm, RL = 10000 ohm, RS = 0 ohm, RG = 1000000 ohm, Rsig = 10000 ohm go through the equation above and give Av = -9.523809524, Rin = 1000000 ohm, Rout = 9090.909091 ohm, Gv = -9.42951438. It applies only when gm, ro, RD, RL, RG, Rsig > 0; RS >= 0: outside that it says what is wrong instead of returning a number.

<a id="elx_mosfet_drain_current"></a>

#### MOSFET drain current (cutoff, triode, saturation) — `elx_mosfet_drain_current`

The operating region and drain current of an enhancement NMOS transistor by the square law: cutoff, triode or saturation with optional channel-length modulation (use magnitudes for PMOS).

$$
I_D = \begin{cases} k_n\left(V_{OV}V_{DS} - \tfrac{1}{2}V_{DS}^2\right) & V_{DS} < V_{OV}\\ \tfrac{1}{2}k_nV_{OV}^2(1+\lambda V_{DS}) & V_{DS}\ge V_{OV}\end{cases}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `kn` | in | process transconductance times aspect ratio, mu Cox W/L | A/V^2 |
| `Vt` | in | threshold voltage | V |
| `VGS` | in | gate-source voltage | V |
| `VDS` | in | drain-source voltage | V |
| `lam` | in | channel-length modulation lambda (default 0) (optional) | 1/V |
| `region` | out | cutoff, triode or saturation | — |
| `ID` | out | drain current | A |
| `VOV` | out | overdrive VGS - Vt | V |
| `VDS_sat` | out | saturation voltage | V |

**Applies when:** kn, Vt > 0; VDS >= 0; long-channel square law.

```shell
$ electronics-engineering solve elx_mosfet_drain_current "kn=2 mA/V^2" "Vt=1 V" "VGS=3 V" "VDS=5 V"
region = saturation
ID = 0.004 A
VOV = 2 V
VDS_sat = 2 V
  by: MOSFET drain current (cutoff, triode, saturation)  (cutoff VOV <= 0; triode VDS < VOV: ID = kn (VOV VDS - VDS^2/2); saturation: ID = (kn/2) VOV^2 (1 + lambda VDS))
  kn = 0.002 A/V^2
  Vt = 1 V
  VGS = 3 V
  VDS = 5 V
  assumes: kn, Vt > 0; VDS >= 0; long-channel square law
```

In the worked example, kn = 2 mA/V^2, Vt = 1 V, VGS = 3 V, VDS = 5 V go through the equation above and give region = saturation, ID = 0.004 A, VOV = 2 V, VDS_sat = 2 V. It applies only when kn, Vt > 0; VDS >= 0; long-channel square law: outside that it says what is wrong instead of returning a number.

<a id="elx_mosfet_gm_ro"></a>

#### MOSFET small-signal gm, ro and intrinsic gain — `elx_mosfet_gm_ro`

The overdrive voltage, gate-source voltage, transconductance gm = sqrt(2 kn ID), output resistance ro = 1/(lambda ID) and intrinsic gain of a MOSFET in saturation at a drain current.

$$
g_m = \sqrt{2k_nI_D} = \frac{2I_D}{V_{OV}},\quad r_o = \frac{1}{\lambda I_D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `kn` | in | mu Cox W/L | A/V^2 |
| `ID` | in | drain bias current | A |
| `lam` | in | channel-length modulation lambda | 1/V |
| `Vt` | in | threshold voltage | V |
| `VOV` | out | overdrive voltage | V |
| `VGS` | out | gate-source voltage | V |
| `gm` | out | transconductance | S |
| `r_o` | out | output resistance | ohm |
| `A0` | out | intrinsic gain gm r_o | — |

**Applies when:** saturation; ID taken without the (1 + lambda VDS) factor.

```shell
$ electronics-engineering solve elx_mosfet_gm_ro "kn=2 mA/V^2" "ID=1 mA" "lam=0.01 1/V" "Vt=0.7 V"
VOV = 1 V
VGS = 1.7 V
gm = 0.002 S
r_o = 100000 ohm
A0 = 200
  by: MOSFET small-signal gm, ro and intrinsic gain  (VOV = sqrt(2 ID/kn); gm = sqrt(2 kn ID) = 2 ID/VOV; ro = 1/(lambda ID))
  kn = 0.002 A/V^2
  ID = 0.001 A
  lam = 0.01 1/V
  Vt = 0.7 V
  assumes: saturation; ID taken without the (1 + lambda VDS) factor
```

In the worked example, kn = 2 mA/V^2, ID = 1 mA, lam = 0.01 1/V, Vt = 0.7 V go through the equation above and give VOV = 1 V, VGS = 1.7 V, gm = 0.002 S, r_o = 100000 ohm. It applies only when saturation; ID taken without the (1 + lambda VDS) factor: outside that it says what is wrong instead of returning a number.

<a id="elx_mosfet_body_effect"></a>

#### MOSFET threshold voltage with body effect — `elx_mosfet_body_effect`

The threshold voltage of a MOSFET whose source is above its body, Vt = Vt0 + gamma (sqrt(2 phiF + VSB) - sqrt(2 phiF)).

$$
V_t = V_{t0} + \gamma\left(\sqrt{2\phi_F + V_{SB}} - \sqrt{2\phi_F}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vt0` | in | zero-bias threshold voltage | V |
| `gamma` | in | body-effect coefficient in V^0.5 | — |
| `phi2F` | in | surface potential 2 phi_F (about 0.6 to 0.9 V) | V |
| `VSB` | in | source-body voltage | V |
| `Vt` | out | threshold voltage | V |
| `dVt` | out | threshold increase | V |

**Applies when:** VSB >= 0; uniform substrate doping.

```shell
$ electronics-engineering solve elx_mosfet_body_effect "Vt0=0.7 V" gamma=0.4 "phi2F=0.7 V" "VSB=3 V"
Vt = 1.134751352 V
dVt = 0.4347513519 V
  by: MOSFET threshold voltage with body effect  (Vt = Vt0 + gamma (sqrt(2 phiF + VSB) - sqrt(2 phiF)))
  Vt0 = 0.7 V
  gamma = 0.4
  phi2F = 0.7 V
  VSB = 3 V
  assumes: VSB >= 0; uniform substrate doping
```

In the worked example, Vt0 = 0.7 V, gamma = 0.4, phi2F = 0.7 V, VSB = 3 V go through the equation above and give Vt = 1.134751352 V, dVt = 0.4347513519 V. It applies only when VSB >= 0; uniform substrate doping: outside that it says what is wrong instead of returning a number.

<a id="elx_mosfet_divider_bias"></a>

#### MOSFET voltage-divider bias with source resistor: Q-point — `elx_mosfet_divider_bias`

The Q-point (gate voltage, VGS, drain current, node voltages and VDS) of an NMOS stage with a gate divider and a source resistor, solving the square law and the gate loop (the physical root), with a saturation check.

$$
V_{OV} = \frac{2(V_G - V_t)}{1 + \sqrt{1 + 2k_nR_S(V_G - V_t)}},\quad I_D = \tfrac{1}{2}k_nV_{OV}^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VDD` | in | supply voltage | V |
| `R1` | in | upper gate resistor | ohm |
| `R2` | in | lower gate resistor | ohm |
| `RD` | in | drain resistor | ohm |
| `RS` | in | source resistor (0 allowed) | ohm |
| `kn` | in | mu Cox W/L | A/V^2 |
| `Vt` | in | threshold voltage | V |
| `VG` | out | gate voltage | V |
| `VGS` | out | gate-source voltage | V |
| `VOV` | out | overdrive | V |
| `ID` | out | drain current | A |
| `VS` | out | source voltage | V |
| `VD` | out | drain voltage | V |
| `VDS` | out | drain-source voltage | V |

**Applies when:** VG > Vt; saturation: VDS >= VOV (otherwise refused); lambda neglected.

```shell
$ electronics-engineering solve elx_mosfet_divider_bias "VDD=15 V" "R1=1000000 ohm" "R2=500000 ohm" "RD=3300 ohm" "RS=1000 ohm" "kn=2 mA/V^2" "Vt=1.5 V"
VG = 5 V
VGS = 2.936491673 V
VOV = 1.436491673 V
ID = 0.002063508327 A
VS = 2.063508327 V
VD = 8.190422521 V
VDS = 6.126914194 V
  by: MOSFET voltage-divider bias with source resistor: Q-point  (VG = VDD R2/(R1 + R2); (kn RS/2) VOV^2 + VOV - (VG - Vt) = 0, positive root; ID = (kn/2) VOV^2)
  VDD = 15 V
  R1 = 1000000 ohm
  R2 = 500000 ohm
  RD = 3300 ohm
  RS = 1000 ohm
  kn = 0.002 A/V^2
  Vt = 1.5 V
  assumes: VG > Vt; saturation: VDS >= VOV (otherwise refused); lambda neglected
```

In the worked example, VDD = 15 V, R1 = 1000000 ohm, R2 = 500000 ohm, RD = 3300 ohm, RS = 1000 ohm, kn = 2 mA/V^2, Vt = 1.5 V go through the equation above and give VG = 5 V, VGS = 2.936491673 V, VOV = 1.436491673 V, ID = 0.002063508327 A. It applies only when VG > Vt; saturation: VDS >= VOV (otherwise refused); lambda neglected: outside that it says what is wrong instead of returning a number.

<a id="elx_source_follower"></a>

#### Source follower (common drain) — `elx_source_follower`

The voltage gain, input resistance, output resistance and overall gain of a MOSFET source follower, including ro.

$$
A_v = \frac{g_mR}{1 + g_mR},\quad R_{out} = R_S\parallel r_o\parallel\frac{1}{g_m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gm` | in | transconductance | S |
| `ro` | in | transistor output resistance | ohm |
| `RS` | in | source resistor | ohm |
| `RL` | in | load resistor | ohm |
| `RG` | in | gate bias resistance | ohm |
| `Rsig` | in | source resistance | ohm |
| `Av` | out | voltage gain gate to output | — |
| `Rin` | out | input resistance | ohm |
| `Rout` | out | output resistance | ohm |
| `Gv` | out | overall gain | — |

**Applies when:** drain at signal ground.

```shell
$ electronics-engineering solve elx_source_follower "gm=2 mS" "ro=50000 ohm" "RS=2000 ohm" "RL=2000 ohm" "RG=1000000 ohm" "Rsig=10000 ohm"
Av = 0.6622516556
Rin = 1000000 ohm
Rout = 396.8253968 ohm
Gv = 0.6556947085
  by: Source follower (common drain)  (Av = gm R/(1 + gm R) with R = RS || RL || ro; Rout = RS || ro || 1/gm)
  gm = 0.002 S
  ro = 50000 ohm
  RS = 2000 ohm
  RL = 2000 ohm
  RG = 1000000 ohm
  Rsig = 10000 ohm
  assumes: drain at signal ground
```

In the worked example, gm = 2 mS, ro = 50000 ohm, RS = 2000 ohm, RL = 2000 ohm, RG = 1000000 ohm, Rsig = 10000 ohm go through the equation above and give Av = 0.6622516556, Rin = 1000000 ohm, Rout = 396.8253968 ohm, Gv = 0.6556947085. It applies only when drain at signal ground: outside that it says what is wrong instead of returning a number.

### Op-amps (14)

<a id="elx_closed_loop_bandwidth"></a>

#### Closed-loop bandwidth from the gain-bandwidth product — `elx_closed_loop_bandwidth`

The closed-loop gain, the noise gain 1 + Rf/R1 and the -3 dB bandwidth GBW/(noise gain) of an inverting or non-inverting amplifier built with a single-pole op-amp.

$$
f_{3dB} = \frac{\text{GBW}}{1 + R_f/R_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `GBW` | in | gain-bandwidth product (unity-gain frequency) | Hz |
| `config` | in | inverting or non-inverting amplifier | — |
| `Rf` | in | feedback resistor | ohm |
| `R1` | in | input (or ground) resistor | ohm |
| `G0` | out | low-frequency closed-loop gain | — |
| `noise_gain` | out | noise gain 1 + Rf/R1 | — |
| `f_3dB` | out | closed-loop -3 dB bandwidth | Hz |

**Applies when:** single-pole op-amp with open-loop gain much larger than the closed-loop gain.

```shell
$ electronics-engineering solve elx_closed_loop_bandwidth "GBW=1000000 Hz" config=non-inverting "Rf=90000 ohm" "R1=10000 ohm"
G0 = 10
noise_gain = 10
f_3dB = 100000 Hz
  by: Closed-loop bandwidth from the gain-bandwidth product  (f_3dB = GBW/(1 + Rf/R1) for both configurations)
  GBW = 1000000 Hz
  Rf = 90000 ohm
  R1 = 10000 ohm
  config = non-inverting
  assumes: single-pole op-amp with open-loop gain much larger than the closed-loop gain
```

In the worked example, GBW = 1000000 Hz, config = non-inverting, Rf = 90000 ohm, R1 = 10000 ohm go through the equation above and give G0 = 10, noise_gain = 10, f_3dB = 100000 Hz. It applies only when single-pole op-amp with open-loop gain much larger than the closed-loop gain: outside that it says what is wrong instead of returning a number.

<a id="elx_inverting_amplifier_finite_gain"></a>

#### Inverting amplifier with finite open-loop gain — `elx_inverting_amplifier_finite_gain`

The exact closed-loop gain of an inverting amplifier built with an op-amp of finite open-loop gain A0, its relative gain error and output.

$$
G = \frac{-R_f/R_1}{1 + \dfrac{1 + R_f/R_1}{A_0}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rf` | in | feedback resistor | ohm |
| `R1` | in | input resistor | ohm |
| `A0` | in | open-loop voltage gain (a ratio, not dB) | — |
| `Vin` | in | input voltage | V |
| `G` | out | closed-loop gain | — |
| `G_ideal` | out | ideal gain -Rf/R1 | — |
| `Vout` | out | output voltage | V |
| `error` | out | relative gain error 1 - G/G_ideal | — |

**Applies when:** A0 > 0 (ratio); infinite input resistance, zero output resistance.

```shell
$ electronics-engineering solve elx_inverting_amplifier_finite_gain "Rf=100000 ohm" "R1=1000 ohm" A0=10000 "Vin=0.01 V"
G = -99.000099
G_ideal = -100
Vout = -0.99000099 V
error = 0.009999009999
  by: Inverting amplifier with finite open-loop gain  (G = -(Rf/R1)/(1 + (1 + Rf/R1)/A0))
  Rf = 100000 ohm
  R1 = 1000 ohm
  A0 = 10000
  Vin = 0.01 V
  assumes: A0 > 0 (ratio); infinite input resistance, zero output resistance
```

In the worked example, Rf = 100000 ohm, R1 = 1000 ohm, A0 = 10000, Vin = 0.01 V go through the equation above and give G = -99.000099, G_ideal = -100, Vout = -0.99000099 V, error = 0.009999009999. It applies only when A0 > 0 (ratio); infinite input resistance, zero output resistance: outside that it says what is wrong instead of returning a number.

<a id="elx_inverting_amplifier"></a>

#### Inverting op-amp amplifier — `elx_inverting_amplifier`

The closed-loop gain, output voltage and input resistance of an ideal inverting amplifier.

$$
V_{out} = -\frac{R_f}{R_{in}}V_{in}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rf` | in | feedback resistor | ohm |
| `Rin` | in | input resistor | ohm |
| `Vin` | in | input voltage | V |
| `G` | out | closed-loop gain -Rf/Rin | — |
| `Vout` | out | output voltage | V |
| `Zin` | out | input resistance | ohm |

**Applies when:** ideal op-amp, output not saturated.

```shell
$ electronics-engineering solve elx_inverting_amplifier "Rf=100000 ohm" "Rin=10000 ohm" "Vin=0.5 V"
G = -10
Vout = -5 V
Zin = 10000 ohm
  by: Inverting op-amp amplifier  (G = -Rf/Rin; Vout = G Vin; Zin = Rin)
  Rf = 100000 ohm
  Rin = 10000 ohm
  Vin = 0.5 V
  assumes: ideal op-amp, output not saturated
```

In the worked example, Rf = 100000 ohm, Rin = 10000 ohm, Vin = 0.5 V go through the equation above and give G = -10, Vout = -5 V, Zin = 10000 ohm. It applies only when ideal op-amp, output not saturated: outside that it says what is wrong instead of returning a number.

<a id="elx_schmitt_trigger"></a>

#### Inverting Schmitt trigger thresholds — `elx_schmitt_trigger`

The upper and lower switching thresholds, the hysteresis width and the centre of an inverting op-amp Schmitt trigger whose positive feedback divider (Rf from the output, R1 to a reference) sets the + input.

$$
V_{TH,TL} = \frac{V_{sat}R_1 + V_{ref}R_f}{R_1 + R_f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rf` | in | resistor from the output to the + input | ohm |
| `R1` | in | resistor from the + input to Vref | ohm |
| `VsatH` | in | high output saturation voltage | V |
| `VsatL` | in | low output saturation voltage (negative for a split supply) | V |
| `Vref` | in | reference voltage (0 V for ground) | V |
| `V_TH` | out | upper threshold | V |
| `V_TL` | out | lower threshold | V |
| `hysteresis` | out | hysteresis width | V |
| `center` | out | centre of the hysteresis band | V |

**Applies when:** VsatH > VsatL; input on the inverting terminal.

```shell
$ electronics-engineering solve elx_schmitt_trigger "Rf=90000 ohm" "R1=10000 ohm" "VsatH=12 V" "VsatL=-12 V" "Vref=0 V"
V_TH = 1.2 V
V_TL = -1.2 V
hysteresis = 2.4 V
center = 0 V
  by: Inverting Schmitt trigger thresholds  (V_T = (Vsat R1 + Vref Rf)/(R1 + Rf); hysteresis = (VsatH - VsatL) R1/(R1 + Rf))
  Rf = 90000 ohm
  R1 = 10000 ohm
  VsatH = 12 V
  VsatL = -12 V
  Vref = 0 V
  assumes: VsatH > VsatL; input on the inverting terminal
```

In the worked example, Rf = 90000 ohm, R1 = 10000 ohm, VsatH = 12 V, VsatL = -12 V, Vref = 0 V go through the equation above and give V_TH = 1.2 V, V_TL = -1.2 V, hysteresis = 2.4 V, center = 0 V. It applies only when VsatH > VsatL; input on the inverting terminal: outside that it says what is wrong instead of returning a number.

<a id="elx_summing_amplifier"></a>

#### Inverting summing amplifier — `elx_summing_amplifier`

The output of an ideal inverting summing amplifier with several inputs, each through its own resistor, and the feedback current.

$$
V_{out} = -R_f\sum_i\frac{V_i}{R_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | input voltages in volts | — |
| `R` | in | input resistors in kilohms, one per input | — |
| `Rf` | in | feedback resistor | ohm |
| `Vout` | out | output voltage | V |
| `I_f` | out | current through the feedback resistor | A |

**Applies when:** one resistor per input; ideal op-amp.

```shell
$ electronics-engineering solve elx_summing_amplifier "V=[1, 2, -0.5]" "R=[10, 20, 5]" "Rf=10000 ohm"
Vout = -1 V
I_f = 0.0001 A
  by: Inverting summing amplifier  (Vout = -Rf sum(Vi/Ri))
  Rf = 10000 ohm
  assumes: one resistor per input; ideal op-amp
```

In the worked example, V = [1, 2, -0.5], R = [10, 20, 5], Rf = 10000 ohm go through the equation above and give Vout = -1 V, I_f = 0.0001 A. It applies only when one resistor per input; ideal op-amp: outside that it says what is wrong instead of returning a number.

<a id="elx_negative_feedback"></a>

#### Negative feedback: closed-loop gain and bandwidth extension — `elx_negative_feedback`

The closed-loop gain A/(1 + A beta), the loop gain, the desensitivity factor (also in dB) and the extended bandwidth of a single-pole amplifier with negative feedback.

$$
A_f = \frac{A}{1 + A\beta},\quad f_{H,f} = f_H(1 + A\beta)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | open-loop gain (a ratio) | — |
| `beta` | in | feedback factor (a ratio) | — |
| `fH` | in | open-loop -3 dB bandwidth | Hz |
| `Af` | out | closed-loop gain | — |
| `loop_gain` | out | loop gain A beta | — |
| `desensitivity` | out | 1 + A beta | — |
| `desensitivity_dB` | out | amount of feedback in dB | — |
| `fH_closed` | out | closed-loop bandwidth | Hz |

**Applies when:** single-pole open-loop response; frequency-independent beta.

```shell
$ electronics-engineering solve elx_negative_feedback A=10000 beta=0.01 "fH=1000 Hz"
Af = 99.00990099
loop_gain = 100
desensitivity = 101
desensitivity_dB = 40.08642748
fH_closed = 101000 Hz
  by: Negative feedback: closed-loop gain and bandwidth extension  (Af = A/(1 + A beta); fH,f = fH (1 + A beta))
  A = 10000
  beta = 0.01
  fH = 1000 Hz
  assumes: single-pole open-loop response; frequency-independent beta
```

In the worked example, A = 10000, beta = 0.01, fH = 1000 Hz go through the equation above and give Af = 99.00990099, loop_gain = 100, desensitivity = 101, desensitivity_dB = 40.08642748. It applies only when single-pole open-loop response; frequency-independent beta: outside that it says what is wrong instead of returning a number.

<a id="elx_noninverting_amplifier"></a>

#### Non-inverting op-amp amplifier — `elx_noninverting_amplifier`

The closed-loop gain 1 + Rf/Rg and the output voltage of an ideal non-inverting amplifier.

$$
V_{out} = \left(1 + \frac{R_f}{R_g}\right)V_{in}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rf` | in | feedback resistor | ohm |
| `Rg` | in | resistor from the inverting input to ground | ohm |
| `Vin` | in | input voltage | V |
| `G` | out | closed-loop gain | — |
| `Vout` | out | output voltage | V |

**Applies when:** ideal op-amp.

```shell
$ electronics-engineering solve elx_noninverting_amplifier "Rf=90000 ohm" "Rg=10000 ohm" "Vin=0.3 V"
G = 10
Vout = 3 V
  by: Non-inverting op-amp amplifier  (G = 1 + Rf/Rg; Vout = G Vin)
  Rf = 90000 ohm
  Rg = 10000 ohm
  Vin = 0.3 V
  assumes: ideal op-amp
```

In the worked example, Rf = 90000 ohm, Rg = 10000 ohm, Vin = 0.3 V go through the equation above and give G = 10, Vout = 3 V. It applies only when ideal op-amp: outside that it says what is wrong instead of returning a number.

<a id="elx_difference_amplifier"></a>

#### Op-amp difference amplifier (with resistor mismatch) — `elx_difference_amplifier`

The output of a one-op-amp difference amplifier with any four resistors, and its differential and common-mode gains (the common-mode gain is zero only when R4/R3 = R2/R1).

$$
V_{out} = \frac{R_4}{R_3+R_4}\left(1+\frac{R_2}{R_1}\right)V_2 - \frac{R_2}{R_1}V_1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R1` | in | resistor from V1 to the inverting input | ohm |
| `R2` | in | feedback resistor | ohm |
| `R3` | in | resistor from V2 to the non-inverting input | ohm |
| `R4` | in | resistor from the non-inverting input to ground | ohm |
| `V1` | in | voltage at the inverting side | V |
| `V2` | in | voltage at the non-inverting side | V |
| `Vout` | out | output voltage | V |
| `Ad` | out | differential gain (per volt of V2 - V1) | — |
| `Acm` | out | common-mode gain | — |

**Applies when:** ideal op-amp.

```shell
$ electronics-engineering solve elx_difference_amplifier "R1=10000 ohm" "R2=100000 ohm" "R3=10000 ohm" "R4=100000 ohm" "V1=1 V" "V2=1.2 V"
Vout = 2 V
Ad = 10
Acm = 0
  by: Op-amp difference amplifier (with resistor mismatch)  (Vout = -(R2/R1) V1 + (R4/(R3 + R4))(1 + R2/R1) V2; Ad = (a2 - a1)/2; Acm = a1 + a2)
  R1 = 10000 ohm
  R2 = 100000 ohm
  R3 = 10000 ohm
  R4 = 100000 ohm
  V1 = 1 V
  V2 = 1.2 V
  assumes: ideal op-amp
```

In the worked example, R1 = 10000 ohm, R2 = 100000 ohm, R3 = 10000 ohm, R4 = 100000 ohm, V1 = 1 V, V2 = 1.2 V go through the equation above and give Vout = 2 V, Ad = 10, Acm = 0. It applies only when ideal op-amp: outside that it says what is wrong instead of returning a number.

<a id="elx_opamp_differentiator"></a>

#### Op-amp differentiator: gain and output amplitude — `elx_opamp_differentiator`

The gain |H| = 2 pi f R C of an ideal inverting differentiator at a frequency and the peak output for a sinusoidal input of given amplitude.

$$
|H(j\omega)| = \omega RC
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | feedback resistor | ohm |
| `C` | in | input capacitor | F |
| `A` | in | peak input amplitude | V |
| `f` | in | input frequency | Hz |
| `gain` | out | gain magnitude | — |
| `Vout_peak` | out | peak output voltage | V |
| `gain_dB` | out | gain in dB | — |

**Applies when:** ideal op-amp (no high-frequency limiting).

```shell
$ electronics-engineering solve elx_opamp_differentiator "R=10000 ohm" "C=10 nF" "A=1 V" "f=1000 Hz"
gain = 0.6283185307
Vout_peak = 0.6283185307 V
gain_dB = -4.036402633
  by: Op-amp differentiator: gain and output amplitude  (|H| = 2 pi f R C; Vout,peak = |H| A)
  R = 10000 ohm
  C = 1e-08 F
  A = 1 V
  f = 1000 Hz
  assumes: ideal op-amp (no high-frequency limiting)
```

In the worked example, R = 10000 ohm, C = 10 nF, A = 1 V, f = 1000 Hz go through the equation above and give gain = 0.6283185307, Vout_peak = 0.6283185307 V, gain_dB = -4.036402633. It applies only when ideal op-amp (no high-frequency limiting): outside that it says what is wrong instead of returning a number.

<a id="elx_opamp_integrator"></a>

#### Op-amp integrator: output ramp and unity-gain frequency — `elx_opamp_integrator`

The output of an ideal inverting integrator after a time with a constant input, its ramp rate, and the frequency at which its gain is one.

$$
V_{out}(t) = V_0 - \frac{1}{RC}\int_0^t V_{in}\,dt
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | input resistor | ohm |
| `C` | in | feedback capacitor | F |
| `Vin` | in | constant input voltage | V |
| `t` | in | time since the start | s |
| `V0` | in | initial output voltage (default 0) (optional) | V |
| `Vout` | out | output voltage at t | V |
| `slope` | out | output ramp rate | V/s |
| `f_unity` | out | unity-gain frequency 1/(2 pi R C) | Hz |

**Applies when:** ideal op-amp, output not saturated.

```shell
$ electronics-engineering solve elx_opamp_integrator "R=10000 ohm" "C=1 uF" "Vin=0.5 V" "t=2 ms"
Vout = -0.1 V
slope = -50 V/s
f_unity = 15.91549431 Hz
  by: Op-amp integrator: output ramp and unity-gain frequency  (Vout = V0 - Vin t/(R C); f = 1/(2 pi R C))
  R = 10000 ohm
  C = 1e-06 F
  Vin = 0.5 V
  t = 0.002 s
  assumes: ideal op-amp, output not saturated
```

In the worked example, R = 10000 ohm, C = 1 uF, Vin = 0.5 V, t = 2 ms go through the equation above and give Vout = -0.1 V, slope = -50 V/s, f_unity = 15.91549431 Hz. It applies only when ideal op-amp, output not saturated: outside that it says what is wrong instead of returning a number.

<a id="elx_opamp_dc_offset"></a>

#### Op-amp output offset from input offset voltage and bias currents — `elx_opamp_dc_offset`

The DC output error of an inverting or non-inverting amplifier (input grounded) caused by the input offset voltage and the input bias and offset currents, and the compensating resistor that cancels the bias-current error.

$$
V_{out} = \left(V_{os} - I_{B+}R_p\right)\left(1 + \frac{R_f}{R_1}\right) + I_{B-}R_f
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vos` | in | input offset voltage (in series with the + input) | V |
| `IB` | in | average input bias current | A |
| `Ios` | in | input offset current IB- minus IB+ | A |
| `Rf` | in | feedback resistor | ohm |
| `R1` | in | resistor from the inverting input to ground | ohm |
| `Rp` | in | resistor from the + input to ground (0 if none) | ohm |
| `Vout_Vos` | out | output error from Vos | V |
| `Vout_bias` | out | output error from the bias currents | V |
| `Vout_total` | out | total output error | V |
| `Rp_optimal` | out | bias-cancelling Rp = R1 \|\| Rf | ohm |

**Applies when:** currents flow into the inputs; IB- = IB + Ios/2, IB+ = IB - Ios/2.

```shell
$ electronics-engineering solve elx_opamp_dc_offset "Vos=2 mV" "IB=100 nA" "Ios=10 nA" "Rf=100000 ohm" "R1=10000 ohm" "Rp=0 ohm"
Vout_Vos = 0.022 V
Vout_bias = 0.0105 V
Vout_total = 0.0325 V
Rp_optimal = 9090.909091 ohm
  by: Op-amp output offset from input offset voltage and bias currents  (Vout = Vos (1 + Rf/R1) + IB- Rf - IB+ Rp (1 + Rf/R1))
  Vos = 0.002 V
  IB = 1e-07 A
  Ios = 1e-08 A
  Rf = 100000 ohm
  R1 = 10000 ohm
  Rp = 0 ohm
  assumes: currents flow into the inputs; IB- = IB + Ios/2, IB+ = IB - Ios/2
```

In the worked example, Vos = 2 mV, IB = 100 nA, Ios = 10 nA, Rf = 100000 ohm, R1 = 10000 ohm, Rp = 0 ohm go through the equation above and give Vout_Vos = 0.022 V, Vout_bias = 0.0105 V, Vout_total = 0.0325 V, Rp_optimal = 9090.909091 ohm. It applies only when currents flow into the inputs; IB- = IB + Ios/2, IB+ = IB - Ios/2: outside that it says what is wrong instead of returning a number.

<a id="elx_two_pole_phase_margin"></a>

#### Phase margin of a two-pole amplifier with feedback — `elx_two_pole_phase_margin`

The loop-gain crossover frequency and the phase margin of an amplifier with two real poles and a frequency-independent feedback factor, and its closed-loop DC gain.

$$
PM = 180^\circ - \arctan\frac{f_c}{f_1} - \arctan\frac{f_c}{f_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A0` | in | DC open-loop gain (a ratio) | — |
| `f1` | in | first pole frequency | Hz |
| `f2` | in | second pole frequency | Hz |
| `beta` | in | feedback factor | — |
| `f_c` | out | loop-gain crossover frequency | Hz |
| `PM_deg` | out | phase margin in degrees | — |
| `Af` | out | closed-loop DC gain | — |

**Applies when:** A0 beta > 1; two real left-half-plane poles.

```shell
$ electronics-engineering solve elx_two_pole_phase_margin A0=100000 "f1=10 Hz" "f2=1000000 Hz" beta=0.1
f_c = 99508.54868 Hz
PM_deg = 84.32304539
Af = 9.9990001
  by: Phase margin of a two-pole amplifier with feedback  (|A0 beta|^2 = (1 + (fc/f1)^2)(1 + (fc/f2)^2) solved for fc^2; PM = 180 - atan(fc/f1) - atan(fc/f2))
  A0 = 100000
  f1 = 10 Hz
  f2 = 1000000 Hz
  beta = 0.1
  assumes: A0 beta > 1; two real left-half-plane poles
```

In the worked example, A0 = 100000, f1 = 10 Hz, f2 = 1000000 Hz, beta = 0.1 go through the equation above and give f_c = 99508.54868 Hz, PM_deg = 84.32304539, Af = 9.9990001. It applies only when A0 beta > 1; two real left-half-plane poles: outside that it says what is wrong instead of returning a number.

<a id="elx_slew_rate_limit"></a>

#### Slew-rate limit: full-power bandwidth — `elx_slew_rate_limit`

The full-power bandwidth SR/(2 pi Vp) of an op-amp, the largest undistorted amplitude at a frequency, the slope a sine wave needs and whether it is slew-limited.

$$
f_{max} = \frac{SR}{2\pi V_p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `SR` | in | slew rate | V/s |
| `Vp` | in | peak output amplitude | V |
| `f` | in | signal frequency | Hz |
| `f_max` | out | full-power bandwidth at Vp | Hz |
| `Vp_max` | out | largest undistorted amplitude at f | V |
| `slope_needed` | out | maximum slope of the sine, 2 pi f Vp | V/s |
| `limited` | out | slew-limited or not slew-limited | — |

**Applies when:** sinusoidal output.

```shell
$ electronics-engineering solve elx_slew_rate_limit "SR=0.5 V/us" "Vp=10 V" "f=20000 Hz"
f_max = 7957.747155 Hz
Vp_max = 3.978873577 V
slope_needed = 1256637.061 V/s
limited = slew-limited
  by: Slew-rate limit: full-power bandwidth  (f_max = SR/(2 pi Vp); Vp,max = SR/(2 pi f); slope = 2 pi f Vp)
  SR = 500000 V/s
  Vp = 10 V
  f = 20000 Hz
  assumes: sinusoidal output
```

In the worked example, SR = 0.5 V/us, Vp = 10 V, f = 20000 Hz go through the equation above and give f_max = 7957.747155 Hz, Vp_max = 3.978873577 V, slope_needed = 1256637.061 V/s, limited = slew-limited. It applies only when sinusoidal output: outside that it says what is wrong instead of returning a number.

<a id="elx_instrumentation_amplifier"></a>

#### Three-op-amp instrumentation amplifier — `elx_instrumentation_amplifier`

The gain and output of the classic three-op-amp instrumentation amplifier: input stage gain 1 + 2 R1/RG times the difference stage gain R3/R2.

$$
V_{out} = \left(1 + \frac{2R_1}{R_G}\right)\frac{R_3}{R_2}(V_2 - V_1)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R1` | in | feedback resistor of each input buffer | ohm |
| `RG` | in | gain-setting resistor between the buffers | ohm |
| `R2` | in | input resistors of the difference stage | ohm |
| `R3` | in | feedback and ground resistors of the difference stage | ohm |
| `V1` | in | input at the inverting side | V |
| `V2` | in | input at the non-inverting side | V |
| `G` | out | differential gain | — |
| `G_stage1` | out | gain of the input stage | — |
| `Vout` | out | output voltage | V |

**Applies when:** ideal op-amps, matched resistors.

```shell
$ electronics-engineering solve elx_instrumentation_amplifier "R1=25000 ohm" "RG=1000 ohm" "R2=10000 ohm" "R3=10000 ohm" "V1=1 V" "V2=1.01 V"
G = 51
G_stage1 = 51
Vout = 0.51 V
  by: Three-op-amp instrumentation amplifier  (G = (1 + 2 R1/RG)(R3/R2); Vout = G (V2 - V1))
  R1 = 25000 ohm
  RG = 1000 ohm
  R2 = 10000 ohm
  R3 = 10000 ohm
  V1 = 1 V
  V2 = 1.01 V
  assumes: ideal op-amps, matched resistors
```

In the worked example, R1 = 25000 ohm, RG = 1000 ohm, R2 = 10000 ohm, R3 = 10000 ohm, V1 = 1 V, V2 = 1.01 V go through the equation above and give G = 51, G_stage1 = 51, Vout = 0.51 V. It applies only when ideal op-amps, matched resistors: outside that it says what is wrong instead of returning a number.

### Oscillators (9)

<a id="elx_555_astable"></a>

#### 555 timer astable: frequency and duty cycle — `elx_555_astable`

The high and low times, period, frequency and duty cycle of a 555 timer in astable mode with resistors RA and RB and timing capacitor C.

$$
t_H = \ln 2\,(R_A + R_B)C,\quad t_L = \ln 2\,R_BC,\quad f = \frac{1}{\ln 2\,(R_A + 2R_B)C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `RA` | in | resistor from Vcc to the discharge pin | ohm |
| `RB` | in | resistor from discharge to threshold/trigger | ohm |
| `C` | in | timing capacitor | F |
| `t_high` | out | output high time | s |
| `t_low` | out | output low time | s |
| `T` | out | period | s |
| `f` | out | frequency | Hz |
| `duty` | out | duty cycle (fraction high) | — |

**Applies when:** thresholds Vcc/3 and 2Vcc/3.

```shell
$ electronics-engineering solve elx_555_astable "RA=1000 ohm" "RB=10000 ohm" "C=10 uF"
t_high = 0.07624618986 s
t_low = 0.06931471806 s
T = 0.1455609079 s
f = 6.869976385 Hz
duty = 0.5238095238
  by: 555 timer astable: frequency and duty cycle  (t_H = ln2 (RA + RB) C; t_L = ln2 RB C; f = 1/(t_H + t_L))
  RA = 1000 ohm
  RB = 10000 ohm
  C = 1e-05 F
  assumes: thresholds Vcc/3 and 2Vcc/3
```

In the worked example, RA = 1000 ohm, RB = 10000 ohm, C = 10 uF go through the equation above and give t_high = 0.07624618986 s, t_low = 0.06931471806 s, T = 0.1455609079 s, f = 6.869976385 Hz. It applies only when thresholds Vcc/3 and 2Vcc/3: outside that it says what is wrong instead of returning a number.

<a id="elx_555_monostable"></a>

#### 555 timer monostable pulse width — `elx_555_monostable`

The output pulse width T = ln 3 R C (about 1.1 R C) of a 555 timer in monostable (one-shot) mode.

$$
T = \ln 3\,RC \approx 1.1\,RC
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | timing resistor | ohm |
| `C` | in | timing capacitor | F |
| `T` | out | pulse width | s |

**Applies when:** threshold at 2Vcc/3; trigger pulse shorter than T.

```shell
$ electronics-engineering solve elx_555_monostable "R=100000 ohm" "C=10 uF"
T = 1.098612289 s
  by: 555 timer monostable pulse width  (T = ln 3 R C)
  R = 100000 ohm
  C = 1e-05 F
  assumes: threshold at 2Vcc/3; trigger pulse shorter than T
```

In the worked example, R = 100000 ohm, C = 10 uF go through the equation above and give T = 1.098612289 s. It applies only when threshold at 2Vcc/3; trigger pulse shorter than T: outside that it says what is wrong instead of returning a number.

<a id="elx_colpitts_oscillator"></a>

#### Colpitts oscillator frequency and feedback ratio — `elx_colpitts_oscillator`

The oscillation frequency of a Colpitts oscillator from its inductor and the series combination of its two tank capacitors, the feedback fraction C1/C2 and the minimum amplifier gain C2/C1.

$$
f_0 = \frac{1}{2\pi\sqrt{L\,\frac{C_1C_2}{C_1+C_2}}},\quad A_{min} = \frac{C_2}{C_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | tank inductance | H |
| `C1` | in | capacitor across the amplifier output | F |
| `C2` | in | capacitor across the amplifier input (feedback side) | F |
| `C_eq` | out | series equivalent capacitance | F |
| `f0` | out | oscillation frequency | Hz |
| `beta` | out | feedback fraction C1/C2 | — |
| `A_min` | out | minimum amplifier gain C2/C1 | — |

**Applies when:** lossless tank, amplifier does not load the tank.

```shell
$ electronics-engineering solve elx_colpitts_oscillator "L=100 uH" "C1=1 nF" "C2=10 nF"
C_eq = 9.090909091e-10 F
f0 = 527857.2298 Hz
beta = 0.1
A_min = 10
  by: Colpitts oscillator frequency and feedback ratio  (C_eq = C1 C2/(C1 + C2); f0 = 1/(2 pi sqrt(L C_eq)); beta = C1/C2)
  L = 0.0001 H
  C1 = 1e-09 F
  C2 = 1e-08 F
  assumes: lossless tank, amplifier does not load the tank
```

In the worked example, L = 100 uH, C1 = 1 nF, C2 = 10 nF go through the equation above and give C_eq = 9.090909091e-10 F, f0 = 527857.2298 Hz, beta = 0.1, A_min = 10. It applies only when lossless tank, amplifier does not load the tank: outside that it says what is wrong instead of returning a number.

<a id="elx_hartley_oscillator"></a>

#### Hartley oscillator frequency — `elx_hartley_oscillator`

The oscillation frequency of a Hartley oscillator with a tapped (or two coupled) inductors and a tank capacitor, and the gain ratio (L1 + M)/(L2 + M).

$$
f_0 = \frac{1}{2\pi\sqrt{(L_1 + L_2 + 2M)C}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L1` | in | inductance of the output-side coil | H |
| `L2` | in | inductance of the feedback-side coil | H |
| `M` | in | mutual inductance (0 for uncoupled coils) | H |
| `C` | in | tank capacitance | F |
| `L_eq` | out | total inductance L1 + L2 + 2M | H |
| `f0` | out | oscillation frequency | Hz |
| `A_min` | out | minimum amplifier gain (L1 + M)/(L2 + M) | — |

**Applies when:** series-aiding coils, lossless tank; M <= sqrt(L1 L2).

```shell
$ electronics-engineering solve elx_hartley_oscillator "L1=100 uH" "L2=20 uH" "M=0 uH" "C=1 nF"
L_eq = 0.00012 H
f0 = 459440.7462 Hz
A_min = 5
  by: Hartley oscillator frequency  (L_eq = L1 + L2 + 2M; f0 = 1/(2 pi sqrt(L_eq C)))
  L1 = 0.0001 H
  L2 = 2e-05 H
  M = 0 H
  C = 1e-09 F
  assumes: series-aiding coils, lossless tank; M <= sqrt(L1 L2)
```

In the worked example, L1 = 100 uH, L2 = 20 uH, M = 0 uH, C = 1 nF go through the equation above and give L_eq = 0.00012 H, f0 = 459440.7462 Hz, A_min = 5. It applies only when series-aiding coils, lossless tank; M <= sqrt(L1 L2): outside that it says what is wrong instead of returning a number.

<a id="elx_opamp_relaxation_oscillator"></a>

#### Op-amp relaxation (astable) oscillator — `elx_opamp_relaxation_oscillator`

The period and frequency of a square-wave op-amp astable: an RC timing network on the inverting input and positive feedback R1/(R1 + R2) to the non-inverting input, symmetric supplies.

$$
T = 2RC\ln\frac{1+\beta}{1-\beta},\quad \beta = \frac{R_1}{R_1+R_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | timing resistor (output to inverting input) | ohm |
| `C` | in | timing capacitor | F |
| `R1` | in | resistor from the + input to ground | ohm |
| `R2` | in | resistor from the output to the + input | ohm |
| `beta` | out | feedback fraction R1/(R1 + R2) | — |
| `T` | out | period | s |
| `f` | out | frequency | Hz |

**Applies when:** symmetric output levels; ideal comparator action.

```shell
$ electronics-engineering solve elx_opamp_relaxation_oscillator "R=10000 ohm" "C=100 nF" "R1=10000 ohm" "R2=10000 ohm"
beta = 0.5
T = 0.002197224577 s
f = 455.1196133 Hz
  by: Op-amp relaxation (astable) oscillator  (T = 2 R C ln((1 + beta)/(1 - beta)) = 2 R C ln(1 + 2 R1/R2))
  R = 10000 ohm
  C = 1e-07 F
  R1 = 10000 ohm
  R2 = 10000 ohm
  assumes: symmetric output levels; ideal comparator action
```

In the worked example, R = 10000 ohm, C = 100 nF, R1 = 10000 ohm, R2 = 10000 ohm go through the equation above and give beta = 0.5, T = 0.002197224577 s, f = 455.1196133 Hz. It applies only when symmetric output levels; ideal comparator action: outside that it says what is wrong instead of returning a number.

<a id="elx_crystal_oscillator"></a>

#### Quartz crystal: series and parallel resonance, Q — `elx_crystal_oscillator`

The series and parallel (anti-) resonant frequencies, their spacing and the quality factor of a quartz crystal from its Butterworth-Van Dyke equivalent circuit.

$$
f_s = \frac{1}{2\pi\sqrt{L_mC_m}},\quad f_p = f_s\sqrt{1 + \frac{C_m}{C_0}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Lm` | in | motional inductance | H |
| `Cm` | in | motional capacitance | F |
| `Rm` | in | motional resistance | ohm |
| `C0` | in | shunt (holder) capacitance | F |
| `fs` | out | series resonance | Hz |
| `fp` | out | parallel resonance | Hz |
| `delta_f` | out | fp - fs | Hz |
| `Q` | out | quality factor | — |

**Applies when:** resonances of the lossless circuit; Q from Rm.

```shell
$ electronics-engineering solve elx_crystal_oscillator "Lm=0.05 H" "Cm=20 fF" "Rm=20 ohm" "C0=5 pF"
fs = 5032921.21 Hz
fp = 5042977.007 Hz
delta_f = 10055.79666 Hz
Q = 79056.9415
  by: Quartz crystal: series and parallel resonance, Q  (fs = 1/(2 pi sqrt(Lm Cm)); fp = fs sqrt(1 + Cm/C0); Q = sqrt(Lm/Cm)/Rm)
  Lm = 0.05 H
  Cm = 2e-14 F
  Rm = 20 ohm
  C0 = 5e-12 F
  assumes: resonances of the lossless circuit; Q from Rm
```

In the worked example, Lm = 0.05 H, Cm = 20 fF, Rm = 20 ohm, C0 = 5 pF go through the equation above and give fs = 5032921.21 Hz, fp = 5042977.007 Hz, delta_f = 10055.79666 Hz, Q = 79056.9415. It applies only when resonances of the lossless circuit; Q from Rm: outside that it says what is wrong instead of returning a number.

<a id="elx_rc_phase_shift_oscillator"></a>

#### RC phase-shift oscillator frequency and gain — `elx_rc_phase_shift_oscillator`

The oscillation frequency of a three-section RC phase-shift oscillator (high-pass ladder: series C, shunt R; or low-pass ladder) and the amplifier gain of 29 it needs.

$$
f_0 = \frac{1}{2\pi\sqrt{6}\,RC},\quad |A| \ge 29
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | resistor of each section | ohm |
| `C` | in | capacitor of each section | F |
| `ladder` | in | highpass (series C, shunt R) or lowpass (series R, shunt C) | — |
| `f0` | out | oscillation frequency | Hz |
| `A_min` | out | minimum inverting amplifier gain | — |
| `beta` | out | feedback network attenuation at f0 | — |

**Applies when:** unloaded ladder (driven by a voltage source, feeding a high impedance).

```shell
$ electronics-engineering solve elx_rc_phase_shift_oscillator "R=10000 ohm" "C=10 nF" ladder=high-pass
f0 = 649.7473344 Hz
A_min = 29
beta = 0.03448275862
  by: RC phase-shift oscillator frequency and gain  (high-pass ladder f = 1/(2 pi sqrt(6) R C); low-pass ladder f = sqrt(6)/(2 pi R C); |beta| = 1/29)
  R = 10000 ohm
  C = 1e-08 F
  ladder = high-pass
  assumes: unloaded ladder (driven by a voltage source, feeding a high impedance)
```

In the worked example, R = 10000 ohm, C = 10 nF, ladder = high-pass go through the equation above and give f0 = 649.7473344 Hz, A_min = 29, beta = 0.03448275862. It applies only when unloaded ladder (driven by a voltage source, feeding a high impedance): outside that it says what is wrong instead of returning a number.

<a id="elx_pll_loop_dynamics"></a>

#### Type-2 PLL with active PI loop filter: natural frequency and damping — `elx_pll_loop_dynamics`

The natural frequency and damping factor of a second-order (type-2) phase-locked loop with phase-detector gain Kd, VCO gain Ko and an active PI loop filter F(s) = (1 + s tau2)/(s tau1).

$$
\omega_n = \sqrt{\frac{K_dK_o}{\tau_1}},\quad \zeta = \frac{\tau_2\omega_n}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Kd` | in | phase-detector gain in V/rad | — |
| `Ko` | in | VCO gain in rad/(s V) | — |
| `tau1` | in | integrator time constant R1 C | s |
| `tau2` | in | zero time constant R2 C | s |
| `wn` | out | natural frequency | rad/s |
| `fn` | out | natural frequency in Hz | Hz |
| `zeta` | out | damping factor | — |

**Applies when:** linear phase model, ideal integrator.

```shell
$ electronics-engineering solve elx_pll_loop_dynamics Kd=0.5 Ko=20000 "tau1=10 ms" "tau2=1 ms"
wn = 1000 rad/s
fn = 159.1549431 Hz
zeta = 0.5
  by: Type-2 PLL with active PI loop filter: natural frequency and damping  (H(s) = K (1 + s tau2)/tau1 / (s^2 + s K tau2/tau1 + K/tau1), K = Kd Ko; wn = sqrt(K/tau1); zeta = tau2 wn/2)
  Kd = 0.5
  Ko = 20000
  tau1 = 0.01 s
  tau2 = 0.001 s
  assumes: linear phase model, ideal integrator
```

In the worked example, Kd = 0.5, Ko = 20000, tau1 = 10 ms, tau2 = 1 ms go through the equation above and give wn = 1000 rad/s, fn = 159.1549431 Hz, zeta = 0.5. It applies only when linear phase model, ideal integrator: outside that it says what is wrong instead of returning a number.

<a id="elx_wien_bridge_oscillator"></a>

#### Wien-bridge oscillator frequency and required gain — `elx_wien_bridge_oscillator`

The oscillation frequency of a Wien-bridge oscillator (series R1 C1, parallel R2 C2) and the non-inverting gain it needs (3 for equal components).

$$
f_0 = \frac{1}{2\pi\sqrt{R_1R_2C_1C_2}},\quad A = 1 + \frac{R_1}{R_2} + \frac{C_2}{C_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R1` | in | series-arm resistor | ohm |
| `C1` | in | series-arm capacitor | F |
| `R2` | in | parallel-arm resistor | ohm |
| `C2` | in | parallel-arm capacitor | F |
| `f0` | out | oscillation frequency | Hz |
| `beta` | out | feedback fraction at f0 | — |
| `A_min` | out | required amplifier gain | — |
| `Rf_over_Rg` | out | gain resistor ratio | — |

**Applies when:** ideal amplifier.

```shell
$ electronics-engineering solve elx_wien_bridge_oscillator "R1=10000 ohm" "C1=16 nF" "R2=10000 ohm" "C2=16 nF"
f0 = 994.7183943 Hz
beta = 0.3333333333
A_min = 3
Rf_over_Rg = 2
  by: Wien-bridge oscillator frequency and required gain  (f0 = 1/(2 pi sqrt(R1 R2 C1 C2)); A = 1 + R1/R2 + C2/C1)
  R1 = 10000 ohm
  C1 = 1.6e-08 F
  R2 = 10000 ohm
  C2 = 1.6e-08 F
  assumes: ideal amplifier
```

In the worked example, R1 = 10000 ohm, C1 = 16 nF, R2 = 10000 ohm, C2 = 16 nF go through the equation above and give f0 = 994.7183943 Hz, beta = 0.3333333333, A_min = 3, Rf_over_Rg = 2. It applies only when ideal amplifier: outside that it says what is wrong instead of returning a number.

### Power supplies (5)

<a id="elx_adjustable_regulator_output"></a>

#### Adjustable three-terminal regulator output voltage — `elx_adjustable_regulator_output`

The output voltage of an adjustable three-terminal regulator (LM317 type: reference between OUT and ADJ, R1 from OUT to ADJ, R2 from ADJ to ground), including the adjust-pin current.

$$
V_{out} = V_{ref}\left(1 + \frac{R_2}{R_1}\right) + I_{adj}R_2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R1` | in | resistor from OUT to ADJ | ohm |
| `R2` | in | resistor from ADJ to ground | ohm |
| `Vref` | in | internal reference (1.25 V for the LM317 type) | V |
| `Iadj` | in | adjust-pin current (default 0) (optional) | A |
| `Vout` | out | output voltage | V |
| `I_R1` | out | current through R1 | A |

**Applies when:** regulator in regulation (input high enough).

```shell
$ electronics-engineering solve elx_adjustable_regulator_output "R1=240 ohm" "R2=720 ohm" "Vref=1.25 V"
Vout = 5 V
I_R1 = 0.005208333333 A
  by: Adjustable three-terminal regulator output voltage  (Vout = Vref (1 + R2/R1) + Iadj R2)
  R1 = 240 ohm
  R2 = 720 ohm
  Vref = 1.25 V
  assumes: regulator in regulation (input high enough)
```

In the worked example, R1 = 240 ohm, R2 = 720 ohm, Vref = 1.25 V go through the equation above and give Vout = 5 V, I_R1 = 0.005208333333 A. It applies only when regulator in regulation (input high enough): outside that it says what is wrong instead of returning a number.

<a id="elx_heat_sink_temperatures"></a>

#### Heat-sink thermal path: junction, case and sink temperatures — `elx_heat_sink_temperatures`

The junction, case and heat-sink temperatures of a power device dissipating P through junction-to-case, case-to-sink (interface) and sink-to-ambient thermal resistances.

$$
T_j = T_a + P\,(\theta_{jc} + \theta_{cs} + \theta_{sa})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | power dissipated | W |
| `Ta` | in | ambient temperature | K |
| `theta_jc` | in | junction-to-case thermal resistance | K/W |
| `theta_cs` | in | case-to-sink thermal resistance | K/W |
| `theta_sa` | in | sink-to-ambient thermal resistance | K/W |
| `Tj` | out | junction temperature | K |
| `Tc` | out | case temperature | K |
| `Ts` | out | sink temperature | K |
| `Tj_C` | out | junction temperature in degC | — |
| `theta_ja` | out | total junction-to-ambient resistance | K/W |

**Applies when:** steady state, series thermal path.

```shell
$ electronics-engineering solve elx_heat_sink_temperatures "P=15 W" "Ta=25 degC" "theta_jc=1.5 K/W" "theta_cs=0.5 K/W" "theta_sa=3 K/W"
Tj = 373.15 K
Tc = 350.65 K
Ts = 343.15 K
Tj_C = 100
theta_ja = 5 K/W
  by: Heat-sink thermal path: junction, case and sink temperatures  (Tj = Ta + P (theta_jc + theta_cs + theta_sa))
  P = 15 W
  Ta = 298.15 K
  theta_jc = 1.5 K/W
  theta_cs = 0.5 K/W
  theta_sa = 3 K/W
  assumes: steady state, series thermal path
```

In the worked example, P = 15 W, Ta = 25 degC, theta_jc = 1.5 K/W, theta_cs = 0.5 K/W, theta_sa = 3 K/W go through the equation above and give Tj = 373.15 K, Tc = 350.65 K, Ts = 343.15 K, Tj_C = 100. It applies only when steady state, series thermal path: outside that it says what is wrong instead of returning a number.

<a id="elx_linear_regulator_efficiency"></a>

#### Linear regulator dissipation and efficiency — `elx_linear_regulator_efficiency`

The input and output power, the dissipation in the pass element and the efficiency of a linear (series) regulator including its quiescent current.

$$
P_D = (V_{in} - V_{out})I_L + V_{in}I_q,\quad \eta = \frac{V_{out}I_L}{V_{in}(I_L + I_q)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | regulated output voltage | V |
| `Iload` | in | load current | A |
| `Iq` | in | quiescent (ground-pin) current | A |
| `P_in` | out | input power | W |
| `P_out` | out | output power | W |
| `P_diss` | out | dissipation in the regulator | W |
| `efficiency` | out | efficiency | — |
| `efficiency_pct` | out | efficiency in per cent | — |

**Applies when:** Vin > Vout.

```shell
$ electronics-engineering solve elx_linear_regulator_efficiency "Vin=12 V" "Vout=5 V" "Iload=500 mA" "Iq=5 mA"
P_in = 6.06 W
P_out = 2.5 W
P_diss = 3.56 W
efficiency = 0.4125412541
efficiency_pct = 41.25412541
  by: Linear regulator dissipation and efficiency  (P_in = Vin (IL + Iq); P_out = Vout IL; P_diss = P_in - P_out; eta = P_out/P_in)
  Vin = 12 V
  Vout = 5 V
  Iload = 0.5 A
  Iq = 0.005 A
  assumes: Vin > Vout
```

In the worked example, Vin = 12 V, Vout = 5 V, Iload = 500 mA, Iq = 5 mA go through the equation above and give P_in = 6.06 W, P_out = 2.5 W, P_diss = 3.56 W, efficiency = 0.4125412541. It applies only when Vin > Vout: outside that it says what is wrong instead of returning a number.

<a id="elx_heat_sink_required"></a>

#### Required heat-sink thermal resistance — `elx_heat_sink_required`

The largest sink-to-ambient thermal resistance (and total junction-to-ambient resistance) that keeps a device dissipating P below its maximum junction temperature.

$$
\theta_{sa} \le \frac{T_{j,max} - T_a}{P} - \theta_{jc} - \theta_{cs}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | power dissipated | W |
| `Ta` | in | ambient temperature | K |
| `Tj_max` | in | maximum junction temperature | K |
| `theta_jc` | in | junction-to-case thermal resistance | K/W |
| `theta_cs` | in | case-to-sink thermal resistance | K/W |
| `theta_ja_max` | out | largest junction-to-ambient resistance | K/W |
| `theta_sa_max` | out | largest sink-to-ambient resistance | K/W |

**Applies when:** a positive result (otherwise no heat sink suffices).

```shell
$ electronics-engineering solve elx_heat_sink_required "P=20 W" "Ta=40 degC" "Tj_max=150 degC" "theta_jc=1 K/W" "theta_cs=0.5 K/W"
theta_ja_max = 5.5 K/W
theta_sa_max = 4 K/W
  by: Required heat-sink thermal resistance  (theta_sa = (Tj,max - Ta)/P - theta_jc - theta_cs)
  P = 20 W
  Ta = 313.15 K
  Tj_max = 423.15 K
  theta_jc = 1 K/W
  theta_cs = 0.5 K/W
  assumes: a positive result (otherwise no heat sink suffices)
```

In the worked example, P = 20 W, Ta = 40 degC, Tj_max = 150 degC, theta_jc = 1 K/W, theta_cs = 0.5 K/W go through the equation above and give theta_ja_max = 5.5 K/W, theta_sa_max = 4 K/W. It applies only when a positive result (otherwise no heat sink suffices): outside that it says what is wrong instead of returning a number.

<a id="elx_series_pass_regulator"></a>

#### Zener-referenced series-pass (emitter-follower) regulator — `elx_series_pass_regulator`

The output voltage, load, base, Zener and resistor currents and the pass-transistor dissipation of a regulator in which a Zener diode biased through Rs sets the base of an emitter-follower pass transistor.

$$
V_{out} = V_Z - V_{BE},\quad I_Z = \frac{V_{in} - V_Z}{R_S} - \frac{I_L}{\beta+1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | unregulated input | V |
| `Vz` | in | Zener voltage | V |
| `Rs` | in | Zener bias resistor | ohm |
| `RL` | in | load resistance | ohm |
| `beta` | in | current gain of the pass transistor | — |
| `VBE` | in | base-emitter voltage | V |
| `Vout` | out | output voltage Vz - VBE | V |
| `I_L` | out | load current | A |
| `I_B` | out | base current | A |
| `I_z` | out | Zener current | A |
| `I_Rs` | out | current in Rs | A |
| `P_Q` | out | pass-transistor dissipation | W |

**Applies when:** Vin > Vz > VBE; the Zener stays in breakdown (Iz > 0).

```shell
$ electronics-engineering solve elx_series_pass_regulator "Vin=15 V" "Vz=10 V" "Rs=1000 ohm" "RL=100 ohm" beta=100 "VBE=0.7 V"
Vout = 9.3 V
I_L = 0.093 A
I_B = 0.0009207920792 A
I_z = 0.004079207921 A
I_Rs = 0.005 A
P_Q = 0.5248514851 W
  by: Zener-referenced series-pass (emitter-follower) regulator  (Vout = Vz - VBE; IB = IL/(beta + 1); Iz = (Vin - Vz)/Rs - IB; P = (Vin - Vout) IC)
  Vin = 15 V
  Vz = 10 V
  Rs = 1000 ohm
  RL = 100 ohm
  beta = 100
  VBE = 0.7 V
  assumes: Vin > Vz > VBE; the Zener stays in breakdown (Iz > 0)
```

In the worked example, Vin = 15 V, Vz = 10 V, Rs = 1000 ohm, RL = 100 ohm, beta = 100, VBE = 0.7 V go through the equation above and give Vout = 9.3 V, I_L = 0.093 A, I_B = 0.0009207920792 A, I_z = 0.004079207921 A. It applies only when Vin > Vz > VBE; the Zener stays in breakdown (Iz > 0): outside that it says what is wrong instead of returning a number.

### Semiconductors (9)

<a id="elx_builtin_potential"></a>

#### Built-in potential of a pn junction — `elx_builtin_potential`

The built-in (contact) potential of an abrupt pn junction from the doping on each side, the intrinsic concentration and the temperature.

$$
V_{bi} = \frac{kT}{q}\ln\frac{N_AN_D}{n_i^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `NA` | in | acceptor concentration on the p side | 1/m^3 |
| `ND` | in | donor concentration on the n side | 1/m^3 |
| `ni` | in | intrinsic carrier concentration | 1/m^3 |
| `T` | in | absolute temperature | K |
| `Vbi` | out | built-in potential | V |

**Applies when:** NA, ND >= 100 ni; non-degenerate, complete ionisation.

```shell
$ electronics-engineering solve elx_builtin_potential "NA=1e+17 cm^-3" "ND=1e+16 cm^-3" "ni=1.5e+10 cm^-3" "T=300 K"
Vbi = 0.7528794135 V
  by: Built-in potential of a pn junction  (Vbi = (kT/q) ln(NA ND / ni^2))
  NA = 1e+23 1/m^3
  ND = 1e+22 1/m^3
  ni = 1.5e+16 1/m^3
  T = 300 K
  assumes: NA, ND >= 100 ni; non-degenerate, complete ionisation
```

In the worked example, NA = 1e+17 cm^-3, ND = 1e+16 cm^-3, ni = 1.5e+10 cm^-3, T = 300 K go through the equation above and give Vbi = 0.7528794135 V. It applies only when NA, ND >= 100 ni; non-degenerate, complete ionisation: outside that it says what is wrong instead of returning a number.

<a id="elx_semiconductor_conductivity"></a>

#### Conductivity and resistivity of a semiconductor — `elx_semiconductor_conductivity`

The conductivity sigma = q (n mu_n + p mu_p) and the resistivity of a semiconductor from its electron and hole concentrations and mobilities.

$$
\sigma = q\,(n\mu_n + p\mu_p),\quad \rho = 1/\sigma
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | electron concentration | 1/m^3 |
| `p` | in | hole concentration | 1/m^3 |
| `mu_n` | in | electron mobility | m^2/(V*s) |
| `mu_p` | in | hole mobility | m^2/(V*s) |
| `sigma` | out | conductivity | 1/(ohm*m) |
| `rho` | out | resistivity | ohm*m |
| `rho_ohm_cm` | out | resistivity in ohm cm | — |

**Applies when:** n, p >= 0 (not both zero); mobilities > 0.

```shell
$ electronics-engineering solve elx_semiconductor_conductivity "n=1e+16 cm^-3" "p=22500 cm^-3" "mu_n=1350 cm^2/(V*s)" "mu_p=480 cm^2/(V*s)"
sigma = 216.2938456 1/(ohm*m)
rho = 0.004623340055 ohm*m
rho_ohm_cm = 0.4623340055
  by: Conductivity and resistivity of a semiconductor  (sigma = q (n mu_n + p mu_p); rho = 1/sigma)
  n = 1e+22 1/m^3
  p = 2.25e+10 1/m^3
  mu_n = 0.135 m^2/(V*s)
  mu_p = 0.048 m^2/(V*s)
  assumes: n, p >= 0 (not both zero); mobilities > 0
```

In the worked example, n = 1e+16 cm^-3, p = 22500 cm^-3, mu_n = 1350 cm^2/(V*s), mu_p = 480 cm^2/(V*s) go through the equation above and give sigma = 216.2938456 1/(ohm*m), rho = 0.004623340055 ohm*m, rho_ohm_cm = 0.4623340055. It applies only when n, p >= 0 (not both zero); mobilities > 0: outside that it says what is wrong instead of returning a number.

<a id="elx_depletion_width"></a>

#### Depletion width, field and capacitance of an abrupt pn junction — `elx_depletion_width`

The depletion-region width (and its parts on each side), the peak electric field and the depletion capacitance per area of an abrupt pn junction at a reverse bias (negative VR for a forward bias below Vbi).

$$
W = \sqrt{\frac{2\varepsilon(V_{bi}+V_R)}{q}\left(\frac{1}{N_A}+\frac{1}{N_D}\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `NA` | in | acceptor concentration (p side) | 1/m^3 |
| `ND` | in | donor concentration (n side) | 1/m^3 |
| `Vbi` | in | built-in potential | V |
| `VR` | in | reverse bias (negative for forward) | V |
| `eps_r` | in | relative permittivity of the semiconductor (11.7 for silicon) | — |
| `W` | out | depletion width | m |
| `W_um` | out | depletion width in micrometres | — |
| `xn` | out | width on the n side | m |
| `xp` | out | width on the p side | m |
| `Emax` | out | peak electric field (V/m) | N/C |
| `Cj_area` | out | depletion capacitance per area | F/m^2 |

**Applies when:** Vbi + VR > 0; abrupt junction, depletion approximation.

```shell
$ electronics-engineering solve elx_depletion_width "NA=1e+17 cm^-3" "ND=1e+16 cm^-3" "Vbi=0.81 V" "VR=5 V" eps_r=11.7
W = 9.090996814e-07 m
W_um = 0.9090996814
xn = 8.264542558e-07 m
xp = 8.264542558e-08 m
Emax = 12781876.66 N/C
Cj_area = 0.0001139522976 F/m^2
  by: Depletion width, field and capacitance of an abrupt pn junction  (W = sqrt(2 eps (Vbi + VR)/q (1/NA + 1/ND)); xn = W NA/(NA+ND); Emax = q ND xn/eps; C' = eps/W)
  NA = 1e+23 1/m^3
  ND = 1e+22 1/m^3
  Vbi = 0.81 V
  VR = 5 V
  eps_r = 11.7
  assumes: Vbi + VR > 0; abrupt junction, depletion approximation
```

In the worked example, NA = 1e+17 cm^-3, ND = 1e+16 cm^-3, Vbi = 0.81 V, VR = 5 V, eps_r = 11.7 go through the equation above and give W = 9.090996814e-07 m, W_um = 0.9090996814, xn = 8.264542558e-07 m, xp = 8.264542558e-08 m. It applies only when Vbi + VR > 0; abrupt junction, depletion approximation: outside that it says what is wrong instead of returning a number.

<a id="elx_drift_current"></a>

#### Drift velocity and drift current in a semiconductor bar — `elx_drift_current`

The electric field, the carrier drift velocity, the drift current density and the current when a voltage is applied along a uniformly doped bar.

$$
J = q\,n\,\mu\,E,\quad E = V/L
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | concentration of the carriers that drift | 1/m^3 |
| `mu` | in | their mobility | m^2/(V*s) |
| `V` | in | voltage applied along the bar | V |
| `L` | in | bar length | m |
| `A` | in | cross-sectional area | m^2 |
| `E` | out | electric field (V/m) | N/C |
| `v_d` | out | drift velocity | m/s |
| `J` | out | drift current density | A/m^2 |
| `I` | out | current | A |

**Applies when:** n, mu, L, A > 0; low field (constant mobility).

```shell
$ electronics-engineering solve elx_drift_current "n=1e+16 cm^-3" "mu=1350 cm^2/(V*s)" "V=5 V" "L=0.001 m" "A=1e-08 m^2"
E = 5000 N/C
v_d = 675 m/s
J = 1081469.228 A/m^2
I = 0.01081469228 A
  by: Drift velocity and drift current in a semiconductor bar  (E = V/L; v_d = mu E; J = q n v_d; I = J A)
  n = 1e+22 1/m^3
  mu = 0.135 m^2/(V*s)
  V = 5 V
  L = 0.001 m
  A = 1e-08 m^2
  assumes: n, mu, L, A > 0; low field (constant mobility)
```

In the worked example, n = 1e+16 cm^-3, mu = 1350 cm^2/(V*s), V = 5 V, L = 0.001 m, A = 1e-08 m^2 go through the equation above and give E = 5000 N/C, v_d = 675 m/s, J = 1081469.228 A/m^2, I = 0.01081469228 A. It applies only when n, mu, L, A > 0; low field (constant mobility): outside that it says what is wrong instead of returning a number.

<a id="elx_einstein_diffusion"></a>

#### Einstein relation and diffusion length — `elx_einstein_diffusion`

The diffusion coefficient D = mu kT/q of carriers from their mobility (Einstein relation) and the diffusion length L = sqrt(D tau) from the minority-carrier lifetime.

$$
D = \mu\frac{kT}{q},\quad L = \sqrt{D\tau}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu` | in | carrier mobility | m^2/(V*s) |
| `T` | in | absolute temperature | K |
| `tau` | in | minority-carrier lifetime | s |
| `D` | out | diffusion coefficient | m^2/s |
| `D_cm2` | out | D in cm^2/s | — |
| `L_diff` | out | diffusion length | m |
| `L_um` | out | diffusion length in micrometres | — |

**Applies when:** mu, T, tau > 0; non-degenerate material.

```shell
$ electronics-engineering solve elx_einstein_diffusion "mu=1350 cm^2/(V*s)" "T=300 K" "tau=1 us"
D = 0.003490019971 m^2/s
D_cm2 = 34.90019971
L_diff = 5.907639098e-05 m
L_um = 59.07639098
  by: Einstein relation and diffusion length  (D = mu kT/q; L = sqrt(D tau))
  mu = 0.135 m^2/(V*s)
  T = 300 K
  tau = 1e-06 s
  assumes: mu, T, tau > 0; non-degenerate material
```

In the worked example, mu = 1350 cm^2/(V*s), T = 300 K, tau = 1 us go through the equation above and give D = 0.003490019971 m^2/s, D_cm2 = 34.90019971, L_diff = 5.907639098e-05 m, L_um = 59.07639098. It applies only when mu, T, tau > 0; non-degenerate material: outside that it says what is wrong instead of returning a number.

<a id="elx_doped_carrier_concentrations"></a>

#### Electron and hole concentrations of doped (compensated) material — `elx_doped_carrier_concentrations`

The equilibrium electron and hole concentrations of a semiconductor with donors and acceptors, from charge neutrality and the mass-action law (exact, so it also holds for light or compensated doping).

$$
n = \frac{N_D - N_A}{2} + \sqrt{\left(\frac{N_D - N_A}{2}\right)^2 + n_i^2},\quad p = \frac{n_i^2}{n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ND` | in | donor concentration | 1/m^3 |
| `NA` | in | acceptor concentration | 1/m^3 |
| `ni` | in | intrinsic carrier concentration | 1/m^3 |
| `n` | out | electron concentration | 1/m^3 |
| `p` | out | hole concentration | 1/m^3 |
| `n_cm3` | out | n in cm^-3 | — |
| `p_cm3` | out | p in cm^-3 | — |
| `type` | out | n-type, p-type or intrinsic | — |

**Applies when:** ND, NA >= 0; ni > 0; complete ionisation, non-degenerate.

```shell
$ electronics-engineering solve elx_doped_carrier_concentrations "ND=1e+16 cm^-3" "NA=0 cm^-3" "ni=1.5e+10 cm^-3"
n = 1e+22 1/m^3
p = 2.25e+10 1/m^3
n_cm3 = 1e+16
p_cm3 = 22500
type = n-type
  by: Electron and hole concentrations of doped (compensated) material  (n - p = ND - NA and n p = ni^2: majority = |ND-NA|/2 + sqrt((ND-NA)^2/4 + ni^2), minority = ni^2/majority)
  ND = 1e+22 1/m^3
  NA = 0 1/m^3
  ni = 1.5e+16 1/m^3
  assumes: ND, NA >= 0; ni > 0; complete ionisation, non-degenerate
```

In the worked example, ND = 1e+16 cm^-3, NA = 0 cm^-3, ni = 1.5e+10 cm^-3 go through the equation above and give n = 1e+22 1/m^3, p = 2.25e+10 1/m^3, n_cm3 = 1e+16, p_cm3 = 22500. It applies only when ND, NA >= 0; ni > 0; complete ionisation, non-degenerate: outside that it says what is wrong instead of returning a number.

<a id="elx_intrinsic_carrier_concentration"></a>

#### Intrinsic carrier concentration — `elx_intrinsic_carrier_concentration`

The intrinsic carrier concentration ni = sqrt(Nc Nv) exp(-Eg/2kT) at a temperature, from the effective densities of states at 300 K (scaled as T^1.5) and the band gap; also the intrinsic level's offset from midgap.

$$
n_i = \sqrt{N_cN_v}\,e^{-E_g/2kT},\quad N_{c,v}(T) = N_{c,v}(300)\left(\tfrac{T}{300}\right)^{3/2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Nc` | in | effective density of states in the conduction band at 300 K | 1/m^3 |
| `Nv` | in | effective density of states in the valence band at 300 K | 1/m^3 |
| `Eg` | in | band gap energy | J |
| `T` | in | absolute temperature | K |
| `ni` | out | intrinsic carrier concentration | 1/m^3 |
| `ni_cm3` | out | ni in cm^-3 | — |
| `Ei_minus_midgap_eV` | out | intrinsic Fermi level above midgap, eV | — |
| `Nc_T` | out | Nc at T | 1/m^3 |
| `Nv_T` | out | Nv at T | 1/m^3 |

**Applies when:** Nc, Nv, Eg, T > 0; non-degenerate statistics, band gap independent of T.

```shell
$ electronics-engineering solve elx_intrinsic_carrier_concentration "Nc=2.8e+19 cm^-3" "Nv=1.04e+19 cm^-3" "Eg=1.12 eV" "T=300 K"
ni = 6.67589872e+15 1/m^3
ni_cm3 = 6675898720
Ei_minus_midgap_eV = -0.01280189354
Nc_T = 2.8e+25 1/m^3
Nv_T = 1.04e+25 1/m^3
  by: Intrinsic carrier concentration  (Nc(T) = Nc (T/300)^1.5, Nv likewise; ni = sqrt(Nc Nv) exp(-Eg/(2kT)); Ei - Emid = (kT/2) ln(Nv/Nc))
  Nc = 2.8e+25 1/m^3
  Nv = 1.04e+25 1/m^3
  Eg = 1.79443783e-19 J
  T = 300 K
  assumes: Nc, Nv, Eg, T > 0; non-degenerate statistics, band gap independent of T
```

In the worked example, Nc = 2.8e+19 cm^-3, Nv = 1.04e+19 cm^-3, Eg = 1.12 eV, T = 300 K go through the equation above and give ni = 6.67589872e+15 1/m^3, ni_cm3 = 6675898720, Ei_minus_midgap_eV = -0.01280189354, Nc_T = 2.8e+25 1/m^3. It applies only when Nc, Nv, Eg, T > 0; non-degenerate statistics, band gap independent of T: outside that it says what is wrong instead of returning a number.

<a id="elx_junction_capacitance"></a>

#### Junction (depletion) capacitance versus bias — `elx_junction_capacitance`

The depletion capacitance of a pn junction at a reverse bias from its zero-bias value, the built-in potential and the grading coefficient (1/2 abrupt, 1/3 linearly graded), as used for varactors.

$$
C_j = \frac{C_{j0}}{\left(1 + V_R/V_{bi}\right)^{m}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cj0` | in | zero-bias junction capacitance | F |
| `Vbi` | in | built-in potential | V |
| `VR` | in | reverse bias (negative for forward) | V |
| `m` | in | grading coefficient (0.5 abrupt, 0.33 linearly graded) | — |
| `Cj` | out | junction capacitance | F |
| `ratio` | out | Cj/Cj0 | — |

**Applies when:** 0 < m < 1; VR > -Vbi.

```shell
$ electronics-engineering solve elx_junction_capacitance "Cj0=2 pF" "Vbi=0.75 V" "VR=5 V" m=0.5
Cj = 7.223151185e-13 F
ratio = 0.3611575593
  by: Junction (depletion) capacitance versus bias  (Cj = Cj0 / (1 + VR/Vbi)^m)
  Cj0 = 2e-12 F
  Vbi = 0.75 V
  VR = 5 V
  m = 0.5
  assumes: 0 < m < 1; VR > -Vbi
```

In the worked example, Cj0 = 2 pF, Vbi = 0.75 V, VR = 5 V, m = 0.5 go through the equation above and give Cj = 7.223151185e-13 F, ratio = 0.3611575593. It applies only when 0 < m < 1; VR > -Vbi: outside that it says what is wrong instead of returning a number.

<a id="elx_thermal_voltage"></a>

#### Thermal voltage kT/q — `elx_thermal_voltage`

The thermal voltage VT = kT/q at an absolute temperature (about 25.85 mV at 300 K), the voltage scale of every diode and transistor equation.

$$
V_T = \frac{kT}{q}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | absolute temperature (degC is converted) | K |
| `VT` | out | thermal voltage | V |
| `VT_mV` | out | thermal voltage in millivolts | — |

**Applies when:** T > 0 K.

```shell
$ electronics-engineering solve elx_thermal_voltage "T=300 K"
VT = 0.02585199979 V
VT_mV = 25.85199979
  by: Thermal voltage kT/q  (VT = k T / q)
  T = 300 K
  assumes: T > 0 K
```

In the worked example, T = 300 K go through the equation above and give VT = 0.02585199979 V, VT_mV = 25.85199979. It applies only when T > 0 K: outside that it says what is wrong instead of returning a number.

### Timing (5)

<a id="elx_cmos_dynamic_power"></a>

#### CMOS dynamic (switching) power — `elx_cmos_dynamic_power`

The dynamic power alpha C VDD^2 f of CMOS logic, the energy drawn from the supply per 0 -> 1 transition, the energy stored on the load and the average supply current.

$$
P_{dyn} = \alpha\,C\,V_{DD}^2f
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha` | in | activity factor: probability of a 0 -> 1 transition per clock cycle | — |
| `C` | in | switched capacitance | F |
| `VDD` | in | supply voltage | V |
| `f` | in | clock frequency | Hz |
| `P` | out | dynamic power | W |
| `E_supply` | out | energy from the supply per transition, C VDD^2 | J |
| `E_stored` | out | energy stored on C, C VDD^2/2 | J |
| `I_avg` | out | average supply current | A |

**Applies when:** 0 <= alpha <= 1; full-swing switching, short-circuit and leakage power not included.

```shell
$ electronics-engineering solve elx_cmos_dynamic_power alpha=0.1 "C=1000 pF" "VDD=1 V" "f=1000000000 Hz"
P = 0.1 W
E_supply = 1e-09 J
E_stored = 5e-10 J
I_avg = 0.1 A
  by: CMOS dynamic (switching) power  (P = alpha C VDD^2 f)
  alpha = 0.1
  C = 1e-09 F
  VDD = 1 V
  f = 1000000000 Hz
  assumes: 0 <= alpha <= 1; full-swing switching, short-circuit and leakage power not included
```

In the worked example, alpha = 0.1, C = 1000 pF, VDD = 1 V, f = 1000000000 Hz go through the equation above and give P = 0.1 W, E_supply = 1e-09 J, E_stored = 5e-10 J, I_avg = 0.1 A. It applies only when 0 <= alpha <= 1; full-swing switching, short-circuit and leakage power not included: outside that it says what is wrong instead of returning a number.

<a id="elx_elmore_delay"></a>

#### Elmore delay of an RC ladder (interconnect) — `elx_elmore_delay`

The Elmore delay to every node of an RC ladder (a driver resistance and wire segments, each followed by a capacitance to ground), the delay at the far end and its 50 % estimate ln 2 tau.

$$
\tau_D = \sum_i C_i\sum_{j\le i}R_j
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | series resistances in ohms, from the driver outwards | — |
| `C` | in | node capacitances in picofarads, one after each resistor | — |
| `tau_D` | out | Elmore delay at the far end | s |
| `t50` | out | 50 % delay estimate ln 2 tau_D | s |
| `delays` | out | Elmore delay at each node, seconds | — |

**Applies when:** one capacitance per resistor; step input.

```shell
$ electronics-engineering solve elx_elmore_delay "R=[100, 50, 50]" "C=[0.1, 0.1, 0.2]"
tau_D = 6.5e-11 s
t50 = 4.505456674e-11 s
delays = [4e-11,5.5e-11,6.5e-11]
  by: Elmore delay of an RC ladder (interconnect)  (tau_k = sum_i C_i R(shared path of nodes i and k))
  assumes: one capacitance per resistor; step input
```

In the worked example, R = [100, 50, 50], C = [0.1, 0.1, 0.2] go through the equation above and give tau_D = 6.5e-11 s, t50 = 4.505456674e-11 s, delays = [4e-11, 5.5e-11, 6.5e-11]. It applies only when one capacitance per resistor; step input: outside that it says what is wrong instead of returning a number.

<a id="elx_setup_hold_timing"></a>

#### Flip-flop setup and hold slack, maximum clock frequency — `elx_setup_hold_timing`

The setup slack, hold slack, maximum clock frequency and a verdict for a register-to-register path from clock-to-Q, logic delays, setup and hold times and clock skew (positive when the capture clock is late).

$$
T \ge t_{cq} + t_{pd} + t_{su} - t_{skew},\quad t_{ccq} + t_{cd} \ge t_h + t_{skew}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | clock period | s |
| `tcq` | in | maximum clock-to-Q delay | s |
| `tpd` | in | maximum logic delay | s |
| `tsu` | in | setup time | s |
| `tskew` | in | clock skew, capture minus launch | s |
| `tccq` | in | minimum (contamination) clock-to-Q | s |
| `tcd` | in | minimum (contamination) logic delay | s |
| `th` | in | hold time | s |
| `setup_slack` | out | setup slack | s |
| `hold_slack` | out | hold slack | s |
| `f_max` | out | maximum clock frequency | Hz |
| `verdict` | out | meets timing, setup violation, hold violation or both | — |

**Applies when:** edge-triggered registers, single clock.

```shell
$ electronics-engineering solve elx_setup_hold_timing "T=5 ns" "tcq=0.3 ns" "tpd=3.5 ns" "tsu=0.2 ns" "tskew=0.1 ns" "tccq=0.1 ns" "tcd=0.2 ns" "th=0.1 ns"
setup_slack = 1.1e-09 s
hold_slack = 1e-10 s
f_max = 256410256.4 Hz
verdict = meets timing
  by: Flip-flop setup and hold slack, maximum clock frequency  (setup slack = T + skew - (tcq + tpd + tsu); hold slack = tccq + tcd - th - skew; f_max = 1/(tcq + tpd + tsu - skew))
  T = 5e-09 s
  tcq = 3e-10 s
  tpd = 3.5e-09 s
  tsu = 2e-10 s
  tskew = 1e-10 s
  tccq = 1e-10 s
  tcd = 2e-10 s
  th = 1e-10 s
  assumes: edge-triggered registers, single clock
```

In the worked example, T = 5 ns, tcq = 0.3 ns, tpd = 3.5 ns, tsu = 0.2 ns, tskew = 0.1 ns, tccq = 0.1 ns, tcd = 0.2 ns, th = 0.1 ns go through the equation above and give setup_slack = 1.1e-09 s, hold_slack = 1e-10 s, f_max = 256410256.4 Hz, verdict = meets timing. It applies only when edge-triggered registers, single clock: outside that it says what is wrong instead of returning a number.

<a id="elx_rc_delay_rise_time"></a>

#### RC delay, 10-90 % rise time and bandwidth — `elx_rc_delay_rise_time`

The time constant, the 50 % delay (ln 2 RC), the 10-90 % rise time (ln 9 RC = 2.2 RC), the -3 dB bandwidth and the rise-time-bandwidth product (0.35) of a first-order RC low-pass.

$$
t_r = \ln 9\,RC \approx 2.2RC,\quad t_rf_{3dB} \approx 0.35
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | resistance | ohm |
| `C` | in | capacitance | F |
| `tau` | out | time constant RC | s |
| `t50` | out | 50 % delay | s |
| `t_rise` | out | 10-90 % rise time | s |
| `f_3dB` | out | -3 dB bandwidth | Hz |
| `tr_times_bw` | out | rise time x bandwidth | — |

**Applies when:** step input, first-order RC.

```shell
$ electronics-engineering solve elx_rc_delay_rise_time "R=1000 ohm" "C=10 pF"
tau = 1e-08 s
t50 = 6.931471806e-09 s
t_rise = 2.197224577e-08 s
f_3dB = 15915494.31 Hz
tr_times_bw = 0.3496991526
  by: RC delay, 10-90 % rise time and bandwidth  (t50 = ln 2 RC; tr = ln 9 RC; f = 1/(2 pi RC); tr f = ln 9/(2 pi))
  R = 1000 ohm
  C = 1e-11 F
  assumes: step input, first-order RC
```

In the worked example, R = 1000 ohm, C = 10 pF go through the equation above and give tau = 1e-08 s, t50 = 6.931471806e-09 s, t_rise = 2.197224577e-08 s, f_3dB = 15915494.31 Hz. It applies only when step input, first-order RC: outside that it says what is wrong instead of returning a number.

<a id="elx_metastability_mtbf"></a>

#### Synchronizer metastability MTBF — `elx_metastability_mtbf`

The mean time between synchronizer failures MTBF = exp(tr/tau)/(T0 fclk fdata) from the resolution time allowed, the flip-flop's time constant and window, and the clock and data rates.

$$
\text{MTBF} = \frac{e^{t_r/\tau}}{T_0f_{clk}f_{data}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tr` | in | resolution time available | s |
| `tau` | in | metastability time constant | s |
| `T0` | in | metastability window T0 | s |
| `fclk` | in | clock frequency | Hz |
| `fdata` | in | rate of asynchronous data changes | Hz |
| `MTBF_s` | out | MTBF | s |
| `MTBF_years` | out | MTBF in years (365.25 days) | — |
| `log10_MTBF_s` | out | log10 of MTBF in seconds | — |

**Applies when:** tr/tau <= 700.

```shell
$ electronics-engineering solve elx_metastability_mtbf "tr=2 ns" "tau=50 ps" "T0=100 ps" "fclk=500000000 Hz" "fdata=10000000 Hz"
MTBF_s = 4.707705337e+11 s
MTBF_years = 14917.81801
log10_MTBF_s = 11.67280927
  by: Synchronizer metastability MTBF  (MTBF = exp(tr/tau)/(T0 fclk fdata))
  tr = 2e-09 s
  tau = 5e-11 s
  T0 = 1e-10 s
  fclk = 500000000 Hz
  fdata = 10000000 Hz
  assumes: tr/tau <= 700
```

In the worked example, tr = 2 ns, tau = 50 ps, T0 = 100 ps, fclk = 500000000 Hz, fdata = 10000000 Hz go through the equation above and give MTBF_s = 4.707705337e+11 s, MTBF_years = 14917.81801, log10_MTBF_s = 11.67280927. It applies only when tr/tau <= 700: outside that it says what is wrong instead of returning a number.

Questions and reports: support@ai999x.com. AI999X — The Future Is Self-Improving.
