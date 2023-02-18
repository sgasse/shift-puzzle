//! Naive, optimal puzzle solver.
//!
//! This runs a breath-first-search in the state space of possible slides until
//! finding the final state. The state space is built on the fly.
//!
use std::{
    collections::{hash_map::DefaultHasher, HashMap, VecDeque},
    hash::{Hash, Hasher},
};

use crate::{
    utils::{get_idx_of_val, get_swappable_neighbours, initialize_fields},
    Error,
};

pub trait Hashed<T> {
    fn hashed(&self) -> u64;
}

impl<T> Hashed<T> for Vec<T>
where
    T: std::hash::Hash,
{
    fn hashed(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

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
    let fields = fields.to_owned();
    let initial_hash = fields.hashed();
    let target_fields = initialize_fields(fields.len());
    let target_hash = target_fields.hashed();

    // Exit early if the puzzle is already solved
    if initial_hash == target_hash {
        return Ok(Vec::with_capacity(0));
    }

    let empty_field_idx = get_idx_of_val(&fields, u8::MAX)?;

    // Map from a state hash to its parent hash and the last swap that led to
    // this state from the parent. We need to the swap information to trace back
    // a path from the start to the target later.
    let mut parent_map = HashMap::new();

    // Hold tuples of (state, state_hash parent_hash, last_swap)
    let mut states_to_explore = VecDeque::from([(
        fields,
        initial_hash,
        // The parent hash of the first state is never used/considered
        0,
        (empty_field_idx, empty_field_idx),
    )]);

    let mut num_iterations = 0;

    // Get state information for unseen state
    while let Some((cur_fields, cur_hash, parent_hash, last_swap)) = states_to_explore.pop_front() {
        num_iterations += 1;

        // Add state hash with parent and last swap to map
        parent_map.insert(cur_hash, (parent_hash, last_swap));

        // If the state is the target state, break
        if cur_hash == target_hash {
            break;
        }

        // The empty field is at the second position of the last swap
        let empty_field_idx = last_swap.1;

        // Determine all reachable next states
        let swappable_neighbours = get_swappable_neighbours(width, height, last_swap.1)?;
        let reachable_tuples: Vec<_> = swappable_neighbours
            .into_iter()
            .map(|neighbour_idx| {
                let mut next_fields = cur_fields.clone();
                let next_swap = (empty_field_idx, neighbour_idx);
                next_fields.swap(next_swap.0, next_swap.1);
                let next_fields_hash = next_fields.hashed();

                // (fields, fields_hash, parent_hash, last_swap)
                (next_fields, next_fields_hash, cur_hash, next_swap)
            })
            .collect();

        // Filter out states which we have previously seen (via a shorter path)
        let unseen_tuples: Vec<_> = reachable_tuples
            .into_iter()
            .filter(|elem_tuple| !parent_map.contains_key(&elem_tuple.1))
            .collect();

        // Add information of unseen states to the queue to explore
        states_to_explore.extend(unseen_tuples.into_iter());
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
                swaps.push(*swap);
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_find_swap_order_zero_moves() -> Result<(), Error> {
        let fields = vec![0, 1, 2, u8::MAX];
        let swap_order = find_swap_order(&fields, 2, 2)?;
        assert_eq!(swap_order, Vec::with_capacity(0));
        Ok(())
    }

    #[test]
    fn test_find_swap_order_one_move() -> Result<(), Error> {
        let fields = vec![0, 1, u8::MAX, 2];
        let swap_order = find_swap_order(&fields, 2, 2)?;
        assert_eq!(swap_order, vec![(2, 3)]);
        Ok(())
    }

    #[test]
    fn test_find_swap_order_four_swaps() -> Result<(), Error> {
        let fields = vec![u8::MAX, 1, 2, 0, 3, 5, 6, 4, 7];
        let swap_order = find_swap_order(&fields, 3, 3)?;
        assert_eq!(swap_order, vec![(0, 3), (3, 4), (4, 7), (7, 8)]);
        Ok(())
    }
}
