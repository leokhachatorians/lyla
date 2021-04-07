pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const MAILBOX: [i32; 120] = [
     -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
     -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
     -1,  0,  1,  2,  3,  4,  5,  6,  7, -1,
     -1,  8,  9, 10, 11, 12, 13, 14, 15, -1,
     -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
     -1, 24, 25, 26, 27, 28, 29, 30, 31, -1,
     -1, 32, 33, 34, 35, 36, 37, 38, 39, -1,
     -1, 40, 41, 42, 43, 44, 45, 46, 47, -1,
     -1, 48, 49, 50, 51, 52, 53, 54, 55, -1,
     -1, 56, 57, 58, 59, 60, 61, 62, 63, -1,
     -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
     -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

pub const MAILBOX64: [i32; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98
];


// Can this peice slide?
pub const CAN_SLIDE: [bool; 6] = [
    false, // Knight
    true,  // Bishop
    true,  // Rook
    true,  // Queen
    false, // King
    false, // Pawn
];

// How many directions a peice can move in
pub const DIRECTION_TOTALS: [i32; 6] = [
    8, // Knight
    4, // Bishop
    4, // Rook
    8, // Queen
    8, // King
    0,  // Pawn
];

// The direction a peice can move
pub const MOVE_DIRECTIONS: [[i32;8]; 6] = [
    [-21, -19, -12, -8, 8, 12, 19, 21],     // Knight
    [-11, -9, 9, 11, 0, 0, 0, 0],           // Bishop
    [-10, -1, 1, 10, 0, 0, 0, 0],           // Rook
    [-11, -10, -9, -1, 1, 9, 10, 11],       // Queen
    [-11, -10, -9, -1, 1, 9, 10, 11],       // King
    [0, 0, 0, 0, 0, 0, 0, 0],               // Pawn
];

// The offsets when sliding in a particular direction
pub const SLIDE_OFFSETS: [i32; 8] = [
 -11, -10, -9, -1, 1, 9, 10, 11
];
