use std::io::{self, Write};

pub struct ProgressBar {
    pub total_steps: u64,
    current_step: u64,
}

impl ProgressBar {
    pub fn new(total_steps: u64) -> ProgressBar {
        ProgressBar {
            total_steps,
            current_step: 0,
        }
    }

    pub fn progress(&mut self) {
        self.current_step += 1;
        let percentage = self.current_step as f64 / self.total_steps as f64 * 100.0;

        eprint!("\rProgress: [");

        let completed_steps = (percentage as u64 / 10) as usize;
        for _ in 0..completed_steps {
            eprint!("#");
        }

        for _ in completed_steps..10 {
            eprint!(" ");
        }

        eprint!("] {:.1}%", percentage);
    }

    pub fn finish(&self) {
        eprintln!("\nDone!");
    }
}
