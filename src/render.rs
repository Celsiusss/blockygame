use crate::game::Game;
use crate::grid::{Position};

use sdl2::keyboard::Keycode;
use sdl2::event::{Event};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

static scale: u32 = 30;

pub fn render(game: &Game, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(Rect::new(0, 0, game.grid.width as u32 * scale, game.grid.height as u32 * scale))?;

    for x in 0..game.grid.width {
        for y in 0..game.grid.height {
            let block = game.grid.get(&Position(x.try_into().unwrap(), y.try_into().unwrap()));
            if block.is_some() {
                canvas.set_draw_color(block.unwrap().block.color);
                canvas.fill_rect(Rect::new(x as i32 * scale as i32, y as i32 * scale as i32, scale, scale))?;
                canvas.set_draw_color(Color::RGBA(255, 255, 255, if block.unwrap().placed {255} else {125}));
            }
            // canvas.draw_rect(Rect::new(x as i32 * scale as i32, y as i32 * scale as i32, scale, scale))?;
        }
    }
    canvas.present();

    return Ok(());
}
