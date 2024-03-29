use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'n', long = "noout", default_value_t = false)]
    noout: bool,

    #[arg(short, long)]
    response: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("nout: {}", args.noout);
    match args.response {
        Some(v) => println!("response: {}", v),
        None => println!("response: empty"),
    }
}
