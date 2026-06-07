# glyph-language

> **The atomic vocabulary of agent cognition.**

[![crates.io](https://img.shields.io/crates/v/glyph-language.svg)](https://crates.io/crates/glyph-language)
[![docs.rs](https://docs.rs/glyph-language/badge.svg)](https://docs.rs/glyph-language)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![tests](https://img.shields.io/badge/tests-14-passing-green.svg)]()

32 cognitive primitive glyphs — a typed enum with parsing, compilation, and color mapping. The fundamental building blocks for expressing agent cognition in code.

---

## Why This Exists

Modern agent systems lack a **shared primitive vocabulary**. Every framework invents its own verbs for "think", "remember", "respond". This creates:

- **Interop friction**: Agents from different frameworks can't reason about each other's operations
- **Cognitive blind spots**: No systematic way to enumerate what an agent *can* do
- **Missing debuggability**: When something goes wrong, there's no shared language for *what* went wrong

`glyph-language` solves this by defining **32 atomic cognitive operations** — the minimal set needed to express any agent behavior. Inspired by cognitive science (perception → memory → action), category theory (composition of primitives), and the design of instruction set architectures.

Think of it as **a cognitive instruction set** — the RISC-V of agent minds.

---

## The 32 Glyphs

```
┌─────────────────────────────────────────────────────────────────┐
│                    GLYPH LANGUAGE v0.1                          │
├─────────────┬───────────┬───────────────────────────────────────┤
│ Glyph       │ Code      │ Description                           │
├─────────────┼───────────┼───────────────────────────────────────┤
│ INPUT       │           │                                       │
│  Perceive   │ per       │ Take in external information          │
│  Sense      │ sns       │ Raw sensory data acquisition          │
│  Remember   │ rem       │ Retrieve from memory                  │
│  Forget     │ fgt       │ Discard from active memory            │
├─────────────┼───────────┼───────────────────────────────────────┤
│ INTERNAL    │           │                                       │
│  Dream      │ drm       │ Offline consolidation & creativity    │
│  Imagine    │ img       │ Generate novel possibilities          │
│  Reflect    │ rfl       │ Self-examine current state            │
├─────────────┼───────────┼───────────────────────────────────────┤
│ OUTPUT      │           │                                       │
│  Create     │ crt       │ Produce new artifacts                 │
│  Destroy    │ dst       │ Remove or decommission artifacts      │
│  Act        │ act       │ Execute an external action            │
│  Compose    │ cps       │ Combine elements into wholes          │
│  Decompose  │ dcp       │ Break wholes into elements            │
├─────────────┼───────────┼───────────────────────────────────────┤
│ SOCIAL      │           │                                       │
│  Connect    │ con       │ Establish a link                     │
│  Disconnect │ dis       │ Break a link                         │
│  Resonate   │ rsn       │ Share knowledge via similarity       │
│  Teach      │ tch       │ Transfer knowledge outward           │
│  Learn      │ lrn       │ Absorb knowledge from outside        │
├─────────────┼───────────┼───────────────────────────────────────┤
│ PROCESS     │           │                                       │
│  Transform  │ trn       │ Change form without changing meaning  │
│  Compute    │ cmp       │ Execute deterministic calculation     │
│  Classify   │ cls       │ Assign to categories                 │
│  Cluster    │ clu       │ Group by similarity                  │
│  Project    │ prj       │ Reduce dimensionality                │
│  Embed      │ emb       │ Map into representation space         │
│  Compress   │ cmps      │ Lossy or lossless size reduction      │
├─────────────┼───────────┼───────────────────────────────────────┤
│ CONTROL     │           │                                       │
│  Predict    │ prd       │ Forecast future state                │
│  Alert      │ alt       │ Signal anomaly or urgency            │
│  Wake       │ wak       │ Transition to active processing      │
│  Sleep      │ slp       │ Transition to idle/consolidation     │
├─────────────┼───────────┼───────────────────────────────────────┤
│ META        │           │                                       │
│  Evolve     │ evo       │ Change behavior over time            │
│  Decay      │ dec       │ Gradual degradation                  │
│  Emerge     │ emg       │ Self-organize from components        │
│  Dissolve   │ dsv       │ Return to components                 │
└─────────────┴───────────┴───────────────────────────────────────┘
```

---

## Architecture

```
                    ┌──────────────┐
                    │  Glyph Text  │  "per rem drm"
                    └──────┬───────┘
                           │ parse()
                    ┌──────▼───────┐
                    │  GlyphSeq    │  [Perceive, Remember, Dream]
                    └──────┬───────┘
                           │ compile()
                    ┌──────▼───────┐
                    │  Compiled    │
                    │  Program     │  [Init→Process→Finalize]
                    └──────┬───────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
       ┌──────▼──┐  ┌──────▼──┐  ┌──────▼──┐
       │ Category│  │  Color  │  │  Phase  │
       │ Input   │  │ Green   │  │  Init   │
       │ Process │  │ Purple  │  │ Process │
       │ Output  │  │  Red    │  │ Finalize│
       └─────────┘  └─────────┘  └─────────┘
```

---

## Installation

```toml
[dependencies]
glyph-language = "0.1.0"
```

---

## Quick Start

```rust
use glyph_language::{Glyph, GlyphSequence, GlyphCategory};

// Use individual glyphs
let g = Glyph::Dream;
println!("{}", g);              // "drm"
println!("{:?}", g.category()); // Internal
println!("{:?}", g.color());    // (100, 100, 255) — blue for internal

// Parse sequences from text
let seq = GlyphSequence::parse("per cmp act").unwrap();
assert_eq!(seq.len(), 3);

// Analyze category distribution
let seq = GlyphSequence::parse("per sns cls clu prd").unwrap();
let dist = seq.category_distribution();
// Input: 2, Process: 2, Control: 1, ...
```

---

## Usage Examples

### Example 1: Agent Cognitive Trace

```rust
use glyph_language::{Glyph, GlyphSequence};

// An agent perceives a problem, classifies it, predicts outcomes, and acts
let trace = GlyphSequence::parse("per cls prd act").unwrap();

// This traces: Input → Process → Control → Output
// A complete perception-to-action loop
```

### Example 2: Cognitive Program Compilation

```rust
use glyph_language::{GlyphSequence, CompiledProgram, OpPhase};

// Compile a multi-step cognitive program
let seq = GlyphSequence::parse("per cmp cls crt act").unwrap();
let program = CompiledProgram::compile(&seq);

assert_eq!(program.operations.len(), 5);

// First operation is Init phase
assert_eq!(program.operations[0].phase, OpPhase::Init);

// Middle operations are Process phase
assert_eq!(program.operations[1].phase, OpPhase::Process);

// Last operation is Finalize phase
assert_eq!(program.operations[4].phase, OpPhase::Finalize);

// Count inputs vs outputs
assert_eq!(program.input_count, 1);  // per
assert_eq!(program.output_count, 2); // crt, act
```

### Example 3: Color Mapping for TUI/Visualization

```rust
use glyph_language::Glyph;

// Map glyphs to terminal colors
for glyph in Glyph::all() {
    let (r, g, b) = glyph.color();
    match glyph.category() {
        GlyphCategory::Input   => assert_eq!(r, 0),     // Green family
        GlyphCategory::Output  => assert_eq!(r, 255),   // Red family
        GlyphCategory::Meta    => assert_eq!(g, 200),   // Cyan family
        _ => {}
    }
}
```

### Example 4: Category Analysis

```rust
use glyph_language::{GlyphSequence, GlyphCategory};

// Analyze a cognitive program's balance
let seq = GlyphSequence::parse("per sns rem cls clu prd alt act crt").unwrap();
let dist = seq.category_distribution();

for (category, count) in dist {
    if count > 0 {
        println!("{:?}: {}", category, count);
    }
}
// Input: 3 (per, sns, rem)
// Process: 2 (cls, clu)
// Control: 2 (prd, alt)
// Output: 2 (act, crt)
```

---

## Design Philosophy

### Why 32 Glyphs?

32 = 2⁵. Five binary decisions define every cognitive operation:

1. **Direction**: Input vs Output
2. **Scope**: Internal vs External
3. **Complexity**: Primitive vs Composed
4. **Temporal**: Now vs Later
5. **Agency**: Self vs Other

This isn't arbitrary — it mirrors the **instruction set architecture** approach from computer science. Just as RISC-V defines ~40 base instructions that compose into any computation, the 32 glyphs compose into any cognitive pattern.

### The Category System

Every glyph falls into exactly one of 7 categories:

| Category | Color | RGB | Philosophy |
|----------|-------|-----|------------|
| Input | 🟢 Green | (0, 200, 100) | Taking in from the world |
| Internal | 🔵 Blue | (100, 100, 255) | Processing within self |
| Output | 🔴 Red | (255, 100, 100) | Acting on the world |
| Social | 🟡 Yellow | (255, 200, 0) | Interacting with others |
| Process | 🟣 Purple | (200, 100, 255) | Transforming information |
| Control | 🟠 Orange | (255, 150, 0) | Regulating flow |
| Meta | 🔵 Cyan | (0, 200, 200) | Operating on the system itself |

### Composition Rules

Glyphs compose into **cognitive programs** — sequences that describe complete agent behaviors:

- **Perception loop**: `per → cls → prd → act` (sense, classify, predict, respond)
- **Learning cycle**: `lrn → rem → cmp → tch` (absorb, store, process, share)
- **Creative burst**: `img → crt → crt → crt → act` (imagine, create, create, create, ship)
- **Dream consolidation**: `drm → cls → rem → fgt → slp` (dream, classify, remember, forget, sleep)
- **Emergency response**: `sns → alt → wak → act` (sense, alert, wake, act)

---

## API Reference

### `Glyph` (enum)

```rust
impl Glyph {
    pub fn all() -> [Glyph; 32]           // All 32 glyphs
    pub fn code(&self) -> &'static str    // 3-letter code (e.g., "drm")
    pub fn category(&self) -> GlyphCategory
    pub fn color(&self) -> (u8, u8, u8)   // RGB tuple
}
```

Implements: `Display`, `FromStr`, `Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`, `Hash`

### `GlyphSequence`

```rust
impl GlyphSequence {
    pub fn new(glyphs: Vec<Glyph>) -> Self
    pub fn parse(text: &str) -> Result<Self, String>
    pub fn len(&self) -> usize
    pub fn is_empty(&self) -> bool
    pub fn category_distribution(&self) -> Vec<(GlyphCategory, usize)>
}
```

### `CompiledProgram`

```rust
impl CompiledProgram {
    pub fn compile(seq: &GlyphSequence) -> Self
    pub operations: Vec<GlyphOp>    // Each with phase and glyph
    pub input_count: usize
    pub output_count: usize
}
```

### `GlyphOp`

```rust
pub struct GlyphOp {
    pub glyph: Glyph,
    pub phase: OpPhase,    // Init | Process | Finalize
}
```

---

## Integration with Exocortex

`glyph-language` is a core component of the [Exocortex](https://github.com/SuperInstance/exocortex) project:

- **Shadow Pipeline**: Glyphs tag every shadow event with cognitive type
- **Dream Cycle**: Dream sequences are glyph programs (`drm → cls → rem → fgt`)
- **TUI Display**: Glyph colors drive the terminal UI palette
- **Cortex Bus**: Events are typed by glyph category for routing
- **Reflex Arc**: Reflex rules match on glyph types (Alert → Wake → Act)

---

## Comparison with Alternatives

| Feature | glyph-language | OpenAI Function Calling | LangChain | AutoGPT |
|---------|---------------|------------------------|-----------|---------|
| Primitive vocabulary | 32 typed glyphs | Unbounded strings | Unbounded | Unbounded |
| Type safety | ✅ Rust enum | ❌ JSON schema | ❌ Python | ❌ Python |
| Parsing | ✅ Built-in | N/A | N/A | N/A |
| Compilation | ✅ Phase analysis | N/A | N/A | N/A |
| Color mapping | ✅ Category-based | N/A | N/A | N/A |
| Zero dependencies | ✅ | ❌ | ❌ | ❌ |
| Cognitive theory | ✅ Category system | ❌ | ❌ | ❌ |

---

## Performance

- **Zero allocations** for single glyph operations (Copy enum)
- **Single parse pass** for sequence text
- **No dependencies** — no supply chain risk
- Compile-time exhaustive matching — the compiler catches missing cases

```
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## Theoretical Background

The glyph language draws from three traditions:

1. **Cognitive Science**: The Input → Process → Output model mirrors the information processing paradigm (Newell & Simon, 1972). The 7 categories map to established cognitive subsystems.

2. **Instruction Set Architecture**: Just as RISC-V defines a small, composable instruction set for hardware, glyphs define a small, composable vocabulary for cognition. The 32-count mirrors the size of classic ISAs.

3. **Category Theory**: Each glyph is a morphism in the category of cognitive operations. Composition of glyphs (sequences) is functorial — the category structure is preserved under compilation.

---

## Future Directions

- **Glyph weights**: Assign importance/energy to each glyph in a sequence
- **Glyph graphs**: DAGs of glyph dependencies (act requires per)
- **Glyph cost model**: Computational cost prediction per glyph type
- **Glyph embeddings**: Learn vector representations from usage patterns
- **Cross-language bindings**: Python, TypeScript, Lua
- **Formal verification**: Prove properties of glyph programs (e.g., "every Act is preceded by Perceive")

---

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-glyph-category`)
3. Write tests for your changes
4. Ensure `cargo test` and `cargo clippy` pass
5. Submit a pull request

Note: Adding new glyphs requires updating `Glyph::all()` and all match arms. The 32-count is a design target, not a hard limit — but every new glyph should justify its existence as a *primitive* (not decomposable into existing glyphs).

---

## License

MIT © [SuperInstance](https://github.com/SuperInstance)

---

*Part of the [Exocortex](https://github.com/SuperInstance/exocortex) project — persistent cognitive substrate for multi-agent systems.*
