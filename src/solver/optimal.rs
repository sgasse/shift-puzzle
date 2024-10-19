//! Naive, optimal puzzle solver
//!
//! This runs a breath-first-search in the state space of possible slides until
//! finding the final state. The state space is built on the fly.
//!

use std::{
    collections::VecDeque,
    hash::{Hash, Hasher},
};

use fnv::FnvHasher;
use rustc_hash::FxHashMap;

use crate::{
    board::{get_empty_field_idx, get_swappable_neighbours, initialize_fields},
    Error,
};

/// Find the swap order to solve a puzzle
///
/// When shifting around the pieces, we can create cycles which lead back to
/// their original state. However the path to a state which we take the first
/// time we see it is guaranteed to be cycle-free since we traverse the graph
/// in a FIFO order. Therefore, we do not store subsequent (longer) paths to
/// states which we already know.
pub fn find_swap_order(
    fields: &[u8],
    width: usize,
    height: usize,
) -> Result<Vec<(usize, usize)>, Error> {
    // Determine initial values
    let state = fields.to_owned();
    let initial_hash = state.hashed();
    let target_state = initialize_fields(state.len());
    let target_hash = target_state.hashed();

    // Exit early if the puzzle is already solved
    if initial_hash == target_hash {
        return Ok(Vec::with_capacity(0));
    }

    let empty_field_idx = get_empty_field_idx(&state)?;

    // Map from a state hash to its parent hash and the last swap that led to
    // this state from the parent. We need the swap information to trace back
    // a path from the start to the target later.
    let mut parent_map = FxHashMap::default();

    // Hold tuples of (state, state_hash, parent_hash, last_swap)
    let mut states_to_explore = VecDeque::from([Step {
        state,
        state_hash: initial_hash,
        // For the first state, the parent hash is never used and can be set to zero.
        parent_hash: 0,
        // The empty field index is extracted from the last swap so we need to initialize it properly.
        swap: Swap {
            regular_idx: 0,
            empty_idx: empty_field_idx,
        },
    }]);

    let mut num_iterations = 0;

    // Get state information for unseen state
    while let Some(step) = states_to_explore.pop_front() {
        num_iterations += 1;
        let Step {
            state,
            state_hash,
            parent_hash,
            swap,
        } = step;

        // Add state hash with parent and last swap to map
        parent_map.insert(state_hash, (parent_hash, swap.clone()));

        // If the state is the target state, break
        if state_hash == target_hash {
            break;
        }

        // Determine all reachable next states
        let swappable_neighbours = get_swappable_neighbours(width, height, swap.empty_idx);
        let unseen_neighbours = swappable_neighbours.filter_map(|neighbour_idx| {
            let mut next_fields = state.clone();

            // Swap fields to calculate hash and check if we already know the state.
            next_fields.swap(neighbour_idx, swap.empty_idx);
            let next_fields_hash = next_fields.hashed();

            if parent_map.contains_key(&next_fields_hash) {
                None
            } else {
                // After swapping the fields, the indices are reversed.
                let next_swap = Swap {
                    regular_idx: swap.empty_idx,
                    empty_idx: neighbour_idx,
                };

                Some(Step {
                    state: next_fields,
                    state_hash: next_fields_hash,
                    parent_hash: state_hash,
                    swap: next_swap,
                })
            }
        });

        // Add information of unseen states to the queue to explore
        states_to_explore.extend(unseen_neighbours);
    }

    log::debug!("Number of iterations in solver: {}", num_iterations);

    // Extract the path of swaps from the initial position to the target if it
    // exists
    match parent_map.contains_key(&target_hash) {
        // TODO: Error?
        false => Ok(Vec::with_capacity(0)),
        true => {
            // Trace back from target to beginning
            let mut swaps = Vec::new();

            let mut next_hash = target_hash;
            while let Some((parent_hash, swap)) = parent_map.get(&next_hash) {
                swaps.push((swap.regular_idx, swap.empty_idx));
                if *parent_hash == initial_hash {
                    break;
                }

                next_hash = *parent_hash;
            }

            log::debug!("Number of swaps to solve: {}", swaps.len());

            Ok(swaps.into_iter().rev().collect())
        }
    }
}

trait Hashed<T> {
    fn hashed(&self) -> u64;
}

impl<T> Hashed<T> for Vec<T>
where
    T: std::hash::Hash,
{
    fn hashed(&self) -> u64 {
        // FnvHasher has a lower collision probability than FxHasher and we are
        // hashing up to millions of states
        let mut s = FnvHasher::with_key(1234);
        self.hash(&mut s);
        s.finish()
    }
}

struct Step {
    state: Vec<u8>,
    state_hash: u64,
    parent_hash: u64,
    swap: Swap,
}

#[derive(Clone)]
struct Swap {
    regular_idx: usize,
    empty_idx: usize,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_find_swap_order_zero_moves() -> Result<(), Error> {
        let fields = vec![0, 1, 2, 3];
        let swap_order = find_swap_order(&fields, 2, 2)?;
        assert_eq!(swap_order, Vec::with_capacity(0));
        Ok(())
    }

    #[test]
    fn test_find_swap_order_one_move() -> Result<(), Error> {
        let fields = vec![0, 1, 3, 2];
        let swap_order = find_swap_order(&fields, 2, 2)?;
        assert_eq!(swap_order, vec![(2, 3)]);
        Ok(())
    }

    #[test]
    fn test_find_swap_order_four_swaps() -> Result<(), Error> {
        let fields = vec![8, 1, 2, 0, 3, 5, 6, 4, 7];
        let swap_order = find_swap_order(&fields, 3, 3)?;
        assert_eq!(swap_order, vec![(0, 3), (3, 4), (4, 7), (7, 8)]);
        Ok(())
    }
}
