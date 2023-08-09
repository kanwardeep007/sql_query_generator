use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// String containing either path or list of ids
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// does testing things
    File {
        /// lists test values
        #[arg(short, long)]
        path: String,
    },
    Ids {
        /// lists test values
        #[arg(short, long)]
        ids: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::File { path } => {}
        Commands::Ids { ids } => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
    }
    let lines = cli.ids;
    let file_path = cli.file_path;
    match lines {
        Some(inner_lines) => {}
        None => {}
    }
    let mut sql_line: String = String::new();
    for line in lines.split("\n") {
        sql_line = sql_line + &format!("\'{}\',", line.to_string());
    }
    let length_of_string = sql_line.len();
    // Truncate the last trailing ','
    sql_line.drain(length_of_string - 1..length_of_string);

    // insert brackets for IN query in sql
    let length_of_string = sql_line.len();
    sql_line.insert(length_of_string, ')');
    sql_line.insert(0, '(');

    print!("{sql_line}");
}
