
use beryllium::*;

pub struct Window {
    sdl: Sdl
}

impl Window {
    pub fn new() -> Self {

        let sdl = Sdl::init(init::InitFlags::EVERYTHING);
        
        sdl.set_gl_context_major_version(4).unwrap();
        sdl.set_gl_context_minor_version(5).unwrap();
        sdl.set_gl_profile(video::GlProfile::Core).unwrap();

        #[cfg(target_os = "macos")] 
        {
            sdl.set_gl_context_flag(video::GlContextFlags::FORWARD_COMPATIBLE).unwrap();
        }

        Self {
            sdl
        }
    }

    pub fn run(&self) {
        'main_loop: loop {
            // Handle events
            while let Some(event) = self.sdl.poll_events() {
                match event {
                    (events::Event::Quit, _) => break 'main_loop,
                    _ => (),
                }
            }
        }
    }
}