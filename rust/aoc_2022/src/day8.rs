extern crate nalgebra as na;

#[derive(Debug, PartialEq, Clone)]
enum Visibility {
    Visible,
    Invisible,
}

#[derive(Debug, PartialEq, Clone)]
struct GridEntry {
    pub value: char,
    pub visibility: Visibility,
}

impl std::fmt::Display for GridEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {})",
            self.value,
            match self.visibility {
                Visibility::Visible => 'V',
                Visibility::Invisible => 'I',
            },
        )
    }
}

pub struct Grid {
    data: na::DMatrix<GridEntry>,
}

impl Grid {
    pub fn new(construct_from: &str) -> Self {
        let rows: usize = construct_from.lines().count();
        let cols: usize = construct_from
            .chars()
            .filter(|c| !c.is_whitespace())
            .count()
            / rows;

        let mut grid = Self {
            data: na::DMatrix::from_row_iterator(
                rows,
                cols,
                construct_from
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .map(|c| GridEntry {
                        value: c,
                        visibility: Visibility::Invisible,
                    }),
            ),
        };

        for (i, mut row) in grid.data.row_iter_mut().enumerate() {
            let mut highest_seen = '0';
            for (j, mut item) in row.column_iter_mut().enumerate() {
                if i == 0 {
                    item.get_mut(0).unwrap().visibility = Visibility::Visible;
                } else if j == 0 || j == cols - 1 {
                    item.get_mut(0).unwrap().visibility = Visibility::Visible;
                    highest_seen = item.get_mut(0).unwrap().value;
                } else if item.get_mut(0).unwrap().value > highest_seen {
                    item.get_mut(0).unwrap().visibility = Visibility::Visible;
                    highest_seen = item.get_mut(0).unwrap().value
                }
            }

            highest_seen = '0';
            for (j, mut item) in row.column_iter_mut().enumerate().rev() {
                if i == 0 {
                    item.get_mut(0).unwrap().visibility = Visibility::Visible;
                } else if j == 0 || j == cols - 1 {
                    item.get_mut(0).unwrap().visibility = Visibility::Visible;
                    highest_seen = item.get_mut(0).unwrap().value;
                } else if item.get_mut(0).unwrap().value > highest_seen {
                    item.get_mut(0).unwrap().visibility = Visibility::Visible;
                    highest_seen = item.get_mut(0).unwrap().value
                }
            }
        }

        for (i, mut col) in grid.data.column_iter_mut().enumerate() {
            let mut highest_seen = '0';
            for (j, mut item) in col.row_iter_mut().enumerate() {
                if i == 0 {
                    item.get_mut(0).unwrap().visibility = Visibility::Visible;
                } else if j == 0 || j == cols - 1 {
                    item.get_mut(0).unwrap().visibility = Visibility::Visible;
                    highest_seen = item.get_mut(0).unwrap().value;
                } else if item.get_mut(0).unwrap().value > highest_seen {
                    item.get_mut(0).unwrap().visibility = Visibility::Visible;
                    highest_seen = item.get_mut(0).unwrap().value
                }
            }

            highest_seen = '0';
            for j in (0..rows).into_iter().rev() {
                if i == 0 {
                    col.get_mut(j).unwrap().visibility = Visibility::Visible;
                } else if j == 0 || j == cols - 1 {
                    col.get_mut(j).unwrap().visibility = Visibility::Visible;
                    highest_seen = col.get_mut(j).unwrap().value;
                } else if col.get_mut(j).unwrap().value > highest_seen {
                    col.get_mut(j).unwrap().visibility = Visibility::Visible;
                    highest_seen = col.get_mut(j).unwrap().value
                }
            }
        }

        grid
    }

    pub fn print(&self) {
        println!("matrix is {}", self.data);
    }

    pub fn count_visible(&self) -> usize {
        self.data
            .iter()
            .filter(|item| item.visibility == Visibility::Visible)
            .count()
    }

    pub fn highest_scenic_view_score(&self) -> usize {
        let (nrows, ncols) = (self.data.nrows(), self.data.ncols());
        let mut highest_score = 0;

        for row in 1..nrows - 1 {
            for col in 1..ncols - 1 {
                // look in each direction
                let mut right_score = 0;
                let mut left_score = 0;
                let mut up_score = 0;
                let mut down_score = 0;

                for tree in self
                    .data
                    .row(row)
                    .columns(col + 1, ncols - col - 1)
                    .into_iter()
                {
                    right_score += 1;
                    if tree.value >= self.data[(row, col)].value {
                        break;
                    }
                }
                for tree in self.data.row(row).columns(0, col).into_iter().rev() {
                    left_score += 1;
                    if tree.value >= self.data[(row, col)].value {
                        break;
                    }
                }
                for tree in self
                    .data
                    .column(col)
                    .rows(row + 1, nrows - row - 1)
                    .into_iter()
                {
                    down_score += 1;
                    if tree.value >= self.data[(row, col)].value {
                        break;
                    }
                }
                for tree in self.data.column(col).rows(0, row).into_iter().rev() {
                    up_score += 1;
                    if tree.value >= self.data[(row, col)].value {
                        break;
                    }
                }
                let cur_score = left_score * right_score * up_score * down_score;
                if cur_score > highest_score {
                    highest_score = cur_score
                }
            }
        }
        highest_score
    }
}

#[cfg(test)]
mod test {
    use super::super::file;
    use super::*;

    #[test]
    fn print_sample_input() {
        let string = file::get_string_from_file("assets/day8_sample.txt");
        let grid = Grid::new(&string);
        grid.print();
    }

    #[test]
    fn expect_visible_count() {
        let string = file::get_string_from_file("assets/day8_sample.txt");
        let grid = Grid::new(&string);
        assert_eq!(21, grid.count_visible());
    }

    #[test]
    fn get_highest_view_score() {
        let string = file::get_string_from_file("assets/day8_sample.txt");
        let grid = Grid::new(&string);
        assert_eq!(8, grid.highest_scenic_view_score());
    }
}
