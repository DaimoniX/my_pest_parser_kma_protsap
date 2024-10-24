use pest_derive::Parser;
use pest::Parser;
use anyhow::Result;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> Result<()> {
    let result = Grammar::parse(Rule::file, "-432.123,100.40\n111.11,-55,20\n")?;
    println!("{:?}", result);

    Ok(())
}
