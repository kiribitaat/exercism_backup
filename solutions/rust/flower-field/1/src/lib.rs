const FLOWER: u8 = b'*';
const EMPTY: char = ' ';

/// All eight adjacent offsets: (Δrow, Δcol).
const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Annotate empty squares with the count of adjacent flowers.
pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(row, cells)| annotate_row(garden, row, cells.as_bytes()))
        .collect()
}

fn annotate_row(garden: &[&str], row: usize, cells: &[u8]) -> String {
    let mut annotated = String::with_capacity(cells.len());
    for (col, &cell) in cells.iter().enumerate() {
        annotated.push(annotate_cell(garden, row, col, cell));
    }
    annotated
}

fn annotate_cell(garden: &[&str], row: usize, col: usize, cell: u8) -> char {
    if cell == FLOWER {
        return char::from(FLOWER);
    }

    match count_adjacent_flowers(garden, row, col) {
        0 => EMPTY,
        n => char::from_digit(n, 10).expect("flower count is at most 8"),
    }
}

fn count_adjacent_flowers(garden: &[&str], row: usize, col: usize) -> u32 {
    NEIGHBOR_OFFSETS
        .iter()
        .filter(|&&(d_row, d_col)| is_flower_at(garden, row, col, d_row, d_col))
        .count() as u32
}

fn is_flower_at(garden: &[&str], row: usize, col: usize, d_row: isize, d_col: isize) -> bool {
    // returns None if the addition overflows (i.e. if the result is negative)
    let Some(neighbor_row) = row.checked_add_signed(d_row) else {
        return false;
    };
    let Some(neighbor_col) = col.checked_add_signed(d_col) else {
        return false;
    };
    if neighbor_row >= garden.len() {
        return false;
    }

    garden[neighbor_row]
        .as_bytes()
        .get(neighbor_col)
        .is_some_and(|&cell| cell == FLOWER)
}
