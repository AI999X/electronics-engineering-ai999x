"""electronics-engineering-ai999x — offline electronics engineering answers with their formula, steps and verification.

    import electronics_engineering_ai999x as m
    m.ask("BJT voltage-divider bias: VCC = 12 V, R1 = 47 kohm, R2 = 10 kohm, RC = 2.2 kohm, RE = 470 ohm, beta = 150, VBE = 0.7 V").show()
    m.solve("elx_common_emitter_amplifier", IC=1e-3, beta=100.0, VT=0.025, RC=5000.0, RL=10e3, RE1=0.0, RB=20e3, Rsig=1000.0)
    m.capabilities()                   # every certified capability

92 certified capabilities: semiconductors (thermal voltage, intrinsic and doped carrier concentrations, conductivity,
drift, the Einstein relation, built-in potential, depletion width, junction capacitance); diodes (Shockley equation and
its inverse, the exact diode-resistor operating point, rectifier averages, capacitor-filter ripple, Zener regulators and
their design, LED resistors); bipolar transistors (fixed, voltage-divider, emitter and collector-feedback bias, hybrid-pi
parameters, CE, CC and CB amplifiers, fT, the Miller effect and time-constant cutoffs, current mirrors, the Wilson
mirror, the cascode, the differential pair); MOSFETs and CMOS (drain current, gm and ro, divider bias, common-source and
source-follower amplifiers, body effect, inverter threshold and delay); op-amps (inverting, non-inverting, summing,
difference and instrumentation amplifiers, finite gain, integrator, differentiator, gain-bandwidth and slew-rate limits,
offsets, Schmitt triggers, feedback, phase margin); active filters (first order, Sallen-Key, multiple feedback,
Butterworth and Chebyshev order and sections); oscillators (RC phase-shift, Wien bridge, Colpitts, Hartley, crystal,
relaxation, 555, PLL); data converters (ADC code, quantisation SNR, ENOB, jitter, R-2R and binary-weighted DACs, SAR,
dual slope, sample-and-hold); timing (RC and Elmore delay, CMOS power, setup and hold, metastability); power supplies
(linear and adjustable regulators, series-pass regulators, heat sinks).
Questions use one documented form, "<capability phrase>: name = value, ..." with units; a question that is not
determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.6.0"
_P = Product("electronics_engineering", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__, "engine": _P.engine()})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
figures, engine = _P.figures, _P.engine   # live figures and the engine in use: the Rust engine on Windows/Linux x86_64
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "figures", "engine", "__version__"]
