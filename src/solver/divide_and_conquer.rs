use std::collections::{HashMap, HashSet, VecDeque};

use crate::board::{
    get_coords_from_idx, get_empty_field_idx, get_idx_from_coords, in_bounds, initialize_fields,
    Coords,
};

pub struct DacPuzzleSolver {
    fields: Vec<u8>,
    forbidden_fields: HashSet<Coords<i32>>,
    width: i32,
    height: i32,
}

impl DacPuzzleSolver {
    pub fn new(fields: &[u8], width: i32, height: i32) -> Self {
        assert_eq!(fields.len() as i32, width * height);

        // TODO: Reject non-square and below 3x3

        Self {
            fields: fields.to_owned(),
            forbidden_fields: HashSet::new(),
            width,
            height,
        }
    }

    pub fn solve_puzzle(&mut self) -> Vec<(usize, usize)> {
        let goal_array = initialize_fields((self.width * self.height) as usize);

        let mut swaps = Vec::new();

        let mut current_row = 0;
        for col in 0..self.width - 1 {
            let cur_coords = Coords {
                row: current_row,
                col,
            };
            let cur_idx: i32 = get_idx_from_coords(cur_coords, self.width);
            if let (Some(cur_field), Some(goal_field)) = (
                self.fields.get(cur_idx as usize),
                goal_array.get(cur_idx as usize),
            ) {
                if cur_field != goal_field {
                    let goal_idx = get_idx_of_val(&goal_array, *goal_field);
                    let field_idx = get_idx_of_val(&self.fields, *goal_field);
                    let iteration_swaps = self.compute_swaps_to_goal_pos(field_idx, goal_idx);

                    swaps.extend(iteration_swaps);
                }
                self.forbidden_fields.insert(cur_coords);
            }
        }

        // Solve complicated part now
        let field_goal_pos = Coords {
            row: current_row,
            col: self.width - 1,
        };
        let field_idx: i32 = get_idx_from_coords(field_goal_pos, self.width);
        let field_value = *goal_array.get(field_idx as usize).expect("Field value");
        let corner_swaps = self.get_corner_swaps_horizontally(field_value, field_goal_pos);
        swaps.extend(corner_swaps);

        swaps
    }

    fn get_corner_swaps_horizontally(
        &mut self,
        field_value: u8,
        field_goal_pos: Coords<i32>,
    ) -> Vec<(usize, usize)> {
        // Move last piece to place two rows below

        // May need a shortcut for a setting like this:
        // 0 1
        // X X 2
        // X X X
        let field_idx = get_idx_of_val(&self.fields, field_value);
        let goal_pos = Coords {
            row: field_goal_pos.row + 2,
            col: field_goal_pos.col,
        };
        let goal_idx = get_idx_from_coords(goal_pos, self.width);
        let mut swaps = self.compute_swaps_to_goal_pos(field_idx, goal_idx);
        let mut empty_idx = get_empty_field_idx(&self.fields) as i32;
        let empty_field = get_coords_from_idx(empty_idx, self.width);

        let empty_target_pos = Coords {
            row: goal_pos.row - 1,
            col: goal_pos.col,
        };

        // Move empty field to one row below
        let moves = self.compute_empty_field_moves(goal_pos, empty_target_pos, empty_field);
        for step in moves {
            let step_idx: i32 = get_idx_from_coords(step, self.width);
            let swap = (empty_idx as usize, step_idx as usize);
            empty_idx = step_idx;
            swaps.push(swap);
            self.fields.swap(swap.0, swap.1)
        }

        // Do fixed swaps
        let moves = get_fixed_corner_moves_horizontally(empty_target_pos);
        for step in moves {
            let step_idx: i32 = get_idx_from_coords(step, self.width);
            let swap = (empty_idx as usize, step_idx as usize);
            empty_idx = step_idx;
            swaps.push(swap);
            self.fields.swap(swap.0, swap.1)
        }

        swaps
    }

    /// Move a field given its index to a goal index.
    fn compute_swaps_to_goal_pos(
        &mut self,
        mut field_idx: i32,
        goal_idx: i32,
    ) -> Vec<(usize, usize)> {
        // let goal_array: Vec<u8> = (0..(fields.len() as u8 - 1)).into_iter().collect();
        let goal_pos = get_coords_from_idx(goal_idx, self.width);

        let mut swaps = Vec::new();

        // Determine initial indices. We will overwrite the values with every
        // iteration of the loop.
        let mut empty_idx = get_empty_field_idx(&self.fields) as i32;

        // Determine the next target on the way to the goal position for the field
        // which we are moving. One iteration of the loop moves the empty field to
        // this target and then swaps the field with the empty field.
        loop {
            let empty_field = get_coords_from_idx(empty_idx, self.width);
            let field_coords = get_coords_from_idx(field_idx, self.width);

            // Identify next target field between field to move and goal field
            // TODO: Abstract
            let delta_coords = Coords {
                row: goal_pos.row - field_coords.row,
                col: goal_pos.col - field_coords.col,
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
            let moves = self.compute_empty_field_moves(field_coords, target_coords, empty_field);

            // Convert the moves to swaps
            let mut iteration_swaps = Vec::new();
            for step in moves {
                let step_idx: i32 = get_idx_from_coords(step, self.width);
                let swap = (empty_idx as usize, step_idx as usize);
                empty_idx = step_idx;
                iteration_swaps.push(swap);
            }
            // Include swapping the empty field and the field we are moving
            let tmp = empty_idx;
            empty_idx = field_idx;
            field_idx = tmp;
            iteration_swaps.push((empty_idx as usize, field_idx as usize));

            // Perform swaps to prepare next iteration
            for swap in iteration_swaps.iter() {
                self.fields.swap(swap.0, swap.1);
            }

            // Add swaps to overall swaps
            swaps.extend(iteration_swaps);
        }
    }

    /// Compute the path of shifting the empty field.
    ///
    /// Fields that may not be moved/touched are specified in `forbidden_fields`.
    fn compute_empty_field_moves(
        &self,
        field: Coords<i32>,
        target_field: Coords<i32>,
        empty_field: Coords<i32>,
    ) -> Vec<Coords<i32>> {
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
                        match in_bounds(neighbour.row, neighbour.col, self.width, self.height)
                            && !seen_neighbours.contains(&neighbour)
                            && !self.forbidden_fields.contains(&neighbour)
                            && neighbour != field
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

    if delta_coords.row != 0 {
        if delta_coords.row < 0 {
            return Coords {
                row: field_coords.row - 1,
                col: field_coords.col,
            };
        } else {
            return Coords {
                row: field_coords.row + 1,
                col: field_coords.col,
            };
        }
    } else {
        return Coords {
            row: field_coords.row,
            col: field_coords.col,
        };
    }
}

/// Get the index of a value in a slice.
///
/// This is a convenience wrapper and panics if the value cannot be found.
fn get_idx_of_val(slice: &[u8], value: u8) -> i32 {
    slice
        .iter()
        .position(|&v| v == value)
        .expect("Expected to find value") as i32
}

fn get_fixed_corner_moves_vertically(empty_pos: Coords<i32>) -> Vec<Coords<i32>> {
    // Assumes this setup e.g. for column 0:
    //  0 1 2   0 1 2   0 1 2   0 1 2   0 1 2   0 1 2   0 1 2   0 1 2   0 1 2
    //  3 X X   3 X X     X X   X   X   X X X   X X X   X X     X   X     X X
    //  X   6     X 6   3 X 6   3 X 6   3   6   3 6     3 6 X   3 6 X   3 6 X
    //
    //   ->
    //
    // 0 1 2   0 1 2
    // 3 X X   3 X X
    //   6 X   6   X
    vec![
        Coords {
            row: empty_pos.row,
            col: empty_pos.col - 1,
        },
        Coords {
            row: empty_pos.row - 1,
            col: empty_pos.col - 1,
        },
        Coords {
            row: empty_pos.row - 1,
            col: empty_pos.col,
        },
        Coords {
            row: empty_pos.row,
            col: empty_pos.col,
        },
        Coords {
            row: empty_pos.row,
            col: empty_pos.col + 1,
        },
        Coords {
            row: empty_pos.row - 1,
            col: empty_pos.col + 1,
        },
        Coords {
            row: empty_pos.row - 1,
            col: empty_pos.col,
        },
        Coords {
            row: empty_pos.row - 1,
            col: empty_pos.col - 1,
        },
        Coords {
            row: empty_pos.row,
            col: empty_pos.col - 1,
        },
        Coords {
            row: empty_pos.row,
            col: empty_pos.col,
        },
    ]
}

fn get_fixed_corner_moves_horizontally(empty_pos: Coords<i32>) -> Vec<Coords<i32>> {
    // Assumes this setup e.g. for row 0:
    // 0 1 2 X   0 1 2     0 1   2   0 1 X 2   0 1 X 2   0 1 X 2   0 1 X 2
    // X X X     X X X X   X X X X   X X   X   X X X     X X X 3   X X X 3
    // X X X 3   X X X 3   X X X 3   X X X 3   X X X 3   X X X     X X   X
    // X X X X   X X X X   X X X X   X X X X   X X X X   X X X X   X X X X
    //
    //   ->
    //
    // 0 1 X 2   0 1   2   0 1 2     0 1 2 3
    // X X   3   X X X 3   X X X 3   X X X
    // X X X X   X X X X   X X X X   X X X X
    // X X X X   X X X X   X X X X   X X X X
    vec![
        Coords {
            row: empty_pos.row - 1,
            col: empty_pos.col,
        },
        Coords {
            row: empty_pos.row - 1,
            col: empty_pos.col - 1,
        },
        Coords {
            row: empty_pos.row,
            col: empty_pos.col - 1,
        },
        Coords {
            row: empty_pos.row,
            col: empty_pos.col,
        },
        Coords {
            row: empty_pos.row + 1,
            col: empty_pos.col,
        },
        Coords {
            row: empty_pos.row + 1,
            col: empty_pos.col - 1,
        },
        Coords {
            row: empty_pos.row,
            col: empty_pos.col - 1,
        },
        Coords {
            row: empty_pos.row - 1,
            col: empty_pos.col - 1,
        },
        Coords {
            row: empty_pos.row - 1,
            col: empty_pos.col,
        },
        Coords {
            row: empty_pos.row,
            col: empty_pos.col,
        },
    ]
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_first_field_correct_4_by_4() {
        let mut test_fields = vec![8, 5, 6, 1, 14, 4, 7, 2, 0, 13, 11, 9, 255, 12, 10, 3];

        let mut solver = DacPuzzleSolver::new(&test_fields, 4, 4);
        let swaps = solver.solve_puzzle();

        for swap in swaps {
            test_fields.swap(swap.0, swap.1);
        }

        assert_eq!(test_fields.get(0), Some(&0));
    }

    #[test]
    fn test_second() {
        let mut test_fields = vec![2, 1, 5, 3, 0, 7, 255, 6, 4];

        let mut solver = DacPuzzleSolver::new(&test_fields, 3, 3);
        let swaps = solver.solve_puzzle();

        for swap in swaps {
            test_fields.swap(swap.0, swap.1);
        }

        assert_eq!(test_fields.get(0), Some(&0));
    }
}
