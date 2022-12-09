use sdl2::pixels::Color;

use crate::{piece::{Shape, Block}, grid::Position};

static L_BLOCK: Block = Block {color: Color::RED};
static J_BLOCK: Block = Block {color: Color::YELLOW};
static S_BLOCK: Block = Block {color: Color::GREEN};
static Z_BLOCK: Block = Block {color: Color::MAGENTA};
static I_BLOCK: Block = Block {color: Color::BLUE};
static O_BLOCK: Block = Block {color: Color::WHITE};
static T_BLOCK: Block = Block {color: Color::CYAN};

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
