use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

pub enum GroupKind {
    Row,
    Column,
    Block,
    All,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoardItem {
    pub value: Option<u8>,
    candidates: [bool; 9],
}

impl BoardItem {
    pub fn known(value: u8) -> Self {
        Self {
            value: Some(value),
            candidates: [
                false, false, false, false, false, false, false, false, false,
            ],
        }
    }

    pub fn unknown() -> Self {
        Self {
            value: None,
            candidates: [true, true, true, true, true, true, true, true, true],
        }
    }

    fn get_candidates(&self) -> Vec<u8> {
        self.candidates
            .iter()
            .enumerate()
            .filter_map(|(i, value): (usize, &bool)| -> Option<u8> {
                if !value {
                    return None;
                }
                u8::try_from(i + 1).ok()
            })
            .collect()
    }

    fn remove_candidate(&mut self, value: u8) {
        self.candidates[usize::from(value - 1)] = false;
    }

    fn set_value(&mut self, value: u8) {
        self.candidates = [false, false, false, false, false, false, false, false, false, ];
        self.value = Some(value);
    }

    fn is_solved(&self) -> bool{
        self.value.is_some()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Board {
    pub array: [[BoardItem; 9]; 9],
    candidates: [[HashSet<u8>; 9]; 9],
}

impl Board {
    const NUMBER_OF_ROWS: usize = 9;
    const NUMBER_OF_COLS: usize = 9;
    const NUMBER_OF_CELLS: usize = Self::NUMBER_OF_ROWS * Self::NUMBER_OF_COLS;
    const TOTAL_RANGE: Range<usize> = 0..Self::NUMBER_OF_ROWS;
    const BLOCK_WIDTH: usize = 3;
    const BLOCK_HEIGHT: usize = 3;
    const OFFSET_BLOCK_WIDTH: usize = Self::BLOCK_WIDTH - 1;
    const OFFSET_BLOCK_HEIGHT: usize = Self::BLOCK_HEIGHT - 1;

    pub fn new(array: [[BoardItem; 9]; 9]) -> Self {
        Self {
            array,
            ..Default::default()
        }
    }

    fn solved_pct(&self) -> f64 {
        let nb_undefined = self.get_undefined_indexes().len();
        let numerator = (Self::NUMBER_OF_CELLS - nb_undefined) as f64;
        100.0 * numerator / (Self::NUMBER_OF_CELLS as f64)
    }

    fn get_all_indexes() -> Vec<(usize, usize)> {
        let mut block_indexes: Vec<(usize, usize)> = vec![];
        for row_index in Self::TOTAL_RANGE {
            for col_index in Self::TOTAL_RANGE {
                block_indexes.push((row_index, col_index));
            }
        }
        block_indexes
    }

    fn get_row(&self, row: usize) -> Vec<&BoardItem> {
        self.array[row].iter().collect()
    }

    fn get_col(&self, col: usize) -> Vec<&BoardItem> {
        self.array
            .iter()
            .map(|row_array| &row_array[col])
            .collect()
    }

    pub fn get_block_bounds_from_index(row: usize, col: usize) -> (usize, usize, usize, usize) {
        let start_row = (row / Self::BLOCK_WIDTH) * Self::BLOCK_WIDTH;
        let end_row = start_row + Self::OFFSET_BLOCK_WIDTH;
        let start_col = (col / Self::BLOCK_HEIGHT) * Self::BLOCK_HEIGHT;
        let end_col = start_col + Self::OFFSET_BLOCK_HEIGHT;
        (start_row, end_row, start_col, end_col)
    }

    fn get_block(&self, row: usize, col: usize) -> Vec<&BoardItem> {
        let (start_row, end_row, start_col, end_col) = Self::get_block_bounds_from_index(row, col);
        self.array
            .iter()
            .enumerate()
            .filter_map(|(i_row, row_array)| {
                if i_row < start_row || i_row > end_row {
                    return None;
                }
                Some(
                    row_array
                        .iter()
                        .enumerate()
                        .filter_map(|(i_col, val)| {
                            if i_col < start_col || i_col > end_col {
                                return None;
                            }
                            Some(val)
                        })
                        .collect::<Vec<&BoardItem>>(),
                )
            })
            .flatten()
            .collect()
    }

    fn get_row_neighbor_indexes(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let row_indexes: Vec<(usize, usize)> = Self::TOTAL_RANGE
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
        let col_indexes: Vec<(usize, usize)> = Self::TOTAL_RANGE
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

    fn update_all_candidates(&mut self) {
        for (row, col) in Board::get_all_indexes() {
            self.update_candidates(row,col);
        }
    }
    fn update_candidates(&mut self, row: usize, col: usize){
        let neighbor_indexes = self
            .get_row_neighbor_indexes(row, col)
            .into_iter()
            .chain(self.get_col_neighbor_indexes(row, col))
            .chain(self.get_block_neighbor_indexes(row, col))
            .collect::<Vec<(usize, usize)>>();
            neighbor_indexes
                .iter()
                .for_each(|(i_row, i_col)| {
                let val = self.array[*i_row][*i_col].value;
                if let Some(val) = val{
                    self.array[row][col].remove_candidate(val);
                }
            }
            )
        ;
    }

    fn evaluate_candidate_values(&self, row: usize, col: usize) -> HashSet<u8> {
        let candidate_values: HashSet<u8> = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let neighbor_indexes = self
            .get_row_neighbor_indexes(row, col)
            .into_iter()
            .chain(self.get_col_neighbor_indexes(row, col))
            .chain(self.get_block_neighbor_indexes(row, col))
            .collect::<Vec<(usize, usize)>>();
        let neighbor_values: HashSet<u8> = HashSet::from_iter(
            neighbor_indexes
                .iter()
                .filter_map(|(i_row, i_col)| self.array[*i_row][*i_col].value),
        );

        candidate_values
            .difference(&neighbor_values)
            .cloned()
            .collect()

    }

    fn get_undefined_indexes(&self) -> Vec<(usize, usize)> {
        let output: Vec<(usize, usize)> = Self::get_all_indexes()
            .iter()
            .filter_map(|(i_row, i_col)| {
                let val = self.array[*i_row][*i_col].value;
                if val.is_none() {
                    Some((*i_row, *i_col))
                } else {
                    None
                }

            })
            .collect();
        output
    }

    fn set_value(&mut self, row: usize, col: usize, value: u8) {
        self.array[row][col].set_value(value);
        // remove this value from neighbors candidates
        self.get_row_neighbor_indexes(row, col)
            .iter()
            .chain(self.get_col_neighbor_indexes(row, col).iter())
            .chain(self.get_block_neighbor_indexes(row, col).iter())
            .for_each(|(r, c)| {
                // if there is only one candidate, it's already solved
                if self.array[*r][*c].is_solved() {
                    return;
                }
                self.array[*r][*c].remove_candidate(value);

                // if there only one candidate, we can set the value to this candidate
                let candidates = self.array[*r][*c].get_candidates();
                if candidates.len() == 1 {
                    self.set_value(*r, *c, *candidates.first().unwrap());
                }
            });
    }

    fn get_value_if_one_value_is_not_possible_in_neighbors(
        neighbor_values: HashSet<u8>,
        candidate_values: &Vec<u8>,
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

    pub fn get_value_if_only_one_candidate(candidate_values: &Vec<u8>) -> Option<u8> {
        if candidate_values.len() != 1 {
            return None;
        }
        let value = *candidate_values.iter().next().unwrap();
        Some(value)
    }

    pub fn get_value_if_not_candidate_in_neighbors(
        neighbor_values: HashSet<u8>,
        candidate_values: &Vec<u8>,
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
        let mut output = Self::new(self.array.to_owned());
        let mut nb_undefined_values: usize = 0;

        output.update_all_candidates();

        loop {
            let undefined_indexes = output.get_undefined_indexes();
            let still_undefined_values: usize = undefined_indexes.len();

            if undefined_indexes.is_empty() || still_undefined_values == nb_undefined_values {
                break;
            }

            nb_undefined_values = still_undefined_values;

            'traversing_board: for (row, col) in &undefined_indexes {
                if let Some(value) =
                    Board::get_value_if_only_one_candidate(&output.array[*row][*col].get_candidates())
                {
                    output.set_value(*row, *col, value);
                    continue;
                }

                for kind in [GroupKind::Row, GroupKind::Column, GroupKind::Block] {
                    let neighbor_ids = match kind {
                        GroupKind::Row => output.get_row_neighbor_indexes(*row, *col),
                        GroupKind::Column => output.get_col_neighbor_indexes(*row, *col),
                        GroupKind::Block => output.get_block_neighbor_indexes(*row, *col),
                        GroupKind::All => todo!(),
                    };
                    let neighbor_candidate_values: Vec<HashSet<u8>> = neighbor_ids
                        .iter()
                        .map(|(r, c)| output.evaluate_candidate_values(*r, *c))
                        .collect();
                    let unique_neighbor_values: HashSet<u8> = neighbor_candidate_values
                        .iter()
                        .flat_map(|x| x.clone())
                        .collect();

                    if let Some(value) = Board::get_value_if_not_candidate_in_neighbors(
                        unique_neighbor_values,
                        &output.array[*row][*col].get_candidates(),
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
                    let to_remove_from_candidates: HashSet<u8> = acc
                        .iter()
                        .filter_map(|(values, count)| {
                            if (*values).len() != *count {
                                return None;
                            }
                            Some(values.clone())
                        })
                        .flatten()
                        .collect();
                    if !to_remove_from_candidates.is_empty() {
                        // println!("row {} col {}", row, col);
                        // println!("board {:?}", output);
                        // println!("candidate_values {:?}", candidate_values);
                        // println!("neighbor_candidate_values {:?}", neighbor_candidate_values);
                        // println!("counts {:?}", acc);
                        // println!("to_remove_from_candidates {:?}", to_remove_from_canditates);

                        for val in to_remove_from_candidates {
                            output.array[*row][*col].remove_candidate(val);
                        }

                        if let Some(value) = Board::get_value_if_only_one_candidate(
                            &output.array[*row][*col].get_candidates(),
                        ) {
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
    use std::{collections::HashSet, vec};

    #[test]
    fn test_board_item() {
        let mut board_item = BoardItem {
            value: None,
            candidates: [true, false, true, false, false, true, true, true, true],
        };
        assert_eq!(board_item.candidates.len(), 9);

        let actual = board_item.get_candidates();
        assert_eq!(actual.len(), 6);
        assert_eq!(actual, vec![1, 3, 6, 7, 8, 9]);

        board_item.remove_candidate(6);
        let actual = board_item.get_candidates();

        assert_eq!(actual, vec![1, 3, 7, 8, 9]);

        let known_item = BoardItem::known(2);
        assert_eq!(known_item.value, Some(2));
        assert_eq!(known_item.get_candidates(), vec![]);

        let unknown_item = BoardItem::unknown();
        assert_eq!(unknown_item.value, None);
        assert_eq!(
            unknown_item.get_candidates(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

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

    #[rustfmt::skip]
    #[test]
    fn test_get_all_indexes() {
        let actual = Board::get_all_indexes();
        assert_eq!(actual ,[
            (0,0), (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7), (0,8),
            (1,0), (1,1), (1,2), (1,3), (1,4), (1,5), (1,6), (1,7), (1,8),
            (2,0), (2,1), (2,2), (2,3), (2,4), (2,5), (2,6), (2,7), (2,8),
            (3,0), (3,1), (3,2), (3,3), (3,4), (3,5), (3,6), (3,7), (3,8),
            (4,0), (4,1), (4,2), (4,3), (4,4), (4,5), (4,6), (4,7), (4,8),
            (5,0), (5,1), (5,2), (5,3), (5,4), (5,5), (5,6), (5,7), (5,8),
            (6,0), (6,1), (6,2), (6,3), (6,4), (6,5), (6,6), (6,7), (6,8),
            (7,0), (7,1), (7,2), (7,3), (7,4), (7,5), (7,6), (7,7), (7,8),
            (8,0), (8,1), (8,2), (8,3), (8,4), (8,5), (8,6), (8,7), (8,8),
        ]);
    }

    #[test]
    fn test_set_methods() {
        let mut input = Board::new([
            [BoardItem::known(5), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1), BoardItem::known(9), BoardItem::known(5), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::known(9), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown()],
            [BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(3)],
            [BoardItem::known(4), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1)],
            [BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(2), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6)],
            [BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(2), BoardItem::known(8), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(4), BoardItem::known(1), BoardItem::known(9), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(5)],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::known(9)],
        ]);
        input.set_value(0, 2, 9);
        let actual = input.array[0][2].value.unwrap();
        assert_eq!(actual, 9);
    }

    #[rustfmt::skip]
    #[test]
    fn test_access_methods() {
        let input = Board::new(
            [
                [BoardItem::known(5), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
                [BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1), BoardItem::known(9), BoardItem::known(5), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
                [BoardItem::unknown(), BoardItem::known(9), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown()],
                [BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(3)],
                [BoardItem::known(4), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1)],
                [BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(2), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6)],
                [BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(2), BoardItem::known(8), BoardItem::unknown()],
                [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(4), BoardItem::known(1), BoardItem::known(9), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(5)],
                [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::known(9)]
            ],
        );

        let actual: Vec<BoardItem> = input.get_row(0).into_iter().cloned().collect();
        assert_eq!(actual, [BoardItem::known(5), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()]);
        let actual: Vec<BoardItem> = input.get_row(2).into_iter().cloned().collect();
        assert_eq!(actual, [BoardItem::unknown(), BoardItem::known(9), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown()]);
        let actual: Vec<BoardItem> = input.get_col(0).into_iter().cloned().collect();
        assert_eq!(actual, [BoardItem::known(5), BoardItem::known(6), BoardItem::unknown(), BoardItem::known(8), BoardItem::known(4), BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()]);
        let actual: Vec<BoardItem> = input.get_col(8).into_iter().cloned().collect();
        assert_eq!(actual, [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(3), BoardItem::known(1), BoardItem::known(6), BoardItem::unknown(), BoardItem::known(5), BoardItem::known(9)]);
        let actual: Vec<BoardItem> = input.get_block(0, 0).into_iter().cloned().collect();
        assert_eq!(actual, [BoardItem::known(5), BoardItem::known(3), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(9), BoardItem::known(8)]);
        let actual: Vec<BoardItem> = input.get_block(3, 0).into_iter().cloned().collect();
        assert_eq!(actual, [BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(4), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown()]);
        let actual: Vec<BoardItem> = input.get_block(3, 3).into_iter().cloned().collect();
        assert_eq!(actual, [BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::known(3), BoardItem::unknown(), BoardItem::known(2), BoardItem::unknown()]);
        let actual = input.evaluate_candidate_values(3, 3);
        assert_eq!(actual, HashSet::from([5, 7, 9]));
        let actual = input.evaluate_candidate_values(7, 8);
        assert_eq!(actual, HashSet::from([5]));
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
        let input = Board::new(
            [
                [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(),],
                [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(),],
                [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(),],
                [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(),],
                [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(),],
                [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(),],
                [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(),],
                [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(),],
                [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(),]
            ],
        );

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
            &vec![1, 2, 3, 4],
        );
        assert_eq!(actual, Some(4));
        let actual = Board::get_value_if_one_value_is_not_possible_in_neighbors(
            HashSet::from([1, 2, 3]),
            &vec![1, 2],
        );
        assert_eq!(actual, None);
        let actual = Board::get_value_if_one_value_is_not_possible_in_neighbors(
            HashSet::from([1, 2, 3]),
            &vec![1, 2, 3],
        );
        assert_eq!(actual, None);
    }

    #[test]
    fn test_01() {
        let input = Board::new([
            [BoardItem::known(5), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1), BoardItem::known(9), BoardItem::known(5), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::known(9), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown()],
            [BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(3)],
            [BoardItem::known(4), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1)],
            [BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(2), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6)],
            [BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(2), BoardItem::known(8), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(4), BoardItem::known(1), BoardItem::known(9), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(5)],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::known(9)],
        ]);

        let actual = input.solve_naive_implementation();

        let expected = Board::new([
            [BoardItem::known(5), BoardItem::known(3), BoardItem::known(4), BoardItem::known(6), BoardItem::known(7), BoardItem::known(8), BoardItem::known(9), BoardItem::known(1), BoardItem::known(2)],
            [BoardItem::known(6), BoardItem::known(7), BoardItem::known(2), BoardItem::known(1), BoardItem::known(9), BoardItem::known(5), BoardItem::known(3), BoardItem::known(4), BoardItem::known(8)],
            [BoardItem::known(1), BoardItem::known(9), BoardItem::known(8), BoardItem::known(3), BoardItem::known(4), BoardItem::known(2), BoardItem::known(5), BoardItem::known(6), BoardItem::known(7)],
            [BoardItem::known(8), BoardItem::known(5), BoardItem::known(9), BoardItem::known(7), BoardItem::known(6), BoardItem::known(1), BoardItem::known(4), BoardItem::known(2), BoardItem::known(3)],
            [BoardItem::known(4), BoardItem::known(2), BoardItem::known(6), BoardItem::known(8), BoardItem::known(5), BoardItem::known(3), BoardItem::known(7), BoardItem::known(9), BoardItem::known(1)],
            [BoardItem::known(7), BoardItem::known(1), BoardItem::known(3), BoardItem::known(9), BoardItem::known(2), BoardItem::known(4), BoardItem::known(8), BoardItem::known(5), BoardItem::known(6)],
            [BoardItem::known(9), BoardItem::known(6), BoardItem::known(1), BoardItem::known(5), BoardItem::known(3), BoardItem::known(7), BoardItem::known(2), BoardItem::known(8), BoardItem::known(4)],
            [BoardItem::known(2), BoardItem::known(8), BoardItem::known(7), BoardItem::known(4), BoardItem::known(1), BoardItem::known(9), BoardItem::known(6), BoardItem::known(3), BoardItem::known(5)],
            [BoardItem::known(3), BoardItem::known(4), BoardItem::known(5), BoardItem::known(2), BoardItem::known(8), BoardItem::known(6), BoardItem::known(1), BoardItem::known(7), BoardItem::known(9)],
        ]);
        assert_eq!(actual.solved_pct(), 100.0);
        assert_eq!(actual.array, expected.array);
    }

    #[test]
    fn test_02() {
        let input = Board::new([
            [BoardItem::unknown(), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1), BoardItem::known(9), BoardItem::known(5), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::known(9), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown()],
            [BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(3)],
            [BoardItem::known(4), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1)],
            [BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6)],
            [BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(2), BoardItem::known(8), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(4), BoardItem::known(1), BoardItem::known(9), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(5)],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::known(9)],
        ]);

        let actual = input.solve_naive_implementation();

        let expected = Board::new([
            [BoardItem::known(5), BoardItem::known(3), BoardItem::known(4), BoardItem::known(6), BoardItem::known(7), BoardItem::known(8), BoardItem::known(9), BoardItem::known(1), BoardItem::known(2)],
            [BoardItem::known(6), BoardItem::known(7), BoardItem::known(2), BoardItem::known(1), BoardItem::known(9), BoardItem::known(5), BoardItem::known(3), BoardItem::known(4), BoardItem::known(8)],
            [BoardItem::known(1), BoardItem::known(9), BoardItem::known(8), BoardItem::known(3), BoardItem::known(4), BoardItem::known(2), BoardItem::known(5), BoardItem::known(6), BoardItem::known(7)],
            [BoardItem::known(8), BoardItem::known(5), BoardItem::known(9), BoardItem::known(7), BoardItem::known(6), BoardItem::known(1), BoardItem::known(4), BoardItem::known(2), BoardItem::known(3)],
            [BoardItem::known(4), BoardItem::known(2), BoardItem::known(6), BoardItem::known(8), BoardItem::known(5), BoardItem::known(3), BoardItem::known(7), BoardItem::known(9), BoardItem::known(1)],
            [BoardItem::known(7), BoardItem::known(1), BoardItem::known(3), BoardItem::known(9), BoardItem::known(2), BoardItem::known(4), BoardItem::known(8), BoardItem::known(5), BoardItem::known(6)],
            [BoardItem::known(9), BoardItem::known(6), BoardItem::known(1), BoardItem::known(5), BoardItem::known(3), BoardItem::known(7), BoardItem::known(2), BoardItem::known(8), BoardItem::known(4)],
            [BoardItem::known(2), BoardItem::known(8), BoardItem::known(7), BoardItem::known(4), BoardItem::known(1), BoardItem::known(9), BoardItem::known(6), BoardItem::known(3), BoardItem::known(5)],
            [BoardItem::known(3), BoardItem::known(4), BoardItem::known(5), BoardItem::known(2), BoardItem::known(8), BoardItem::known(6), BoardItem::known(1), BoardItem::known(7), BoardItem::known(9)],
        ]);
        // assert_eq!(actual.solved_pct(), 100.0);
        assert_eq!(actual.array, expected.array);
    }
}
