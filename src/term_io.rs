pub mod term_io {
    use std::thread;
    use std::time::Duration;
    use std::io::{ self, Write };

    fn term_io() {
        // Loop to simulate overwriting terminal output
        for i in 0..=10 {
            // Create a string to print
            let message = format!("Progress: {}%", i * 10);

            // Clear the previous line
            print!("\r{}", " ".repeat(message.len()));

            // Print the message without a 
            print!("{}", message);

            // Flush the output to the terminal
            io::stdout().flush().expect("Failed to flush stdout");

            // Sleep for a short duration to see the effect
            thread::sleep(Duration::from_millis(5000));
        }

        println!("\nDone!");
    }

    // #[cfg(test)]

    // use std::time::Instant;

    // use crate::timer::timer::report_function_timing;

    // use super::*;

    // mod term_io_test {
    //     use std::time::Instant;

    //     use crate::timer::timer::report_function_timing;

    //     use super::*;

    //     #[test]
    //     fn test_term_io() {
    //         term_io();
    //     }
    // }
}
