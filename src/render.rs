use crate::game::Game;
use crate::grid::{Position};
use crate::piece::{BlockData, get_default_piece, PieceType};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

static SCALE: u32 = 30;

pub fn render(game: &Game, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(Rect::new(0, 0, game.grid.width as u32 * SCALE, game.grid.height as u32 * SCALE))?;

    for x in 0..game.grid.width {
        for y in 0..game.grid.height {
            let block = game.grid.get(&Position(x.try_into().unwrap(), y.try_into().unwrap()));
            if let Some(block) = block {
                render_block(canvas, &block, (x as i32, y as i32), SCALE)?;
            }
            // canvas.draw_rect(Rect::new(x as i32 * scale as i32, y as i32 * scale as i32, scale, scale))?;
        }
    }

    render_preview(game, canvas)?;
    canvas.present();

    return Ok(());
}

fn render_preview(game: &Game, canvas: &mut Canvas<Window>) -> Result<(), String> {
    let pos = game.grid.width as u32 * SCALE + 1 * SCALE;

    canvas.set_draw_color(Color::WHITE);
    canvas.draw_rect(Rect::new(pos as i32, 0, 100, 100))?;

    let shape = get_default_piece(*game.grid.queue.back().unwrap());


    for (x, row) in shape.shape.into_iter().enumerate() {
        for (y, block) in row.into_iter().enumerate() {
            if let Some(block) = block {
                render_block(canvas, &BlockData { block, placed: true, ghost: false }, (x as i32 + (game.grid.width + 1) as i32, y as i32), SCALE)?;
            }
        }
    }

    Ok(())
}

fn render_block(canvas: &mut Canvas<Window>, block: &BlockData, (x, y): (i32, i32), scale: u32) -> Result<(), String> {
    let color = block.block.color.to_sdl_color();
    let highlight = block.block.color.mult(1.5).to_sdl_color();
    let ghost = block.block.color.mult(0.4).to_sdl_color();

    let rect = Rect::new(x * scale as i32, y * scale as i32, scale, scale);
    if !block.ghost {
        canvas.set_draw_color(color);
        canvas.fill_rect(rect)?;
        canvas.set_draw_color(highlight);
        canvas.draw_rect(rect)?;
    } else {
        canvas.set_draw_color(ghost);
        canvas.fill_rect(rect)?;
    }

    Ok(())
}
