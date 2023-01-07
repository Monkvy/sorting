use sfml::graphics::{RenderWindow, RectangleShape, Transformable, Shape, Color, RenderTarget};
use rand::Rng;


pub fn generate(len: usize) -> Vec<u16> {
    let mut rng = rand::thread_rng();
    let mut array = Vec::with_capacity(len);
    for _ in 0..len {
        array.push(rng.gen_range(0..101));
    }
    array
}


pub fn draw(win: &mut RenderWindow, win_size: (f32, f32), bar_spacing: f32, array: &[u16]) {
    let bar_num = array.len() as f32;
    let bar_width = (win_size.0 - (bar_num + 1.) * bar_spacing) / bar_num;
    let mut x: f32 = bar_spacing;

    for elm in array {
        let mut bar = RectangleShape::new();
        let bar_height = win_size.1 * *elm as f32 / 100.;

        bar.set_size((bar_width as f32, bar_height));
        bar.set_position((x, win_size.1 - bar_height));
        bar.set_fill_color(Color::WHITE);
        win.draw(&bar);

        x += bar_width + bar_spacing;
    }
}
