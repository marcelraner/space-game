use std::time::Duration;
use std::time::SystemTime;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;

use logger::debug;

struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

type Position = Vector2<f32>;
type Offset = Vector2<i32>;

trait RenderWithSdl2 {
    fn render_with_sdl2(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        offset: &Offset,
        texture_map: &Vec<Texture>,
    );
}

type TextureId = usize;

// axis-aligned bounding box
struct AABB {
    min: Vector2<f32>,
    max: Vector2<f32>,
}

impl AABB {
    fn new(min: Vector2<f32>, max: Vector2<f32>) -> Self {
        Self { min, max }
    }

    fn intersect(&self, other: &AABB) -> bool {
        (self.min.x <= other.max.x && self.max.x >= other.min.x)
            && (self.min.y <= other.max.y && self.max.y >= other.min.y)
    }
}

struct Spaceship {
    position: Position,
    alignment_rad: f32,
    alignment: Vector2<f32>,
    rotation: f32,
    velocity: Vector2<f32>,
    texture_id: TextureId,
    aabb: AABB,
}

impl Spaceship {
    fn new(position_x: f32, position_y: f32, texture_id: TextureId) -> Self {
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
            velocity: Vector2::<f32>::new(0.0, 0.0),
            texture_id,
            aabb: AABB::new(
                Vector2::<f32>::new(position_x - 128.0, position_y - 128.0),
                Vector2::<f32>::new(position_x + 128.0, position_y + 128.0),
            ),
        }
    }

    fn move_forward(&mut self, delta_time: f32) {
        self.velocity.x += self.alignment.x * 100.0 * delta_time;
        self.velocity.y += self.alignment.y * 100.0 * delta_time;
    }

    fn move_backward(&mut self, delta_time: f32) {
        self.velocity.x -= self.alignment.x * 100.0 * delta_time;
        self.velocity.y -= self.alignment.y * 100.0 * delta_time;
    }

    fn move_left(&mut self, delta_time: f32) {
        self.velocity.x += self.alignment.y * 100.0 * delta_time;
        self.velocity.y -= self.alignment.x * 100.0 * delta_time;
    }

    fn move_right(&mut self, delta_time: f32) {
        self.velocity.x -= self.alignment.y * 100.0 * delta_time;
        self.velocity.y += self.alignment.x * 100.0 * delta_time;
    }

    fn rotate_left(&mut self, delta_time: f32) {
        self.rotation -= 1.0 * delta_time;
    }

    fn rotate_right(&mut self, delta_time: f32) {
        self.rotation += 1.0 * delta_time;
    }

    fn update(&mut self, delta_time: f32) {
        // calculate position
        self.position.x += (self.velocity.x * delta_time) as f32;
        self.position.y += (self.velocity.y * delta_time) as f32;

        // adjust bounding box
        self.aabb.min.x = self.position.x - 128.0;
        self.aabb.min.y = self.position.y - 128.0;
        self.aabb.max.x = self.position.x + 128.0;
        self.aabb.max.y = self.position.y + 128.0;

        // calculate alignment
        self.alignment_rad += self.rotation * delta_time;
        self.alignment_rad %= std::f32::consts::PI * 2.0;
        self.alignment.x = self.alignment_rad.cos();
        self.alignment.y = self.alignment_rad.sin();
    }
}

impl RenderWithSdl2 for Spaceship {
    fn render_with_sdl2(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        offset: &Offset,
        texture_map: &Vec<Texture>,
    ) {
        let position_x = offset.x + self.position.x as i32;
        let position_y = offset.y + self.position.y as i32;

        // draw texture
        canvas
            .copy_ex(
                &texture_map[self.texture_id],
                None,
                Rect::new(position_x as i32 - 64, position_y as i32 - 128, 128, 256),
                ((self.alignment_rad + std::f32::consts::PI * 0.5)
                    * 180.0
                    * std::f32::consts::FRAC_1_PI) as f64,
                None,
                false,
                false,
            )
            .unwrap();

        // draw bounding box
        canvas
            .draw_rect(sdl2::rect::Rect::new(
                self.aabb.min.x as i32 + offset.x,
                self.aabb.min.y as i32 + offset.y,
                256,
                256,
            ))
            .unwrap();

        // draw direction
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

struct Asteroid {
    position: Position,
    texture_id: TextureId,
    aabb: AABB,
}

impl Asteroid {
    fn new(position_x: f32, position_y: f32, texture_id: TextureId) -> Self {
        Self {
            position: Position {
                x: position_x,
                y: position_y,
            },
            texture_id,
            aabb: AABB::new(
                Vector2::<f32>::new(position_x - 64.0, position_y - 64.0),
                Vector2::<f32>::new(position_x + 64.0, position_y + 64.0),
            ),
        }
    }
}

impl RenderWithSdl2 for Asteroid {
    fn render_with_sdl2(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        offset: &Offset,
        texture_map: &Vec<Texture>,
    ) {
        let position_x = offset.x + self.position.x as i32;
        let position_y = offset.y + self.position.y as i32;

        // draw texture
        canvas
            .copy(
                &texture_map[self.texture_id],
                None,
                Rect::new(position_x - 64, position_y - 64, 128, 128),
            )
            .unwrap();

        // draw bounding box
        canvas
            .draw_rect(sdl2::rect::Rect::new(
                self.aabb.min.x as i32 + offset.x,
                self.aabb.min.y as i32 + offset.y,
                128,
                128,
            ))
            .unwrap();
    }
}

struct Space {
    player_spaceship: Spaceship,
    asteroids: Vec<Asteroid>,
}

impl Space {
    fn new(player_spaceship: Spaceship) -> Self {
        Self {
            player_spaceship,
            asteroids: Vec::new(),
        }
    }

    fn add_asteroid(&mut self, asteroid: Asteroid) {
        self.asteroids.push(asteroid);
    }
}

fn detect_collisions(space: &Space) {
    for asteroid in &space.asteroids {
        if space.player_spaceship.aabb.intersect(&asteroid.aabb) {
            debug!("collision");
        }
    }
}

fn render(canvas: &mut Canvas<sdl2::video::Window>, space: &Space, texture_map: &Vec<Texture>) {
    let window_size = canvas.window().size();
    let offset = Offset {
        x: (window_size.0 >> 1) as i32,
        y: (window_size.1 >> 1) as i32,
    };

    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    canvas.clear();
    canvas.set_draw_color(sdl2::pixels::Color::YELLOW);

    // render spaceship
    space
        .player_spaceship
        .render_with_sdl2(canvas, &offset, texture_map);

    // render asteroids
    for asteroid in &space.asteroids {
        asteroid.render_with_sdl2(canvas, &offset, texture_map);
    }

    canvas.present();
}

fn main() {
    let mut current_time = SystemTime::now();

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
        //.software()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture_map: Vec<Texture> = Vec::new();
    texture_map.push(
        texture_creator
            .load_texture("resources/spaceship.png")
            .unwrap(),
    );
    texture_map.push(
        texture_creator
            .load_texture("resources/asteroid_01.png")
            .unwrap(),
    );
    texture_map.push(
        texture_creator
            .load_texture("resources/asteroid_02.png")
            .unwrap(),
    );
    texture_map.push(
        texture_creator
            .load_texture("resources/asteroid_03.png")
            .unwrap(),
    );

    let mut space = Space::new(Spaceship::new(0.0, 0.0, 0));
    space.add_asteroid(Asteroid::new(250.0, 0.0, 1));
    space.add_asteroid(Asteroid::new(-300.0, 100.0, 2));
    space.add_asteroid(Asteroid::new(-200.0, -200.0, 3));

    let mut event_pump = sdl_context.event_pump().unwrap();

    'mainloop: loop {
        let frame_time = current_time.elapsed().unwrap();
        current_time = SystemTime::now();

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

        let frame_time_secs = frame_time.as_secs_f32();
        debug!("frame_time: {:?}", frame_time_secs);

        // move spaceship
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            space.player_spaceship.move_backward(frame_time_secs);
        }
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            space.player_spaceship.move_forward(frame_time_secs);
        }

        if keyboard_state.is_scancode_pressed(Scancode::A) {
            space.player_spaceship.rotate_left(frame_time_secs);
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            space.player_spaceship.rotate_right(frame_time_secs);
        }

        if keyboard_state.is_scancode_pressed(Scancode::Q) {
            space.player_spaceship.move_left(frame_time_secs);
        }
        if keyboard_state.is_scancode_pressed(Scancode::E) {
            space.player_spaceship.move_right(frame_time_secs);
        }

        space.player_spaceship.update(frame_time_secs);
        detect_collisions(&space);

        render(&mut canvas, &space, &texture_map);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
