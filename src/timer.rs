

pub mod timer {
    use std::time::{Instant};

    pub fn report_function_timing(start: Instant, name: &str) {
        let elapsed = start.elapsed();
        println!("{} execution time: {:?}", name, elapsed);
    }
}