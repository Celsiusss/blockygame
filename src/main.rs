
mod grid;
mod piece;
mod render;
mod game;
mod shapes;

use game::Game;
use grid::{Position};
use piece::get_random_piece;
use render::render;
use sdl2::{event::Event, keyboard::Keycode};
use time::Duration;

fn main() -> Result<(), String> {
    let mut game = Game::new();
    let (shape, piece_type) = get_random_piece();
    game.grid.set_active(piece::ActivePiece::new(piece_type));

    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let window = video.window("game", 500, 800).build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();


    

    let mut event_pump = context.event_pump().unwrap();
    let mut i = 0;

    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(60)?;

    let mut time = time::OffsetDateTime::now_utc();
    let mut delta: Duration = Duration::new(0, 0);
    
    let mut timer = Duration::new(0, 0);
    
    'running: loop {
        for event in event_pump.poll_iter() {
            game.handle_event(&event);
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        game.input_update(time - delta);
        render(&game, &mut canvas)?;
        //std::thread::sleep(time::Duration::from_millis(1));
        fps_manager.delay();

        delta = time::OffsetDateTime::now_utc() - time;
        time = time::OffsetDateTime::now_utc();

        timer += delta;

        if timer >= Duration::SECOND {
            timer = timer - Duration::SECOND;
            game.grid.move_active_down();
        }
    }


    Ok(())
}
