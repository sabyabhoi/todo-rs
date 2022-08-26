use clap::Parser;
use cli_table::{Table, format::Justify, print_stdout, WithTitle};
use color_eyre::Result;

#[derive(Table)]
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
    done: Option<u8>
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let mut todos: Vec<Todo> = Vec::new();

    if let Some(todo) = args.add  {
	todos.push(Todo{
	    id: todos.len() as u8 + 1,
	    text: todo
	});
    }

    if let Some(id) = args.done {
	todos.remove(id as usize - 1);
    }

    print_stdout(todos.with_title())?;

    Ok(())
}
