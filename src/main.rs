use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("Command: {}", args.operation);
}

#[derive(Parser, Debug)]
#[command(version,about, long_about= None)]
struct Args {
    #[arg(short, long)]
    operation: String,
}
