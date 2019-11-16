extern crate enigo;
use enigo::{Enigo, EventListener, MouseButton, MouseControllable};
use std::thread;
use std::time::Duration;

fn main() {
    let wait_time = Duration::from_secs(2);
    let mut enigo = Enigo::new();

    enigo.register_mouse_event(MouseButton::Left);
    enigo.mouse_events().iter().for_each(|but| match but {
        MouseButton::Left => println!("Left button clicked"),
        _ => (),
    });

    thread::sleep(wait_time);

    enigo.mouse_click(MouseButton::Left);
}
