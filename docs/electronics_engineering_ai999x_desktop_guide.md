# Electronics Engineering: the desktop app, user guide

**Electronics Engineering**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Electronics Engineering covers semiconductors and pn junctions, diode circuits and rectifiers, BJT and MOSFET biasing and amplifiers, op-amp circuits, active filters, oscillators and 555 timers, ADCs and DACs, digital timing and power supplies. Its desktop app holds all 92 calculations in 11 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-electronics-engineering-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-electronics-engineering-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-electronics-engineering-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-electronics-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-electronics-engineering-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-electronics-engineering-1.6.5-linux-x86_64.tar.gz
./ai999x-electronics-engineering-1.6.5-linux-x86_64/ai999x-electronics-engineering
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Electronics Engineering app, opened on BJT voltage-divider bias: exact Q-point with its worked example.](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/hero.webp)

*The Electronics Engineering app, opened on BJT voltage-divider bias: exact Q-point with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open BJT › BJT voltage-divider bias: exact Q-point (or type part of its name in the search box). It opens with a worked example filled in: “BJT voltage-divider bias: VCC = 12 V, R1 = 47000 ohm, R2 = 10000 ohm, RC = 2200 ohm, RE = 470 ohm, beta = 150, VBE = 0.7 V” The answer: VTH = 2.105263158 V, RTH = 8245.614035 ohm, IB = 1.773972436e-05 A, IC = 0.002660958654 A.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![BJT voltage-divider bias: exact Q-point: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/scene-1.webp)

*BJT voltage-divider bias: exact Q-point: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it: the app reads the quantities and their units, picks the calculation and fills in its form.

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Sallen-Key low-pass filter: f0, Q and gain](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/scene-2.webp)

*Sallen-Key low-pass filter: f0, Q and gain*

![555 timer astable: frequency and duty cycle](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/scene-3.webp)

*555 timer astable: frequency and duty cycle*

![Operating point of a diode in series with a resistor (exact)](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/scene-4.webp)

*Operating point of a diode in series with a resistor (exact)*

![CMOS gate propagation delay (switch-resistor model)](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/scene-5.webp)

*CMOS gate propagation delay (switch-resistor model)*

![ADC output code, LSB and quantisation error](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/scene-6.webp)

*ADC output code, LSB and quantisation error*

![Common-source amplifier with source degeneration and ro](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/scene-7.webp)

*Common-source amplifier with source degeneration and ro*

![Closed-loop bandwidth from the gain-bandwidth product](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/scene-8.webp)

*Closed-loop bandwidth from the gain-bandwidth product*

![BJT voltage-divider bias: exact Q-point: a chart, enlarged.](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/chart-1.webp)

*BJT voltage-divider bias: exact Q-point: a chart, enlarged.*

![Sallen-Key low-pass filter: f0, Q and gain: a chart, enlarged.](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/chart-2.webp)

*Sallen-Key low-pass filter: f0, Q and gain: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Electronics Engineering app in the dark theme.](https://www.ai999x.com/?ax_shot=electronics-engineering-ai999x/hero-dark.webp)

*The Electronics Engineering app in the dark theme.*

## 7. Every calculation

92 calculations in 11 topics; each has its own entry on the model's page (https://www.ai999x.com/models/electronics-engineering-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Active filters | Butterworth filter: Q of each second-order section, Butterworth or Chebyshev low-pass filter order, Chebyshev type I filter: section Q and frequencies, Equal-component Sallen-Key low-pass design, First-order active low-pass filter (inverting), Multiple-feedback (MFB) band-pass filter, Multiple-feedback (MFB) low-pass filter, Sallen-Key high-pass filter: f0, Q and gain, Sallen-Key low-pass filter: f0, Q and gain |
| BJT | BJT beta cutoff and transition frequency fT, BJT cascode: output resistance, transconductance and gain, BJT collector-feedback bias: Q-point, BJT current mirror with finite beta, BJT differential pair: differential and common-mode gain, CMRR, BJT dual-supply emitter bias: Q-point, BJT fixed (base-resistor) bias: Q-point, BJT small-signal (hybrid-pi) parameters, BJT terminal currents from beta, BJT voltage-divider bias: exact Q-point, Common-base amplifier, Common-collector (emitter-follower) amplifier, Common-emitter amplifier: gain, input and output resistance, Common-emitter high-frequency cutoff: Miller effect and open-circuit time constants, Common-emitter low-frequency cutoff by short-circuit time constants, Wilson current mirror output current |
| CMOS | CMOS gate propagation delay (switch-resistor model), CMOS inverter switching threshold VM |
| Data converters | ADC output code, LSB and quantisation error, ADC quantisation SNR with oversampling, Binary-weighted resistor DAC, DAC output voltage (R-2R or any binary DAC), Dual-slope integrating ADC, Effective number of bits (ENOB) from SINAD, Sample-and-hold droop, SNR limit from sampling-clock (aperture) jitter, Successive-approximation (SAR) ADC conversion |
| Diodes | Capacitor-input filter: ripple voltage and DC output, Diode current from the Shockley equation, Diode voltage for a given current, LED series (current-limiting) resistor, Operating point of a diode in series with a resistor (exact), Rectifier output: average, RMS, ripple factor and PIV, Zener regulator design: series resistor and Zener rating, Zener shunt regulator: load voltage and currents |
| MOSFET | Common-source amplifier with source degeneration and ro, MOSFET drain current (cutoff, triode, saturation), MOSFET small-signal gm, ro and intrinsic gain, MOSFET threshold voltage with body effect, MOSFET voltage-divider bias with source resistor: Q-point, Source follower (common drain) |
| Op-amps | Closed-loop bandwidth from the gain-bandwidth product, Inverting amplifier with finite open-loop gain, Inverting op-amp amplifier, Inverting Schmitt trigger thresholds, Inverting summing amplifier, Negative feedback: closed-loop gain and bandwidth extension, Non-inverting op-amp amplifier, Op-amp difference amplifier (with resistor mismatch), Op-amp differentiator: gain and output amplitude, Op-amp integrator: output ramp and unity-gain frequency, Op-amp output offset from input offset voltage and bias currents, Phase margin of a two-pole amplifier with feedback, Slew-rate limit: full-power bandwidth, Three-op-amp instrumentation amplifier |
| Oscillators | 555 timer astable: frequency and duty cycle, 555 timer monostable pulse width, Colpitts oscillator frequency and feedback ratio, Hartley oscillator frequency, Op-amp relaxation (astable) oscillator, Quartz crystal: series and parallel resonance, Q, RC phase-shift oscillator frequency and gain, Type-2 PLL with active PI loop filter: natural frequency and damping, Wien-bridge oscillator frequency and required gain |
| Power supplies | Adjustable three-terminal regulator output voltage, Heat-sink thermal path: junction, case and sink temperatures, Linear regulator dissipation and efficiency, Required heat-sink thermal resistance, Zener-referenced series-pass (emitter-follower) regulator |
| Semiconductors | Built-in potential of a pn junction, Conductivity and resistivity of a semiconductor, Depletion width, field and capacitance of an abrupt pn junction, Drift velocity and drift current in a semiconductor bar, Einstein relation and diffusion length, Electron and hole concentrations of doped (compensated) material, Intrinsic carrier concentration, Junction (depletion) capacitance versus bias, Thermal voltage kT/q |
| Timing | CMOS dynamic (switching) power, Elmore delay of an RC ladder (interconnect), Flip-flop setup and hold slack, maximum clock frequency, RC delay, 10-90 % rise time and bandwidth, Synchronizer metastability MTBF |

## 8. Other ways to use it

The same engine runs on the command line (`electronics-engineering`, its own guide), in Python (`pip install electronics-engineering-ai999x`, `import electronics_engineering_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/electronics-engineering-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/electronics-engineering-ai999x/
