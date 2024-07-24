use std::io::Result;

mod tui;
mod data_collection;

fn main() -> Result<()> {
    tui::main::main()?;

    Ok(())
}