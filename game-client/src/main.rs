use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::render::Canvas;

use logger::debug;

struct Vector2<T> {
    x: T,
    y: T,
}

type Position = Vector2<i32>;
type Offset = Vector2<i32>;

trait RenderWithSdl2 {
    fn render_with_sdl2(&self, canvas: &mut Canvas<sdl2::video::Window>, offset: Offset);
}

struct Spaceship {
    position: Position,
    alignment_rad: f32,
    alignment: Vector2<f32>,
    rotation: f32,
    velocity: Vector2<f32>,
}

impl Spaceship {
    fn new(position_x: i32, position_y: i32) -> Self {
        let alignment_rad = std::f32::consts::PI / -2.0;
        Self {
            position: Position {
                x: position_x,
                y: position_y,
            },
            alignment_rad,
            alignment: Vector2::<f32> {
                x: alignment_rad.cos(),
                y: alignment_rad.sin(),
            },
            rotation: 0.0,
            velocity: Vector2::<f32> { x: 0.0, y: 0.0 },
        }
    }

    fn move_forward(&mut self) {
        self.velocity.x += self.alignment.x;
        self.velocity.y += self.alignment.y;
    }

    fn move_backward(&mut self) {
        self.velocity.x -= self.alignment.x;
        self.velocity.y -= self.alignment.y;
    }

    fn move_left(&mut self) {
        self.velocity.x += self.alignment.y;
        self.velocity.y -= self.alignment.x;
    }

    fn move_right(&mut self) {
        self.velocity.x -= self.alignment.y;
        self.velocity.y += self.alignment.x;
    }

    fn rotate_left(&mut self) {
        self.rotation -= 0.01;
    }

    fn rotate_right(&mut self) {
        self.rotation += 0.01;
    }

    fn update(&mut self) {
        // calculate position
        self.position.x += self.velocity.x as i32;
        self.position.y += self.velocity.y as i32;

        // calculate alignment
        self.alignment_rad += self.rotation;
        self.alignment_rad %= std::f32::consts::PI * 2.0;
        self.alignment.x = self.alignment_rad.cos();
        self.alignment.y = self.alignment_rad.sin();
    }
}

impl RenderWithSdl2 for Spaceship {
    fn render_with_sdl2(&self, canvas: &mut Canvas<sdl2::video::Window>, offset: Offset) {
        let position_x = offset.x + self.position.x;
        let position_y = offset.y + self.position.y;
        canvas
            .draw_rect(sdl2::rect::Rect::new(
                position_x - 8,
                position_y - 8,
                16,
                16,
            ))
            .unwrap();
        canvas
            .draw_line(
                sdl2::rect::Point::new(position_x, position_y),
                sdl2::rect::Point::new(
                    position_x + (self.alignment.x * 24.0) as i32,
                    position_y + (self.alignment.y * 24.0) as i32,
                ),
            )
            .unwrap();
    }
}

struct Space {
    player_spaceship: Spaceship,
}

impl Space {
    fn new(player_spaceship: Spaceship) -> Self {
        Self { player_spaceship }
    }
}

fn render(canvas: &mut Canvas<sdl2::video::Window>, space: &Space) {
    let window_size = canvas.window().size();
    let offset = Offset {
        x: (window_size.0 >> 1) as i32,
        y: (window_size.1 >> 1) as i32,
    };

    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    canvas.clear();
    canvas.set_draw_color(sdl2::pixels::Color::YELLOW);
    space.player_spaceship.render_with_sdl2(canvas, offset);
    canvas.present();
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("space-game", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut space = Space::new(Spaceship::new(0, 0));

    let mut event_pump = sdl_context.event_pump().unwrap();

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                e => {
                    debug!("{:?}", e);
                }
            }
        }

        let keyboard_state = event_pump.keyboard_state();

        // move spaceship
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            space.player_spaceship.move_backward();
        }
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            space.player_spaceship.move_forward();
        }

        if keyboard_state.is_scancode_pressed(Scancode::A) {
            space.player_spaceship.rotate_left();
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            space.player_spaceship.rotate_right();
        }

        if keyboard_state.is_scancode_pressed(Scancode::Q) {
            space.player_spaceship.move_left();
        }
        if keyboard_state.is_scancode_pressed(Scancode::E) {
            space.player_spaceship.move_right();
        }

        space.player_spaceship.update();

        render(&mut canvas, &space);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
