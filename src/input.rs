
use sdl2::event::Event;
use sdl2::EventPump;
use super::controller::{ ButtonStatus, Button };
// use self::keyboard::Keycode;


pub enum EmulatorEvent {
    ControllerEvent { status : ButtonStatus, button : Button },
    Continue,
    Exit,
}

pub struct EmulatorInput {
    pump : EventPump,
}

impl EmulatorInput {
    pub fn events(&mut self) -> Vec<EmulatorEvent> {
        self.pump.poll_iter().map(
            |event| match event {
                Event::Quit {..} |
                Event::KeyDown {..} => EmulatorEvent::Exit,
                _ => EmulatorEvent::Continue,
            }
        ).collect()
    }
    pub fn new(pump : EventPump) -> EmulatorInput {
        EmulatorInput {
            pump : pump,
        }
    }
}
