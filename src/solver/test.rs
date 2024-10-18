#[cfg(test)]
#[rustfmt::skip]
pub(crate) mod examples {
    pub(crate) const SHUFFLED_ONE_STEPS_9_1: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 8, 7];
    pub(crate) const SHUFFLED_ONE_STEPS_9_2: [u8; 9] = [0, 1, 2, 3, 4, 8, 6, 7, 5];

    pub(crate) const SHUFFLED_THREE_STEPS_9_1: [u8; 9] = [0, 8, 1, 3, 4, 2, 6, 7, 5];
    pub(crate) const SHUFFLED_THREE_STEPS_9_2: [u8; 9] = [0, 1, 2, 8, 4, 5, 3, 6, 7];
    pub(crate) const SHUFFLED_THREE_STEPS_9_3: [u8; 9] = [0, 8, 2, 3, 1, 5, 6, 4, 7];

    pub(crate) const SHUFFLED_TEN_STEPS_9_1: [u8; 9] = [4, 3, 1, 0, 8, 2, 6, 7, 5];
    pub(crate) const SHUFFLED_TEN_STEPS_9_2: [u8; 9] = [0, 1, 2, 7, 8, 5, 3, 4, 6];
    pub(crate) const SHUFFLED_TEN_STEPS_9_3: [u8; 9] = [0, 4, 1, 3, 2, 5, 6, 7, 8];

    pub(crate) const SHUFFLED_FIFTEEN_STEPS_9_1: [u8; 9] = [4, 1, 2, 0, 5, 8, 3, 7, 6];
    pub(crate) const SHUFFLED_FIFTEEN_STEPS_9_2: [u8; 9] = [0, 4, 5, 3, 1, 8, 6, 7, 2];
    pub(crate) const SHUFFLED_FIFTEEN_STEPS_9_3: [u8; 9] = [3, 0, 1, 5, 4, 2, 6, 8, 7];

    pub(crate) const SHUFFLED_TWENTY_STEPS_9_1: [u8; 9] = [3, 4, 0, 6, 2, 5, 7, 1, 8];
    pub(crate) const SHUFFLED_TWENTY_STEPS_9_2: [u8; 9] = [7, 3, 0, 4, 6, 1, 5, 2, 8];
    pub(crate) const SHUFFLED_TWENTY_STEPS_9_3: [u8; 9] = [0, 7, 2, 6, 3, 5, 1, 4, 8];

    pub(crate) const SHUFFLED_ONE_STEP_16_1: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 12, 13, 14, 11];
    pub(crate) const SHUFFLED_ONE_STEP_16_2: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 14];

    pub(crate) const SHUFFLED_THREE_STEPS_16_1: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 15, 10, 11, 12, 9, 13, 14];
    pub(crate) const SHUFFLED_THREE_STEPS_16_2: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 15, 12, 13, 14];
    pub(crate) const SHUFFLED_THREE_STEPS_16_3: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 14, 10, 12, 13, 15, 11];

    pub(crate) const SHUFFLED_TEN_STEPS_16_1: [u8; 16] = [4, 0, 2, 3, 1, 5, 6, 7, 8, 13, 9, 11, 12, 15, 10, 14];
    pub(crate) const SHUFFLED_TEN_STEPS_16_2: [u8; 16] = [4, 0, 1, 2, 5, 6, 3, 15, 8, 9, 10, 7, 12, 13, 14, 11];
    pub(crate) const SHUFFLED_TEN_STEPS_16_3: [u8; 16] = [0, 2, 5, 3, 4, 1, 10, 6, 8, 9, 14, 7, 12, 15, 13, 11];

    pub(crate) const SHUFFLED_FIFTEEN_STEPS_16_1: [u8; 16] = [5, 1, 2, 3, 0, 9, 4, 6, 13, 12, 10, 7, 8, 15, 14, 11];
    pub(crate) const SHUFFLED_FIFTEEN_STEPS_16_2: [u8; 16] = [4, 0, 3, 7, 8, 6, 9, 2, 12, 1, 15, 10, 13, 5, 14, 11];
    pub(crate) const SHUFFLED_FIFTEEN_STEPS_16_3: [u8; 16] = [5, 2, 6, 3, 1, 15, 0, 10, 4, 9, 11, 7, 8, 12, 13, 14];

    pub(crate) const SHUFFLED_TWENTY_STEPS_16_1: [u8; 16] = [0, 1, 2, 3, 4, 5, 7, 11, 10, 8, 13, 6, 9, 15, 12, 14];
    pub(crate) const SHUFFLED_TWENTY_STEPS_16_2: [u8; 16] = [4, 5, 0, 2, 1, 9, 7, 3, 8, 13, 15, 10, 12, 14, 6, 11];
    pub(crate) const SHUFFLED_TWENTY_STEPS_16_3: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 12, 8, 11, 14, 10, 15, 9, 13];

    pub(crate) const SHUFFLED_TWENTY_STEPS_36_1: [u8; 36] = [0, 1, 2, 3, 4, 5, 6, 7, 14, 9, 10, 11, 12, 8, 13, 15, 16, 17, 18, 19, 26, 20, 21, 22, 24, 25, 35, 27, 28, 23, 30, 31, 32, 33, 34, 29];
    pub(crate) const SHUFFLED_TWENTY_STEPS_36_2: [u8; 36] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 19, 20, 26, 21, 22, 23, 35, 31, 25, 27, 28, 29, 24, 18, 30, 32, 33, 34];
    pub(crate) const SHUFFLED_TWENTY_STEPS_36_3: [u8; 36] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 18, 14, 15, 16, 17, 24, 12, 20, 21, 22, 23, 31, 30, 25, 26, 27, 28, 19, 35, 32, 33, 34, 29];

    /// Generate a random shuffled field.
    #[test]
    fn generate_random_field() {
        use crate::board::{get_shuffle_sequence, initialize_fields};

        let sequence = get_shuffle_sequence(4, 4 * 4 - 1, 3).unwrap();
        let mut fields = initialize_fields(4 * 4);

        for swap in sequence {
            fields.swap(swap.0, swap.1);
        }

        println!("const SHUFFLED_THREE_STEPS: [u8; 16] = {:?};", fields);
    }
}

#[cfg(test)]
mod dac_tests {
    use crate::{board::initialize_fields, solver::divide_and_conquer::DacPuzzleSolver};

    macro_rules! dac_can_solve_test {
        ($size:expr; $($examples:expr),+) => {
            use super::examples::*;

            let solved = initialize_fields($size * $size);
            for fields in [$($examples,)+] {
                let mut fields: Vec<_> = fields.into();
                let mut solver = DacPuzzleSolver::new(&fields, $size, $size).unwrap();
                let swaps = solver.solve_puzzle().unwrap();

                for swap in swaps {
                    fields.swap(swap.0, swap.1);
                }

                assert_eq!(&fields, &solved);
            }
        };
    }

    #[test]
    fn dac_can_solve_3x3() {
        dac_can_solve_test!(
            3;
            SHUFFLED_ONE_STEPS_9_1,
            SHUFFLED_ONE_STEPS_9_2,
            SHUFFLED_THREE_STEPS_9_1,
            SHUFFLED_THREE_STEPS_9_2,
            SHUFFLED_THREE_STEPS_9_3,
            SHUFFLED_TEN_STEPS_9_1,
            SHUFFLED_TEN_STEPS_9_2,
            SHUFFLED_TEN_STEPS_9_3,
            SHUFFLED_FIFTEEN_STEPS_9_1,
            SHUFFLED_FIFTEEN_STEPS_9_2,
            SHUFFLED_FIFTEEN_STEPS_9_3,
            SHUFFLED_TWENTY_STEPS_9_1,
            SHUFFLED_TWENTY_STEPS_9_2,
            SHUFFLED_TWENTY_STEPS_9_3
        );
    }

    #[test]
    fn dac_can_solve_4x4() {
        dac_can_solve_test!(
            4;
            SHUFFLED_ONE_STEP_16_1,
            SHUFFLED_ONE_STEP_16_2,
            SHUFFLED_THREE_STEPS_16_1,
            SHUFFLED_THREE_STEPS_16_2,
            SHUFFLED_THREE_STEPS_16_3,
            SHUFFLED_TEN_STEPS_16_1,
            SHUFFLED_TEN_STEPS_16_2,
            SHUFFLED_TEN_STEPS_16_3,
            SHUFFLED_FIFTEEN_STEPS_16_1,
            SHUFFLED_FIFTEEN_STEPS_16_2,
            SHUFFLED_FIFTEEN_STEPS_16_3,
            SHUFFLED_TWENTY_STEPS_16_1,
            SHUFFLED_TWENTY_STEPS_16_2,
            SHUFFLED_TWENTY_STEPS_16_3
        );
    }

    #[test]
    fn dac_can_solve_6x6() {
        dac_can_solve_test!(
            6;
            SHUFFLED_TWENTY_STEPS_36_1,
            SHUFFLED_TWENTY_STEPS_36_2,
            SHUFFLED_TWENTY_STEPS_36_3
        );
    }
}

#[cfg(test)]
mod optimal_tests {
    use crate::{board::initialize_fields, solver::optimal::find_swap_order};

    macro_rules! optimal_can_solve_test {
        ($size:expr; $($examples:expr),+) => {
            use super::examples::*;

            let solved = initialize_fields($size * $size);
            for fields in [$($examples,)+] {
                let mut fields: Vec<_> = fields.into();
                let swaps = find_swap_order(&fields, $size, $size).unwrap();

                for swap in swaps {
                    fields.swap(swap.0, swap.1);
                }

                assert_eq!(&fields, &solved);
            }
        };
    }

    #[test]
    fn optimal_can_solve_3x3() {
        optimal_can_solve_test!(
            3;
            SHUFFLED_ONE_STEPS_9_1,
            SHUFFLED_ONE_STEPS_9_2,
            SHUFFLED_THREE_STEPS_9_1,
            SHUFFLED_THREE_STEPS_9_2,
            SHUFFLED_THREE_STEPS_9_3,
            SHUFFLED_TEN_STEPS_9_1,
            SHUFFLED_TEN_STEPS_9_2,
            SHUFFLED_TEN_STEPS_9_3,
            SHUFFLED_FIFTEEN_STEPS_9_1,
            SHUFFLED_FIFTEEN_STEPS_9_2,
            SHUFFLED_FIFTEEN_STEPS_9_3,
            SHUFFLED_TWENTY_STEPS_9_1,
            SHUFFLED_TWENTY_STEPS_9_2,
            SHUFFLED_TWENTY_STEPS_9_3
        );
    }

    #[test]
    fn optimal_can_solve_4x4() {
        optimal_can_solve_test!(
            4;
            SHUFFLED_ONE_STEP_16_1,
            SHUFFLED_ONE_STEP_16_2,
            SHUFFLED_THREE_STEPS_16_1,
            SHUFFLED_THREE_STEPS_16_2,
            SHUFFLED_THREE_STEPS_16_3,
            SHUFFLED_TEN_STEPS_16_1,
            SHUFFLED_TEN_STEPS_16_2,
            SHUFFLED_TEN_STEPS_16_3
        );
    }
}
