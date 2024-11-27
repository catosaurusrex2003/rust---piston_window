// pub fn write(color: Color, font_size: u32, text: &str, x: i32, y: i32, g: &mut G2d) {
//     let t = Text {
//         color,
//         font_size,
//         round: true,
//     };
//     t.draw_pos(text, [x, y], _, draw_state, transform, g)
// }

use find_folder::Search;
use piston_window::{types::Color, Context, G2d, Glyphs, PistonWindow, Text, Transformed};

pub fn load_fonts(window: &mut PistonWindow) -> Glyphs {
    let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let font_path = assets.join("Roboto.ttf");
    window.load_font(font_path).unwrap()
}

pub fn write_text(
    color: Color,
    font_size: u32,
    text: &str,
    x: i32,
    y: i32,
    glyphs: &mut Glyphs,
    con: &Context,
    g: &mut G2d,
) {
    let transform = con.transform.trans(x as f64, y as f64);

    Text::new_color(color, font_size)
        .draw(text, glyphs, &con.draw_state, transform, g)
        .unwrap(); // Handle errors appropriately in production
}
