use bitflags::bitflags;

type Position = u64;

fn bit_to_position(bit: Position) -> Result<String, String> {
    if bit == 0 {
        return Err("No piece present".to_string());
    }

    let index = bit_scan(bit);
    return Ok(index_to_position(index));
}

fn index_to_position(index: usize) -> String {
    let row = index / 8 + 1;
    let col = index % 8;
    let col = (b'a' as u8 + col as u8) as char;
    return format!("{}{}", col, row);
}

const MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23, 3, 12, 16, 59, 41, 19, 24, 54, 4, 64, 13, 10, 17, 62, 60, 28, 42,
    30, 20, 51, 25, 44, 55, 47, 5, 32, 64, 38, 14, 22, 11, 58, 18, 53, 63, 9, 61, 27, 29, 50, 43,
    46, 31, 37, 21, 57, 52, 8, 26, 49, 45, 36, 56, 7, 48, 35, 6, 34, 33,
];

fn bit_scan(bit: Position) -> usize {
    let remainder = bit % 67;
    return MOD67TABLE[remainder as usize];
}

#[derive(Debug, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq)]
struct Piece {
    position: Position,
    color: Color,
    piece_type: PieceType,
}

#[derive(Debug)]
enum Square {
    Empty,
    Occupied(usize),
}

struct Game {
    pieces: Vec<Piece>,
    squares: Vec<Square>,
    turn: Color,
    castling_rights: CastlingRights,
    en_passant: Option<Position>,
    half_move_clock: u8,
    full_move_number: u16,
}

bitflags! {
    #[derive(Debug)]
    struct CastlingRights: u8 {
        const WHITE_KING_SIDE = 0b0001;
        const WHITE_QUEEN_SIDE = 0b0010;
        const BLACK_KING_SIDE = 0b0100;
        const BLACK_QUEEN_SIDE = 0b1000;
    }
}

impl Game {
    fn to_string(&self) -> String {
        let mut board = String::new();
        let mut temp = String::new();

        for (index, square) in self.squares.iter().enumerate() {
            match square {
                Square::Empty => temp.push_str(&index_to_position(index)),
                Square::Occupied(piece_index) => {
                    temp.push_str(&self.pieces[*piece_index].to_string())
                }
            }

            if (index + 1) % 8 == 0 {
                temp.push_str("\n");
                board.insert_str(0, &temp);
                temp.clear();
            }
        }

        board.insert_str(0, &temp);

        return board;
    }

    fn from_fen(fen: &str) -> Game {
        let mut game = Game {
            pieces: vec![],
            squares: vec![],
            turn: Color::White,
            castling_rights: CastlingRights::all(),
            en_passant: None,
            half_move_clock: 0,
            full_move_number: 1,
        };

        return game;
    }
}

impl Piece {
    fn to_string(&self) -> String {
        let mut result = match self.piece_type {
            PieceType::Pawn => "p ",
            PieceType::Knight => "n ",
            PieceType::Bishop => "b ",
            PieceType::Rook => "r ",
            PieceType::Queen => "q ",
            PieceType::King => "k ",
        }
        .to_string();

        if self.color == Color::White {
            result.make_ascii_uppercase();
        }

        return result;
    }
}

fn main() {
    let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{}", game.to_string());
}
