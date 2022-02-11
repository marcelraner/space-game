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
}

impl Spaceship {
    fn new(position_x: i32, position_y: i32) -> Self {
        Self {
            position: Position {
                x: position_x,
                y: position_y,
            },
        }
    }
}

impl RenderWithSdl2 for Spaceship {
    fn render_with_sdl2(&self, canvas: &mut Canvas<sdl2::video::Window>, offset: Offset) {
        let rect = sdl2::rect::Rect::new(
            offset.x + self.position.x - 5,
            offset.y + self.position.y - 5,
            10,
            10,
        );
        canvas.draw_rect(rect).unwrap();
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
        if keyboard_state.is_scancode_pressed(Scancode::Down) {
            space.player_spaceship.position.y += 10;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Left) {
            space.player_spaceship.position.x -= 10;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            space.player_spaceship.position.x += 10;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Up) {
            space.player_spaceship.position.y -= 10;
        }

        render(&mut canvas, &space);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
