use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

use super::utils;

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = utils::to_coord(x);
    let gui_y = utils::to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, utils::BLOCK_SIZE, utils::BLOCK_SIZE],
        con.transform,
        g,
    )
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = utils::to_coord(x);
    let y = utils::to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            utils::BLOCK_SIZE * (width as f64),
            utils::BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    )
}
