//! # glyph-language — Cognitive Primitive Glyphs
//!
//! 32 fundamental cognitive operations as a typed enum, with parsing,
//! compilation, and color mapping. The atomic vocabulary of agent cognition.

use std::fmt;
use std::str::FromStr;

// ─── Glyph ───────────────────────────────────────────────────────────────────

/// 32 cognitive primitive glyphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Glyph {
    Perceive,
    Remember,
    Dream,
    Forget,
    Imagine,
    Create,
    Destroy,
    Connect,
    Disconnect,
    Transform,
    Compute,
    Sense,
    Act,
    Reflect,
    Classify,
    Cluster,
    Predict,
    Alert,
    Wake,
    Sleep,
    Learn,
    Teach,
    Evolve,
    Decay,
    Emerge,
    Dissolve,
    Resonate,
    Compose,
    Decompose,
    Project,
    Embed,
    Compress,
}

impl Glyph {
    /// All 32 glyphs in order.
    pub fn all() -> [Glyph; 32] {
        use Glyph::*;
        [Perceive, Remember, Dream, Forget, Imagine, Create, Destroy, Connect,
         Disconnect, Transform, Compute, Sense, Act, Reflect, Classify, Cluster,
         Predict, Alert, Wake, Sleep, Learn, Teach, Evolve, Decay,
         Emerge, Dissolve, Resonate, Compose, Decompose, Project, Embed, Compress]
    }

    /// Short code name (lowercase).
    pub fn code(&self) -> &'static str {
        match self {
            Glyph::Perceive => "per", Glyph::Remember => "rem", Glyph::Dream => "drm",
            Glyph::Forget => "fgt", Glyph::Imagine => "img", Glyph::Create => "crt",
            Glyph::Destroy => "dst", Glyph::Connect => "con", Glyph::Disconnect => "dis",
            Glyph::Transform => "trn", Glyph::Compute => "cmp", Glyph::Sense => "sns",
            Glyph::Act => "act", Glyph::Reflect => "rfl", Glyph::Classify => "cls",
            Glyph::Cluster => "clu", Glyph::Predict => "prd", Glyph::Alert => "alt",
            Glyph::Wake => "wak", Glyph::Sleep => "slp", Glyph::Learn => "lrn",
            Glyph::Teach => "tch", Glyph::Evolve => "evo", Glyph::Decay => "dec",
            Glyph::Emerge => "emg", Glyph::Dissolve => "dsv", Glyph::Resonate => "rsn",
            Glyph::Compose => "cps", Glyph::Decompose => "dcp", Glyph::Project => "prj",
            Glyph::Embed => "emb", Glyph::Compress => "cmps",
        }
    }

    /// Category of the glyph.
    pub fn category(&self) -> GlyphCategory {
        match self {
            Glyph::Perceive | Glyph::Sense | Glyph::Remember | Glyph::Forget => GlyphCategory::Input,
            Glyph::Dream | Glyph::Imagine | Glyph::Reflect => GlyphCategory::Internal,
            Glyph::Create | Glyph::Destroy | Glyph::Act | Glyph::Compose | Glyph::Decompose => GlyphCategory::Output,
            Glyph::Connect | Glyph::Disconnect | Glyph::Resonate | Glyph::Teach | Glyph::Learn => GlyphCategory::Social,
            Glyph::Transform | Glyph::Compute | Glyph::Classify | Glyph::Cluster | Glyph::Project | Glyph::Embed | Glyph::Compress => GlyphCategory::Process,
            Glyph::Predict | Glyph::Alert | Glyph::Wake | Glyph::Sleep => GlyphCategory::Control,
            Glyph::Evolve | Glyph::Decay | Glyph::Emerge | Glyph::Dissolve => GlyphCategory::Meta,
        }
    }

    /// Map glyph to a display color (RGB tuple).
    pub fn color(&self) -> (u8, u8, u8) {
        match self.category() {
            GlyphCategory::Input => (0, 200, 100),    // Green
            GlyphCategory::Internal => (100, 100, 255), // Blue
            GlyphCategory::Output => (255, 100, 100),  // Red
            GlyphCategory::Social => (255, 200, 0),    // Yellow
            GlyphCategory::Process => (200, 100, 255),  // Purple
            GlyphCategory::Control => (255, 150, 0),    // Orange
            GlyphCategory::Meta => (0, 200, 200),       // Cyan
        }
    }
}

impl fmt::Display for Glyph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for Glyph {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        for glyph in Glyph::all() {
            if glyph.code() == lower.as_str() || format!("{:?}", glyph).to_lowercase() == lower {
                return Ok(glyph);
            }
        }
        Err(format!("Unknown glyph: {}", s))
    }
}

// ─── Glyph Category ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlyphCategory {
    Input,
    Internal,
    Output,
    Social,
    Process,
    Control,
    Meta,
}

// ─── Glyph Sequence ──────────────────────────────────────────────────────────

/// A sequence of glyphs — a cognitive program.
#[derive(Debug, Clone)]
pub struct GlyphSequence {
    pub glyphs: Vec<Glyph>,
}

impl GlyphSequence {
    pub fn new(glyphs: Vec<Glyph>) -> Self {
        Self { glyphs }
    }

    /// Parse a glyph sequence from space-separated codes.
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut glyphs = Vec::new();
        for token in text.split_whitespace() {
            glyphs.push(Glyph::from_str(token)?);
        }
        Ok(Self { glyphs })
    }

    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    /// Count glyphs by category.
    pub fn category_distribution(&self) -> Vec<(GlyphCategory, usize)> {
        use std::collections::HashMap;
        let mut counts: HashMap<GlyphCategory, usize> = HashMap::new();
        for g in &self.glyphs {
            *counts.entry(g.category()).or_insert(0) += 1;
        }
        vec![
            (GlyphCategory::Input, *counts.get(&GlyphCategory::Input).unwrap_or(&0)),
            (GlyphCategory::Internal, *counts.get(&GlyphCategory::Internal).unwrap_or(&0)),
            (GlyphCategory::Output, *counts.get(&GlyphCategory::Output).unwrap_or(&0)),
            (GlyphCategory::Social, *counts.get(&GlyphCategory::Social).unwrap_or(&0)),
            (GlyphCategory::Process, *counts.get(&GlyphCategory::Process).unwrap_or(&0)),
            (GlyphCategory::Control, *counts.get(&GlyphCategory::Control).unwrap_or(&0)),
            (GlyphCategory::Meta, *counts.get(&GlyphCategory::Meta).unwrap_or(&0)),
        ]
    }
}

// ─── Compiled Glyph Program ──────────────────────────────────────────────────

/// A compiled glyph program — validates sequences and extracts operations.
#[derive(Debug, Clone)]
pub struct CompiledProgram {
    pub operations: Vec<GlyphOp>,
    pub input_count: usize,
    pub output_count: usize,
}

/// A compiled glyph operation.
#[derive(Debug, Clone)]
pub struct GlyphOp {
    pub glyph: Glyph,
    pub phase: OpPhase,
}

/// Operation phase in execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpPhase {
    Init,
    Process,
    Finalize,
}

impl CompiledProgram {
    /// Compile a glyph sequence into a program.
    pub fn compile(seq: &GlyphSequence) -> Self {
        let mut operations = Vec::new();
        let mut input_count = 0;
        let mut output_count = 0;

        for (i, glyph) in seq.glyphs.iter().enumerate() {
            let phase = if i == 0 {
                OpPhase::Init
            } else if i == seq.glyphs.len() - 1 && seq.glyphs.len() > 1 {
                OpPhase::Finalize
            } else {
                OpPhase::Process
            };

            if matches!(glyph.category(), GlyphCategory::Input) {
                input_count += 1;
            }
            if matches!(glyph.category(), GlyphCategory::Output) {
                output_count += 1;
            }

            operations.push(GlyphOp { glyph: *glyph, phase });
        }

        Self { operations, input_count, output_count }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_32_glyphs() {
        assert_eq!(Glyph::all().len(), 32);
    }

    #[test]
    fn test_glyph_codes_unique() {
        use std::collections::HashSet;
        let codes: HashSet<&str> = Glyph::all().iter().map(|g| g.code()).collect();
        assert_eq!(codes.len(), 32);
    }

    #[test]
    fn test_glyph_from_str_code() {
        assert_eq!(Glyph::from_str("per").unwrap(), Glyph::Perceive);
        assert_eq!(Glyph::from_str("drm").unwrap(), Glyph::Dream);
    }

    #[test]
    fn test_glyph_from_str_name() {
        assert_eq!(Glyph::from_str("Perceive").unwrap(), Glyph::Perceive);
        assert_eq!(Glyph::from_str("dream").unwrap(), Glyph::Dream);
    }

    #[test]
    fn test_glyph_from_str_unknown() {
        assert!(Glyph::from_str("xyz").is_err());
    }

    #[test]
    fn test_glyph_display() {
        assert_eq!(format!("{}", Glyph::Remember), "rem");
    }

    #[test]
    fn test_category_mapping() {
        assert_eq!(Glyph::Perceive.category(), GlyphCategory::Input);
        assert_eq!(Glyph::Dream.category(), GlyphCategory::Internal);
        assert_eq!(Glyph::Create.category(), GlyphCategory::Output);
        assert_eq!(Glyph::Connect.category(), GlyphCategory::Social);
        assert_eq!(Glyph::Compute.category(), GlyphCategory::Process);
        assert_eq!(Glyph::Alert.category(), GlyphCategory::Control);
        assert_eq!(Glyph::Evolve.category(), GlyphCategory::Meta);
    }

    #[test]
    fn test_glyph_color() {
        let (r, g, b) = Glyph::Perceive.color();
        assert!(r <= 255 && g <= 255 && b <= 255);
    }

    #[test]
    fn test_glyph_sequence_parse() {
        let seq = GlyphSequence::parse("per rem drm").unwrap();
        assert_eq!(seq.len(), 3);
        assert_eq!(seq.glyphs[0], Glyph::Perceive);
    }

    #[test]
    fn test_glyph_sequence_parse_invalid() {
        assert!(GlyphSequence::parse("per xyz").is_err());
    }

    #[test]
    fn test_glyph_sequence_empty() {
        let seq = GlyphSequence::parse("").unwrap();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_category_distribution() {
        let seq = GlyphSequence::parse("per sns act crt").unwrap();
        let dist = seq.category_distribution();
        let input_count = dist.iter().find(|(c, _)| *c == GlyphCategory::Input).unwrap().1;
        assert_eq!(input_count, 2);
    }

    #[test]
    fn test_compiled_program() {
        let seq = GlyphSequence::parse("per cmp act").unwrap();
        let prog = CompiledProgram::compile(&seq);
        assert_eq!(prog.operations.len(), 3);
        assert_eq!(prog.operations[0].phase, OpPhase::Init);
        assert_eq!(prog.operations[1].phase, OpPhase::Process);
        assert_eq!(prog.operations[2].phase, OpPhase::Finalize);
    }

    #[test]
    fn test_compiled_program_counts() {
        let seq = GlyphSequence::parse("per sns crt act").unwrap();
        let prog = CompiledProgram::compile(&seq);
        assert_eq!(prog.input_count, 2);
        assert_eq!(prog.output_count, 2);
    }
}
