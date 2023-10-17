use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub struct InputDriver {
    events: sdl2::EventPump,
}

impl InputDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Self {
            events: sdl_context.event_pump().unwrap(),
        }
    }

}
