use clap::Parser;
use task::Task;

mod args;
mod task;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    let _task = Task::from_file(&args.task)?;
    Ok(())
}
