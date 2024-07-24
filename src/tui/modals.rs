use crate::data_collection::{cpu_usage_percentage, read_mem_info, CpuInfo, MemInfo};

#[derive(Debug)]
pub struct DataSets {
    pub cpu_percent: Vec<(f64, f64)>,
    pub mem_percent: Vec<(f64, f64)>,
}

impl DataSets {
    fn new() -> DataSets {
        DataSets {
            cpu_percent: vec![(0.0, 0.0); 60],
            mem_percent: vec![(0.0, 0.0); 60],
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub cpu_stats: Vec<CpuInfo>,
    pub memory_stats: MemInfo,
    pub exit: bool,
    pub datasets: DataSets,
}

impl App {
    pub fn new() -> App {
        let cpu_stats = cpu_usage_percentage().expect("Not able to fetch cpu stats");

        let memory_stats = read_mem_info().expect("not able to read mem stats");

        let datasets = DataSets::new();

        App {
            cpu_stats,
            memory_stats,
            exit: false,
            datasets,
        }
    }

    pub fn update(&mut self) {
        self.cpu_stats = cpu_usage_percentage().expect("Not able to fetch cpu stats");

        self.memory_stats = read_mem_info().expect("not able to read mem stats");

        update_cpu_dataset(self, self.cpu_stats[0].percentage);
        update_mem_dataset(self, self.memory_stats.percentage);

    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}

fn update_cpu_dataset(app: &mut App, percentage: f64 ) {
    let cpu_point = (app.datasets.cpu_percent.len() as f64, percentage);
    app.datasets.cpu_percent.push(cpu_point);

    if app.datasets.cpu_percent.len() > 60 {
        app.datasets.cpu_percent.remove(0);
    }

    for (i, point) in app.datasets.cpu_percent.iter_mut().enumerate() {
        point.0 = (i + 1) as f64;
    }
}

fn update_mem_dataset(app: &mut App, percentage: f64 ) {
    let mem_point = (app.datasets.mem_percent.len() as f64, percentage);
    app.datasets.mem_percent.push(mem_point);

    if app.datasets.mem_percent.len() > 60 {
        app.datasets.mem_percent.remove(0);
    }

    for (i, point) in app.datasets.mem_percent.iter_mut().enumerate() {
        point.0 = (i + 1) as f64;
    }
}