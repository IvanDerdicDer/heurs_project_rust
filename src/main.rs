use instance_parser::parse_instance;
use anyhow::Result;

mod instance_parser;
mod ant_colony;

fn main() -> Result<()> {
    let instance = parse_instance("instances/inst1.TXT")?;

    println!("{:?}", instance);

    Ok(())
}
