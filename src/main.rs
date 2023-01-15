mod array;
mod algorithm;

use sfml::{
    graphics::{RenderWindow, RenderTarget, Color},
    window::{Style, Event, Key}
};


fn main() {
    // Create sfml window
    let mut win = RenderWindow::new((800, 600), "Sorting algorithms", Style::CLOSE, &Default::default());
    win.set_framerate_limit(144);
    
    let mut sort = algorithm::Selection::new(array::reversed(10));

    // Main loop
    while win.is_open() {
        while let Some(event) = win.poll_event() {
            match event {
                Event::Closed => win.close(),
                Event::KeyPressed { code, alt: _, ctrl: _, shift: _, system: _ } if code == Key::Space => sort.step(),
                _ => ()
            }
        }

        // Algorithm step
        
        win.clear(Color::BLACK);
        array::draw(&mut win, (800., 600.), 2., sort.arr.clone());
        win.display();
    }
}
