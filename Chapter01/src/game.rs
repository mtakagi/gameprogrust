extern crate sdl2;

pub struct Game {
    sdl_context: sdl2::Sdl,
    window: sdl2::video::Window,
    is_running: bool,
}

impl Game {
    pub fn new() -> Result<Game, String>
    {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
                        .window("Game Programming in C++ (Chapter 1)", 1024, 768)
                        .position(100, 100)
                        .opengl()
                        .build()
                        .unwrap();
        
        return Ok(Game {sdl_context: sdl_context, window: window, is_running: true});
    }

    pub fn runloop(&mut self)
    {
        while self.is_running {
            self.proccess_input();
            self.update_game();
            self.generate_output();
        }
    }

    pub fn shutdown(&self)
    {

    }

    fn proccess_input(&mut self)
    {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => self.is_running = false,
                _ => {}
            }
        }
    }

    fn update_game(&self)
    {

    }

    fn generate_output(&self)
    {

    }
}