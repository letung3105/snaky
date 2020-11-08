use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::mint;
use ggez::timer;
use ggez::{Context, GameResult};

const GRID_SIZE: f32 = 20.0;
const GRID_HEIGHT: i32 = 40;
const GRID_WIDTH: i32 = 40;
const FPS: u32 = 10;

const WINDOW_WIDTH: f32 = GRID_SIZE * GRID_WIDTH as f32;
const WINDOW_HEIGHT: f32 = GRID_SIZE * GRID_HEIGHT as f32;

/// eukalyptus color palette
const C_PALETTE: [(u8, u8, u8); 5] = [
    (155, 167, 166),
    (129, 140, 135),
    (118, 143, 133),
    (50, 60, 56),
    (27, 29, 27),
];

const C_RED: (u8, u8, u8) = (255, 0, 0);

/// direction of an object in the grid
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

/// manages position of an `Apple` object
struct Apple {
    pos: mint::Point2<i32>,
}

impl Apple {
    /// create an Apple at the given grid position
    fn new(pos: mint::Point2<i32>) -> Self {
        Self { pos }
    }

    /// change position of the object
    fn set_pos(&mut self, pos: mint::Point2<i32>) {
        self.pos = pos
    }

    /// draw the object to the game window
    fn draw(&self, ctx: &mut Context, grid_size: f32) -> GameResult {
        let apple = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::Fill(graphics::FillOptions::default()),
            graphics::Rect::new(
                self.pos.x as f32 * grid_size,
                self.pos.y as f32 * grid_size,
                GRID_SIZE,
                GRID_SIZE,
            ),
            C_RED.into(),
        )?;
        graphics::draw(ctx, &apple, graphics::DrawParam::default())?;
        Ok(())
    }
}

/// representation of the player in the game
struct Snake {
    heading: Heading,
    pos_head: mint::Point2<i32>,
    pos_body: Vec<mint::Point2<i32>>,
}

impl Snake {
    /// create a new snake at a  given grid position
    fn new(pos_head: mint::Point2<i32>) -> Self {
        Self {
            heading: Heading::Left,
            pos_head,
            pos_body: Vec::new(),
        }
    }

    /// change the position of all the grids of which the snake is comprised
    fn update_pos(&mut self, grid_width: i32, grid_height: i32) {
        let mut prev_pos = self.pos_head;
        match self.heading {
            Heading::Up => {
                self.pos_head.y = modulo(self.pos_head.y - 1, grid_height);
            }
            Heading::Down => {
                self.pos_head.y = modulo(self.pos_head.y + 1, grid_height);
            }
            Heading::Left => {
                self.pos_head.x = modulo(self.pos_head.x - 1, grid_width);
            }
            Heading::Right => {
                self.pos_head.x = modulo(self.pos_head.x + 1, grid_width);
            }
        }
        for pos in self.pos_body.iter_mut() {
            std::mem::swap(pos, &mut prev_pos);
        }
    }

    /// check of the head is overlapped with the apple's position
    fn can_eat(&self, apple: &Apple) -> bool {
        self.pos_head == apple.pos
    }

    /// add a new item to the body of the snake
    fn grow(&mut self) {
        self.pos_body.push(self.pos_head);
    }

    /// check if the snake bites itself
    fn is_dead(&self) -> bool {
        for pos_body in &self.pos_body {
            if *pos_body == self.pos_head {
                return true;
            }
        }
        false
    }

    /// draw the snake as rectangles on the screen
    fn draw(&self, ctx: &mut Context, grid_size: f32) -> GameResult {
        for pos_body in &self.pos_body {
            let snake_body = graphics::Mesh::new_rectangle(
                ctx,
                // graphics::DrawMode::Fill(graphics::FillOptions::default()),
                graphics::DrawMode::Fill(graphics::FillOptions::default()),
                graphics::Rect::new(
                    pos_body.x as f32 * grid_size,
                    pos_body.y as f32 * grid_size,
                    grid_size,
                    grid_size,
                ),
                C_PALETTE[0].into(),
            )?;
            graphics::draw(ctx, &snake_body, graphics::DrawParam::default())?;
        }
        let snake_head = graphics::Mesh::new_rectangle(
            ctx,
            // graphics::DrawMode::Fill(graphics::FillOptions::default()),
            graphics::DrawMode::Fill(graphics::FillOptions::default()),
            graphics::Rect::new(
                self.pos_head.x as f32 * grid_size,
                self.pos_head.y as f32 * grid_size,
                grid_size,
                grid_size,
            ),
            C_PALETTE[1].into(),
        )?;
        graphics::draw(ctx, &snake_head, graphics::DrawParam::default())?;
        Ok(())
    }
}

/// game's state
struct MainState {
    snake: Snake,
    apple: Apple,
    is_over: bool,
}

impl MainState {
    /// create a new game state
    fn new() -> GameResult<MainState> {
        let pos_snake_head = rand_point2(GRID_WIDTH, GRID_HEIGHT);

        let mut pos_apple = rand_point2(GRID_WIDTH, GRID_HEIGHT);
        while pos_apple == pos_snake_head {
            pos_apple = rand_point2(GRID_WIDTH, GRID_HEIGHT);
        }

        let s = MainState {
            snake: Snake::new(pos_snake_head),
            apple: Apple::new(pos_apple),
            is_over: false,
        };
        Ok(s)
    }

    fn restart(&mut self) {
        let pos_snake_head = rand_point2(GRID_WIDTH, GRID_HEIGHT);

        let mut pos_apple = rand_point2(GRID_WIDTH, GRID_HEIGHT);
        while pos_apple == pos_snake_head {
            pos_apple = rand_point2(GRID_WIDTH, GRID_HEIGHT);
        }

        self.snake = Snake::new(pos_snake_head);
        self.apple = Apple::new(pos_apple);
        self.is_over = false;
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, FPS) {
            if self.is_over {
                continue;
            }
            self.snake.update_pos(GRID_WIDTH, GRID_HEIGHT);
            if self.snake.is_dead() {
                self.is_over = true;
            }
            if self.snake.can_eat(&self.apple) {
                let mut new_apple_pos = rand_point2(GRID_WIDTH, GRID_HEIGHT);
                while new_apple_pos == self.snake.pos_head {
                    new_apple_pos = rand_point2(GRID_WIDTH, GRID_HEIGHT);
                }
                self.apple.set_pos(new_apple_pos);
                self.snake.grow();
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
            event::KeyCode::Return => {
                if self.is_over {
                    self.restart();
                }
            }
            event::KeyCode::Escape => {
                if self.is_over {
                    event::quit(ctx);
                }
            }
            _ => println!("Invalid key press!"),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, C_PALETTE[4].into());
        if self.is_over {
            let mut txt_game_over = graphics::Text::new(
                graphics::TextFragment::new("GAME OVER\n")
                    .color(C_PALETTE[0].into())
                    .scale(graphics::Scale::uniform(40.0)),
            );
            txt_game_over.add(
                graphics::TextFragment::new("Press spacebar to restart.\n")
                    .color(C_PALETTE[0].into())
                    .scale(graphics::Scale::uniform(32.0)),
            );
            txt_game_over.add(
                graphics::TextFragment::new("Press escape to quit.\n")
                    .color(C_PALETTE[0].into())
                    .scale(graphics::Scale::uniform(32.0)),
            );

            let txt_game_over_dims = txt_game_over.dimensions(ctx);
            let txt_game_over_dst = mint::Point2 {
                x: (WINDOW_WIDTH - txt_game_over_dims.0 as f32) / 2.0,
                y: (WINDOW_HEIGHT - txt_game_over_dims.1 as f32) / 2.0,
            };
            graphics::draw(
                ctx,
                &txt_game_over,
                graphics::DrawParam::default().dest(txt_game_over_dst),
            )?;
        } else {
            self.snake.draw(ctx, GRID_SIZE)?;
            self.apple.draw(ctx, GRID_SIZE)?;
        }
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}

/// perform the modulo operation on the two given numbers
fn modulo(x: i32, m: i32) -> i32 {
    let r = x % m;
    if r < 0 {
        r + m
    } else {
        r
    }
}

fn rand_point2(grid_width: i32, grid_height: i32) -> mint::Point2<i32> {
    mint::Point2 {
        x: modulo(rand::random::<i32>(), grid_width),
        y: modulo(rand::random::<i32>(), grid_height),
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("01", "Tung L. Vo")
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .window_setup(conf::WindowSetup::default().title("Snaky"));

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
