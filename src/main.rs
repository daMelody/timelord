use anyhow::Result;
use clap::Clap;

mod timer;

fn main() -> Result<()> {
    let args = Args::parse();
    let timer = timer::Timer::new(args);
    timer.run()
}

#[derive(Clap, Debug)]
pub struct Args {
    /// number of cycles
    #[clap(short, long, default_value = "2")]
    cycle: u16,
    /// number of working sessions before a break
    #[clap(short, long, default_value = "4")]
    flow: u16,
    /// duration of working period
    #[clap(short, long, default_value = "25")]
    work: u64,
    /// duration of short rest period
    #[clap(short, long, default_value = "5")]
    rest: u64,
    /// duration of long break period
    #[clap(short, long, default_value = "25")]
    break_time: u64,
}
