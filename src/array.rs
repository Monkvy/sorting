use sfml::graphics::{RenderWindow, RectangleShape, Transformable, Shape, Color, RenderTarget};
use rand::Rng;


#[derive(Clone)]
pub enum State {
    Sorted,
    NotSorted,
    Selected,
}
impl State {
    pub fn color(&self) -> Color {
        match self {
            State::Sorted => Color::GREEN,
            State::NotSorted => Color::WHITE,
            State::Selected => Color::YELLOW,
        }
    }
}

#[derive(Clone)]
pub struct Elm {
    pub val: u16,
    pub state: State
}
impl Elm {
    pub fn new(val: u16) -> Self {
        Self { val, state: State::NotSorted }
    }
}


pub fn random(len: usize) -> Vec<Elm> {
    let mut rng = rand::thread_rng();
    let mut array = Vec::with_capacity(len);
    for _ in 0..len {
        array.push(Elm::new(rng.gen_range(0..101)));
    }
    array
}

pub fn reversed(len: usize) -> Vec<Elm> {
    let mut array = Vec::with_capacity(len);
    for i in 0..len {
        array.push(Elm::new((i as u16 + 1) * 10));
    }

    array.reverse();
    array
}



pub fn draw(win: &mut RenderWindow, win_size: (f32, f32), bar_spacing: f32, array: Vec<Elm>) {
    let bar_num = array.len() as f32;
    let bar_width = (win_size.0 - (bar_num + 1.) * bar_spacing) / bar_num;
    let mut x: f32 = bar_spacing;

    for elm in array {
        let mut bar = RectangleShape::new();
        let bar_height = win_size.1 * elm.val as f32 / 100.;

        bar.set_size((bar_width as f32, bar_height));
        bar.set_position((x, win_size.1 - bar_height));
        bar.set_fill_color(elm.state.color());
        win.draw(&bar);

        x += bar_width + bar_spacing;
    }
}
