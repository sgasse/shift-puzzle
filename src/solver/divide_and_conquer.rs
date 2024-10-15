//! Divide and conquer puzzle solver
//!
//! See also:
//! https://www.kopf.com.br/kaplof/how-to-solve-any-slide-puzzle-regardless-of-its-size/
//!

use std::collections::{HashMap, HashSet, VecDeque};

use simple_error::bail;

use crate::{
    board::{
        get_coords_from_idx, get_idx_from_coords, get_idx_of_val, in_bounds, initialize_fields,
        Coords,
    },
    Error,
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
    /// Create a new solver instance.
    pub fn new(fields: &[u8], width: i32, height: i32) -> Result<Self, Error> {
        if fields.len() as i32 != width * height {
            bail!("DacPuzzleSolver: Fields and width*height do not match");
        }

        if width != height {
            bail!("DacPuzzleSolver: Only square puzzles are supported");
        }

        if width < 3 || height < 3 {
            bail!("DacPuzzleSolver: Puzzles below 3x3 are not supported");
        }

        let empty_field_val = (width * height - 1) as u8;
        let empty_field_idx = get_idx_of_val(fields, empty_field_val)? as i32;
        let empty_field_pos = get_coords_from_idx(empty_field_idx, width);

        Ok(Self {
            fields: fields.to_owned(),
            forbidden_fields: HashSet::new(),
            width,
            height,
            empty_field_pos,
            swaps: Vec::new(),
            goal_array: initialize_fields((width * height) as usize),
        })
    }

    /// Solve a slide-puzzle by finding the required swaps (empty field moves).
    pub fn solve_puzzle(&mut self) -> Result<Vec<(usize, usize)>, Error> {
        // We alternate phases of solving rows and columns
        let mut phase = SolverPhase::Row;

        // As working_row and working_col increase, they lock out growing parts
        // of already ordered fields. The array below shows the order of solving
        // a 4x4 field, E stand for end.
        // 0 0 0 0
        // 1 2 2 2
        // 1 3 E E
        // 1 3 E E
        let mut working_row = 0;
        let mut working_col = 0;

        'row_col_loop: loop {
            // Exit alternating row/column solving if only a square of 2x2 is
            // left
            if self.width - working_col < 2 || self.height - working_row < 2 {
                break 'row_col_loop;
            }

            match phase {
                SolverPhase::Row => {
                    // Solve fields in the row starting at `working_col` until
                    // the second last.
                    for col in working_col..self.width - 1 {
                        // Position that we want to fill with the right value/field
                        let cur_pos = Coords {
                            row: working_row,
                            col,
                        };
                        let cur_pos_value = self.value_at_pos(cur_pos)?;
                        let cur_pos_goal_value = self.goal_value_of_pos(cur_pos)?;
                        if cur_pos_value != cur_pos_goal_value {
                            let goal_value_pos = self.pos_of_value(cur_pos_goal_value)?;
                            self.swap_field_to_goal_pos(goal_value_pos, cur_pos, phase)?;
                        }
                        self.forbidden_fields.insert(cur_pos);
                    }
                }

                SolverPhase::Column => {
                    // Solve fields in the column starting at `working_row`
                    // until the second last.
                    for row in working_row..self.height - 1 {
                        // Position that we want to fill with the right value/field
                        let cur_pos = Coords {
                            row,
                            col: working_col,
                        };
                        let cur_pos_value = self.value_at_pos(cur_pos)?;
                        let cur_pos_goal_value = self.goal_value_of_pos(cur_pos)?;
                        if cur_pos_value != cur_pos_goal_value {
                            let goal_value_pos = self.pos_of_value(cur_pos_goal_value)?;
                            self.swap_field_to_goal_pos(goal_value_pos, cur_pos, phase)?;
                        }
                        self.forbidden_fields.insert(cur_pos);
                    }
                }
            }

            // Solve last field in the row
            let cur_pos = match phase {
                SolverPhase::Row => Coords {
                    row: working_row,
                    col: self.width - 1,
                },
                SolverPhase::Column => Coords {
                    row: self.height - 1,
                    col: working_col,
                },
            };

            // Only enter the deterministic routine if the field is not yet in place
            let cur_pos_value = self.value_at_pos(cur_pos)?;
            let cur_pos_goal_value = self.goal_value_of_pos(cur_pos)?;
            if cur_pos_value != cur_pos_goal_value {
                self.swap_corner_fields_to_goal(cur_pos_goal_value, cur_pos, phase)?;
            }

            // Prepare next iteration step
            match phase {
                SolverPhase::Row => {
                    working_row += 1;
                    phase = SolverPhase::Column;
                }
                SolverPhase::Column => {
                    working_col += 1;
                    phase = SolverPhase::Row
                }
            }
        }

        self.solve_last_four_fields()?;

        Ok(self.swaps.clone())
    }

    /// Move a field to a goal position.
    fn swap_field_to_goal_pos(
        &mut self,
        mut goal_value_pos: Coords<i32>,
        goal_pos: Coords<i32>,
        phase: SolverPhase,
    ) -> Result<(), Error> {
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
            // if so.
            if delta_coords == (Coords { row: 0, col: 0 }) {
                return Ok(());
            }

            // Identify target coordinates to move to
            let target_coords = identify_next_step_field(goal_value_pos, delta_coords, phase);

            // Compute the moves required to bring the empty field to the target
            // field position and apply them.
            let moves = self.compute_empty_field_moves(
                goal_value_pos,
                target_coords,
                self.empty_field_pos,
            )?;
            self.apply_empty_field_moves_as_swaps(&moves)?;

            // Include swapping the empty field and the field we are moving
            let tmp = self.empty_field_pos;
            self.apply_empty_field_moves_as_swaps(&[goal_value_pos])?;
            goal_value_pos = tmp;
        }
    }

    /// Swap the corner fields at the end of a row/column into position.
    fn swap_corner_fields_to_goal(
        &mut self,
        field_value: u8,
        field_goal_pos: Coords<i32>,
        phase: SolverPhase,
    ) -> Result<(), Error> {
        // Determine the target and intermediate position based on whether we
        // are solving a row or a column
        let (goal_pos, empty_field_target_pos) = match phase {
            SolverPhase::Row => {
                (
                    Coords {
                        // The currently targeted field should end up two rows below its
                        // final goal position in the same column
                        row: field_goal_pos.row + 2,
                        col: field_goal_pos.col,
                    },
                    Coords {
                        // The empty field should end up in the same column but one row
                        // below the final goal position/one row above the goal position
                        row: field_goal_pos.row + 1,
                        col: field_goal_pos.col,
                    },
                )
            }
            SolverPhase::Column => {
                (
                    Coords {
                        // The currently targeted field should end up two columns
                        // to the right of the final goal position in the same row
                        row: field_goal_pos.row,
                        col: field_goal_pos.col + 2,
                    },
                    Coords {
                        // The empty field should end up in the same row but one
                        // column to the right of the final goal position/one column
                        // to the left of the goal position.
                        row: field_goal_pos.row,
                        col: field_goal_pos.col + 1,
                    },
                )
            }
        };

        let value_cur_pos = self.pos_of_value(field_value)?;

        // It can happen that we enter this function in a state like this:
        // 0 1
        // X X 2
        // X X X
        // In this case, our routine would fail to find a path because it cannot
        // move the targeted field (2) or any of the already sorted fields (0
        // and 1). Thus, we have to check for and handle this case explicitly.
        let empty_field_val = (self.width * self.height - 1) as u8;
        if self.value_at_pos(field_goal_pos)? == empty_field_val
            && self.value_at_pos(empty_field_target_pos)? == field_value
        {
            // Just swap the field into position and return
            self.apply_empty_field_moves_as_swaps(&[empty_field_target_pos])?;
            return Ok(());
        }

        // Move the last field in the row to the right column but two rows
        // further down
        // Example goal state (empty field may be somewhere else):
        // 0 1 X
        // X X X
        // X   2
        self.swap_field_to_goal_pos(value_cur_pos, goal_pos, phase)?;

        // Move the empty field in between the goal position of the last field
        // in the original row and its current position two fields down
        // Example goal state:
        // 0 1 X
        // X X
        // X X 2
        let moves =
            self.compute_empty_field_moves(goal_pos, empty_field_target_pos, self.empty_field_pos)?;
        self.apply_empty_field_moves_as_swaps(&moves)?;

        // Apply deterministic order of swaps from the state that we set up
        // Goal state:
        // 0 1 2
        // X X
        // X X X
        let moves = match phase {
            SolverPhase::Row => get_fixed_corner_moves_horizontally(empty_field_target_pos),
            SolverPhase::Column => get_fixed_corner_moves_vertically(empty_field_target_pos),
        };
        self.apply_empty_field_moves_as_swaps(&moves)?;

        Ok(())
    }

    /// Compute the path of shifting the empty field.
    ///
    /// Fields that may not be moved/touched are specified in `forbidden_fields`.
    fn compute_empty_field_moves(
        &self,
        field: Coords<i32>,
        target_field: Coords<i32>,
        empty_field: Coords<i32>,
    ) -> Result<Vec<Coords<i32>>, Error> {
        // Look-up of parents of fields. This enables us to trace back the path to
        // our empty field once we reach the target field.
        let mut parent_field = HashMap::new();

        // Set of seen fields and queue of fields to explore for BFS algorithm.
        let mut seen_neighbours: HashSet<Coords<i32>> = HashSet::new();
        let mut to_discover = VecDeque::from([empty_field]);

        // Run BFS (excluding forbidden fields) from empty field until we find
        // the target field.
        'expansion: while let Some(cur_field) = to_discover.pop_front() {
            // Mark neighbour as seen/processed for BFS. We do this before
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
                // BFS search.
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
        Ok(parents)
    }

    /// Solve the 2x2 square on the bottom right.
    ///
    /// The empty field has to be in the bottom right corner in the final solved
    /// state. The remaining three fields can be rotated.
    /// We solve the puzzle by moving the empty field to the bottom right and
    /// then running full cycles of rotating the empty field around until the
    /// position fits.
    ///
    /// X X X   X X X                                X X X
    /// X 5     X 5 7  -> multiple of four steps ->  X 4 5
    /// X 4 7   X 4                                  X 7  
    ///
    fn solve_last_four_fields(&mut self) -> Result<(), Error> {
        // These four positions/moves describe one full cycle of the empty
        // field clockwise
        let last_fields_cycle = vec![
            Coords {
                row: self.height - 1,
                col: self.width - 2,
            },
            Coords {
                row: self.height - 2,
                col: self.width - 2,
            },
            Coords {
                row: self.height - 2,
                col: self.width - 1,
            },
            Coords {
                row: self.height - 1,
                col: self.width - 1,
            },
        ];

        let outer_last_field = last_fields_cycle[3];
        let inner_last_field = last_fields_cycle[1];

        // Ensure empty field is in the bottom right position
        if self.empty_field_pos != outer_last_field {
            if self.empty_field_pos == inner_last_field {
                self.apply_empty_field_moves_as_swaps(&[last_fields_cycle[0]])?;
            }

            self.apply_empty_field_moves_as_swaps(&[outer_last_field])?;
        }

        // Cycle last fields until the square is solved
        while self.fields != self.goal_array {
            self.apply_empty_field_moves_as_swaps(&last_fields_cycle)?
        }

        Ok(())
    }

    /// Apply a path of moves as swaps.
    ///
    /// This records the swaps in `self.swaps` and updates the `self.fields`
    /// and the `self.empty_field_pos` accordingly.
    fn apply_empty_field_moves_as_swaps(&mut self, moves: &[Coords<i32>]) -> Result<(), Error> {
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

        Ok(())
    }

    /// Get the position (`Coords<T>`) of a value
    fn pos_of_value(&self, val: u8) -> Result<Coords<i32>, Error> {
        let idx = get_idx_of_val(&self.fields, val)? as i32;
        Ok(get_coords_from_idx(idx, self.width))
    }

    /// Get the value at a given position
    fn value_at_pos(&self, pos: Coords<i32>) -> Result<u8, Error> {
        let idx: i32 = get_idx_from_coords(pos, self.width);
        self.fields
            .get(idx as usize)
            .copied()
            .ok_or_else(|| -> Error {
                simple_error::simple_error!("Index of value not found").into()
            })
    }

    /// Get the goal value that a position should have in the solved puzzle
    fn goal_value_of_pos(&self, pos: Coords<i32>) -> Result<u8, Error> {
        let idx: usize = get_idx_from_coords::<i32>(pos, self.width) as usize;
        self.goal_array
            .get(idx)
            .copied()
            .ok_or_else(|| simple_error::simple_error!("Index of goal value not found").into())
    }
}

/// Identify the next step to move a field to on the way to the goal position.
///
/// Depending on whether we solve a row or a column, we move the field first
/// horizontally or first vertically.
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

/// Get the required moves to solve a prepared corner state of a column.
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

/// Get the required moves to solve a prepared corner state of a row.
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
    fn test_solving_regular_4_by_4() -> Result<(), Error> {
        let mut fields = vec![8, 5, 6, 1, 14, 4, 7, 2, 0, 13, 11, 9, 15, 12, 10, 3];
        let target_fields = initialize_fields(fields.len());

        let mut solver = DacPuzzleSolver::new(&fields, 4, 4)?;
        let swaps = solver.solve_puzzle()?;

        for swap in swaps {
            fields.swap(swap.0, swap.1);
        }

        assert_eq!(fields, target_fields);

        Ok(())
    }

    #[test]
    fn test_corner_case_corner_presolved_row_end() -> Result<(), Error> {
        let mut fields = vec![2, 1, 5, 3, 0, 7, 8, 6, 4];
        let target_fields = initialize_fields(fields.len());

        let mut solver = DacPuzzleSolver::new(&fields, 3, 3)?;
        let swaps = solver.solve_puzzle()?;

        for swap in swaps {
            fields.swap(swap.0, swap.1);
        }

        assert_eq!(fields, target_fields);

        Ok(())
    }

    #[test]
    fn test_corner_case() -> Result<(), Error> {
        let mut fields = vec![2, 1, 5, 7, 3, 4, 0, 6, 8];
        let target_fields = initialize_fields(fields.len());

        let mut solver = DacPuzzleSolver::new(&fields, 3, 3)?;
        let swaps = solver.solve_puzzle()?;

        for swap in swaps {
            fields.swap(swap.0, swap.1);
        }

        assert_eq!(fields, target_fields);

        Ok(())
    }
}
