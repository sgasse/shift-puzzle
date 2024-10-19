#[derive(thiserror::Error, Debug)]
pub enum LibError {
    #[error("value {0} not found")]
    ValueNotFound(u8),
    #[error("pos (row: {row}, col: {col}) out of bounds")]
    PosOutOfBounds { row: i32, col: i32 },
    #[error("algorithm terminated without finding a solution")]
    TerminatedWithoutSolution,
    #[error("maximum number of steps ({0}) reached without finding a solution")]
    MaxNumStepsReached(usize),
    #[error("board is not square with width {width} and height {height}")]
    NotSquare { width: i32, height: i32 },
    #[error("fields slice with {len} fields does not match board expecting {expected} fields")]
    FieldsBoardMismatch { len: usize, expected: i32 },
    #[error("boards below 3x3 are not supported")]
    Below3x3,
}
