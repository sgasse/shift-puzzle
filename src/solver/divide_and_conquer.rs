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
                    let goal_idx = goal_array
                        .iter()
                        .position(|&v| v == *goal_field)
                        .expect("Should have field") as i32;
                    let field_idx = self
                        .fields
                        .iter()
                        .position(|&v| v == *goal_field)
                        .expect("Field") as i32;
                    let iteration_swaps = self.compute_swaps_to_goal_pos(field_idx, goal_idx);

                    swaps.extend(iteration_swaps);
                }
                self.forbidden_fields.insert(cur_coords);
            }
        }

        // Solve complicated part now

        swaps
    }

    fn get_corner_swaps_horizontally() {
        // Move last piece to place two rows below
        // Move empty field to one row below
        // Do fixed swaps
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_move_first_in_place() {
        let mut test_fields = vec![8, 5, 6, 1, 14, 4, 7, 2, 0, 13, 11, 9, 255, 12, 10, 3];

        let mut solver = DacPuzzleSolver::new(&test_fields, 4, 4);
        let swaps = solver.solve_puzzle();

        for swap in swaps {
            test_fields.swap(swap.0, swap.1);
        }

        assert_eq!(test_fields.get(0), Some(&0));
    }
}
