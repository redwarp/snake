use opengl_graphics::{GlGraphics, OpenGL};

use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonEvent, Key, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::PistonWindow;

use std::collections::LinkedList;

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(green, gl);
        });
        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.snake.update(args);
    }

    fn pressed(&mut self, button: &Button) {
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
        body.push_back((4,5));
        Snake {
            body: body,
            direction: Direction::Right,
        }
    }

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            for square in squares {
                graphics::rectangle(red, square, transform, gl);
            }
        })
    }

    fn update(&mut self, _args: &UpdateArgs) {
        let mut new_head = self.body.front().expect("Snake has no body").clone();
        match self.direction {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }
        self.body.push_front(new_head);
        self.body.pop_back();
    }
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: PistonWindow = WindowSettings::new("Snake", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake::new(),
    };

    let mut settings = EventSettings::new();
    settings.ups = 4;
    let mut events = Events::new(settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(k) = e.button_args() {
            game.pressed(&k.button);
        }
    }
}
