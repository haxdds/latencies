
use std::time::Instant;
use tokio::net::TcpStream;
use std::time::Duration;

#[tokio::main]
async fn main() {
    benchmark_location().await;
}


async fn measure_tcp_latency(host: &str, port: u16) -> Duration {
    let start = Instant::now();
    let _ = TcpStream::connect((host, port)).await;
    start.elapsed()
}

async fn benchmark_location() {
    let host = "clob.polymarket.com";
    
    // Measure 100 times to get stable average
    let mut latencies = vec![];
    for _ in 0..100 {
        let latency = measure_tcp_latency(host, 443).await;
        latencies.push(latency);
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    // Sort latencies for percentile calculations (convert to nanoseconds for sorting)
    let mut sorted_nanos: Vec<u128> = latencies.iter()
        .map(|d| d.as_nanos())
        .collect();
    sorted_nanos.sort();
    
    // Convert sorted nanos back to Duration for calculations
    let sorted_durations: Vec<Duration> = sorted_nanos.iter()
        .map(|&ns| Duration::from_nanos(ns as u64))
        .collect();
    
    // Basic statistics
    let count = latencies.len();
    let sum = latencies.iter().sum::<Duration>();
    let avg = sum / count as u32;
    let min = sorted_durations.first().unwrap();
    let max = sorted_durations.last().unwrap();
    
    // Median (p50)
    let median = if count % 2 == 0 {
        (sorted_durations[count / 2 - 1] + sorted_durations[count / 2]) / 2
    } else {
        sorted_durations[count / 2]
    };
    
    // Percentiles
    let p95_idx = (count as f64 * 0.95).ceil() as usize - 1;
    let p99_idx = (count as f64 * 0.99).ceil() as usize - 1;
    let p95 = sorted_durations[p95_idx.min(count - 1)];
    let p99 = sorted_durations[p99_idx.min(count - 1)];
    
    // Standard deviation
    let avg_nanos = avg.as_nanos() as f64;
    let variance = sorted_nanos.iter()
        .map(|&ns| {
            let diff = ns as f64 - avg_nanos;
            diff * diff
        })
        .sum::<f64>() / count as f64;
    let std_dev_nanos = variance.sqrt();
    let std_dev = Duration::from_nanos(std_dev_nanos as u64);
    
    // Print statistics
    println!("Statistics (n={}):", count);
    println!("  Min:      {:?}", min);
    println!("  Max:      {:?}", max);
    println!("  Average:  {:?}", avg);
    println!("  Median:   {:?} (p50)", median);
    println!("  p95:      {:?}", p95);
    println!("  p99:      {:?}", p99);
    println!("  Std Dev:  {:?}", std_dev);
    

}