extern crate sdl2;

const THICKNESS: u32 = 15;
const PADDLE_HEIGHT: f32 = 100.0;

struct Vector2 {
    x: f32,
    y: f32,
}

pub struct Game {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    is_running: bool,
    paddle_position: Vector2,
    ball_position: Vector2,
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

                                        
        let canvas = window.into_canvas()
                                .accelerated()
                                .present_vsync()
                                .build()
                                .unwrap();

        let paddle_position = Vector2 { x: 10.0, y: 768.0 / 2.0 };
        let ball_position = Vector2 { x: 1024.0/2.0, y: 768.0/2.0 };
        
        return Ok(Game {sdl_context: sdl_context, canvas: canvas, is_running: true, paddle_position: paddle_position ,ball_position: ball_position});
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

    fn generate_output(&mut self)
    {
        let mut wall = sdl2::rect::Rect::new(0, 0, 1024, THICKNESS);
        let ball = sdl2::rect::Rect::new((self.ball_position.x - (THICKNESS / 2) as f32) as i32,
                                         (self.ball_position.y - (THICKNESS / 2) as f32) as i32,
                                         THICKNESS,
                                         THICKNESS);
        let paddle = sdl2::rect::Rect::new(self.paddle_position.x as i32,
                                           (self.paddle_position.y - PADDLE_HEIGHT / 2.0) as i32,
                                            THICKNESS,
                                            PADDLE_HEIGHT as u32);

        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 255, 255));
        self.canvas.clear();
        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 255));
        
        let _ = self.canvas.fill_rect(wall);

        wall.set_y(768 - THICKNESS as i32);

        let _ = self.canvas.fill_rect(wall);

        wall.set_x(1024 - THICKNESS as i32);
        wall.set_y(0);
        wall.set_width(THICKNESS);
        wall.set_height(1024);

        let _ = self.canvas.fill_rect(wall);
        let _ = self.canvas.fill_rect(paddle);
        let _ = self.canvas.fill_rect(ball);

        self.canvas.present();
    }
}