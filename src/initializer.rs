use sdl2::Sdl;
use crate::event_handler::EventHandler;
use crate::graphics::Graphics;
use crate::audio::Audio;

pub struct Initializer{
    sdl_context: Sdl
}

impl Initializer{
    pub fn new()->Self{
        let context = sdl2::init().unwrap();

        return Initializer{
            sdl_context: context
        };
    }

    pub fn init_event_handler(&self)->EventHandler{
        return EventHandler::init(&self.sdl_context);
    }

    pub fn init_graphics(&self, title:&str, x:u32, y:u32, background_color:u8, vsync:bool)->Graphics{    
        return Graphics::init(&self.sdl_context, title, x, y, background_color,vsync);
    }
    
    pub fn init_audio(&self, freq:i32, channels:u8, buffer_size:u16)->Audio{
        return Audio::init(freq, channels, buffer_size);
    }
}