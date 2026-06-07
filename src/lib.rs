//! # glyph-language
//!
//! Cognitive primitive glyphs for symbolic cognitive architectures.
//!
//! Provides 32 fundamental cognitive operations as glyphs that can be parsed,
//! compiled into operations, and mapped to display colors.

/// The 32 cognitive primitive glyphs representing fundamental mental operations.
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
    /// Returns all 32 glyphs in canonical order.
    pub fn all() -> &'static [Glyph; 32] {
        const ALL: [Glyph; 32] = [
            Glyph::Perceive, Glyph::Remember, Glyph::Dream, Glyph::Forget,
            Glyph::Imagine, Glyph::Create, Glyph::Destroy, Glyph::Connect,
            Glyph::Disconnect, Glyph::Transform, Glyph::Compute, Glyph::Sense,
            Glyph::Act, Glyph::Reflect, Glyph::Classify, Glyph::Cluster,
            Glyph::Predict, Glyph::Alert, Glyph::Wake, Glyph::Sleep,
            Glyph::Learn, Glyph::Teach, Glyph::Evolve, Glyph::Decay,
            Glyph::Emerge, Glyph::Dissolve, Glyph::Resonate, Glyph::Compose,
            Glyph::Decompose, Glyph::Project, Glyph::Embed, Glyph::Compress,
        ];
        &ALL
    }

    /// Returns the string representation of a glyph.
    pub fn as_str(&self) -> &'static str {
        match self {
            Glyph::Perceive => "perceive",
            Glyph::Remember => "remember",
            Glyph::Dream => "dream",
            Glyph::Forget => "forget",
            Glyph::Imagine => "imagine",
            Glyph::Create => "create",
            Glyph::Destroy => "destroy",
            Glyph::Connect => "connect",
            Glyph::Disconnect => "disconnect",
            Glyph::Transform => "transform",
            Glyph::Compute => "compute",
            Glyph::Sense => "sense",
            Glyph::Act => "act",
            Glyph::Reflect => "reflect",
            Glyph::Classify => "classify",
            Glyph::Cluster => "cluster",
            Glyph::Predict => "predict",
            Glyph::Alert => "alert",
            Glyph::Wake => "wake",
            Glyph::Sleep => "sleep",
            Glyph::Learn => "learn",
            Glyph::Teach => "teach",
            Glyph::Evolve => "evolve",
            Glyph::Decay => "decay",
            Glyph::Emerge => "emerge",
            Glyph::Dissolve => "dissolve",
            Glyph::Resonate => "resonate",
            Glyph::Compose => "compose",
            Glyph::Decompose => "decompose",
            Glyph::Project => "project",
            Glyph::Embed => "embed",
            Glyph::Compress => "compress",
        }
    }

    /// Try to parse a glyph from a string.
    pub fn from_name(s: &str) -> Option<Glyph> {
        match s {
            "perceive" => Some(Glyph::Perceive),
            "remember" => Some(Glyph::Remember),
            "dream" => Some(Glyph::Dream),
            "forget" => Some(Glyph::Forget),
            "imagine" => Some(Glyph::Imagine),
            "create" => Some(Glyph::Create),
            "destroy" => Some(Glyph::Destroy),
            "connect" => Some(Glyph::Connect),
            "disconnect" => Some(Glyph::Disconnect),
            "transform" => Some(Glyph::Transform),
            "compute" => Some(Glyph::Compute),
            "sense" => Some(Glyph::Sense),
            "act" => Some(Glyph::Act),
            "reflect" => Some(Glyph::Reflect),
            "classify" => Some(Glyph::Classify),
            "cluster" => Some(Glyph::Cluster),
            "predict" => Some(Glyph::Predict),
            "alert" => Some(Glyph::Alert),
            "wake" => Some(Glyph::Wake),
            "sleep" => Some(Glyph::Sleep),
            "learn" => Some(Glyph::Learn),
            "teach" => Some(Glyph::Teach),
            "evolve" => Some(Glyph::Evolve),
            "decay" => Some(Glyph::Decay),
            "emerge" => Some(Glyph::Emerge),
            "dissolve" => Some(Glyph::Dissolve),
            "resonate" => Some(Glyph::Resonate),
            "compose" => Some(Glyph::Compose),
            "decompose" => Some(Glyph::Decompose),
            "project" => Some(Glyph::Project),
            "embed" => Some(Glyph::Embed),
            "compress" => Some(Glyph::Compress),
            _ => None,
        }
    }

    /// Returns the category of this glyph.
    pub fn category(&self) -> GlyphCategory {
        match self {
            Glyph::Perceive | Glyph::Sense => GlyphCategory::Perception,
            Glyph::Remember | Glyph::Forget => GlyphCategory::Memory,
            Glyph::Dream | Glyph::Imagine => GlyphCategory::Creative,
            Glyph::Create | Glyph::Destroy => GlyphCategory::Generative,
            Glyph::Connect | Glyph::Disconnect => GlyphCategory::Relational,
            Glyph::Transform | Glyph::Compute => GlyphCategory::Processing,
            Glyph::Act | Glyph::Alert | Glyph::Wake => GlyphCategory::Active,
            Glyph::Reflect | Glyph::Classify | Glyph::Cluster => GlyphCategory::Analytical,
            Glyph::Predict | Glyph::Learn | Glyph::Teach => GlyphCategory::Cognitive,
            Glyph::Sleep | Glyph::Decay | Glyph::Dissolve => GlyphCategory::Passive,
            Glyph::Evolve | Glyph::Emerge => GlyphCategory::Emergent,
            Glyph::Resonate | Glyph::Compose | Glyph::Decompose
            | Glyph::Project | Glyph::Embed | Glyph::Compress => GlyphCategory::Structural,
        }
    }
}

/// Categories of glyphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlyphCategory {
    Perception,
    Memory,
    Creative,
    Generative,
    Relational,
    Processing,
    Active,
    Analytical,
    Cognitive,
    Passive,
    Emergent,
    Structural,
}

/// Parses glyph sequences from text.
pub struct GlyphParser;

impl GlyphParser {
    /// Parse a space-separated sequence of glyph names into a Vec of Glyphs.
    pub fn parse(input: &str) -> Result<Vec<Glyph>, String> {
        let mut glyphs = Vec::new();
        for token in input.split_whitespace() {
            let token = token.trim_matches(|c: char| !c.is_alphanumeric());
            match Glyph::from_name(token) {
                Some(g) => glyphs.push(g),
                None => return Err(format!("unknown glyph: '{}'", token)),
            }
        }
        Ok(glyphs)
    }

    /// Parse a pipe-separated sequence (e.g., "perceive|remember|dream").
    pub fn parse_piped(input: &str) -> Result<Vec<Glyph>, String> {
        let mut glyphs = Vec::new();
        for token in input.split('|') {
            let token = token.trim();
            match Glyph::from_name(token) {
                Some(g) => glyphs.push(g),
                None => return Err(format!("unknown glyph: '{}'", token)),
            }
        }
        Ok(glyphs)
    }

    /// Parse comma-separated glyphs.
    pub fn parse_csv(input: &str) -> Result<Vec<Glyph>, String> {
        let mut glyphs = Vec::new();
        for token in input.split(',') {
            let token = token.trim();
            match Glyph::from_name(token) {
                Some(g) => glyphs.push(g),
                None => return Err(format!("unknown glyph: '{}'", token)),
            }
        }
        Ok(glyphs)
    }
}

/// A compiled glyph operation with a target and parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct GlyphOp {
    pub glyph: Glyph,
    pub target: Option<String>,
    pub sequence_index: usize,
}

/// Compiles glyph sequences into executable operations.
pub struct GlyphCompiler;

impl GlyphCompiler {
    /// Compile a sequence of glyphs into operations.
    pub fn compile(glyphs: &[Glyph]) -> Vec<GlyphOp> {
        glyphs
            .iter()
            .enumerate()
            .map(|(i, g)| GlyphOp {
                glyph: *g,
                target: None,
                sequence_index: i,
            })
            .collect()
    }

    /// Compile with targets assigned from a parallel list.
    pub fn compile_with_targets(glyphs: &[Glyph], targets: &[&str]) -> Vec<GlyphOp> {
        glyphs
            .iter()
            .enumerate()
            .map(|(i, g)| GlyphOp {
                glyph: *g,
                target: targets.get(i).map(|s| s.to_string()),
                sequence_index: i,
            })
            .collect()
    }

    /// Check if a glyph sequence forms a valid pattern (e.g., perceive→remember→learn).
    pub fn validate_pattern(glyphs: &[Glyph]) -> bool {
        if glyphs.is_empty() {
            return false;
        }
        // Each glyph must not be the same as its predecessor
        for w in glyphs.windows(2) {
            if w[0] == w[1] {
                return false;
            }
        }
        true
    }
}

/// RGB color representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlyphColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl GlyphColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        GlyphColor { r, g, b }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

/// Maps glyphs to display colors.
pub fn glyph_to_color(glyph: &Glyph) -> GlyphColor {
    match glyph {
        Glyph::Perceive => GlyphColor::new(100, 149, 237),   // cornflower
        Glyph::Remember => GlyphColor::new(218, 165, 32),     // goldenrod
        Glyph::Dream => GlyphColor::new(148, 103, 189),       // purple
        Glyph::Forget => GlyphColor::new(169, 169, 169),      // dark gray
        Glyph::Imagine => GlyphColor::new(255, 105, 180),     // hot pink
        Glyph::Create => GlyphColor::new(50, 205, 50),        // lime green
        Glyph::Destroy => GlyphColor::new(220, 20, 60),       // crimson
        Glyph::Connect => GlyphColor::new(0, 191, 255),       // deep sky blue
        Glyph::Disconnect => GlyphColor::new(105, 105, 105),  // dim gray
        Glyph::Transform => GlyphColor::new(255, 165, 0),     // orange
        Glyph::Compute => GlyphColor::new(70, 130, 180),      // steel blue
        Glyph::Sense => GlyphColor::new(255, 215, 0),         // gold
        Glyph::Act => GlyphColor::new(255, 69, 0),            // red-orange
        Glyph::Reflect => GlyphColor::new(186, 85, 211),      // medium orchid
        Glyph::Classify => GlyphColor::new(0, 128, 128),      // teal
        Glyph::Cluster => GlyphColor::new(72, 209, 204),      // medium turquoise
        Glyph::Predict => GlyphColor::new(138, 43, 226),      // blue violet
        Glyph::Alert => GlyphColor::new(255, 0, 0),           // red
        Glyph::Wake => GlyphColor::new(255, 255, 100),        // bright yellow
        Glyph::Sleep => GlyphColor::new(25, 25, 112),         // midnight blue
        Glyph::Learn => GlyphColor::new(34, 139, 34),         // forest green
        Glyph::Teach => GlyphColor::new(210, 105, 30),        // chocolate
        Glyph::Evolve => GlyphColor::new(0, 255, 127),        // spring green
        Glyph::Decay => GlyphColor::new(139, 90, 43),         // brown
        Glyph::Emerge => GlyphColor::new(0, 255, 255),        // cyan
        Glyph::Dissolve => GlyphColor::new(192, 192, 192),    // silver
        Glyph::Resonate => GlyphColor::new(238, 130, 238),    // violet
        Glyph::Compose => GlyphColor::new(255, 20, 147),      // deep pink
        Glyph::Decompose => GlyphColor::new(128, 0, 0),       // maroon
        Glyph::Project => GlyphColor::new(65, 105, 225),      // royal blue
        Glyph::Embed => GlyphColor::new(107, 142, 35),        // olive drab
        Glyph::Compress => GlyphColor::new(47, 79, 79),       // dark slate gray
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_glyphs_count() {
        assert_eq!(Glyph::all().len(), 32);
    }

    #[test]
    fn test_glyph_roundtrip() {
        for g in Glyph::all() {
            assert_eq!(Glyph::from_name(g.as_str()), Some(*g));
        }
    }

    #[test]
    fn test_unknown_glyph() {
        assert_eq!(Glyph::from_name("nonexistent"), None);
    }

    #[test]
    fn test_parse_space_separated() {
        let result = GlyphParser::parse("perceive remember dream").unwrap();
        assert_eq!(result, vec![Glyph::Perceive, Glyph::Remember, Glyph::Dream]);
    }

    #[test]
    fn test_parse_piped() {
        let result = GlyphParser::parse_piped("create|destroy|transform").unwrap();
        assert_eq!(result, vec![Glyph::Create, Glyph::Destroy, Glyph::Transform]);
    }

    #[test]
    fn test_parse_csv() {
        let result = GlyphParser::parse_csv("learn, teach, evolve").unwrap();
        assert_eq!(result, vec![Glyph::Learn, Glyph::Teach, Glyph::Evolve]);
    }

    #[test]
    fn test_parse_error() {
        assert!(GlyphParser::parse("perceive unknown_glyph").is_err());
    }

    #[test]
    fn test_compile_basic() {
        let glyphs = vec![Glyph::Sense, Glyph::Remember, Glyph::Learn];
        let ops = GlyphCompiler::compile(&glyphs);
        assert_eq!(ops.len(), 3);
        assert_eq!(ops[0].sequence_index, 0);
        assert_eq!(ops[2].glyph, Glyph::Learn);
    }

    #[test]
    fn test_compile_with_targets() {
        let glyphs = vec![Glyph::Act, Glyph::Reflect];
        let targets = vec!["world", "self"];
        let ops = GlyphCompiler::compile_with_targets(&glyphs, &targets);
        assert_eq!(ops[0].target, Some("world".to_string()));
        assert_eq!(ops[1].target, Some("self".to_string()));
    }

    #[test]
    fn test_validate_pattern_valid() {
        let glyphs = vec![Glyph::Perceive, Glyph::Remember, Glyph::Learn];
        assert!(GlyphCompiler::validate_pattern(&glyphs));
    }

    #[test]
    fn test_validate_pattern_empty() {
        assert!(!GlyphCompiler::validate_pattern(&[]));
    }

    #[test]
    fn test_validate_pattern_duplicate() {
        let glyphs = vec![Glyph::Dream, Glyph::Dream];
        assert!(!GlyphCompiler::validate_pattern(&glyphs));
    }

    #[test]
    fn test_glyph_color_hex() {
        let color = glyph_to_color(&Glyph::Alert);
        assert_eq!(color.to_hex(), "#ff0000");
    }

    #[test]
    fn test_glyph_color_unique() {
        let colors: Vec<_> = Glyph::all().iter().map(|g| glyph_to_color(g)).collect();
        for i in 0..colors.len() {
            for j in (i + 1)..colors.len() {
                assert_ne!(colors[i], colors[j], "Glyphs {} and {} share a color", i, j);
            }
        }
    }

    #[test]
    fn test_glyph_categories() {
        assert_eq!(Glyph::Perceive.category(), GlyphCategory::Perception);
        assert_eq!(Glyph::Remember.category(), GlyphCategory::Memory);
        assert_eq!(Glyph::Dream.category(), GlyphCategory::Creative);
        assert_eq!(Glyph::Sleep.category(), GlyphCategory::Passive);
        assert_eq!(Glyph::Evolve.category(), GlyphCategory::Emergent);
    }
}
