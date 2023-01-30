use std::collections::{HashMap, HashSet, VecDeque};

use crate::board::{get_empty_field_idx, get_row_col_from_idx, in_bounds};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coords<T> {
    row: T,
    col: T,
}

pub fn move_first_in_place(fields: &mut [u8], width: usize, height: usize, field: u8) {
    let width = width as i32;
    let height = height as i32;

    let target_array: Vec<u8> = (0..(fields.len() as u8 - 1)).into_iter().collect();
    let t_idx = target_array
        .iter()
        .position(|&v| v == field)
        .expect("Should have field") as i32;
    let (t_row, t_col) = get_row_col_from_idx(t_idx, width);

    let mut empty_idx = get_empty_field_idx(&fields) as i32;
    let mut field_idx = fields.iter().position(|&v| v == field).expect("Field") as i32;

    loop {
        let (e_row, e_col) = get_row_col_from_idx(empty_idx, width);
        let (f_row, f_col) = get_row_col_from_idx(field_idx, width);

        // Identify next field between field to move and target field
        // For the upper row, move horizontal first
        let d_col = t_col - f_col;
        let d_row = t_row - f_row;

        let (s_row, s_col) = identify_next_step_field_horiz_first(f_row, f_col, d_row, d_col);

        let moves = compute_empty_field_moves(
            Coords {
                row: f_row,
                col: f_col,
            },
            Coords {
                row: s_row,
                col: s_col,
            },
            Coords {
                row: e_row,
                col: e_col,
            },
            width,
            height,
        );
        dbg!(moves);
        break;

        // Move empty field to that field without touching the field to move
        // or already fixed fields

        // Move through swaps
    }
}

fn identify_next_step_field_horiz_first(
    f_row: i32,
    f_col: i32,
    d_row: i32,
    d_col: i32,
) -> (i32, i32) {
    // Move horizontal first
    if d_col != 0 {
        if d_col < 0 {
            return (f_row, f_col - 1);
        } else {
            return (f_row, f_col + 1);
        }
    }

    // d_row cannot be larger than zero because it would be in the ordered
    // block otherwise
    assert!(d_row <= 0);

    if d_row != 0 {
        return (f_row - 1, f_col);
    } else {
        return (f_row, f_col);
    }
}

fn compute_empty_field_moves(
    field: Coords<i32>,
    step_field: Coords<i32>,
    empty_field: Coords<i32>,
    width: i32,
    height: i32,
) -> Vec<Coords<i32>> {
    let mut forbidden_fields = HashSet::new();
    forbidden_fields.insert(field);

    let mut parent_field = HashMap::new();
    let mut seen_neighbours: HashSet<Coords<i32>> = HashSet::new();
    let mut to_discover = VecDeque::from([empty_field]);

    // BFS from empty field until we find the step field
    'expansion: while let Some(next_field) = to_discover.pop_front() {
        seen_neighbours.insert(next_field);
        let neighbours: Vec<Coords<i32>> = {
            [(-1, 0), (1, 0), (0, 1), (0, -1)]
                .iter()
                .filter_map(|(d_row, d_col)| {
                    let neighbour = Coords {
                        row: next_field.row + d_row,
                        col: next_field.col + d_col,
                    };
                    match in_bounds(neighbour.row, neighbour.col, width, height)
                        && !seen_neighbours.contains(&neighbour)
                        && !forbidden_fields.contains(&neighbour)
                    {
                        true => Some(neighbour),
                        false => None,
                    }
                })
                .collect()
        };
        for neighbour in neighbours {
            parent_field.insert(neighbour, next_field);
            to_discover.push_back(neighbour);
            if neighbour == step_field {
                break 'expansion;
            }
        }
    }

    // Trace back path and convert to swaps
    let mut cur_field = step_field;
    let mut parents = vec![cur_field];
    while cur_field != empty_field {
        let parent = *parent_field.get(&cur_field).expect("Should have parent");
        parents.push(parent);
        cur_field = parent;
    }
    parents.reverse();
    parents
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_move_first_in_place() {
        let mut test_fields = vec![8, 5, 6, 1, 0, 14, 7, 2, 255, 4, 11, 9, 12, 13, 10, 3];
        move_first_in_place(&mut test_fields, 4, 4, 0);
    }
}
