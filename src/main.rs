mod chess {
    enum Color {
        Black,
        White,
    }

    enum Rank {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King,
    }

    struct Piece {
        color: Color,
        rank: Rank,
    }

    struct Board {
        matrix: [[Option<Piece>; 8]; 8],
    }
}

fn main() {
    println!("Hello, world!");
}
