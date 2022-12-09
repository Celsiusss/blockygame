use sdl2::{keyboard::Keycode, event::Event};
use time::OffsetDateTime;

use crate::{grid::{Grid, Direction}};

static UNTIL_REPEAT_DELAY: time::Duration = time::Duration::milliseconds(300);
static REPEAT_DELAY: time::Duration = time::Duration::milliseconds(50);

#[derive(Debug)]
struct Keypress {
    key_code: Keycode,
    repeat: bool,
    duration: time::Duration
}
impl Keypress {
    fn new(key_code: Keycode) -> Self {
        Keypress { key_code, repeat: false, duration: time::Duration::new(0, 0) }
    }
}

#[derive(Debug)]
struct InputHandler {
    keys_pressed: Vec<Keypress>
}
impl InputHandler {
    pub fn new() -> Self {
        InputHandler { keys_pressed: Vec::new() }
    }

    pub fn update(&mut self, last_update: OffsetDateTime, grid: &mut Grid) {
        let time = time::OffsetDateTime::now_utc();
        let delta = time - last_update;
        for keypress in &mut self.keys_pressed {
            if keypress.duration == time::Duration::ZERO {
                Self::handle_key(&keypress.key_code, grid);
            }
            keypress.duration += delta;
            if keypress.repeat {
                if keypress.duration >= REPEAT_DELAY {
                    keypress.duration = REPEAT_DELAY - keypress.duration;
                    Self::handle_key(&keypress.key_code, grid);
                }
            } else {
                if keypress.duration >= UNTIL_REPEAT_DELAY {
                    keypress.duration = UNTIL_REPEAT_DELAY - keypress.duration;
                    keypress.repeat = true;
                }
            }
        }
    }

    fn handle_key(key: &Keycode, grid: &mut Grid) {
        match key {
            Keycode::D => {
                grid.move_active(Direction::RIGHT);
            }
            Keycode::A => {
                grid.move_active(Direction::LEFT);
            }
            Keycode::Space => {
                grid.drop();
            }
            Keycode::K => {
                grid.rotate(Direction::RIGHT);
            }
            Keycode::J => {
                grid.rotate(Direction::LEFT);
            }
            _ => {}
        }
    }
    
    pub fn handle_event(&mut self, grid: &mut Grid, event: &Event) {
        println!("{:?}", event);
        match event {
            Event::KeyDown { keycode: Some(keycode), .. } => {
                if self.keys_pressed.iter().any(|key| &key.key_code == keycode) {
                    return;
                }
                self.keys_pressed.push(Keypress::new(keycode.to_owned()))
            }
            Event::KeyUp { keycode: Some(keycode), .. } => {
                let key_index = self.keys_pressed.iter().position(|key| &key.key_code == keycode);
                if let Some(key_index) = key_index {
                    self.keys_pressed.swap_remove(key_index);
                }
            }
            _ => ()
        }
    }
}

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
