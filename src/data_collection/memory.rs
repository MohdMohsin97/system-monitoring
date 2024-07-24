use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Result},
};

#[derive(Debug)]
pub struct MemInfo {
    pub total: u64,
    pub available: u64,
    pub cache: u64,
    pub percentage: f64,
}

pub fn read_mem_info() -> Result<MemInfo> {
    if let Ok(file) = File::open("/proc/meminfo") {
        let reader = BufReader::new(file);

        let mut mem_info = MemInfo {
            total: 0,
            available: 0,
            cache: 0,
            percentage: 0.0
        };
        for line in reader.lines() {
            let line = line?;

            match line.split_whitespace().collect::<Vec<&str>>()[..] {
                ["MemTotal:", value, ..] => mem_info.total = value.parse().unwrap(),
                ["MemAvailable:", value, ..] => mem_info.available = value.parse().unwrap(),
                ["Cached:", value, ..] => mem_info.cache = value.parse().unwrap(),
                _ => {}
            }
        }

        mem_info.percentage = mem_usage_percentage(&mem_info);

        return Ok(mem_info);
    }
    Err(Error::new(ErrorKind::NotFound, "Memory data not found"))
}

fn mem_usage_percentage(mem_info : &MemInfo) -> f64 {
    let mem_usage_percentage =
        (mem_info.total - mem_info.available) as f64 / mem_info.total as f64 * 100 as f64;

    mem_usage_percentage
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mem_usage() {
        read_mem_info();
    }
}
