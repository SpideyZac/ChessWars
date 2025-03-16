const KNIGHT_MOVES: [(i8, i8); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

const KING_MOVES: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct Piece {
    piece_type: PieceType,
    position: (u16, u16),
    board_size: u16,
}

impl Piece {
    pub fn new(piece_type: PieceType, position: (u16, u16), board_size: u16) -> Self {
        Self {
            piece_type,
            position,
            board_size,
        }
    }

    pub fn get_legal_moves(&self, board_pieces: &Vec<Piece>) -> Vec<(u16, u16)> {
        match self.piece_type {
            PieceType::Pawn => self.get_king_moves(), // Pawns can move the same as a king
            PieceType::Rook => self.get_rook_moves(board_pieces),
            PieceType::Knight => self.get_knight_moves(),
            PieceType::Bishop => self.get_bishop_moves(board_pieces),
            PieceType::Queen => self // Queens can move like rooks and bishops
                .get_rook_moves(board_pieces)
                .iter()
                .chain(self.get_bishop_moves(board_pieces).iter())
                .copied()
                .collect(),
            PieceType::King => self.get_king_moves(),
        }
    }

    fn get_rook_moves(&self, board_pieces: &Vec<Piece>) -> Vec<(u16, u16)> {
        let mut moves = vec![];

        // Check moves to the right
        for x in (self.position.0 + 1)..self.board_size {
            moves.push((x, self.position.1));
            // Check if there is a piece in the way, if so stop checking
            if board_pieces
                .iter()
                .any(|piece| piece.position == (x, self.position.1))
            {
                break;
            }
        }

        // Check moves to the left
        for x in (0..self.position.0).rev() {
            moves.push((x, self.position.1));
            // Check if there is a piece in the way, if so stop checking
            if board_pieces
                .iter()
                .any(|piece| piece.position == (x, self.position.1))
            {
                break;
            }
        }

        // Check moves up
        for y in (self.position.1 + 1)..self.board_size {
            moves.push((self.position.0, y));
            // Check if there is a piece in the way, if so stop checking
            if board_pieces
                .iter()
                .any(|piece| piece.position == (self.position.0, y))
            {
                break;
            }
        }

        // Check moves down
        for y in (0..self.position.1).rev() {
            moves.push((self.position.0, y));
            // Check if there is a piece in the way, if so stop checking
            if board_pieces
                .iter()
                .any(|piece| piece.position == (self.position.0, y))
            {
                break;
            }
        }

        moves
    }

    fn get_knight_moves(&self) -> Vec<(u16, u16)> {
        let mut moves = Vec::with_capacity(8);
        for (dx, dy) in KNIGHT_MOVES.iter() {
            let new_x = self.position.0 as i32 + *dx as i32;
            let new_y = self.position.1 as i32 + *dy as i32;

            // Check if the new position is within the board
            if new_x >= 0
                && new_x < self.board_size as i32
                && new_y >= 0
                && new_y < self.board_size as i32
            {
                moves.push((new_x as u16, new_y as u16));
            }
        }

        moves
    }

    fn get_bishop_moves(&self, board_pieces: &Vec<Piece>) -> Vec<(u16, u16)> {
        let mut moves = vec![];

        // Check moves to the top right
        for i in 1..self.board_size {
            let new_x = self.position.0 as i32 + i as i32;
            let new_y = self.position.1 as i32 + i as i32;

            // Check if the new position is within the board
            if new_x >= self.board_size as i32 || new_y >= self.board_size as i32 {
                break;
            }

            moves.push((new_x as u16, new_y as u16));

            // Check if there is a piece in the way, if so stop checking
            if board_pieces
                .iter()
                .any(|piece| piece.position == (new_x as u16, new_y as u16))
            {
                break;
            }
        }

        // Check moves to the top left
        for i in 1..self.board_size {
            let new_x = self.position.0 as i32 - i as i32;
            let new_y = self.position.1 as i32 + i as i32;

            // Check if the new position is within the board
            if new_x < 0 || new_y >= self.board_size as i32 {
                break;
            }

            moves.push((new_x as u16, new_y as u16));

            // Check if there is a piece in the way, if so stop checking
            if board_pieces
                .iter()
                .any(|piece| piece.position == (new_x as u16, new_y as u16))
            {
                break;
            }
        }

        // Check moves to the bottom right
        for i in 1..self.board_size {
            let new_x = self.position.0 as i32 + i as i32;
            let new_y = self.position.1 as i32 - i as i32;

            // Check if the new position is within the board
            if new_x >= self.board_size as i32 || new_y < 0 {
                break;
            }

            moves.push((new_x as u16, new_y as u16));

            // Check if there is a piece in the way, if so stop checking
            if board_pieces
                .iter()
                .any(|piece| piece.position == (new_x as u16, new_y as u16))
            {
                break;
            }
        }

        // Check moves to the bottom left
        for i in 1..self.board_size {
            let new_x = self.position.0 as i32 - i as i32;
            let new_y = self.position.1 as i32 - i as i32;

            // Check if the new position is within the board
            if new_x < 0 || new_y < 0 {
                break;
            }

            moves.push((new_x as u16, new_y as u16));

            // Check if there is a piece in the way, if so stop checking
            if board_pieces
                .iter()
                .any(|piece| piece.position == (new_x as u16, new_y as u16))
            {
                break;
            }
        }

        moves
    }

    fn get_king_moves(&self) -> Vec<(u16, u16)> {
        let mut moves = Vec::with_capacity(8);
        for (dx, dy) in KING_MOVES.iter() {
            let new_x = self.position.0 as i32 + *dx as i32;
            let new_y = self.position.1 as i32 + *dy as i32;

            // Check if the new position is within the board
            if new_x >= 0
                && new_x < self.board_size as i32
                && new_y >= 0
                && new_y < self.board_size as i32
            {
                moves.push((new_x as u16, new_y as u16));
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_moves() {
        // Test rook moves from the center of the board
        let rook = Piece::new(PieceType::Rook, (4, 4), 8);
        let board_pieces = vec![];
        let moves = rook.get_rook_moves(&board_pieces);
        assert_eq!(moves.len(), 14);

        // Test rook moves with pieces blocking the way
        let rook = Piece::new(PieceType::Rook, (4, 4), 8);
        let board_pieces = vec![
            Piece::new(PieceType::Pawn, (4, 2), 8),
            Piece::new(PieceType::Pawn, (4, 6), 8),
            Piece::new(PieceType::Pawn, (2, 4), 8),
            Piece::new(PieceType::Pawn, (6, 4), 8),
        ];
        let moves = rook.get_rook_moves(&board_pieces);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_knight_moves() {
        // Test knight moves from the center of the board
        let knight = Piece::new(PieceType::Knight, (4, 4), 8);
        let moves = knight.get_knight_moves();
        assert_eq!(moves.len(), 8);

        // Test knight moves from the corner of the board
        let knight = Piece::new(PieceType::Knight, (0, 0), 8);
        let moves = knight.get_knight_moves();
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_bishop_moves() {
        // Test bishop moves from the center of the board
        let bishop = Piece::new(PieceType::Bishop, (4, 4), 8);
        let board_pieces = vec![];
        let moves = bishop.get_bishop_moves(&board_pieces);
        assert_eq!(moves.len(), 13);

        // Test bishop moves with pieces blocking the way
        let bishop = Piece::new(PieceType::Bishop, (4, 4), 8);
        let board_pieces = vec![
            Piece::new(PieceType::Pawn, (2, 2), 8),
            Piece::new(PieceType::Pawn, (6, 6), 8),
            Piece::new(PieceType::Pawn, (2, 6), 8),
            Piece::new(PieceType::Pawn, (6, 2), 8),
        ];
        let moves = bishop.get_bishop_moves(&board_pieces);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_king_moves() {
        // Test king moves from the center of the board
        let king = Piece::new(PieceType::King, (4, 4), 8);
        let moves = king.get_king_moves();
        assert_eq!(moves.len(), 8);

        // Test king moves from the corner of the board
        let king = Piece::new(PieceType::King, (0, 0), 8);
        let moves = king.get_king_moves();
        assert_eq!(moves.len(), 3);
    }
}
