// src/main.rs
//! Valentine's Day Rainbow Heart TUI - Ratatui + Crossterm
//! Draws an animated, thick heart with cycling rainbow colors.
//! Press 'q' or ESC to quit.

use std::io;
use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    style::Color,
    widgets::canvas::{Canvas, Context, Points},
    Frame, Terminal,
};

fn main() -> Result<()> {
    // Initialize error handling and terminal
    color_eyre::install()?;
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    
    // Run the main event loop
    let res = run(&mut terminal);
    
    // Cleanup
    disable_raw_mode()?;
    terminal.show_cursor()?;
    
    res
}

/// Main event loop with animation timing and input handling.
fn run(terminal: &mut Terminal<CrosstermBackend<&mut io::Stdout>>) -> Result<()> {
    let mut tick: u64 = 0;
    let mut last_tick = Instant::now();
    const TICK_RATE: Duration = Duration::from_millis(80); // ~12.5 FPS

    loop {
        // Draw current frame
        terminal.draw(|f| draw_ui(f, tick))?;

        // Calculate poll timeout for smooth timing
        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_default();

        // Handle input events
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    break;
                }
            }
        }

        // Advance animation frame
        if last_tick.elapsed() >= TICK_RATE {
            tick = tick.wrapping_add(1);
            last_tick = Instant::now();
        }
    }
    
    Ok(())
}

/// Render the main UI with animated rainbow heart canvas.
fn draw_ui(frame: &mut Frame, tick: u64) {
    let area = frame.area();
    let color = rainbow_color(tick);

    let canvas = Canvas::default()
        .x_bounds([-2.0, 2.0])
        .y_bounds([-2.0, 2.0])
        .paint(|ctx| {
            // Draw thick heart (0.05 world units thickness)
            draw_heart(ctx, color, 0.05);
        });

    frame.render_widget(canvas, area);
}

/// Generate rainbow color from tick counter using predefined bright colors.
fn rainbow_color(tick: u64) -> Color {
    const RAINBOW: [Color; 5] = [
        Color::Red,
        Color::Yellow,
        Color::Magenta,
        Color::LightRed,
        Color::LightMagenta,
    ];

    RAINBOW[(tick as usize) % RAINBOW.len()]
}

/// Draw thick Valentine heart using parametric equation with multiple layers.
fn draw_heart(ctx: &mut Context, color: Color, thickness: f64) {
    const STEPS: usize = 1000;
    const LAYERS: usize = 4;

    for layer in 0..LAYERS {
        // Scale each layer outward for thickness effect
        let scale = 1.0 + (layer as f64) * thickness;
        let mut points = Vec::with_capacity(STEPS + 1);

        for i in 0..=STEPS {
            let t = (i as f64 / STEPS as f64) * std::f64::consts::TAU;

            // Classic heart parametric equation
            let x = 16.0 * t.sin().powi(3);
            let y = 13.0 * t.cos()
                - 5.0 * (2.0 * t).cos()
                - 2.0 * (3.0 * t).cos()
                - (4.0 * t).cos();

            // Normalize to [-2, 2] bounds and apply thickness scaling
            points.push((
                (x / 10.0) * scale,
                (y / 10.0) * scale,
            ));
        }

        ctx.draw(&Points {
            coords: &points,
            color,
        });
    }
}
