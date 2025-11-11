use thiserror::Error;
use pest::error::Error as PestError;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Grammar parsing error: {0}")]
    Pest(#[from] PestError<Rule>),
}

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "cow.pest"]
pub struct CowParser;

use pest::Parser;

pub fn parse_cow_source(source: &str) -> Result<pest::iterators::Pairs<Rule>, ParseError> {
    let pairs = CowParser::parse(Rule::program, source)?;
    Ok(pairs)
}

pub fn get_commands(source: &str) -> Result<Vec<String>, ParseError> {
    let pairs = CowParser::parse(Rule::program, source)?;

    let commands = pairs
        .filter(|pair| pair.as_rule() == Rule::command)
        .map(|pair| pair.as_str().to_string())
        .collect();

    Ok(commands)
}