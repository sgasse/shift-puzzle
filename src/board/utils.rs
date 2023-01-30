#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coords<T> {
    pub row: T,
    pub col: T,
}

/// Determine the index of the empty field (`u8::MAX`) in a slice of fields.
pub fn get_empty_field_idx(fields: &[u8]) -> usize {
    fields
        .iter()
        .position(|&field| field == u8::MAX)
        .expect("Should have empty field as u8::MAX")
}

/// Get the left/top coordinates based on the index of a board field.
pub fn get_left_top(idx: usize, width: usize, unit_size: usize) -> (usize, usize) {
    let (row, col): (usize, usize) = get_row_col_from_idx(idx, width);
    let left = col * unit_size;
    let top = row * unit_size;

    (left, top)
}

/// Get the row/column coordinates for a linear array representing a board.
pub fn get_row_col_from_idx<U>(idx: U, width: U) -> (U, U)
where
    U: std::ops::Div<Output = U>,
    U: std::ops::Rem<Output = U>,
    U: Copy,
{
    let row = idx / width;
    let col = idx % width;

    (row, col)
}

/// Get the index into a linear array based on row/column coordinates.
pub fn get_idx_from_row_col<T, U>(row: T, col: T, width: T) -> U
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    U: std::convert::From<T>,
{
    row.mul(width).add(col).into()
}

pub fn get_coords_from_idx<U>(idx: U, width: U) -> Coords<U>
where
    U: std::ops::Div<Output = U>,
    U: std::ops::Rem<Output = U>,
    U: Copy,
{
    let (row, col) = get_row_col_from_idx(idx, width);
    Coords { row, col }
}

/// Check if row/column coordinates are within a field defined by width/height.
pub fn in_bounds<T, U>(row: T, col: T, width: U, height: U) -> bool
where
    T: PartialOrd<T>,
    T: PartialOrd<U>,
    T: Default,
{
    let t_zero: T = T::default();
    t_zero <= row && row < height && t_zero <= col && col < width
}
