use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

const WIDTH: f32 = 500.0;
const HEIGHT: f32 = 500.0;
const GRID_SIZE: f32 = 25.0;

enum Heading {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    heading: Heading,
    pos_head: (i32, i32),
    // pos_body: Vec<(i32, i32)>,
}

impl Snake {
    fn new() -> Self {
        Self {
            heading: Heading::Left,
            pos_head: (0, 0),
            //pos_body: Vec::new(),
        }
    }
}

struct MainState {
    snake: Snake,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            snake: Snake::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 5) {
            match self.snake.heading {
                Heading::Up => {
                    self.snake.pos_head.1 =
                        modulo(self.snake.pos_head.1 - 1, (HEIGHT / GRID_SIZE) as i32);
                }
                Heading::Down => {
                    self.snake.pos_head.1 =
                        modulo(self.snake.pos_head.1 + 1, (HEIGHT / GRID_SIZE) as i32);
                }
                Heading::Left => {
                    self.snake.pos_head.0 =
                        modulo(self.snake.pos_head.0 - 1, (WIDTH / GRID_SIZE) as i32);
                }
                Heading::Right => {
                    self.snake.pos_head.0 =
                        modulo(self.snake.pos_head.0 + 1, (WIDTH / GRID_SIZE) as i32);
                }
            }
            println!("{} -- {}", self.snake.pos_head.0, self.snake.pos_head.1);
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
            event::KeyCode::Up => self.snake.heading = Heading::Up,
            event::KeyCode::Down => self.snake.heading = Heading::Down,
            event::KeyCode::Left => self.snake.heading = Heading::Left,
            event::KeyCode::Right => self.snake.heading = Heading::Right,
            event::KeyCode::Escape => event::quit(ctx),
            _ => println!("Invalid key press!"),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rect_mesh = graphics::Mesh::new_rectangle(
            ctx,
            // graphics::DrawMode::Fill(graphics::FillOptions::default()),
            graphics::DrawMode::Fill(graphics::FillOptions::default()),
            graphics::Rect::new(
                self.snake.pos_head.0 as f32 * GRID_SIZE,
                self.snake.pos_head.1 as f32 * GRID_SIZE,
                GRID_SIZE,
                GRID_SIZE,
            ),
            graphics::WHITE,
        )?;

        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;
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
        .window_mode(conf::WindowMode::default().dimensions(WIDTH, HEIGHT));

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
