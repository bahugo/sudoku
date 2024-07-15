use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    u8,
};

use ndarray::{s, Array1, Array2, ArrayView1};

pub enum GroupKind {
    Row = 0,
    Column = 1,
    Block = 2,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub array: Array2<u8>,
}

impl Board {
    fn get_row(&self, row: usize) -> ArrayView1<u8> {
        self.array.row(row)
    }
    fn get_col(&self, col: usize) -> ArrayView1<u8> {
        self.array.column(col)
    }

    pub fn get_block_bounds_from_index(row: usize, col: usize) -> (usize, usize, usize, usize) {
        const BLOCK_WIDTH: usize = 3;
        const BLOCK_HEIGHT: usize = 3;
        const OFFSET_BLOCK_WIDTH: usize = BLOCK_WIDTH - 1;
        const OFFSET_BLOCK_HEIGHT: usize = BLOCK_HEIGHT - 1;
        let start_row = (row / BLOCK_WIDTH) * BLOCK_WIDTH;
        let end_row = start_row + OFFSET_BLOCK_WIDTH;
        let start_col = (col / BLOCK_HEIGHT) * BLOCK_HEIGHT;
        let end_col = start_col + OFFSET_BLOCK_HEIGHT;
        (start_row, end_row, start_col, end_col)
    }

    fn get_block(&self, row: usize, col: usize) -> Array1<u8> {
        let (start_row, end_row, start_col, end_col) = Self::get_block_bounds_from_index(row, col);
        Array1::from_iter(
            self.array
                .slice(s![start_row..(end_row + 1), start_col..(end_col + 1)])
                .iter()
                .cloned(),
        )
    }

    fn get_row_neighbor_indexes(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        const TOTAL_RANGE: Range<usize> = 0..9;

        let row_indexes: Vec<(usize, usize)> = TOTAL_RANGE
            .filter_map(|a| {
                if col == a {
                    return None;
                }
                Some((row, a))
            })
            .collect();
        row_indexes
    }

    fn get_col_neighbor_indexes(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        const TOTAL_RANGE: Range<usize> = 0..9;

        let col_indexes: Vec<(usize, usize)> = TOTAL_RANGE
            .filter_map(|a| {
                if row == a {
                    return None;
                }
                Some((a, col))
            })
            .collect();
        col_indexes
    }

    fn get_block_neighbor_indexes(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut block_indexes: Vec<(usize, usize)> = vec![];

        let (start_row, end_row, start_col, end_col) = Self::get_block_bounds_from_index(row, col);
        for row_index in start_row..(end_row + 1) {
            for col_index in start_col..(end_col + 1) {
                if row_index == row && col_index == col {
                    continue;
                }
                block_indexes.push((row_index, col_index));
            }
        }
        block_indexes
    }

    fn get_neighbor_values(&self, row: usize, col: usize) -> HashSet<u8> {
        let mut neighbor_values = HashSet::from_iter(self.get_row(row).into_iter().cloned());
        neighbor_values.extend(self.get_col(col).into_iter().cloned());
        neighbor_values.extend(self.get_block(row, col).iter().cloned());
        neighbor_values.remove(&0);
        neighbor_values
    }

    fn get_candidate_values(&self, row: usize, col: usize) -> HashSet<u8> {
        let candidate_values: HashSet<u8> = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let neighbor_values = self.get_neighbor_values(row, col);
        HashSet::difference(&candidate_values, &neighbor_values)
            .cloned()
            .collect()
    }

    fn get_partial_candidate_values(
        &self,
        kind: &GroupKind,
        row: usize,
        col: usize,
    ) -> HashSet<u8> {
        let candidate_values: HashSet<u8> = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut neighbor_values: HashSet<u8> = match kind {
            GroupKind::Row => HashSet::from_iter(self.get_row(row).into_iter().cloned()),
            GroupKind::Column => HashSet::from_iter(self.get_col(col).into_iter().cloned()),
            GroupKind::Block => HashSet::from_iter(self.get_block(row, col).iter().cloned()),
        };

        neighbor_values.remove(&0);

        HashSet::difference(&candidate_values, &neighbor_values)
            .cloned()
            .collect()
    }
    fn get_undefined_indexes(&self) -> Vec<(usize, usize)> {
        let output: Vec<(usize, usize)> = self
            .array
            .indexed_iter()
            .filter(|value| *value.1 == 0)
            .map(|a| a.0)
            .collect();
        output
    }
    fn set_value(&mut self, row: usize, col: usize, value: u8) {
        *(self.array.get_mut((row, col)).unwrap()) = value;
    }

    fn get_value_if_one_value_is_not_possible_in_neighbors(
        neighbor_values: HashSet<u8>,
        candidate_values: &HashSet<u8>,
    ) -> Option<u8> {
        let not_candidate_in_neighbors: Vec<u8> = candidate_values
            .iter()
            .filter(|x| !neighbor_values.contains(x))
            .copied()
            .collect::<Vec<_>>();
        if not_candidate_in_neighbors.len() == 1 {
            let value = *not_candidate_in_neighbors.first().unwrap();
            return Some(value);
        }
        None
    }

    pub fn get_value_if_only_one_candidate(candidate_values: &HashSet<u8>) -> Option<u8> {
        if candidate_values.len() != 1 {
            return None;
        }
        let value = *candidate_values.iter().next().unwrap();
        Some(value)
    }

    pub fn get_value_if_not_candidate_in_neighbors(
        neighbor_values: HashSet<u8>,
        candidate_values: &HashSet<u8>,
    ) -> Option<u8> {
        if let Some(value) = Self::get_value_if_one_value_is_not_possible_in_neighbors(
            neighbor_values,
            candidate_values,
        ) {
            return Some(value);
        }
        None
    }

    pub fn solve_naive_implementation(&self) -> Board {
        let mut output = Self {
            array: self.array.to_owned(),
        };
        let mut nb_undefined_values: usize = 0;
        loop {
            let undefined_indexes = output.get_undefined_indexes();
            let still_undefined_values: usize = undefined_indexes.len();

            if undefined_indexes.is_empty() || still_undefined_values == nb_undefined_values {
                break;
            }

            nb_undefined_values = still_undefined_values;

            'traversing_board: for (row, col) in &undefined_indexes {
                let mut candidate_values: HashSet<u8> = output
                    .get_candidate_values(*row, *col)
                    .into_iter()
                    .collect();

                if let Some(value) = Board::get_value_if_only_one_candidate(&candidate_values) {
                    output.set_value(*row, *col, value);
                    continue;
                }

                for kind in [GroupKind::Row, GroupKind::Column, GroupKind::Block] {
                    let neighbor_ids = match kind {
                        GroupKind::Row => output.get_row_neighbor_indexes(*row, *col),
                        GroupKind::Column => output.get_col_neighbor_indexes(*row, *col),
                        GroupKind::Block => output.get_block_neighbor_indexes(*row, *col),
                    };
                    let neighbor_candidate_values: Vec<HashSet<u8>> = neighbor_ids
                        .iter()
                        .map(|(r, c)| output.get_partial_candidate_values(&kind, *r, *c))
                        .collect();
                    let unique_neighbor_values: HashSet<u8> = neighbor_candidate_values
                        .iter()
                        .flat_map(|x| x.clone())
                        .collect();
                    if let Some(value) = Board::get_value_if_not_candidate_in_neighbors(
                        unique_neighbor_values,
                        &candidate_values,
                    ) {
                        output.set_value(*row, *col, value);
                        continue 'traversing_board;
                    }

                    // Count neighbor_values occurences
                    let mut acc: HashMap<Vec<u8>, usize> = HashMap::new();
                    // let mut val_count = HashMap::new();
                    neighbor_candidate_values.iter().for_each(|x| {
                        let mut key: Vec<u8> = x.iter().copied().collect();
                        key.sort();
                        if key.is_empty() {
                            return;
                        }
                        if let Some(res) = acc.get_mut(&key) {
                            *res += 1;
                        } else {
                            acc.insert(key, 1);
                        }
                    });
                    let to_remove_from_canditates: Vec<u8> = acc
                        .iter()
                        .filter_map(|(values, count)| {
                            if (*values).len() != *count {
                                return None;
                            }
                            Some(values.clone())
                        })
                        .flatten()
                        .collect();
                    if !to_remove_from_canditates.is_empty() {
                        to_remove_from_canditates.iter().for_each(|x| {
                            candidate_values.take(x);
                        });
                        // println!("neighbor_candidate_values {:?}", neighbor_candidate_values);
                        // println!("counts {:?}", acc);
                        // println!("to_remove_from_candidates {:?}", to_remove_from_canditates);
                        if let Some(value) =
                            Board::get_value_if_only_one_candidate(&candidate_values)
                        {
                            output.set_value(*row, *col, value);
                            continue 'traversing_board;
                        }
                    }
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ndarray::array;
    use std::collections::HashSet;

    #[rustfmt::skip]
    #[test]
    fn test_get_block_bounds_from_indexes(){

        let actual = Board::get_block_bounds_from_index(0, 0);
        assert_eq!(actual ,(0, 2, 0, 2));

        let actual = Board::get_block_bounds_from_index(4, 0);
        assert_eq!(actual ,(3, 5, 0, 2));

        let actual = Board::get_block_bounds_from_index(4, 5);
        assert_eq!(actual ,(3, 5, 3, 5));
    }

    #[test]
    fn test_set_methods() {
        let mut input = Board {
            array: array![
                [5, 3, 0, 0, 7, 0, 0, 0, 0],
                [6, 0, 0, 1, 9, 5, 0, 0, 0],
                [0, 9, 8, 0, 0, 0, 0, 6, 0],
                [8, 0, 0, 0, 6, 0, 0, 0, 3],
                [4, 0, 0, 8, 0, 3, 0, 0, 1],
                [7, 0, 0, 0, 2, 0, 0, 0, 6],
                [0, 6, 0, 0, 0, 0, 2, 8, 0],
                [0, 0, 0, 4, 1, 9, 0, 0, 5],
                [0, 0, 0, 0, 8, 0, 0, 7, 9]
            ],
        };
        input.set_value(0, 2, 9);
        let actual = *input.array.get([0, 2]).unwrap();
        assert_eq!(actual, 9);
    }

    #[rustfmt::skip]
    #[test]
    fn test_access_methods() {
        let input = Board {
            array: array![
                [5, 3, 0, 0, 7, 0, 0, 0, 0],
                [6, 0, 0, 1, 9, 5, 0, 0, 0],
                [0, 9, 8, 0, 0, 0, 0, 6, 0],
                [8, 0, 0, 0, 6, 0, 0, 0, 3],
                [4, 0, 0, 8, 0, 3, 0, 0, 1],
                [7, 0, 0, 0, 2, 0, 0, 0, 6],
                [0, 6, 0, 0, 0, 0, 2, 8, 0],
                [0, 0, 0, 4, 1, 9, 0, 0, 5],
                [0, 0, 0, 0, 8, 0, 0, 7, 9]
            ],
        };

        let actual = input.get_row(0);
        assert_eq!(actual, array![5, 3, 0, 0, 7, 0, 0, 0, 0]);
        let actual = input.get_row(2);
        assert_eq!(actual, array![0, 9, 8, 0, 0, 0, 0, 6, 0]);
        let actual = input.get_col(0);
        assert_eq!(actual, array![5, 6, 0, 8, 4, 7, 0, 0, 0]);
        let actual = input.get_col(8);
        assert_eq!(actual, array![0, 0, 0, 3, 1, 6, 0, 5, 9]);
        let actual = input.get_block(0, 0);
        assert_eq!(actual, array![5, 3, 0, 6, 0, 0, 0, 9, 8]);
        let actual = input.get_block(3, 0);
        assert_eq!(actual, array![8, 0, 0, 4, 0, 0, 7, 0, 0]);
        let actual = input.get_block(3, 3);
        assert_eq!(actual, array![0, 6, 0, 8, 0, 3, 0, 2, 0]);
        let actual = input.get_neighbor_values(3, 3);
        assert_eq!(actual, HashSet::from([1, 2, 3, 4, 6, 8]));
        let actual = input.get_neighbor_values(7, 7);
        assert_eq!(actual, HashSet::from([1, 2, 4, 5, 6, 7, 8, 9]));
        let actual = input.get_neighbor_values(7, 8);
        assert_eq!(actual, HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]));
        let actual = input.get_candidate_values(3, 3);
        assert_eq!(actual, HashSet::from([5, 7, 9]));
        let actual = input.get_candidate_values(7, 8);
        assert_eq!(actual, HashSet::from([]));
        let actual = input.get_undefined_indexes();
        let expected = vec![
            (0,2),(0,3),(0,5),(0,6),(0,7),(0,8),
            (1,1),(1,2),(1,6),(1,7),(1,8),
            (2,0),(2,3),(2,4),(2,5),(2,6),(2,8),
            (3,1),(3,2),(3,3),(3,5),(3,6),(3,7),
            (4,1),(4,2),(4,4),(4,6),(4,7),
            (5,1),(5,2),(5,3),(5,5),(5,6),(5,7),
            (6,0),(6,2),(6,3),(6,4),(6,5),(6,8),
            (7,0),(7,1),(7,2),(7,6),(7,7),
            (8,0),(8,1),(8,2),(8,3),(8,5),(8,6),
            ];
        assert_eq!(actual, expected);
     }

    #[rustfmt::skip]
    #[test]
    fn test_neighbor_indexes_methods() {
        let input = Board {
            array: array![
                [0, 0, 0, 0, 0, 0, 0, 0, 0,],
                [0, 0, 0, 0, 0, 0, 0, 0, 0,],
                [0, 0, 0, 0, 0, 0, 0, 0, 0,],
                [0, 0, 0, 0, 0, 0, 0, 0, 0,],
                [0, 0, 0, 0, 0, 0, 0, 0, 0,],
                [0, 0, 0, 0, 0, 0, 0, 0, 0,],
                [0, 0, 0, 0, 0, 0, 0, 0, 0,],
                [0, 0, 0, 0, 0, 0, 0, 0, 0,],
                [0, 0, 0, 0, 0, 0, 0, 0, 0,]
            ],
        };

        let actual = input.get_row_neighbor_indexes(0, 0);
        assert_eq!(actual, vec![(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), ]);

        let actual = input.get_col_neighbor_indexes(0, 0);
        assert_eq!(actual, vec![ (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), ]);

        let actual = input.get_block_neighbor_indexes(0, 0);
        assert_eq!(actual, vec![ (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2), ]);

        let actual = input.get_block_neighbor_indexes(5, 7);
        assert_eq!(actual, vec![ (3, 6), (3, 7), (3, 8), (4, 6), (4, 7), (4, 8), (5, 6), (5, 8), ]);

    }

    #[test]
    fn test_get_value_if_one_value_is_not_possible_in_neighbors() {
        let actual = Board::get_value_if_one_value_is_not_possible_in_neighbors(
            HashSet::from([1, 2, 3]),
            &HashSet::from([1, 2, 3, 4]),
        );
        assert_eq!(actual, Some(4));
        let actual = Board::get_value_if_one_value_is_not_possible_in_neighbors(
            HashSet::from([1, 2, 3]),
            &HashSet::from([1, 2]),
        );
        assert_eq!(actual, None);
        let actual = Board::get_value_if_one_value_is_not_possible_in_neighbors(
            HashSet::from([1, 2, 3]),
            &HashSet::from([1, 2, 3]),
        );
        assert_eq!(actual, None);
    }

    #[test]
    fn test_01() {
        let input = Board {
            array: array![
                [5, 3, 0, 0, 7, 0, 0, 0, 0],
                [6, 0, 0, 1, 9, 5, 0, 0, 0],
                [0, 9, 8, 0, 0, 0, 0, 6, 0],
                [8, 0, 0, 0, 6, 0, 0, 0, 3],
                [4, 0, 0, 8, 0, 3, 0, 0, 1],
                [7, 0, 0, 0, 2, 0, 0, 0, 6],
                [0, 6, 0, 0, 0, 0, 2, 8, 0],
                [0, 0, 0, 4, 1, 9, 0, 0, 5],
                [0, 0, 0, 0, 8, 0, 0, 7, 9]
            ],
        };

        let actual = input.solve_naive_implementation();

        let expected = Board {
            array: array![
                [5, 3, 4, 6, 7, 8, 9, 1, 2],
                [6, 7, 2, 1, 9, 5, 3, 4, 8],
                [1, 9, 8, 3, 4, 2, 5, 6, 7],
                [8, 5, 9, 7, 6, 1, 4, 2, 3],
                [4, 2, 6, 8, 5, 3, 7, 9, 1],
                [7, 1, 3, 9, 2, 4, 8, 5, 6],
                [9, 6, 1, 5, 3, 7, 2, 8, 4],
                [2, 8, 7, 4, 1, 9, 6, 3, 5],
                [3, 4, 5, 2, 8, 6, 1, 7, 9]
            ],
        };
        assert_eq!(actual.array, expected.array);
    }

    #[test]
    fn test_02() {
        let input = Board {
            array: array![
                [0, 3, 0, 0, 7, 0, 0, 0, 0],
                [6, 0, 0, 1, 9, 5, 0, 0, 0],
                [0, 9, 8, 0, 0, 0, 0, 6, 0],
                [8, 0, 0, 0, 6, 0, 0, 0, 3],
                [4, 0, 0, 8, 0, 3, 0, 0, 1],
                [7, 0, 0, 0, 0, 0, 0, 0, 6],
                [0, 6, 0, 0, 0, 0, 2, 8, 0],
                [0, 0, 0, 4, 1, 9, 0, 0, 5],
                [0, 0, 0, 0, 8, 0, 0, 7, 9]
            ],
        };

        let actual = input.solve_naive_implementation();

        let expected = Board {
            array: array![
                [5, 3, 4, 6, 7, 8, 9, 1, 2],
                [6, 7, 2, 1, 9, 5, 3, 4, 8],
                [1, 9, 8, 3, 4, 2, 5, 6, 7],
                [8, 5, 9, 7, 6, 1, 4, 2, 3],
                [4, 2, 6, 8, 5, 3, 7, 9, 1],
                [7, 1, 3, 9, 2, 4, 8, 5, 6],
                [9, 6, 1, 5, 3, 7, 2, 8, 4],
                [2, 8, 7, 4, 1, 9, 6, 3, 5],
                [3, 4, 5, 2, 8, 6, 1, 7, 9]
            ],
        };
        assert_eq!(actual.array, expected.array);
    }
}
