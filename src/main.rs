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
}

fn main() {
    println!("Hello, world!");
}
