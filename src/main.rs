mod cli;

fn main() {
    let cli = cli::parse();

    println!("{:?}", cli);
}
