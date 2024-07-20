use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind, Result};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct CpuTimes {
    cpu: String,
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64,
}

fn read_cpu_times() -> Result<Vec<CpuTimes>> {
    if let Ok(file) = File::open("/proc/stat") {
        let reader = io::BufReader::new(file);

        let mut cpu_times: Vec<CpuTimes> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.starts_with("cpu") {
                let times = parse_cputimes(line);
                cpu_times.push(times);
            }
        }
        return Ok(cpu_times);
    }
    Err(Error::new(ErrorKind::NotFound, "CPU data not found"))
}

fn parse_cputimes(line: String) -> CpuTimes {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let times = CpuTimes {
        cpu: parts[0].to_owned(),
        user: parts[1].parse().unwrap(),
        nice: parts[2].parse().unwrap(),
        system: parts[3].parse().unwrap(),
        idle: parts[4].parse().unwrap(),
        iowait: parts[5].parse().unwrap(),
        irq: parts[6].parse().unwrap(),
        softirq: parts[7].parse().unwrap(),
        steal: parts[8].parse().unwrap(),
        guest: parts[9].parse().unwrap(),
        guest_nice: parts[10].parse().unwrap(),
    };
    times
}

fn calculate_cpu_usage(
    prev_times: &Vec<CpuTimes>,
    curr_times: &Vec<CpuTimes>,
) -> Vec<(String, f64)> {
    let mut cpu_percentages: Vec<(String, f64)> = Vec::new();

    for i in 0..prev_times.len() {
        let prev = &prev_times[i];
        let curr = &curr_times[i];

        let prev_idle = prev.idle + prev.iowait;
        let curr_idle = curr.idle + curr.iowait;

        let prev_non_idle =
            prev.user + prev.nice + prev.system + prev.irq + prev.softirq + prev.steal;
        let curr_non_idle =
            curr.user + curr.nice + curr.system + curr.irq + curr.softirq + curr.steal;

        let prev_total = prev_idle + prev_non_idle;
        let curr_total = curr_idle + curr_non_idle;

        let totald = curr_total - prev_total;
        let idled = curr_idle - prev_idle;

        let cpu_percentage = (totald - idled) as f64 / totald as f64 * 100.0;

        cpu_percentages.push((prev.cpu.clone(), cpu_percentage));
    }

    cpu_percentages
}

pub fn cpu_usage_percentage() -> Result<Vec<(String, f64)>> {
    let prev = read_cpu_times().expect("Failed to read CPU times");
    // Wait for a second
    sleep(Duration::new(1, 0));
    // Second snapshot
    let curr = read_cpu_times().expect("Failed to read CPU times");

    let cpu_usage: Vec<(String, f64)> = calculate_cpu_usage(&prev, &curr);

    // for usage in &cpu_usage {
    //     println!("{} usage is {:.2}", usage.0, usage.1);
    // }

    Ok(cpu_usage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_percentage() {
        let cpu_usage = cpu_usage_percentage().unwrap();

        for usage in cpu_usage {
            println!("{} usage is {:.2}", usage.0, usage.1);
        }
    }
}
