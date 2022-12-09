use crate::piece::{Shape, BlockData, ActivePiece, Block, get_random_piece, PieceType, get_piece_roation};

#[derive(Clone, Copy, Debug)]
pub struct Position(pub isize, pub isize);

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, _rhs: Position) -> Position {
        Position(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}
impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
    }
}

pub enum Direction { LEFT, RIGHT }

pub struct Grid {
    array: Vec<Option<BlockData>>,
    pub width: usize,
    pub height: usize,
    pub active_piece: Option<ActivePiece>
}

impl Grid {
    pub fn create(width: usize, height: usize, value: Option<BlockData>) -> Grid {
        return Grid {
            array: (0..(width * height)).into_iter().map(|_| value).collect(),
            width: width,
            height: height,
            active_piece: None
        };
    }

    pub fn set(&mut self, Position(x, y): Position, value: Option<BlockData>) {
        self.array[x as usize + self.width * y as usize] = value;
    }

    pub fn get(&self, Position(x, y): &Position) -> Option<BlockData> {
        if self.active_piece.is_none() {
            return self.get_from_array(&Position(*x, *y));
        }

        let piece = self.active_piece.as_ref().unwrap();

        let Position(px, py) = piece.position;
        if x >= &px && x < &(px + 4) && y >= &py && y < &(py + 4) {
            let block = piece.shape.shape[(y - py) as usize][(x - px) as usize];
            if let Some(block) = block {
                return Some(BlockData { block, placed: false })
            }
        }
        self.get_from_array(&Position(*x, *y))
    }

    fn get_from_array(&self, Position(x, y): &Position) -> Option<BlockData> {
        self.array[*x as usize + self.width * *y as usize]
    }

    pub fn place_active(&mut self) {
        if self.active_piece.is_none() { return };

        let piece: ActivePiece;
        {
            piece = self.active_piece.unwrap().clone();
        }

        let Position(pos_x, pos_y) = piece.position;
        for (y, row) in piece.shape.shape.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                if let Some(block) = block {
                    println!("{}, {}", x, y);
                    println!("{}, {}", pos_x, pos_y);
                    let block_data = BlockData { block: block.to_owned(), placed: true };
                    self.set(Position(x as isize + pos_x, y as isize + pos_y), Some(block_data.clone()));
                }
            }
        }
        let (shape, piece_type) = get_random_piece();
        let new_active_piece = ActivePiece::new(piece_type);
        self.set_active(new_active_piece);
    }

    pub fn set_active(&mut self, piece: ActivePiece) {
        self.active_piece = Some(piece);
    }

    pub fn move_active(&mut self, direction: Direction) {
        if self.active_piece.is_none() {
            return;
        }
        let active = self.active_piece.as_ref().unwrap();
        let mut new_position = active.position.clone();
        new_position += match direction {
            Direction::LEFT => Position(-1, 0),
            Direction::RIGHT => Position(1, 0),
        };

        let Position(x, y) = new_position + active.shape.pos;

        let (x_bound, y_bound) = active.shape.bounds;

        let mut tmp = self.active_piece.as_ref().unwrap().clone();
        tmp.position = new_position;
        
        if !self.check_valid(&tmp) {
            return;
        }

        self.active_piece.as_mut().unwrap().position = new_position;
    }

    pub fn move_active_down(&mut self) {
        if self.active_piece.is_none() {
            return;
        }
        if let Some(piece) = self.active_piece.as_mut() {
            piece.position += Position(0, 1);
        }
        let valid = self.check_valid(&self.active_piece.as_ref().unwrap());
        if let Some(piece) = self.active_piece.as_mut() {
            if !valid {
                piece.position += Position(0, -1);
            }
        }
    }

    pub fn drop(&mut self) {
        if self.active_piece.is_none() { return; }

        loop {
            if let Some(piece) = self.active_piece.as_mut() {
                piece.position += Position(0, 1);
            }

            let piece = self.active_piece.as_ref().unwrap();
            let is_valid = self.check_valid(&piece);
            if !is_valid {
                if let Some(piece) = self.active_piece.as_mut() {
                    piece.position += Position(0, -1);
                }
                self.place_active();
                break;
            }
        }
    }

    fn check_valid(&self, piece: &ActivePiece) -> bool {

        let Position(pos_x, pos_y) = piece.position + piece.shape.pos;
        let (x_bound, y_bound) = piece.shape.bounds;

        for x in pos_x..(pos_x + x_bound as isize) {
            for y in pos_y..(pos_y + y_bound as isize) {
                if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
                    return false;
                }
                let active_block = piece.shape.shape[(y - piece.position.1) as usize][(x - piece.position.0) as usize];
                let block = self.get_from_array(&Position(x.try_into().unwrap(), y.try_into().unwrap()));

                if block.is_some() && active_block.is_some() {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn rotate(&mut self, direction: Direction) {
        if self.active_piece.is_none() { return };
        
        let active_piece = self.active_piece.as_ref().unwrap();
        let rotation = match direction {
            Direction::LEFT => {
                let rot = (active_piece.rotation + 1) % 4;
                if rot < 0 { 3 } else { rot }
            },
            Direction::RIGHT => (active_piece.rotation + 1) % 4,
        };
        
        let active_piece = self.active_piece.as_ref().unwrap();

        let shape = *get_piece_roation(active_piece.piece_type, rotation as usize);
        let mut tmp = active_piece.clone();
        tmp.shape = shape;
        tmp.rotation = rotation;
        
        let is_valid = self.check_valid(&tmp);
        if !is_valid {
            return;
        }
        
        if let Some(active_piece) = self.active_piece.as_mut() {
            active_piece.shape = shape;
            active_piece.rotation = rotation;
        }
    }
}


