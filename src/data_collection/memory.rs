use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Result},
};

struct MemInfo {
    total: u64,
    available: u64,
    cache: u64,
}

fn read_mem_info() -> Result<MemInfo> {
    if let Ok(file) = File::open("/proc/meminfo") {
        let reader = BufReader::new(file);

        let mut mem_info = MemInfo {
            total: 0,
            available: 0,
            cache: 0,
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

        return Ok(mem_info);
    }
    Err(Error::new(ErrorKind::NotFound, "Memory data not found"))
}

pub fn mem_usage_percentage() {
    let mem_info = read_mem_info().unwrap();
    let mem_usage_percentage =
        (mem_info.total - mem_info.available) as f64 / mem_info.total as f64 * 100 as f64;

    println!("Memory Usage: {:.2}%", mem_usage_percentage);
    println!("Cached: {}", mem_info.cache);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mem_usage() {
        mem_usage_percentage();
    }
}
