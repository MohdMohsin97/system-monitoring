use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
    Frame,
};

use crate::data_collection;

pub struct App {
    cpu_percent: Vec<(f64, f64)>,
    mem_percent: Vec<(f64, f64)>,
}

impl App {
    pub fn new() -> App {
        App {
            cpu_percent: vec![(0.0, 0.0); 60],
            mem_percent: vec![(0.0, 0.0); 60],
        }
    }

    pub fn update(&mut self) {
        let mem_percent = data_collection::mem_usage_percentage();

        let cpu_percents = data_collection::cpu_usage_percentage().unwrap();

        let cpu_percent = cpu_percents[0].1;

        let mem_point = (self.cpu_percent.len() as f64, mem_percent);
        self.mem_percent.push(mem_point);

        let cpu_point = (self.cpu_percent.len() as f64, cpu_percent);
        self.cpu_percent.push(cpu_point);

        if self.cpu_percent.len() > 60 {
            self.cpu_percent.remove(0);
        }

        if self.mem_percent.len() > 60 {
            self.mem_percent.remove(0);
        }

        for (i, point) in self.cpu_percent.iter_mut().enumerate() {
            point.0 = (i + 1) as f64;
        }

        for (i, point) in self.mem_percent.iter_mut().enumerate() {
            point.0 = (i + 1) as f64;
        }

    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.size());

    let cpu_per_dataset = vec![Dataset::default()
        .name("CPU")
        .marker(symbols::Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .graph_type(GraphType::Line)
        .data(&app.cpu_percent)];

    let chart = Chart::new(cpu_per_dataset)
        .block(Block::default().title("CPU Usage"))
        .x_axis(
            Axis::default()
                .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
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
                .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
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
    f.render_widget(chart, chunks[0]);

    let mem_per_dataset = vec![Dataset::default()
        .name("Memory")
        .marker(symbols::Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .graph_type(GraphType::Line)
        .data(&app.mem_percent)];

    let chart = Chart::new(mem_per_dataset)
        .block(Block::default().title("Memory Usage"))
        .x_axis(
            Axis::default()
                .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
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
                .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
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
    f.render_widget(chart, chunks[1]);
}
