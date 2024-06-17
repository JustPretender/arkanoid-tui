mod ball;
mod bottom;
mod brick;
mod game;
mod letters;
mod paddle;
mod rectf64;
mod walls;

use crate::game::{GameEvent, GameOptions};
use crate::paddle::Direction;
use clap::Parser;
use crossterm::event::{KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, ExecutableCommand};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::{style::Stylize, Color, Constraint, Layout, Marker, Rect};
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;
use std::io::stdout;
use std::time::Duration;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct ArkanoidOpts {
    /// Number of bricks, rendered by the game
    #[arg(long, default_value_t = 10)]
    brick_count: u16,
    /// Possible marker value: Dot, Braille, Bar, Block, HalfBlock
    #[arg(long, default_value_t = Marker::HalfBlock)]
    marker: Marker,
    /// Game FPS
    #[arg(long, default_value_t = 24)]
    fps: u16,
}

fn main() -> anyhow::Result<()> {
    let opts = ArkanoidOpts::parse();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let game_options = GameOptions::default()
        .paddle_color(Color::LightGreen)
        .walls_color(Color::Blue)
        .ball_speed(5.)
        .area(Rect::new(0, 0, 360, 180).into())
        .brick_count(opts.brick_count);
    let mut game = game_options.clone().build();
    let mut pause = false;

    loop {
        terminal.draw(|frame| {
            let vertical =
                Layout::vertical([Constraint::Percentage(99), Constraint::Percentage(2)]);
            let [game_area, controls_area] = vertical.areas(frame.size());
            frame.render_widget(
                Canvas::default()
                    .marker(opts.marker)
                    .x_bounds([0.0, 360.0])
                    .y_bounds([0.0, 180.0])
                    .paint(|ctx| {
                        ctx.draw(&game);
                    }),
                game_area,
            );
            frame.render_widget(
                Paragraph::new("\nUse ← → to move, TAB to restart, ↵ to pause.")
                    .centered()
                    .bold(),
                controls_area,
            );
        })?;

        let tick = 1000 / opts.fps as u64;
        let tick_duration = Duration::from_millis(tick);
        let mut next_event = None;

        if event::poll(tick_duration)? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Left => {
                            next_event = Some(GameEvent::MovePad {
                                direction: Direction::Left,
                            });
                        }
                        KeyCode::Right => {
                            next_event = Some(GameEvent::MovePad {
                                direction: Direction::Right,
                            });
                        }
                        KeyCode::Tab => {
                            game = game_options.clone().build();
                        }
                        KeyCode::Enter => {
                            pause = !pause;
                        }
                        _ => {}
                    }
                }
            }
        }

        if !pause {
            if let Some(event) = next_event {
                game.event(event);
            }
            game.event(GameEvent::Tick {
                dt: tick as f64 / 100.,
            });
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
