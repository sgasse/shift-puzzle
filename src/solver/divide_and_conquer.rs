use std::collections::{HashMap, HashSet, VecDeque};

use crate::board::{
    get_coords_from_idx, get_idx_from_coords, in_bounds, initialize_fields, Coords,
};

pub struct DacPuzzleSolver {
    fields: Vec<u8>,
    forbidden_fields: HashSet<Coords<i32>>,
    width: i32,
    height: i32,
    empty_field_pos: Coords<i32>,
    swaps: Vec<(usize, usize)>,
    goal_array: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SolverPhase {
    Row,
    Column,
}

impl DacPuzzleSolver {
    pub fn new(fields: &[u8], width: i32, height: i32) -> Self {
        assert_eq!(fields.len() as i32, width * height);

        // TODO: Reject non-square and below 3x3
        let empty_field_idx = get_idx_of_val(fields, u8::MAX);
        let empty_field_pos = get_coords_from_idx(empty_field_idx, width);

        Self {
            fields: fields.to_owned(),
            forbidden_fields: HashSet::new(),
            width,
            height,
            empty_field_pos,
            swaps: Vec::new(),
            goal_array: initialize_fields((width * height) as usize),
        }
    }

    fn get_field(&self, pos: Coords<i32>) -> u8 {
        let idx: usize = get_idx_from_coords::<i32, i32>(pos, self.width) as usize;
        assert!(idx < self.fields.len());
        *self.fields.get(idx).unwrap()
    }

    fn get_goal_value(&self, pos: Coords<i32>) -> u8 {
        let idx: usize = get_idx_from_coords::<i32, i32>(pos, self.width) as usize;
        assert!(idx < self.goal_array.len());
        *self.goal_array.get(idx).unwrap()
    }

    pub fn solve_puzzle(&mut self) -> Vec<(usize, usize)> {
        let mut phase = SolverPhase::Row;
        let mut working_row = 0;
        let mut working_col = 0;

        'row_col_loop: loop {
            if self.width - working_col < 2 || self.height - working_row < 2 {
                break 'row_col_loop;
            }

            match phase {
                SolverPhase::Row => {
                    // Solve fields in the row starting at `working_col` until
                    // the second last.
                    for col in working_col..self.width - 1 {
                        let cur_pos = Coords {
                            row: working_row,
                            col,
                        };
                        let cur_pos_value = self.get_field(cur_pos);
                        let cur_pos_goal_value = self.get_goal_value(cur_pos);
                        if cur_pos_value != cur_pos_goal_value {
                            let goal_value_pos = self.get_pos_of_val(cur_pos_goal_value);
                            self.swap_field_to_goal_pos(goal_value_pos, cur_pos, phase);
                        }
                        self.forbidden_fields.insert(cur_pos);
                    }

                    // Solve last field in the row
                    let cur_pos = Coords {
                        row: working_row,
                        col: self.width - 1,
                    };

                    // Only enter the blind routine if the field is not yet in place
                    let cur_pos_value = self.get_field(cur_pos);
                    let cur_pos_goal_value = self.get_goal_value(cur_pos);
                    if cur_pos_value != cur_pos_goal_value {
                        self.swap_corner_fields_to_goal_horizontally(
                            cur_pos_goal_value,
                            cur_pos,
                            phase,
                        );
                    }

                    // Prepare next iteration step
                    working_row += 1;
                }
                SolverPhase::Column => {
                    // Solve fields in the column starting at `working_row`
                    // until the second last.
                    for row in working_row..self.height - 1 {
                        let cur_pos = Coords {
                            row,
                            col: working_col,
                        };
                        let cur_pos_value = self.get_field(cur_pos);
                        let cur_pos_goal_value = self.get_goal_value(cur_pos);
                        if cur_pos_value != cur_pos_goal_value {
                            let goal_value_pos = self.get_pos_of_val(cur_pos_goal_value);
                            self.swap_field_to_goal_pos(goal_value_pos, cur_pos, phase);
                        }
                        self.forbidden_fields.insert(cur_pos);
                    }

                    // Solve last field in the column
                    let cur_pos = Coords {
                        row: self.height - 1,
                        col: working_col,
                    };

                    // Only enter the blind routine if the field is not yet in place
                    let cur_pos_value = self.get_field(cur_pos);
                    let cur_pos_goal_value = self.get_goal_value(cur_pos);
                    if cur_pos_value != cur_pos_goal_value {
                        self.swap_corner_fields_to_goal_horizontally(
                            cur_pos_goal_value,
                            cur_pos,
                            phase,
                        );
                    }

                    // Prepare next iteration step
                    working_col += 1;

                    // TODO: Remove
                    break 'row_col_loop;
                }
            }

            phase = match phase {
                SolverPhase::Row => SolverPhase::Column,
                SolverPhase::Column => SolverPhase::Row,
            };
        }
        self.swaps.clone()
    }

    /// Move a field given its index to a goal index.
    fn swap_field_to_goal_pos(
        &mut self,
        mut goal_value_pos: Coords<i32>,
        goal_pos: Coords<i32>,
        phase: SolverPhase,
    ) {
        // Determine the next target on the way to the goal position for the field
        // which we are moving. One iteration of the loop moves the empty field to
        // this target and then swaps the field with the empty field.
        loop {
            // Identify next target field between field to move and goal field
            let delta_coords = Coords {
                row: goal_pos.row - goal_value_pos.row,
                col: goal_pos.col - goal_value_pos.col,
            };

            // Check if the field we are moving reached the goal field and return
            // swaps if so.
            if delta_coords == (Coords { row: 0, col: 0 }) {
                return;
            }

            // For the upper row, move horizontal first
            let target_coords = identify_next_step_field(goal_value_pos, delta_coords, phase);

            // Compute the moves required to bring the empty field to the target
            // field position and apply them.
            let moves =
                self.compute_empty_field_moves(goal_value_pos, target_coords, self.empty_field_pos);
            self.apply_empty_field_moves_as_swaps(&moves);

            // Include swapping the empty field and the field we are moving
            let tmp = self.empty_field_pos;
            self.apply_empty_field_moves_as_swaps(&[goal_value_pos]);
            goal_value_pos = tmp;
        }
    }

    fn value_at_pos(&self, pos: Coords<i32>) -> u8 {
        let idx: i32 = get_idx_from_coords(pos, self.width);
        *self.fields.get(idx as usize).expect("Index should exist")
    }

    fn swap_corner_fields_to_goal_horizontally(
        &mut self,
        field_value: u8,
        field_goal_pos: Coords<i32>,
        phase: SolverPhase,
    ) {
        let goal_pos = Coords {
            // The currently targeted field should end up two rows below its
            // final goal position in the same column
            row: field_goal_pos.row + 2,
            col: field_goal_pos.col,
        };
        let empty_field_target_pos = Coords {
            // The empty field should end up in the same column but one row
            // below the final goal position/one row above the goal position
            row: goal_pos.row - 1,
            col: goal_pos.col,
        };

        let value_cur_pos = self.get_pos_of_val(field_value);

        // It can happen that we enter this function in a state like this:
        // 0 1
        // X X 2
        // X X X
        // In this case, our routine would fail to find a path because it cannot
        // move the targeted field (2) or any of the already sorted fields (0
        // and 1). Thus, we have to check for and handle this case explicitly.
        if self.value_at_pos(field_goal_pos) == u8::MAX
            && self.value_at_pos(Coords {
                row: field_goal_pos.row + 1,
                col: field_goal_pos.col,
            }) == field_value
        {
            // Just swap the field into position and return
            self.apply_empty_field_moves_as_swaps(&[empty_field_target_pos]);
            return;
        }

        // Move the last field in the row to the right column but two rows
        // further down
        // Example goal state (empty field may be somwhere else):
        // 0 1 X
        // X X X
        // X   2
        self.swap_field_to_goal_pos(value_cur_pos, goal_pos, phase);

        // Move the empty field in between the goal position of the last field
        // in the original row and its current position two fields down
        // Example goal state:
        // 0 1 X
        // X X
        // X X 2
        let moves =
            self.compute_empty_field_moves(goal_pos, empty_field_target_pos, self.empty_field_pos);
        self.apply_empty_field_moves_as_swaps(&moves);

        // Apply deterministic order of swaps from the state that we set up
        // Goal state:
        // 0 1 2
        // X X
        // X X X
        let moves = get_fixed_corner_moves_horizontally(empty_field_target_pos);
        self.apply_empty_field_moves_as_swaps(&moves);
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

    fn apply_empty_field_moves_as_swaps(&mut self, moves: &[Coords<i32>]) {
        for step in moves {
            let step_idx: i32 = get_idx_from_coords(*step, self.width);
            let empty_field_idx: i32 = get_idx_from_coords(self.empty_field_pos, self.width);

            // Create and apply swap
            let swap = (empty_field_idx as usize, step_idx as usize);
            self.swaps.push(swap);
            self.fields.swap(swap.0, swap.1);

            // Update empty field index
            self.empty_field_pos = *step;
        }
    }

    fn get_pos_of_val(&self, val: u8) -> Coords<i32> {
        let idx = get_idx_of_val(&self.fields, val);
        get_coords_from_idx(idx, self.width)
    }
}

fn identify_next_step_field(
    field_coords: Coords<i32>,
    delta_coords: Coords<i32>,
    phase: SolverPhase,
) -> Coords<i32> {
    match phase {
        SolverPhase::Row => {
            // Move horizontally first
            if delta_coords.col != 0 {
                return Coords {
                    row: field_coords.row,
                    col: field_coords.col + delta_coords.col.signum(),
                };
            }

            if delta_coords.row != 0 {
                return Coords {
                    row: field_coords.row + delta_coords.row.signum(),
                    col: field_coords.col,
                };
            }
        }
        SolverPhase::Column => {
            // Move vertically first
            if delta_coords.row != 0 {
                return Coords {
                    row: field_coords.row + delta_coords.row.signum(),
                    col: field_coords.col,
                };
            }

            if delta_coords.col != 0 {
                return Coords {
                    row: field_coords.row,
                    col: field_coords.col + delta_coords.col.signum(),
                };
            }
        }
    }

    field_coords
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
    fn test_corner_case_corner_solved() {
        let mut test_fields = vec![2, 1, 5, 3, 0, 7, 255, 6, 4];

        let mut solver = DacPuzzleSolver::new(&test_fields, 3, 3);
        let swaps = solver.solve_puzzle();

        for swap in swaps {
            test_fields.swap(swap.0, swap.1);
        }

        assert_eq!(test_fields.get(0), Some(&0));

        // Should have parent error [2, 1, 5, 7, 3, 4, 0, 6, 255]
    }
}
