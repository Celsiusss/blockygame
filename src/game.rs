use sdl2::{event::Event};

use crate::{grid::Grid, input::InputHandler};

pub struct Game {
    pub grid: Grid,
    input_handler: InputHandler,
}

impl Game {
    pub fn new() -> Self {
        let grid: Grid = Grid::create(10, 20, None);

        Game {
            grid,
            input_handler: InputHandler::new()
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        self.input_handler.handle_event(&mut self.grid, event)
    }

    pub fn input_update(&mut self, last_update: time::OffsetDateTime) {
        self.input_handler.update(last_update, &mut self.grid)
        
    }
}
