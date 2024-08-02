mod chess {
    use core::panic;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Color {
        Black,
        White,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Rank {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Piece {
        color: Color,
        rank: Rank,
    }

    type Matrix = [[Option<Piece>; 8]; 8];

    trait ChessBoard {
        fn map_index(column: char, row: u8) -> (usize, usize);
        fn index(&self, column: usize, row: usize) -> &Option<Piece>;
        fn index_mut(&mut self, column: usize, row: usize) -> &mut Option<Piece>;
        fn remove(&mut self, column: usize, row: usize) -> Option<Piece>;
        fn verify(&self, position: (char, u8), piece: Option<Piece>) -> bool;
        fn change_position(&mut self, start: (char, u8), end: (char, u8));
    }

    impl ChessBoard for Matrix {
        fn map_index(column: char, row: u8) -> (usize, usize) {
            match (column, row) {
                (column @ ('A'..='H'), row @ (1..=8)) => {
                    ((column as usize) - 65, (row as usize) - 1)
                }
                _ => panic!("Index out of bounds"),
            }
        }

        fn index(&self, column: usize, row: usize) -> &Option<Piece> {
            &self[row][column]
        }

        fn index_mut(&mut self, column: usize, row: usize) -> &mut Option<Piece> {
            &mut self[row][column]
        }

        fn remove(&mut self, column: usize, row: usize) -> Option<Piece> {
            if let &Some(piece) = self.index(column, row) {
                *self.index_mut(column, row) = None;
                Some(piece)
            } else {
                None
            }
        }

        fn verify(&self, position: (char, u8), piece: Option<Piece>) -> bool {
            let (column, row) = Matrix::map_index(position.0, position.1);
            self.index(column, row) == &piece
        }

        fn change_position(&mut self, start: (char, u8), end: (char, u8)) {
            let (start_column, start_row) = Self::map_index(start.0, start.1);
            let (end_column, end_row) = Self::map_index(end.0, end.1);

            if let Some(piece) = self.remove(start_column, start_row) {
                *self.index_mut(end_column, end_row) = Some(piece);
                return;
            }
            panic!("Starting square is empty")
        }
    }
}

fn main() {
    println!("Hello, world!");
}
