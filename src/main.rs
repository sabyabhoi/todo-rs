use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter}, path::Path,
};

use clap::Parser;
use cli_table::{format::Justify, print_stdout, Table, WithTitle};
use color_eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Table, Serialize, Deserialize)]
struct Todo {
    #[table(title = "ID", justify = "Justify::Right")]
    id: u8,
    #[table(title = "Todos")]
    text: String,
}

#[derive(Parser, Debug)]
struct Args {
    /// Add a todo to the list of todos
    #[clap(short, long, value_parser)]
    add: Option<String>,

    /// Remove a todo by ID
    #[clap(short, long, value_parser)]
    done: Option<u8>,
}

fn read_from_json(filename: &str) -> Result<Vec<Todo>> {
    if !Path::new(filename).exists() {
	File::create(filename)?;
	return Ok(Vec::new());
    }
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

fn write_to_file(filename: &str, todos: &Vec<Todo>) -> Result<()> {
    let file = OpenOptions::new().write(true).truncate(true).open(filename)?;
    let writer = BufWriter::new(file);

    Ok(serde_json::to_writer_pretty(writer, todos)?)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let mut todos: Vec<Todo> = read_from_json("todos.json")?;

    if let Some(todo) = args.add {
        todos.push(Todo {
            id: todos.len() as u8 + 1,
            text: todo,
        });
    }

    if let Some(id) = args.done {
        todos.remove(id as usize - 1);
    }

    print_stdout(todos.with_title())?;
    write_to_file("todos.json", &todos)?;

    Ok(())
}
