// src/main.rs
use std::io;
use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Color,
    widgets::canvas::{Canvas, Context, Points},
    Frame, Terminal,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let res = run(&mut terminal);

    disable_raw_mode()?;
    terminal.show_cursor()?;

    res
}

fn run(terminal: &mut Terminal<CrosstermBackend<&mut io::Stdout>>) -> Result<()> {
    let mut tick: u64 = 0;
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(80);

    loop {
        // Draw current frame
        terminal.draw(|f| draw_ui(f, tick))?;

        // Handle input and ticking
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_millis(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            tick = tick.wrapping_add(1);
            last_tick = Instant::now();
        }
    }
    Ok(())
}

fn _draw_ui(frame: &mut Frame, tick: u64) {
    let area = frame.area();

    let color = rainbow_color(tick);

    let canvas = Canvas::default()
        .x_bounds([-2.0, 2.0])
        .y_bounds([-2.0, 2.0])
        .paint(move |ctx| _draw_heart(ctx, area, color));

    frame.render_widget(canvas, area);
}

fn draw_ui(frame: &mut Frame, tick: u64) {
    let area = frame.area();

    let color = rainbow_color(tick);

    let canvas = Canvas::default()
        .x_bounds([-2.0, 2.0])
        .y_bounds([-2.0, 2.0])
        .paint(move |ctx| {
            // thickness in world units (try 0.03...0.08)
            draw_heart(ctx, area, color, 0.05);
        });

    frame.render_widget(canvas, area);
}


// Simple rainbow over named colors
fn rainbow_color(tick: u64) -> Color {
    // Cycle through some bright colors
    const COLORS: [Color; 5] = [
        Color::Red,
        Color::Yellow,
        Color::Magenta,
		Color::LightRed,
        Color::LightMagenta,
    ];

    let idx = (tick as usize) % COLORS.len();
    COLORS[idx]
}

/// Draw a Valentine-style heart using a parametric equation on the Canvas.
fn _draw_heart(ctx: &mut Context, _area: Rect, color: Color) {
    let mut pts = Vec::new();
    let steps = 1000;
    for i in 0..=steps {
        let t = (i as f64) * std::f64::consts::PI * 2.0 / (steps as f64);

        let x = 16.0 * (t.sin().powi(3));
        let y = 13.0 * t.cos()
            - 5.0 * (2.0 * t).cos()
            - 2.0 * (3.0 * t).cos()
            - (4.0 * t).cos();

        let x_norm = x / 10.0;
        let y_norm = y / 10.0;

        pts.push((x_norm, y_norm));
    }

    let heart = Points {
        coords: &pts,
        color,
    };

    ctx.draw(&heart);
}

/// Draw a thicker Valentine-style heart by rendering several scaled curves.
fn draw_heart(ctx: &mut Context, _area: Rect, color: Color, thickness: f64) {
    let steps = 1000;
    // Number of "layers" to draw around the base heart
    let layers = 4;

    for layer in 0..layers {
        // Scale factor: inner to outer
        let scale = 1.0 + (layer as f64) * thickness;

        let mut pts = Vec::with_capacity(steps + 1);
        for i in 0..=steps {
            let t = (i as f64) * std::f64::consts::PI * 2.0 / (steps as f64);

            let x = 16.0 * (t.sin().powi(3));
            let y = 13.0 * t.cos()
                - 5.0 * (2.0 * t).cos()
                - 2.0 * (3.0 * t).cos()
                - (4.0 * t).cos();

            // Normalize and apply scale
            let x_norm = (x / 10.0) * scale;
            let y_norm = (y / 10.0) * scale;

            pts.push((x_norm, y_norm));
        }

        ctx.draw(&Points {
            coords: &pts,
            color,
        });
    }
}
