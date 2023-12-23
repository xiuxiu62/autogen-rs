use std::fmt;

fn main() {}

#[macro_export]
macro_rules! piece {
    (bp) => {
        Some(Piece {
            kind: PieceKind::Pawn,
            color: PieceColor::Black,
        })
    };
    (br) => {
        Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::Black,
        })
    };
    (bh) => {
        Some(Piece {
            kind: PieceKind::Horse,
            color: PieceColor::Black,
        })
    };
    (bb) => {
        Some(Piece {
            kind: PieceKind::Bishop,
            color: PieceColor::Black,
        })
    };
    (bq) => {
        Some(Piece {
            kind: PieceKind::Queen,
            color: PieceColor::Black,
        })
    };
    (bk) => {
        Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::Black,
        })
    };
    (wp) => {
        Some(Piece {
            kind: PieceKind::Pawn,
            color: PieceColor::White,
        })
    };
    (wr) => {
        Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::White,
        })
    };
    (wh) => {
        Some(Piece {
            kind: PieceKind::Horse,
            color: PieceColor::White,
        })
    };
    (wb) => {
        Some(Piece {
            kind: PieceKind::Bishop,
            color: PieceColor::White,
        })
    };
    (wq) => {
        Some(Piece {
            kind: PieceKind::Queen,
            color: PieceColor::White,
        })
    };
    (wk) => {
        Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::White,
        })
    };
    (no) => {
        None
    };
}

#[macro_export]
#[rustfmt::skip]
macro_rules! default_board {
    () => {
        [
    /* 1 */ piece!(br), piece!(bh), piece!(bb), piece!(bq), piece!(bk), piece!(bb), piece!(bh), piece!(br),
    /* 2 */ piece!(bp), piece!(bp), piece!(bp), piece!(bp), piece!(bp), piece!(bp), piece!(bp), piece!(bp),
    /* 3 */ piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no),
    /* 4 */ piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no),
    /* 5 */ piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no),
    /* 6 */ piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no), piece!(no),
    /* 7 */ piece!(wp), piece!(wp), piece!(wp), piece!(wp), piece!(wp), piece!(wp), piece!(wp), piece!(wp),
    /* 8 */ piece!(wr), piece!(wh), piece!(wb), piece!(wq), piece!(wk), piece!(wb), piece!(wh), piece!(wr),
    //      a           b           c           d           e           f           g           h
        ]
    };
}

struct Piece {
    kind: PieceKind,
    color: PieceColor,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.color, self.kind)
    }
}

enum PieceKind {
    Pawn,
    Rook,
    Horse,
    Bishop,
    Queen,
    King,
}

impl fmt::Display for PieceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self {
            PieceKind::Pawn => 'p',
            PieceKind::Rook => 'r',
            PieceKind::Horse => 'h',
            PieceKind::Bishop => 'b',
            PieceKind::Queen => 'q',
            PieceKind::King => 'k',
        };

        write!(f, "{char}")
    }
}

// #[derive(Debug)]
enum PieceColor {
    Black,
    White,
}

impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self {
            PieceColor::Black => 'b',
            PieceColor::White => 'w',
        };

        write!(f, "{char}")
    }
}

#[repr(transparent)]
pub struct Board([Option<Piece>; 64]);

impl Board {
    pub fn apply(&mut self, Movement { from, to }: Movement) {
        println!("{from} {to}");
        self.0[to] = std::mem::take(&mut self.0[from]); // this avoids a shallow copy
    }
}

impl Default for Board {
    fn default() -> Self {
        Self(default_board!())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_markers = "  a  b  c  d  e  f  g  h";
        let separator = format!("  {}", "-".repeat(rank_markers.len() - 1));
        let board = self
            .0
            .iter()
            .enumerate()
            .fold("".to_owned(), |message, (i, piece)| {
                let morpheme = match piece {
                    Some(piece) => format!("{piece}"),
                    None => "  ".to_owned(),
                };

                match i % 8 {
                    0 => format!("{message}{}|{}", (i + 8) / 8, morpheme), // start of the file
                    7 => format!("{message}|{}\n{separator}\n", morpheme), // end of the file
                    _ => format!("{message}|{}", morpheme),                // middle of the file
                }
            });

        writeln!(f, "{board}{rank_markers}")
    }
}

struct BoardAgent {
    agent: Box<dyn Agent>,
}

#[derive(Debug, PartialEq, Eq)]
struct Movement {
    from: usize,
    to: usize,
}

impl Movement {
    // TODO: implement parse error and return results rather than options
    // TODO: enusure the direction of indexes is correct, i think UCI boards might go from bottom left to top right.
    //  if this is the case i need to flip the index ordering
    fn from_uci(data: &str) -> Option<Self> {
        if data.len() != 4 {
            return None; // ensure there are exactly 4 characters
        }

        // SAFETY(unwarp): we've already ensured there are 4 characters in the collection
        let [from_file, from_rank, to_file, to_rank] =
            data.chars().collect::<Vec<char>>().try_into().unwrap();

        let from = Self::convert_coordinates(from_file, from_rank)?;
        let to = Self::convert_coordinates(to_file, to_rank)?;

        Some(Self { from, to })
    }

    fn convert_coordinates(file: char, rank: char) -> Option<usize> {
        if !matches!(file, 'a'..='h') || !matches!(file, 'a'..='h') {
            return None;
        }

        // some ascii magic to save a few cycles
        let column = (file as u8 - b'a') as usize;
        let row = (rank as u8 - b'1') as usize;

        Some((7 - row) * 8 + column)
    }
}

#[cfg(test)]
mod test {
    use crate::{Board, Movement};

    #[test]
    fn movement_parsing_works() {
        let data_set = ["e2a4", "b3b5", "h7f5"];
        let expected = [
            Movement { from: 12, to: 24 },
            Movement { from: 17, to: 33 },
            Movement { from: 55, to: 37 },
        ];

        data_set
            .into_iter()
            .map(|value| (value, Movement::from_uci(value)))
            .zip(expected)
            .for_each(|((input, maybe_actual), expected)| match maybe_actual {
                Some(actual) => assert_eq!(actual, expected),
                None => panic!("Failed to parse a valid Movement"),
            });
    }

    #[test]
    fn movement_works() {
        let data_set = ["b2b3", "e7e5", "c1b2", "b8c6"];
        let mut board = Board::default();

        println!("{board}");
        data_set.into_iter().for_each(|value| {
            board.apply(Movement::from_uci(value).unwrap());

            println!("{board}");
        });
    }
}
