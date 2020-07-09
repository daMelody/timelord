use notifica;
use std::thread;
use std::time::Duration;

use anyhow::Result;

use crate::Args;
#[derive(Debug)]
pub struct Timer {
    cycles: u16,
    flow: u16,
    work: u64,
    rest: u64,
    break_time: u64,
}

impl Timer {
    pub fn new(args: Args) -> Self {
        Timer {
            cycles: args.cycle,
            flow: args.flow,
            work: args.work,
            rest: args.rest,
            break_time: args.break_time,
        }
    }

    pub fn run(&self) -> Result<()> {
        // Send notification that timer is starting
        notifica::notify("TIMELORD", "timelord commencing reign");
        // the time cycle
        for cycle in 0..self.cycles {
            self.describe_cycle(self.cycles - cycle);
            for flow in 0..self.flow {
                self.do_work(flow + 1); // need to adjust for zero-th index
                self.do_rest();
            }
            self.do_break_time();
        }
        notifica::notify(
            "TIMELORD",
            "Congratulations! You have survived the timelord",
        );
        Ok(())
    }

    fn describe_cycle(&self, cycle: u16) {
        let mut cycle_description = String::from("You have ");
        cycle_description.push_str(&cycle.to_string());
        cycle_description.push_str(" cycles to go");
        notifica::notify("TIMELORD", &cycle_description);
    }

    fn do_work(&self, flow: u16) {
        // start WORK period
        let mut work_description = String::from("Working period #");
        work_description.push_str(&flow.to_string());
        work_description.push_str(" starts now");
        notifica::notify("TIMELORD", &work_description);
        match more_than_five_minutes(self.work) {
            true => {
                thread::sleep(Duration::from_secs((self.work * 60) - 300));
                notifica::notify("TIMELORD", "You have five minutes left in your break");
                thread::sleep(Duration::from_secs(300));
            }
            false => {
                thread::sleep(Duration::from_secs(self.work * 60));
            }
        }
        notifica::notify("TIMELORD", "Work period is over!");
    }

    fn do_rest(&self) {
        notifica::notify("TIMELORD", "Rest period starting now");
        match more_than_five_minutes(self.rest) {
            true => {
                thread::sleep(Duration::from_secs((self.rest * 60) - 300));
                notifica::notify("TIMELORD", "You have five minutes left in your break");
                thread::sleep(Duration::from_secs(300));
            }
            false => {
                thread::sleep(Duration::from_secs(self.rest * 60));
            }
        }
    }

    fn do_break_time(&self) {
        notifica::notify("TIMELORD", "Break time starts now");
        match more_than_five_minutes(self.break_time) {
            true => {
                thread::sleep(Duration::from_secs((self.break_time * 60) - 300));
                notifica::notify("TIMELORD", "You have five minutes left in your break");
                thread::sleep(Duration::from_secs(300));
            }
            false => {
                thread::sleep(Duration::from_secs(self.break_time * 60));
            }
        }
        notifica::notify("TIMELORD", "Break time is over!");
    }
}

/// Utility method to check whether a given time value is greater than five minutes
///   - used when deciding whether to give a five minute warning
fn more_than_five_minutes(time: u64) -> bool {
    if time * 60 > 300 {
        true
    } else {
        false
    }
}
