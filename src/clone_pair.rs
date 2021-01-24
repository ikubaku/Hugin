#[derive(Clone, Debug, PartialEq)]
pub struct CodePosition {
    lines: u32,
    columns: u32,
}

impl CodePosition {
    pub fn new(lines: u32, columns: u32) -> Self {
        CodePosition { lines, columns }
    }
}

#[derive(Debug, PartialEq)]
pub struct CodeSlice {
    start: CodePosition,
    end: CodePosition,
}

impl CodeSlice {
    pub fn new(start: CodePosition, end: CodePosition) -> Self {
        CodeSlice { start, end }
    }
}

#[derive(Debug, PartialEq)]
pub struct ClonePair {
    project: CodeSlice,
    example_sketch: CodeSlice,
}

impl ClonePair {
    pub fn new(project: CodeSlice, example_sketch: CodeSlice) -> Self {
        ClonePair {
            project,
            example_sketch,
        }
    }
}
