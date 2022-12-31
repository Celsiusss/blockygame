use sdl2::pixels::{Color as C};

use crate::{piece::{Shape, Block}, grid::Position};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    rgb: (u8, u8, u8)
}
impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { rgb: (r, g, b) }
    }
    pub const fn to_sdl_color(&self) -> C {
        C::RGB(self.rgb.0, self.rgb.1, self.rgb.2)
    }
    pub fn mult(&self, value: f32) -> Self {
        Color {
            rgb: (
                (self.rgb.0 as f32 * value).clamp(0.0, 256.0) as u8,
                (self.rgb.1 as f32 * value).clamp(0.0, 256.0) as u8,
                (self.rgb.2 as f32 * value).clamp(0.0, 256.0) as u8,
            )
        }
    }
}

static RED: Color = Color::new(200, 0, 0);
static YELLOW: Color = Color::new(0, 200, 200);
static GREEN: Color = Color::new(0, 200, 0);
static MAGENTA: Color = Color::new(100, 0, 200);
static BLUE: Color = Color::new(0, 0, 200);
static WHITE: Color = Color::new(200, 200, 200);
static CYAN: Color = Color::new(0, 100, 200);

static L_BLOCK: Block = Block {color: RED };
static J_BLOCK: Block = Block {color: YELLOW};
static S_BLOCK: Block = Block {color: GREEN };
static Z_BLOCK: Block = Block {color: MAGENTA };
static I_BLOCK: Block = Block {color: BLUE };
static O_BLOCK: Block = Block {color: WHITE };
static T_BLOCK: Block = Block {color: CYAN };

pub type ShapeRotations = [Shape; 4];

pub static Ls: ShapeRotations = [
    Shape {
        shape: [
            [None, None, Some(L_BLOCK), None],
            [Some(L_BLOCK), Some(L_BLOCK), Some(L_BLOCK), None],
            [None, None, None, None],
            [None, None, None, None],
        ],
        bounds: (3, 2),
        pos: Position(0, 0)
    },
    Shape {
        shape: [
            [None, Some(L_BLOCK), None, None],
            [None, Some(L_BLOCK), None, None],
            [None, Some(L_BLOCK), Some(L_BLOCK), None],
            [None, None, None, None],
        ],
        bounds: (2, 3),
        pos: Position(1, 0)
    },
    Shape {
        shape: [
            [None, None, None, None],
            [Some(L_BLOCK), Some(L_BLOCK), Some(L_BLOCK), None],
            [Some(L_BLOCK), None, None, None],
            [None, None, None, None],
        ],
        bounds: (3, 2),
        pos: Position(0, 1)
    },
    Shape {
        shape: [
            [Some(L_BLOCK), Some(L_BLOCK), None, None],
            [None, Some(L_BLOCK), None, None],
            [None, Some(L_BLOCK), None, None],
            [None, None, None, None],
        ],
        bounds: (2, 3),
        pos: Position(0, 0)
    }
];

pub static Js: ShapeRotations = [
    Shape {
        shape: [
            [Some(J_BLOCK), None, None, None],
            [Some(J_BLOCK), Some(J_BLOCK), Some(J_BLOCK), None],
            [None, None, None, None],
            [None, None, None, None],
        ],
        bounds: (3, 2),
        pos: Position(0, 0)
    },
    Shape {
        shape: [
            [None, Some(J_BLOCK), Some(J_BLOCK), None],
            [None, Some(J_BLOCK), None, None],
            [None, Some(J_BLOCK), None, None],
            [None, None, None, None],
        ],
        bounds: (2, 3),
        pos: Position(1, 0)
    },
    Shape {
        shape: [
            [None, None, None, None],
            [Some(J_BLOCK), Some(J_BLOCK), Some(J_BLOCK), None],
            [None, None, Some(J_BLOCK), None],
            [None, None, None, None],
        ],
        bounds: (3, 2),
        pos: Position(0, 1)
    },
    Shape {
        shape: [
            [None, Some(J_BLOCK), None, None],
            [None, Some(J_BLOCK), None, None],
            [Some(J_BLOCK), Some(J_BLOCK), None, None],
            [None, None, None, None],
        ],
        bounds: (2, 3),
        pos: Position(0, 0)
    }
];

pub static Is: ShapeRotations = [
    Shape {
        shape: [
            [None, None, None, None],
            [Some(I_BLOCK), Some(I_BLOCK), Some(I_BLOCK), Some(I_BLOCK)],
            [None, None, None, None],
            [None, None, None, None],
        ],
        bounds: (4, 1),
        pos: Position(0, 1)
    },
    Shape {
        shape: [
            [None, None, Some(I_BLOCK), None],
            [None, None, Some(I_BLOCK), None],
            [None, None, Some(I_BLOCK), None],
            [None, None, Some(I_BLOCK), None],
        ],
        bounds: (1, 4),
        pos: Position(2, 0)
    },
    Shape {
        shape: [
            [None, None, None, None],
            [None, None, None, None],
            [Some(I_BLOCK), Some(I_BLOCK), Some(I_BLOCK), Some(I_BLOCK)],
            [None, None, None, None],
        ],
        bounds: (4, 1),
        pos: Position(0, 2)
    },
    Shape {
        shape: [
            [None, Some(I_BLOCK), None, None],
            [None, Some(I_BLOCK), None, None],
            [None, Some(I_BLOCK), None, None],
            [None, Some(I_BLOCK), None, None],
        ],
        bounds: (1, 4),
        pos: Position(1, 0)
    }
];

pub static Os: ShapeRotations = [
    Shape {
        shape: [
            [Some(O_BLOCK), Some(O_BLOCK), None, None],
            [Some(O_BLOCK), Some(O_BLOCK), None, None],
            [None, None, None, None],
            [None, None, None, None],
        ],
        bounds: (2, 2),
        pos: Position(0, 0)
    },
    Shape {
        shape: [
            [Some(O_BLOCK), Some(O_BLOCK), None, None],
            [Some(O_BLOCK), Some(O_BLOCK), None, None],
            [None, None, None, None],
            [None, None, None, None],
        ],
        bounds: (2, 2),
        pos: Position(0, 0)
    },
    Shape {
        shape: [
            [Some(O_BLOCK), Some(O_BLOCK), None, None],
            [Some(O_BLOCK), Some(O_BLOCK), None, None],
            [None, None, None, None],
            [None, None, None, None],
        ],
        bounds: (2, 2),
        pos: Position(0, 0)
    },
    Shape {
        shape: [
            [Some(O_BLOCK), Some(O_BLOCK), None, None],
            [Some(O_BLOCK), Some(O_BLOCK), None, None],
            [None, None, None, None],
            [None, None, None, None],
        ],
        bounds: (2, 2),
        pos: Position(0, 0)
    }
];

pub static Ss: ShapeRotations = [
    Shape {
        shape: [
            [None, Some(S_BLOCK), Some(S_BLOCK), None],
            [Some(S_BLOCK), Some(S_BLOCK), None, None],
            [None, None, None, None],
            [None, None, None, None],
        ],
        bounds: (3, 2),
        pos: Position(0, 0)
    },
    Shape {
        shape: [
            [None, Some(S_BLOCK), None, None],
            [None, Some(S_BLOCK), Some(S_BLOCK), None],
            [None, None, Some(S_BLOCK), None],
            [None, None, None, None],
        ],
        bounds: (2, 3),
        pos: Position(1, 0)
    },
    Shape {
        shape: [
            [None, None, None, None],
            [None, Some(S_BLOCK), Some(S_BLOCK), None],
            [Some(S_BLOCK), Some(S_BLOCK), None, None],
            [None, None, None, None],
        ],
        bounds: (3, 2),
        pos: Position(0, 1)
    },
    Shape {
        shape: [
            [Some(S_BLOCK), None, None, None],
            [Some(S_BLOCK), Some(S_BLOCK), None, None],
            [None, Some(S_BLOCK), None, None],
            [None, None, None, None],
        ],
        bounds: (2, 3),
        pos: Position(0, 0)
    }
];

pub static Zs: ShapeRotations = [
    Shape {
        shape: [
            [Some(Z_BLOCK), Some(Z_BLOCK), None, None],
            [None, Some(Z_BLOCK), Some(Z_BLOCK), None],
            [None, None, None, None],
            [None, None, None, None],
        ],
        bounds: (3, 2),
        pos: Position(0, 0)
    },
    Shape {
        shape: [
            [None, None, Some(Z_BLOCK), None],
            [None, Some(Z_BLOCK), Some(Z_BLOCK), None],
            [None, Some(Z_BLOCK), None, None],
            [None, None, None, None],
        ],
        bounds: (2, 3),
        pos: Position(1, 0)
    },
    Shape {
        shape: [
            [None, None, None, None],
            [Some(Z_BLOCK), Some(Z_BLOCK), None, None],
            [None, Some(Z_BLOCK), Some(Z_BLOCK), None],
            [None, None, None, None],
        ],
        bounds: (3, 2),
        pos: Position(0, 1)
    },
    Shape {
        shape: [
            [None, Some(Z_BLOCK), None, None],
            [Some(Z_BLOCK), Some(Z_BLOCK), None, None],
            [Some(Z_BLOCK), None, None, None],
            [None, None, None, None],
        ],
        bounds: (2, 3),
        pos: Position(0, 0)
    }
];

pub static Ts: ShapeRotations = [
    Shape {
        shape: [
            [None, Some(T_BLOCK), None, None],
            [Some(T_BLOCK), Some(T_BLOCK), Some(T_BLOCK), None],
            [None, None, None, None],
            [None, None, None, None],
        ],
        bounds: (3, 2),
        pos: Position(0, 0)
    },
    Shape {
        shape: [
            [None, Some(T_BLOCK), None, None],
            [None, Some(T_BLOCK), Some(T_BLOCK), None],
            [None, Some(T_BLOCK), None, None],
            [None, None, None, None],
        ],
        bounds: (2, 3),
        pos: Position(1, 0)
    },
    Shape {
        shape: [
            [None, None, None, None],
            [Some(T_BLOCK), Some(T_BLOCK), Some(T_BLOCK), None],
            [None, Some(T_BLOCK), None, None],
            [None, None, None, None],
        ],
        bounds: (3, 2),
        pos: Position(0, 1)
    },
    Shape {
        shape: [
            [None, Some(T_BLOCK), None, None],
            [Some(T_BLOCK), Some(T_BLOCK), None, None],
            [None, Some(T_BLOCK), None, None],
            [None, None, None, None],
        ],
        bounds: (2, 3),
        pos: Position(0, 0)
    }
];
