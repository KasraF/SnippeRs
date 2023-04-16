use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 600)]
    pub timeout: u32,

    #[arg(default_value = "resources/tasks/basic.json")]
    pub task: String,
}
