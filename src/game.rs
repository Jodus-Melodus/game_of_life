pub struct Game {
    board: Vec<Vec<u8>>,
    generation: u32,
    size: usize,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Game {
            board: vec![vec![0; size]; size],
            generation: 0,
            size,
        }
    }

    pub fn from_vec_matrix(board: Vec<Vec<u8>>) -> Self {
        Game {
            board: board.clone(),
            generation: 0,
            size: board.len(),
        }
    }

    pub fn step(&self) -> Self {
        let mut new_board = vec![vec![0; self.size]; self.size];

        for row in 1..self.size - 1 {
            for col in 1..self.size - 1 {
                let neighbour_count = self.count_alive_neighbours(row, col);
                if self.board[row][col] == 1 {
                    if neighbour_count < 2 || neighbour_count > 3 {
                        new_board[row][col] = 0;
                    } else {
                        new_board[row][col] = 1;
                    }
                } else {
                    if neighbour_count == 3 {
                        new_board[row][col] = 1;
                    }
                }
            }
        }

        Game {
            board: new_board,
            generation: self.generation + 1,
            size: self.size,
        }
    }

    pub fn show(&self) {
        for row in &self.board {
            let row_str: String = row
                .iter()
                .map(|&cell| if cell == 1 { '#' } else { ' ' })
                .collect();
            println!("{}", row_str);
        }
    }

    fn count_alive_neighbours(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        let neighbor_offsets: [(isize, isize); 8] = [
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        for (dx, dy) in neighbor_offsets.iter() {
            let new_row = (row as isize + dx) as usize;
            let new_col = (col as isize + dy) as usize;

            if new_row < self.size && new_col < self.size && self.board[new_row][new_col] == 1 {
                count += 1;
            }
        }

        count
    }
}
