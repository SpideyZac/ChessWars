const KNIGHT_MOVES: [(i32, i32); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

const KING_MOVES: [(i32, i32); 8] = [
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
    player_id: u8,
    board_size: u16,
    piece_type: PieceType,
    position: (u16, u16),
    legal_moves: Vec<(u16, u16)>,
}

impl Piece {
    pub fn new(
        player_id: u8,
        board_size: u16,
        piece_type: PieceType,
        position: (u16, u16),
    ) -> Self {
        Self {
            player_id,
            board_size,
            piece_type,
            position,
            legal_moves: vec![],
        }
    }

    pub fn init_legal_moves(&mut self, board: &Vec<Piece>) {
        self.legal_moves = self.get_legal_moves(board);
    }

    fn get_legal_moves(&self, board: &Vec<Piece>) -> Vec<(u16, u16)> {
        match self.piece_type {
            PieceType::Pawn => self.get_king_legal_moves(board), // Pawns move like kings
            PieceType::Rook => self.get_rook_legal_moves(board),
            PieceType::Knight => self.get_knight_legal_moves(board),
            PieceType::Bishop => self.get_bishop_legal_moves(board),
            PieceType::Queen => self // Queens move like knights and bishops
                .get_knight_legal_moves(board)
                .iter()
                .chain(self.get_bishop_legal_moves(board).iter())
                .cloned()
                .collect(),
            PieceType::King => self.get_king_legal_moves(board),
        }
    }

    fn get_rook_legal_moves(&self, board: &Vec<Piece>) -> Vec<(u16, u16)> {
        let mut legal_moves = vec![];

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let mut x = self.position.0 as i32 + dx;
            let mut y = self.position.1 as i32 + dy;

            while x >= 0 && x < self.board_size as i32 && y >= 0 && y < self.board_size as i32 {
                let position = (x as u16, y as u16);

                if let Some(piece) = board.iter().find(|p| p.position == position) {
                    if piece.player_id != self.player_id {
                        legal_moves.push(position);
                    }

                    break;
                }

                legal_moves.push(position);
                x += dx;
                y += dy;
            }
        }

        legal_moves
    }

    fn get_knight_legal_moves(&self, board: &Vec<Piece>) -> Vec<(u16, u16)> {
        let mut legal_moves = vec![];

        for (dx, dy) in KNIGHT_MOVES.iter() {
            let x = self.position.0 as i32 + dx;
            let y = self.position.1 as i32 + dy;

            if x >= 0 && x < self.board_size as i32 && y >= 0 && y < self.board_size as i32 {
                let position = (x as u16, y as u16);

                if let Some(piece) = board.iter().find(|p| p.position == position) {
                    if piece.player_id != self.player_id {
                        legal_moves.push(position);
                    }
                } else {
                    legal_moves.push(position);
                }
            }
        }

        legal_moves
    }

    fn get_bishop_legal_moves(&self, board: &Vec<Piece>) -> Vec<(u16, u16)> {
        let mut legal_moves = vec![];

        for (dx, dy) in [(1, 1), (1, -1), (-1, 1), (-1, -1)].iter() {
            let mut x = self.position.0 as i32 + dx;
            let mut y = self.position.1 as i32 + dy;

            while x >= 0 && x < self.board_size as i32 && y >= 0 && y < self.board_size as i32 {
                let position = (x as u16, y as u16);

                if let Some(piece) = board.iter().find(|p| p.position == position) {
                    if piece.player_id != self.player_id {
                        legal_moves.push(position);
                    }

                    break;
                }

                legal_moves.push(position);
                x += dx;
                y += dy;
            }
        }

        legal_moves
    }

    fn get_king_legal_moves(&self, board: &Vec<Piece>) -> Vec<(u16, u16)> {
        let mut legal_moves = vec![];

        for (dx, dy) in KING_MOVES.iter() {
            let x = self.position.0 as i32 + dx;
            let y = self.position.1 as i32 + dy;

            if x >= 0 && x < self.board_size as i32 && y >= 0 && y < self.board_size as i32 {
                let position = (x as u16, y as u16);

                if let Some(piece) = board.iter().find(|p| p.position == position) {
                    if piece.player_id != self.player_id {
                        legal_moves.push(position);
                    }
                } else {
                    legal_moves.push(position);
                }
            }
        }

        legal_moves
    }

    pub fn make_move(&mut self, board: &mut Vec<Piece>, position: (u16, u16)) {
        assert!(self.legal_moves.contains(&position));
        self.maybe_capture_piece(board, &position);
        self.position = position;
    }

    fn maybe_capture_piece(&mut self, board: &mut Vec<Piece>, position: &(u16, u16)) {
        if let Some(index) = board.iter().position(|p| p.position == *position) {
            board.remove(index);
        }
    }
}
