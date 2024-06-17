use crate::ball::{Ball, EllasticCollision};
use crate::bottom::Bottom;
use crate::brick::Brick;
use crate::letters::Word;
use crate::paddle::{Direction, Paddle};
use crate::rectf64::Rectf64;
use crate::walls::Walls;
use rand::{seq::SliceRandom, thread_rng};
use ratatui::style::Color;
use ratatui::widgets::canvas::{Painter, Shape};

/// Width of a brick.
const BRICK_WIDTH: f64 = 14.0;

/// Height of a brick.
const BRICK_HEIGHT: f64 = 5.0;

/// Width of the wall.
const WALL_W: f64 = 2.0;

/// Height of the wall.
const WALL_H: f64 = 2.0;

/// Represents the state of the game.
#[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
pub enum GameState {
    /// The game is running.
    #[default]
    Running,
    /// The player has lost the game.
    Lost,
    /// The player has won the game.
    Won,
}

/// Represents an event in the game.
#[derive(Debug)]
pub enum GameEvent {
    /// Event to move the paddle in a specified direction.
    MovePad { direction: Direction },
    /// Event to update the game state with a time delta.
    Tick { dt: f64 },
}

/// Represents the options for configuring the game.
#[derive(Default, Clone, Debug)]
pub struct GameOptions {
    /// The number of bricks in the game.
    brick_count: u16,
    /// The color of the walls.
    walls_color: Color,
    /// The color of the paddle.
    paddle_color: Color,
    /// The rectangular area defining the game space.
    area: Rectf64,
    ball_speed: f64,
}

impl GameOptions {
    /// Sets the number of bricks in the game.
    ///
    /// # Parameters
    /// - `count`: The number of bricks.
    ///
    /// # Returns
    /// The updated `GameOptions`.
    pub fn brick_count(mut self, count: u16) -> Self {
        self.brick_count = count;
        self
    }

    /// Sets the color of the walls.
    ///
    /// # Parameters
    /// - `color`: The color of the walls.
    ///
    /// # Returns
    /// The updated `GameOptions`.
    pub fn walls_color(mut self, color: Color) -> Self {
        self.walls_color = color;
        self
    }

    /// Sets the color of the paddle.
    ///
    /// # Parameters
    /// - `color`: The color of the paddle.
    ///
    /// # Returns
    /// The updated `GameOptions`.
    pub fn paddle_color(mut self, color: Color) -> Self {
        self.paddle_color = color;
        self
    }

    /// Sets the game area.
    ///
    /// # Parameters
    /// - `area`: The rectangular area defining the game space.
    ///
    /// # Returns
    /// The updated `GameOptions`.
    pub fn area(mut self, area: Rectf64) -> Self {
        self.area = area;
        self
    }

    pub fn ball_speed(mut self, v: f64) -> Self {
        self.ball_speed = v;
        self
    }

    /// Builds and returns a `Game` instance with the specified options.
    ///
    /// # Returns
    /// A `Game` instance.
    pub fn build(mut self) -> Game {
        let brick_area = BRICK_HEIGHT * BRICK_WIDTH;
        let available_space = (self.area.height - WALL_H) / 2.0 * (self.area.width - WALL_W * 2.0);
        let max_brick_count = (available_space / brick_area) as u16;
        self.brick_count = self.brick_count.min(max_brick_count);
        let bricks_rect = Rectf64 {
            x: self.area.x + WALL_W,
            y: self.area.y + self.area.height / 2.0,
            width: self.area.width - 2.0 * WALL_W,
            height: self.area.height / 2.0 - WALL_H,
        };
        let pad_x = bricks_rect.width as usize % BRICK_WIDTH as usize / 2;
        let mut coords = vec![];
        for x in (bricks_rect.left() as usize + pad_x
            ..=(bricks_rect.right() - BRICK_WIDTH) as usize - pad_x)
            .step_by(BRICK_WIDTH as usize)
        {
            for y in (bricks_rect.bottom() as usize
                ..bricks_rect.top() as usize - BRICK_HEIGHT as usize)
                .step_by(BRICK_HEIGHT as usize)
            {
                coords.push(Rectf64 {
                    x: x as f64,
                    y: y as f64,
                    width: BRICK_WIDTH,
                    height: BRICK_HEIGHT,
                });
            }
        }
        coords.shuffle(&mut thread_rng());
        let bricks = coords
            .into_iter()
            .take(self.brick_count as usize)
            .map(|area| Brick::new(area))
            .collect();
        let paddle_h = self.area.height / 50.0;
        let paddle_w = self.area.width / 10.0;
        let paddle_area = Rectf64 {
            x: self.area.x + WALL_W,
            y: self.area.y + WALL_H,
            width: paddle_w,
            height: paddle_h,
        };
        let paddle = Paddle::new(
            paddle_area.clone(),
            self.area.x + WALL_W,
            self.area.x + self.area.width - WALL_W,
            8.0,
            self.paddle_color,
        );
        let walls = Walls::new(
            Rectf64 {
                x: self.area.x,
                y: self.area.y,
                width: WALL_W,
                height: self.area.height,
            },
            Rectf64 {
                x: self.area.x + self.area.width - WALL_W,
                y: self.area.y,
                width: WALL_W,
                height: self.area.height,
            },
            Rectf64 {
                x: self.area.x,
                y: self.area.y + self.area.height - WALL_H,
                width: self.area.width,
                height: WALL_H,
            },
            self.walls_color,
        );
        let radius = 3.;
        let ball = Ball::new(
            paddle_area.left() + 2. * radius,
            paddle_area.top() + radius,
            radius,
            self.ball_speed,
            self.ball_speed,
        );
        let bottom = Bottom::new(
            Rectf64 {
                x: self.area.x,
                y: self.area.y,
                width: self.area.width,
                height: 1.0,
            },
            Color::Gray,
        );

        Game {
            area: self.area,
            paddle,
            ball,
            walls,
            bottom,
            bricks,
            state: Default::default(),
            score: 0,
        }
    }
}

/// Represents the game state and logic.
#[derive(Debug, Default)]
pub struct Game {
    /// The rectangular area defining the game space.
    area: Rectf64,
    /// The current state of the game.
    state: GameState,
    /// The paddle in the game.
    paddle: Paddle,
    /// The ball in the game.
    ball: Ball,
    /// The walls in the game.
    walls: Walls,
    /// The bottom boundary of the game.
    bottom: Bottom,
    /// The bricks in the game.
    bricks: Vec<Brick>,
    /// The current score of the game.
    score: u16,
}

impl Game {
    /// Processes a game event.
    ///
    /// # Parameters
    /// - `game_event`: The game event to process.
    pub fn event(&mut self, game_event: GameEvent) {
        if self.state != GameState::Running {
            return;
        }

        match game_event {
            GameEvent::MovePad { direction } => match direction {
                Direction::Left => {
                    self.paddle.mov(Direction::Left);
                }
                Direction::Right => {
                    self.paddle.mov(Direction::Right);
                }
            },
            GameEvent::Tick { dt } => {
                self.move_ball(dt);
            }
        }
    }

    /// Moves the ball and checks for collisions.
    ///
    /// # Parameters
    /// - `dt`: The time delta for the movement.
    ///
    /// TODO: maybe I need to predict collisions
    /// instead of acting upon them, but for now
    /// this implementation is ok.
    pub fn move_ball(&mut self, dt: f64) {
        // Move the ball and check if it possibly
        // fell down. If yes - the game is lost.
        self.ball.mov(dt);
        if self.bottom.collide(&mut self.ball) {
            self.state = GameState::Lost;
            return;
        }

        // Check if the ball collided with any of the bricks
        // and if it did - remove those.
        let mut bricks = vec![];
        for mut brick in std::mem::take(&mut self.bricks).into_iter() {
            if brick.collide(&mut self.ball) {
                self.score += 1;
            } else {
                bricks.push(brick);
            }
        }
        // If no bricks left - the game is won.
        if bricks.is_empty() {
            self.state = GameState::Won;
        }
        std::mem::swap(&mut self.bricks, &mut bricks);

        // Process ball collision with the walls and the
        // paddle.
        self.walls.collide(&mut self.ball);
        self.paddle.collide(&mut self.ball);
    }
}

impl Shape for Game {
    fn draw(&self, painter: &mut Painter) {
        self.walls.draw(painter);
        self.paddle.draw(painter);
        self.ball.draw(painter);
        self.bricks.iter().for_each(|brick| brick.draw(painter));

        match &self.state {
            GameState::Lost => {
                Word::new(
                    "game over".to_string(),
                    (
                        self.area.x + self.area.width * 0.35,
                        self.area.y + self.area.height / 2.,
                    ),
                    12.0,
                    Color::Red,
                )
                .draw(painter);
            }
            GameState::Won => {
                Word::new(
                    "you won".to_string(),
                    (
                        self.area.x + self.area.width * 0.35,
                        self.area.y + self.area.height / 2.,
                    ),
                    12.0,
                    Color::LightGreen,
                )
                .draw(painter);
            }
            _ => {}
        }

        Word::new(
            format!("score: {}", self.score),
            (
                self.area.x + self.area.width * 0.01,
                self.area.y + self.area.height * 0.95,
            ),
            7.0,
            Color::White,
        )
        .draw(painter);
    }
}
