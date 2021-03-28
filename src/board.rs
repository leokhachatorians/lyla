#[derive(Debug)]
pub enum Peice {
    Pawn,
    Knight,
    Bishop,
    Rook,
    King,
    Queen,
    Empty,
}

#[derive(Debug)]
pub enum Color {
    White,
    Black,
    Empty,
}

pub struct Square {
    pub peice: Peice,
    pub color: Color,
}

impl Square {
    pub fn new(peice: Peice, color: Color) -> Square {
        Square { peice, color }
    }
}

pub struct Board {
    pub elements: Vec<Square>
}


impl Board {
    pub fn generate_empty_board() -> Board {
        let mut elements: Vec<Square> = Vec::new();

        // Generate Empty Board
        for _ in 0..64 {
        elements.push(
                Square {
                    peice: Peice::Empty,
                    color: Color::Empty,
                }
            )
        }

        Board { elements }
    }

    pub fn new(fen: &str) -> Board {
        let mut file = 0;
        let mut rank = 7;
        let mut board = Board::generate_empty_board();

        // Populate via fen string 
        for c in fen.chars() {
            if c == '/' {
                rank -= 1;
                file = 0;
            }
            else if c == ' ' {
                break;
            }
            else {
                if c.is_ascii_digit() {
                    file += c.to_digit(10).unwrap();
                }
                else {
                    let color = if c.is_uppercase() { Color::White } else { Color::Black };
                    let peice = match c.to_ascii_lowercase() {
                        'k' => Peice::King,
                        'q' => Peice::Queen,
                        'r' => Peice::Rook,
                        'n' => Peice::Knight,
                        'b' => Peice::Bishop,
                        'p' => Peice::Pawn,
                        _ => panic!("yeah idk what happened")
                    };

                    board.elements[(rank * 8 + file) as usize] = Square { color, peice };
                    file += 1;
                }
            }
        }

        board
    }
}
