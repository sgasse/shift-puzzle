use std::collections::{HashMap, HashSet, VecDeque};

use crate::board::{
    get_coords_from_idx, get_empty_field_idx, get_idx_from_coords, in_bounds, Coords,
};

/// Move a field into its goal place.
pub fn compute_swaps_to_goal_pos(
    fields: &[u8],
    width: usize,
    height: usize,
    field_value: u8,
) -> Vec<(usize, usize)> {
    let width = width as i32;
    let height = height as i32;
    let mut fields = fields.to_owned();

    let goal_array: Vec<u8> = (0..(fields.len() as u8 - 1)).into_iter().collect();
    let g_idx = goal_array
        .iter()
        .position(|&v| v == field_value)
        .expect("Should have field") as i32;
    let goal_coords = get_coords_from_idx(g_idx, width);

    let mut swaps = Vec::new();

    // Determine initial indices. We will overwrite the values with every
    // iteration of the loop.
    let mut empty_idx = get_empty_field_idx(&fields) as i32;
    let mut field_idx = fields
        .iter()
        .position(|&v| v == field_value)
        .expect("Field") as i32;

    // Determine the next target on the way to the goal position for the field
    // which we are moving. One iteration of the loop moves the empty field to
    // this target and then swaps the field with the empty field.
    loop {
        let empty_field = get_coords_from_idx(empty_idx, width);
        let field_coords = get_coords_from_idx(field_idx, width);

        // Identify next target field between field to move and goal field
        // TODO: Abstract
        let delta_coords = Coords {
            row: goal_coords.row - field_coords.row,
            col: goal_coords.col - field_coords.col,
        };

        // Check if the field we are moving reached the goal field and return
        // swaps if so.
        if delta_coords == (Coords { row: 0, col: 0 }) {
            return swaps;
        }

        // For the upper row, move horizontal first
        let target_coords = identify_next_step_field_horiz_first(field_coords, delta_coords);

        // Compute the moves required to bring the empty field to the target
        // field position.
        let moves =
            compute_empty_field_moves(field_coords, target_coords, empty_field, width, height);
        dbg!(&moves);

        // Convert the moves to swaps
        let mut iteration_swaps = Vec::new();
        for step in moves {
            let step_idx: i32 = get_idx_from_coords(step, width);
            let swap = (empty_idx as usize, step_idx as usize);
            empty_idx = step_idx;
            iteration_swaps.push(swap);
        }
        // Include swapping the empty field and the field we are moving
        let tmp = empty_idx;
        empty_idx = field_idx;
        field_idx = tmp;
        iteration_swaps.push((empty_idx as usize, field_idx as usize));
        dbg!(&iteration_swaps);

        // Perform swaps to prepare next iteration
        for swap in iteration_swaps.iter() {
            fields.swap(swap.0, swap.1);
        }

        // Add swaps to overall swaps
        swaps.extend(iteration_swaps);
    }
}

fn identify_next_step_field_horiz_first(
    field_coords: Coords<i32>,
    delta_coords: Coords<i32>,
) -> Coords<i32> {
    // Move horizontal first
    if delta_coords.col != 0 {
        if delta_coords.col < 0 {
            return Coords {
                row: field_coords.row,
                col: field_coords.col - 1,
            };
        } else {
            return Coords {
                row: field_coords.row,
                col: field_coords.col + 1,
            };
        }
    }

    // delta_coords.row cannot be larger than zero because it would be in the ordered
    // block otherwise
    assert!(delta_coords.row <= 0);

    if delta_coords.row != 0 {
        return Coords {
            row: field_coords.row - 1,
            col: field_coords.col,
        };
    } else {
        return Coords {
            row: field_coords.row,
            col: field_coords.col,
        };
    }
}

/// Compute the path of shifting the empty field.
///
/// Fields that may not be moved/touched are specified in `forbidden_fields`.
fn compute_empty_field_moves(
    field: Coords<i32>,
    target_field: Coords<i32>,
    empty_field: Coords<i32>,
    width: i32,
    height: i32,
) -> Vec<Coords<i32>> {
    // TODO: Move forbidden fields out
    let mut forbidden_fields = HashSet::new();
    forbidden_fields.insert(field);

    // Look-up of parents of fields. This enables us to trace back the path to
    // our empty field once we reach the target field.
    let mut parent_field = HashMap::new();

    // Set of seen fields and queue of fields to explore for Dijkstra algorithm.
    let mut seen_neighbours: HashSet<Coords<i32>> = HashSet::new();
    let mut to_discover = VecDeque::from([empty_field]);

    // Run Dijkstra (excluding forbidden fields) from empty field until we find
    // the target field.
    'expansion: while let Some(cur_field) = to_discover.pop_front() {
        // Mark neighbour as seen/processed for Dijkstra. We do this before
        // looping through the neighbours so that we can break as soon as we
        // see the target field.
        seen_neighbours.insert(cur_field);

        // Identify neighbours
        let neighbours: Vec<Coords<i32>> = {
            [(-1, 0), (1, 0), (0, 1), (0, -1)]
                .iter()
                .filter_map(|(d_row, d_col)| {
                    let neighbour = Coords {
                        row: cur_field.row + d_row,
                        col: cur_field.col + d_col,
                    };
                    // Filter out fields which are outside of the board, already
                    // processed or in the forbidden set.
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

        // Add the current field as parent for all neighbours and queue them
        // to be processed.
        for neighbour in neighbours {
            parent_field.insert(neighbour, cur_field);
            to_discover.push_back(neighbour);
            // If our target field is among the neighbours, terminate the
            // Dijkstra search.
            if neighbour == target_field {
                break 'expansion;
            }
        }
    }

    // Trace back path from the target field to the beginning
    let mut cur_field = target_field;
    let mut parents = vec![cur_field];
    while cur_field != empty_field {
        cur_field = *parent_field.get(&cur_field).expect("Should have parent");
        parents.push(cur_field);
    }

    // Remove the empty field itself as move
    parents.pop();

    // Reverse to start from the beginning and return
    parents.reverse();
    parents
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_move_first_in_place() {
        let mut test_fields = vec![8, 5, 6, 1, 14, 4, 7, 2, 0, 13, 11, 9, 255, 12, 10, 3];
        let swaps = compute_swaps_to_goal_pos(&mut test_fields, 4, 4, 0);

        for swap in swaps {
            test_fields.swap(swap.0, swap.1);
        }

        assert_eq!(test_fields.get(0), Some(&0));
    }
}
