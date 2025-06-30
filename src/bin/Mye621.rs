use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long,num_args = 0..)]
    tags: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();
    for i in args.tags.unwrap_or_default().into_iter() {
        println!("{}", i)
    }
}
