use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg()]
    script: Option<String>
}

fn main() {
    let _args = Args::parse();
}
