use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List of ids separated by \n inside double quotes
    ids: String,
}

fn main() {
    let cli = Cli::parse();

    let lines = cli.ids;
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
