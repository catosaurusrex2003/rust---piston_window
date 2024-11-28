extern crate piston_window;
extern crate rand;

mod dashboard;
mod graphics;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use graphics::utils::to_coord_u32;
use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut writer = graphics::write::Writer::new(&mut window);
    let mut game = Game::new(width, height-10);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
            writer.write_text(None, 30, "Hello world", 15, 15, device, &c, g);
        });
        if let Some(update_args) = event.update_args() {
            game.update(update_args.dt);
        }
    }
}
