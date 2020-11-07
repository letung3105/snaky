use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
use rand;

const GRID_SIZE: f32 = 25.0;
const GRID_HEIGHT: i32 = 20;
const GRID_WIDTH: i32 = 20;
const FPS: u32 = 6;

const WINDOW_WIDTH: f32 = GRID_SIZE * GRID_WIDTH as f32;
const WINDOW_HEIGHT: f32 = GRID_SIZE * GRID_HEIGHT as f32;

enum Heading {
    Up,
    Down,
    Left,
    Right,
}

struct Apple {
    pos: (i32, i32),
}

impl Apple {
    fn new(grid_width: i32, grid_height: i32) -> Self {
        Self {
            pos: (
                modulo(rand::random::<i32>(), grid_width),
                modulo(rand::random::<i32>(), grid_height),
            ),
        }
    }

    fn update(&mut self, grid_width: i32, grid_height: i32) {
        self.pos.0 = modulo(rand::random::<i32>(), grid_width);
        self.pos.1 = modulo(rand::random::<i32>(), grid_height);
    }

    fn draw(&self, ctx: &mut Context, grid_size: f32) -> GameResult {
        let apple = graphics::Mesh::new_rectangle(
            ctx,
            // graphics::DrawMode::Fill(graphics::FillOptions::default()),
            graphics::DrawMode::Fill(graphics::FillOptions::default()),
            graphics::Rect::new(
                self.pos.0 as f32 * GRID_SIZE,
                self.pos.1 as f32 * GRID_SIZE,
                GRID_SIZE,
                GRID_SIZE,
            ),
            graphics::Color::from_rgb(255, 0, 0),
        )?;
        graphics::draw(ctx, &apple, graphics::DrawParam::default())?;
        Ok(())
    }
}

struct Snake {
    heading: Heading,
    pos_head: (i32, i32),
    pos_body: Vec<(i32, i32)>,
}

impl Snake {
    fn new() -> Self {
        Self {
            heading: Heading::Left,
            pos_head: (0, 0),
            pos_body: Vec::new(),
        }
    }

    fn update(&mut self, apple: &Apple, grid_width: i32, grid_height: i32) {
        let mut prev_pos = self.pos_head;

        match self.heading {
            Heading::Up => {
                self.pos_head.1 = modulo(self.pos_head.1 - 1, grid_height);
            }
            Heading::Down => {
                self.pos_head.1 = modulo(self.pos_head.1 + 1, grid_height);
            }
            Heading::Left => {
                self.pos_head.0 = modulo(self.pos_head.0 - 1, grid_width);
            }
            Heading::Right => {
                self.pos_head.0 = modulo(self.pos_head.0 + 1, grid_width);
            }
        }

        for pos in self.pos_body.iter_mut() {
            let tmp = *pos;
            *pos = prev_pos;
            prev_pos = tmp;
        }

        if self.can_eat(apple) {
            self.pos_body.push(prev_pos);
        }
    }

    fn can_eat(&self, apple: &Apple) -> bool {
        self.pos_head == apple.pos
    }

    fn draw(&self, ctx: &mut Context, grid_size: f32) -> GameResult {
        for pos_body in &self.pos_body {
            let snake_body = graphics::Mesh::new_rectangle(
                ctx,
                // graphics::DrawMode::Fill(graphics::FillOptions::default()),
                graphics::DrawMode::Fill(graphics::FillOptions::default()),
                graphics::Rect::new(
                    pos_body.0 as f32 * grid_size,
                    pos_body.1 as f32 * grid_size,
                    grid_size,
                    grid_size,
                ),
                graphics::WHITE,
            )?;
            graphics::draw(ctx, &snake_body, graphics::DrawParam::default())?;
        }

        let snake_head = graphics::Mesh::new_rectangle(
            ctx,
            // graphics::DrawMode::Fill(graphics::FillOptions::default()),
            graphics::DrawMode::Fill(graphics::FillOptions::default()),
            graphics::Rect::new(
                self.pos_head.0 as f32 * grid_size,
                self.pos_head.1 as f32 * grid_size,
                grid_size,
                grid_size,
            ),
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &snake_head, graphics::DrawParam::default())?;
        ggez::timer::yield_now();
        Ok(())
    }
}

struct MainState {
    snake: Snake,
    apple: Apple,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            snake: Snake::new(),
            apple: Apple::new(GRID_WIDTH, GRID_HEIGHT),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, FPS) {
            self.snake.update(&self.apple, GRID_WIDTH, GRID_HEIGHT);
            if self.snake.can_eat(&self.apple) {
                self.apple.update(GRID_WIDTH, GRID_HEIGHT);
            }
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key_code: event::KeyCode,
        _key_mods: event::KeyMods,
        _repeat: bool,
    ) {
        match key_code {
            event::KeyCode::Up => match self.snake.heading {
                Heading::Down => {}
                _ => self.snake.heading = Heading::Up,
            },
            event::KeyCode::Down => match self.snake.heading {
                Heading::Up => {}
                _ => self.snake.heading = Heading::Down,
            },
            event::KeyCode::Left => match self.snake.heading {
                Heading::Right => {}
                _ => self.snake.heading = Heading::Left,
            },
            event::KeyCode::Right => match self.snake.heading {
                Heading::Left => {}
                _ => self.snake.heading = Heading::Right,
            },
            event::KeyCode::Escape => event::quit(ctx),
            _ => println!("Invalid key press!"),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.apple.draw(ctx, GRID_SIZE)?;
        self.snake.draw(ctx, GRID_SIZE)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn modulo(x: i32, m: i32) -> i32 {
    let r = x % m;
    if r < 0 {
        r + m
    } else {
        r
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("01", "Tung L. Vo")
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
