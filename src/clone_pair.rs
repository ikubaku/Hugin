use serde_derive::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CodePosition {
    lines: u32,
    columns: u32,
}

impl CodePosition {
    pub fn new(lines: u32, columns: u32) -> Self {
        CodePosition { lines, columns }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct CodeSlice {
    start: CodePosition,
    end: CodePosition,
}

impl CodeSlice {
    pub fn new(start: CodePosition, end: CodePosition) -> Self {
        CodeSlice { start, end }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Scores {
    project_part: f64,
    example_sketch_part: f64,
}

impl Scores {
    pub fn new(project_part: f64, example_sketch_part: f64) -> Self {
        Scores {
            project_part,
            example_sketch_part,
        }
    }
}

// NOTE: We can't store scores as bare fields (like project_score: f64) because not everything is
// serializable into TOML format.
#[derive(Debug, PartialEq, Serialize)]
pub struct ClonePair {
    project: CodeSlice,
    example_sketch: CodeSlice,
    scores: Scores,
}

impl ClonePair {
    pub fn new(
        project: CodeSlice,
        project_score: f64,
        example_sketch: CodeSlice,
        example_sketch_score: f64,
    ) -> Self {
        ClonePair {
            project,
            example_sketch,
            scores: Scores::new(project_score, example_sketch_score),
        }
    }
}
