use find_folder::Search;
// use piston_window::*;
use piston_window::{types::Color, Context, G2d, Glyphs, PistonWindow, Text, Transformed, GfxDevice};
use super::utils;

pub struct Writer {
    glyph: Glyphs,
}

impl Writer {
    pub fn new(window: &mut PistonWindow) -> Writer {
        let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let font_path = assets.join("Roboto.ttf");
        let glyph = window.load_font(font_path).unwrap();
        Writer { glyph }
    }

    pub fn write_text(
        &mut self,
        color: Option<Color>,
        font_size: u32,
        text: &str,
        x: i32,
        y: i32,
        device: &mut GfxDevice,
        con: &Context,
        g: &mut G2d,
    ) {
        let x = utils::to_coord(x);
        let y = utils::to_coord(y);
        let transform = con.transform.trans(x as f64, y as f64);
        let color = color.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        Text::new_color(color, font_size)
            .draw(text, &mut self.glyph, &con.draw_state, transform, g)
            .unwrap(); // Handle errors appropriately in production
        self.glyph.factory.encoder.flush(device);
    }
}
