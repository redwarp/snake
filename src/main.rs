use opengl_graphics::{GlGraphics, OpenGL};

use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonEvent, Key, RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::PistonWindow;

use rand::Rng;
use std::collections::LinkedList;

const WIDTH: u8 = 30;
const HEIGHT: u8 = 20;

pub struct Game {
    gl: GlGraphics,
    size: (u8, u8),
    snake: Snake,
    food: Food,
    direction_updated: bool,
    score: u32,
}

impl Game {
    fn new(gl: GlGraphics, size: (u8, u8)) -> Self {
        let mut game = Game {
            gl,
            size,
            snake: Snake::new(),
            food: Food { position: (0, 0) },
            direction_updated: false,
            score: 0,
        };
        game.generate_food();

        game
    }

    fn render(&mut self, args: &RenderArgs) {
        let background_color: [f32; 4] = [0.99, 0.93, 0.8, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(background_color, gl);
        });
        self.food.render(&mut self.gl, args);
        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self) -> bool {
        self.snake.update(&self.food);

        if self.snake.is_eating(&self.food) {
            self.generate_food();
            self.score += 1;
        }

        self.direction_updated = false;

        !self.is_loosing()
    }

    fn pressed(&mut self, button: &Button) {
        if self.direction_updated {
            return;
        }
        let previous_direction = self.snake.direction.clone();
        self.snake.direction = match button {
            &Button::Keyboard(Key::Up) if previous_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if previous_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if previous_direction != Direction::Right => {
                Direction::Left
            }
            &Button::Keyboard(Key::Right) if previous_direction != Direction::Left => {
                Direction::Right
            }
            _ => previous_direction,
        };
        if previous_direction != self.snake.direction {
            self.direction_updated = true
        }
    }

    fn generate_food(&mut self) {
        let (width, height) = self.size;
        let index = rand::thread_rng().gen_range(0, width as i32 * height as i32);
        self.food.position = (index % width as i32, index / width as i32);
    }

    fn is_loosing(&self) -> bool {
        self.snake.is_eating_itself() || self.snake.is_out_of_bounds(self.size)
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    direction: Direction,
}

impl Snake {
    fn new() -> Self {
        let mut body = LinkedList::new();
        body.push_front((5, 5));
        body.push_back((4, 5));
        Snake {
            body: body,
            direction: Direction::Right,
        }
    }

    fn head(&self) -> &(i32, i32) {
        self.body.front().expect("The snake has no body")
    }

    fn update(&mut self, food: &Food) {
        let mut new_head = self.head().clone();
        match self.direction {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }
        self.body.push_front(new_head);
        if !self.is_eating(food) {
            self.body.pop_back();
        }
    }

    fn is_eating(&self, food: &Food) -> bool {
        self.head() == &food.position
    }

    fn is_eating_itself(&self) -> bool {
        let head = self.head().clone();
        self.body.iter().skip(1).any(|ring| &head == ring)
    }

    fn is_out_of_bounds(&self, bounds: (u8, u8)) -> bool {
        let width = bounds.0 as i32;
        let height = bounds.1 as i32;
        let (x, y) = self.head().clone();
        if x < 0 || x >= width || y < 0 || y >= height {
            true
        } else {
            false
        }
    }
}

struct Food {
    position: (i32, i32),
}

trait Renderable {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs);
}

impl Renderable for Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let red: [f32; 4] = [0.99, 0.16, 0.03, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square((x * 20) as f64 + 0.5, (y * 20) as f64 + 0.5, 19_f64)
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            for square in squares {
                graphics::rectangle(red, square, transform, gl);
            }
        })
    }
}

impl Renderable for Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let blue: [f32; 4] = [0.08, 0.4, 0.53, 1.0];
        let (x, y) = self.position;

        let square =
            graphics::rectangle::square((x * 20) as f64 + 0.5, (y * 20) as f64 + 0.5, 19_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(blue, square, transform, gl);
        })
    }
}

struct Color(u32);

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> Self {
        let filter: u32 = 0xff;
        let a = (color.0 >> 24 & filter) as f32 / 255.0;
        let r = (color.0 >> 16 & filter) as f32 / 255.0;
        let g = (color.0 >> 8 & filter) as f32 / 255.0;
        let b = (color.0 & filter) as f32 / 255.0;

        [r, g, b, a]
    }
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [20 * WIDTH as u32, 20 * HEIGHT as u32])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(GlGraphics::new(opengl), (WIDTH, HEIGHT));

    let mut settings = EventSettings::new();
    settings.ups = 8;
    let mut events = Events::new(settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(_args) = e.update_args() {
            if !game.update() {
                break;
            }
        }

        if let Some(args) = e.button_args() {
            game.pressed(&args.button);
        }
    }
}
