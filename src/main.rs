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
#[cfg(feature = "debug")]
use anyhow::Context;
use clap::Parser;
use crossterm::event::{KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, ExecutableCommand};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::{style::Stylize, Color, Constraint, Layout, Rect};
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;
#[cfg(feature = "debug")]
use std::fs::File;
use std::io::stdout;
use std::time::Duration;
#[cfg(feature = "debug")]
use tracing::Level;
#[cfg(feature = "debug")]
use tracing_appender::non_blocking;
#[cfg(feature = "debug")]
use tracing_appender::non_blocking::WorkerGuard;
#[cfg(feature = "debug")]
use tracing_subscriber::EnvFilter;

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
    #[cfg(feature = "debug")]
    /// Enable tracing and debug logging
    #[arg(long, action)]
    tracing: bool,
    #[cfg(feature = "debug")]
    /// Manual ball
    #[arg(long, action)]
    manual_ball: bool,
}

fn main() -> anyhow::Result<()> {
    let opts = ArkanoidOpts::parse();

    // setup tracing and keep its guard
    #[cfg(feature = "debug")]
    let mut _tracing_guard = None;
    #[cfg(feature = "debug")]
    if opts.tracing {
        _tracing_guard = Some(init_tracing()?);
    }

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let game_options = GameOptions::default()
        .paddle_color(Color::LightGreen)
        .walls_color(Color::Blue)
        .ball_speed(2.)
        .area(Rect::new(0, 0, 360, 180).into())
        .brick_count(opts.brick_count);
    let mut game = game_options.clone().build();
    let mut pause = false;

    loop {
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
                        #[cfg(feature = "debug")]
                        KeyCode::Left if opts.manual_ball => {
                            next_event = Some(GameEvent::MoveBallManual {
                                direction: Direction::Left,
                            })
                        }
                        KeyCode::Left => {
                            next_event = Some(GameEvent::MovePad {
                                direction: Direction::Left,
                            });
                        }
                        #[cfg(feature = "debug")]
                        KeyCode::Right if opts.manual_ball => {
                            next_event = Some(GameEvent::MoveBallManual {
                                direction: Direction::Right,
                            });
                        }
                        KeyCode::Right => {
                            next_event = Some(GameEvent::MovePad {
                                direction: Direction::Right,
                            });
                        }
                        #[cfg(feature = "debug")]
                        KeyCode::Up if opts.manual_ball => {
                            next_event = Some(GameEvent::MoveBallManual {
                                direction: Direction::Up,
                            });
                        }
                        #[cfg(feature = "debug")]
                        KeyCode::Down if opts.manual_ball => {
                            next_event = Some(GameEvent::MoveBallManual {
                                direction: Direction::Down,
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
            #[cfg(feature = "debug")]
            if !opts.manual_ball {
                game.event(GameEvent::Tick);
            }
            #[cfg(not(feature = "debug"))]
            game.event(GameEvent::Tick);
        }

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
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// Initialize the tracing subscriber to log to a file
///
/// This function initializes the tracing subscriber to log to a file named `tracing.log` in the
/// current directory. The function returns a [`WorkerGuard`] that must be kept alive for the
/// duration of the program to ensure that logs are flushed to the file on shutdown. The logs are
/// written in a non-blocking fashion to ensure that the logs do not block the main thread.
#[cfg(feature = "debug")]
fn init_tracing() -> anyhow::Result<WorkerGuard> {
    let file = File::create("tracing.log").context("Failed to create tracing.log")?;
    let (non_blocking, guard) = non_blocking(file);

    // By default, the subscriber is configured to log all events with a level of `DEBUG` or higher,
    // but this can be changed by setting the `RUST_LOG` environment variable.
    let env_filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(env_filter)
        .init();
    Ok(guard)
}
