use std::io::{stdout, Result, Stdout};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
    Frame, Terminal,
};

use crate::tui::modals::App;

pub fn main() -> Result<()> {
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = init()?;

    let app = App::new();

    run_app(&mut terminal, app)?;

    restore()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    while !app.exit {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    use KeyCode::*;
                    match key.code {
                        Char('q') | Esc => app.exit(),
                        _ => {}
                    }
                }
            }
        }

        app.update();
    }
    Ok(())
}

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
fn init() -> Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.size());

    let cpu_chart = create_cpu_chart(app);
    f.render_widget(cpu_chart, chunks[0]);

    let mem_chart = create_mem_chart(app);
    f.render_widget(mem_chart, chunks[1]);
}

fn create_cpu_chart(app: &mut App) -> Chart {
    let cpu_per_dataset = vec![Dataset::default()
        .name("CPU")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .graph_type(GraphType::Line)
        .data(&app.datasets.cpu_percent)];

    let chart = Chart::new(cpu_per_dataset)
        .block(Block::default().title("CPU Usage"))
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::White))
                .bounds([0.0, 60.0])
                .labels(
                    ["60s", "50s", "40s", "30s", "20s", "10s", "0s"]
                        .iter()
                        .cloned()
                        .map(Span::from)
                        .collect(),
                ),
        )
        .y_axis(
            Axis::default()
                .title(Span::styled(
                    format!("CPU : {:.2}", app.cpu_stats[0].percentage),
                    Style::default().fg(Color::Red),
                ))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, 100.0])
                .labels(
                    ["0%", "20%", "40%", "60%", "80%", "100%"]
                        .iter()
                        .cloned()
                        .map(Span::from)
                        .collect(),
                ),
        );
    return chart;
}

fn create_mem_chart(app: &mut App) -> Chart {
    let mem_usage = (app.memory_stats.total - app.memory_stats.available) as f64 / (1024.0 * 1024.0);
    let total_mem = app.memory_stats.total as f64 / (1024.0 * 1024.0);

    let mem_per_dataset = vec![Dataset::default()
        .name("CPU")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .graph_type(GraphType::Line)
        .data(&app.datasets.mem_percent)];

    let chart = Chart::new(mem_per_dataset)
        .block(Block::default().title("CPU Usage"))
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::White))
                .bounds([0.0, 60.0])
                .labels(
                    ["60s", "50s", "40s", "30s", "20s", "10s", "0s"]
                        .iter()
                        .cloned()
                        .map(Span::from)
                        .collect(),
                ),
        )
        .y_axis(
            Axis::default()
                .title(Span::styled(
                    format!("Memory : {:.2}Gb / {:.2}Gb", mem_usage, total_mem),
                    Style::default().fg(Color::Red),
                ))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, 100.0])
                .labels(
                    ["0%", "20%", "40%", "60%", "80%", "100%"]
                        .iter()
                        .cloned()
                        .map(Span::from)
                        .collect(),
                ),
        );
    return chart;
}
