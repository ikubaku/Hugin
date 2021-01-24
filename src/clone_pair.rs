#[derive(Debug)]
pub struct CodePosition {
    lines: u32,
    columns: u32,
}

#[derive(Debug)]
pub struct CodeSlice {
    start: CodePosition,
    end: CodePosition,
}

#[derive(Debug)]
pub struct ClonePair {
    project: CodeSlice,
    example_sketch: CodeSlice,
}
