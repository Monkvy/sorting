mod array;

use sfml::{
    graphics::{RenderWindow, RenderTarget, Color},
    window::{Style, Event}
};


fn main() {
    // Create sfml window
    let mut win = RenderWindow::new((800, 600), "Sorting algorithms", Style::CLOSE, &Default::default());
    win.set_framerate_limit(144);
    
    let arr = array::generate(100);

    // Main loop
    while win.is_open() {
        while let Some(event) = win.poll_event() {
            match event {
                Event::Closed => win.close(),
                _ => ()
            }
        }
        win.clear(Color::BLACK);
        
        array::draw(&mut win, (800., 600.), 2., &arr[..]);

        win.display();
    }
}
