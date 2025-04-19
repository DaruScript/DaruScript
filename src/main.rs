use clap::Parser;
use daru_script::calculator::type_check;
use std::io::{self, Write};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // file_path: Option<String>,
    #[clap(short, long)]
    printast: bool,

    #[clap(short, long)]
    typecheck: bool,
}

fn main() {
    use daru_script::calculator::run;
    use daru_script::calculator::gen_ast;

    let cli = Cli::parse();

    // if let Some(file_path) = cli.file_path {
    //     todo!()
    // }

    println!("calculator");

    // REPL
    loop {
        print!("> ");
        io::stdout().flush().expect("flush failed");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() || ["quit()", "exit()", "やめてください"].contains(&line) {
            break;
        }

        println!("{:?}", line);
        if cli.printast {
            println!("{}", gen_ast(&line));
        } else if cli.typecheck {
            println!("{}", type_check(&line));
        } else {
            println!("{}", run(&line));
        }
    }
}
