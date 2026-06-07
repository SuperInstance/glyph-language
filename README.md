# Glyph Language

[![crates.io](https://img.shields.io/crates/v/glyph-language.svg)](https://crates.io/crates/glyph-language)
[![docs.rs](https://docs.rs/glyph-language/badge.svg)](https://docs.rs/glyph-language)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **32 cognitive primitive glyphs — the atomic vocabulary of agent cognition, with parsing, compilation, and color mapping.**

---

## The Problem

Agent cognition needs a foundational vocabulary — a set of atomic operations that every higher-level process builds upon. Without such a vocabulary, each agent reinvents its own primitives, leading to incompatible cognitive architectures and no shared language for reasoning about agent behavior.

## Why This Exists

Glyph Language defines exactly **32 fundamental cognitive operations** as a typed enum, organized into 7 categories. These glyphs are the "machine code" of agent cognition — every thought, decision, and action can be decomposed into sequences of these primitives. The crate provides parsing from short codes, compilation into phased programs, and color mapping for visualization.

## The 32 Glyphs

```
Input:     Perceive(per) Sense(sns) Remember(rem) Forget(fgt)
Internal:  Dream(drm) Imagine(img) Reflect(rfl)
Output:    Create(crt) Destroy(dst) Act(act) Compose(cps) Decompose(dcp)
Social:    Connect(con) Disconnect(dis) Resonate(rsn) Teach(tch) Learn(lrn)
Process:   Transform(trn) Compute(cmp) Classify(cls) Cluster(clu) Project(prj) Embed(emb) Compress(cmps)
Control:   Predict(prd) Alert(alt) Wake(wak) Sleep(slp)
Meta:      Evolve(evo) Decay(dec) Emerge(emg) Dissolve(dsv)
```

## Installation

```toml
[dependencies]
glyph-language = "0.1"
```

## API Reference

### `Glyph`

The 32 cognitive primitives as a typed enum:

```rust
use glyph_language::Glyph;
use std::str::FromStr;

// Access all 32 glyphs
assert_eq!(Glyph::all().len(), 32);

// Parse from short code
let g = Glyph::from_str("per").unwrap();
assert_eq!(g, Glyph::Perceive);

// Parse from full name
let g = Glyph::from_str("Dream").unwrap();
assert_eq!(g, Glyph::Dream);

// Category and color
assert_eq!(Glyph::Perceive.category(), GlyphCategory::Input);
let (r, g, b) = Glyph::Perceive.color(); // (0, 200, 100) green
```

### `GlyphCategory`

7 categories organizing the glyphs:

```rust
use glyph_language::GlyphCategory;

// Input:    Perceive, Sense, Remember, Forget
// Internal: Dream, Imagine, Reflect
// Output:   Create, Destroy, Act, Compose, Decompose
// Social:   Connect, Disconnect, Resonate, Teach, Learn
// Process:  Transform, Compute, Classify, Cluster, Project, Embed, Compress
// Control:  Predict, Alert, Wake, Sleep
// Meta:     Evolve, Decay, Emerge, Dissolve
```

### `GlyphSequence`

A cognitive program as a sequence of glyphs:

```rust
use glyph_language::GlyphSequence;

let seq = GlyphSequence::parse("per cmp crt act").unwrap();
assert_eq!(seq.len(), 4);

// Category distribution
let dist = seq.category_distribution();
// [(Input, 1), (Process, 1), (Output, 2), ...]
```

### `CompiledProgram`

Compiled glyph sequence with phase annotations:

```rust
use glyph_language::{GlyphSequence, CompiledProgram, OpPhase};

let seq = GlyphSequence::parse("per cmp act").unwrap();
let prog = CompiledProgram::compile(&seq);

assert_eq!(prog.operations.len(), 3);
assert_eq!(prog.operations[0].phase, OpPhase::Init);     // first
assert_eq!(prog.operations[1].phase, OpPhase::Process);  // middle
assert_eq!(prog.operations[2].phase, OpPhase::Finalize); // last

assert_eq!(prog.input_count, 1);  // per
assert_eq!(prog.output_count, 1); // act
```

## Usage Examples

### Example 1: Parse and Execute a Cognitive Program

```rust
use glyph_language::*;

let program = GlyphSequence::parse("per rem cmp crt act").unwrap();
// Perceive → Remember → Compute → Create → Act

let compiled = CompiledProgram::compile(&program);
println!("Operations: {}", compiled.operations.len());
println!("Input phase: {} ops", compiled.input_count);
println!("Output phase: {} ops", compiled.output_count);
```

### Example 2: Analyze Glyph Distribution

```rust
use glyph_language::*;

let seq = GlyphSequence::parse("per sns cls clu prd").unwrap();
let dist = seq.category_distribution();

for (cat, count) in dist {
    if count > 0 {
        println!("{:?}: {} glyphs", cat, count);
    }
}
```

### Example 3: All Unique Codes

```rust
use glyph_language::Glyph;
use std::collections::HashSet;

let codes: HashSet<&str> = Glyph::all().iter().map(|g| g.code()).collect();
assert_eq!(codes.len(), 32); // all codes are unique
```

## Color Mapping

Each glyph category maps to a display color:

| Category | Color | RGB |
|----------|-------|-----|
| Input | Green | (0, 200, 100) |
| Internal | Blue | (100, 100, 255) |
| Output | Red | (255, 100, 100) |
| Social | Yellow | (255, 200, 0) |
| Process | Purple | (200, 100, 255) |
| Control | Orange | (255, 150, 0) |
| Meta | Cyan | (0, 200, 200) |

## Performance

| Operation | Complexity |
|-----------|-----------|
| Parse glyph | O(32) scan |
| Parse sequence | O(n × 32) |
| Compile | O(n) |
| Category distribution | O(n) |
| Display | O(1) |

## License

Licensed under the [MIT License](LICENSE).

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests for your changes
4. Push and open a Pull Request
