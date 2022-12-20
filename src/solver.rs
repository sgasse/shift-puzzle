use crate::board::{get_empty_field_idx, get_swappable_neighbours, initialize_fields};
use std::collections::{HashMap, VecDeque};

pub trait AsStringHash<T> {
    fn as_string_hash(&self) -> String;
}

impl<T> AsStringHash<T> for Vec<T>
where
    T: core::fmt::Debug,
{
    fn as_string_hash(&self) -> String {
        format!("{:?}", &self)
    }
}

pub fn find_swap_order(fields: &Vec<u8>, width: usize, height: usize) -> Vec<(usize, usize)> {
    let fields = fields.clone();
    let initial_hash = fields.as_string_hash();
    let target_fields = initialize_fields(fields.len());
    let target_hash = target_fields.as_string_hash();

    if initial_hash == target_hash {
        return Vec::with_capacity(0);
    }

    let empty_field_idx = get_empty_field_idx(&fields);

    // Map to trace back from the target state to the origin state
    let mut parent_map = HashMap::new();

    // Holds tuples of (state, state_hash parent_hash, last_swap)
    let mut states_to_explore = VecDeque::from([(
        fields,
        initial_hash.clone(),
        "".to_owned(),
        (empty_field_idx, empty_field_idx),
    )]);

    // Get state and last swap from queue
    while let Some((cur_fields, cur_hash, parent_hash, last_swap)) = states_to_explore.pop_front() {
        // Add state with last swap to map
        parent_map.insert(cur_hash.clone(), (parent_hash, last_swap));

        // If the state is the target state, break
        if cur_hash == target_hash {
            break;
        }

        // Determine all reachable next states
        let reachable_tuples: Vec<_> = get_swappable_neighbours(width, height, empty_field_idx)
            .into_iter()
            .map(|neighbour_idx| {
                let mut next_fields = cur_fields.clone();
                // The empty field is at the second position of the last swap
                let next_swap = (last_swap.1, neighbour_idx);
                next_fields.swap(next_swap.0, next_swap.1);
                let next_fields_hash = next_fields.as_string_hash();

                (next_fields, next_fields_hash, cur_hash.clone(), next_swap)
            })
            .collect();

        // For each next states, check if they already exist in the map
        let unseen_tuples: Vec<_> = reachable_tuples
            .into_iter()
            .filter(|elem_tuple| !parent_map.contains_key(&elem_tuple.1))
            .collect();

        // If they do not exist, add them with the last swap to the queue
        states_to_explore.extend(unseen_tuples.into_iter());
    }

    match parent_map.contains_key(&target_hash) {
        false => Vec::with_capacity(0),
        true => {
            // Trace back from target to beginning
            let mut swaps = Vec::new();

            let mut next_hash = target_hash;
            while let Some((parent_hash, swap)) = parent_map.get(&next_hash) {
                swaps.push(swap.clone());
                if *parent_hash == initial_hash {
                    break;
                }

                next_hash = parent_hash.clone();
            }

            swaps.into_iter().rev().collect()
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_find_swap_order_zero_moves() {
        let fields = vec![0, 1, 2, u8::MAX];
        let swap_order = find_swap_order(&fields, 2, 2);
        assert_eq!(swap_order, Vec::with_capacity(0));
    }

    #[test]
    fn test_find_swap_order_one_move() {
        let fields = vec![0, 1, u8::MAX, 2];
        let swap_order = find_swap_order(&fields, 2, 2);
        assert_eq!(swap_order, vec![(2, 3)]);
    }
}
