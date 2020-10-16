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
    timer: sdl2::TimerSubsystem,
    is_running: bool,
    tick_count: u32,
    paddle_direction: i32,
    paddle_position: Vector2,
    ball_position: Vector2,
    ball_velocity: Vector2,
}

impl Game {
    pub fn new() -> Result<Game, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let timer_subsystem = sdl_context.timer()?;

        let window = video_subsystem
            .window("Game Programming in C++ (Chapter 1)", 1024, 768)
            .position(100, 100)
            .opengl()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        let paddle_position = Vector2 {
            x: 10.0,
            y: 768.0 / 2.0,
        };
        let ball_position = Vector2 {
            x: 1024.0 / 2.0,
            y: 768.0 / 2.0,
        };
        let ball_velocity = Vector2 {
            x: -200.0,
            y: 235.0,
        };

        return Ok(Game {
            sdl_context: sdl_context,
            canvas: canvas,
            timer: timer_subsystem,
            tick_count: 0,
            is_running: true,
            paddle_position: paddle_position,
            paddle_direction: 0,
            ball_position: ball_position,
            ball_velocity: ball_velocity,
        });
    }

    pub fn runloop(&mut self) {
        while self.is_running {
            self.proccess_input();
            self.update_game();
            self.generate_output();
        }
    }

    fn proccess_input(&mut self) {
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

        let keyboard_state = event_pump.keyboard_state();

        self.paddle_direction = 0;

        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            self.paddle_direction -= 1;
        } else if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            self.paddle_direction += 1;
        }
    }

    fn update_game(&mut self) {
        loop {
            if (self.tick_count + 16).wrapping_sub(self.timer.ticks()) > 0 {
                break;
            }
        }

        let mut delta_time = (self.timer.ticks() - self.tick_count) as f32 / 1000.0;

        if delta_time > 0.05 {
            delta_time = 0.05;
        }

        self.tick_count = self.timer.ticks();

        if self.paddle_direction != 0 {
            self.paddle_position.y += self.paddle_direction as f32 * 300.0 * delta_time;

            if self.paddle_position.y < (PADDLE_HEIGHT / 2.0 + THICKNESS as f32) {
                self.paddle_position.y = PADDLE_HEIGHT / 2.0 + THICKNESS as f32;
            } else if self.paddle_position.y > (768.0 - PADDLE_HEIGHT / 2.0 - THICKNESS as f32) {
                self.paddle_position.y = 768.0 - PADDLE_HEIGHT / 2.0 - THICKNESS as f32
            }
        }

        self.ball_position.x += self.ball_velocity.x * delta_time;
        self.ball_position.y += self.ball_velocity.y * delta_time;

        let mut diff = self.paddle_position.y - self.ball_position.y;
        diff = if diff > 0.0 { diff } else { -diff };

        if diff <= PADDLE_HEIGHT / 2.0 as f32
            && self.ball_position.x <= 25.0
            && self.ball_position.x >= 20.0
            && self.ball_velocity.x < 0.0
        {
            self.ball_velocity.x *= -1.0;
        } else if self.ball_position.x <= 0.0 {
            self.is_running = false;
        } else if self.ball_position.x >= (1024.0 - THICKNESS as f32) && self.ball_velocity.x > 0.0
        {
            self.ball_velocity.x *= -1.0;
        }

        if self.ball_position.y <= THICKNESS as f32 && self.ball_velocity.y < 0.0 {
            self.ball_velocity.y *= -1.0;
        } else if self.ball_position.y >= 768.0 - THICKNESS as f32 && self.ball_velocity.y > 0.0 {
            self.ball_velocity.y *= -1.0;
        }
    }

    fn generate_output(&mut self) {
        let mut wall = sdl2::rect::Rect::new(0, 0, 1024, THICKNESS);
        let ball = sdl2::rect::Rect::new(
            (self.ball_position.x - (THICKNESS / 2) as f32) as i32,
            (self.ball_position.y - (THICKNESS / 2) as f32) as i32,
            THICKNESS,
            THICKNESS,
        );
        let paddle = sdl2::rect::Rect::new(
            self.paddle_position.x as i32,
            (self.paddle_position.y - PADDLE_HEIGHT / 2.0) as i32,
            THICKNESS,
            PADDLE_HEIGHT as u32,
        );

        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 255, 255));
        self.canvas.clear();
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 255));

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
