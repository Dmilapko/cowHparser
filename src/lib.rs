use pest::error::Error as PestError;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Grammar parsing error: {0}")]
    Pest(#[from] PestError<Rule>),
}

#[derive(Parser)]
#[grammar = "cow.pest"]
pub struct CowParser;

pub fn parse_cow_source(source: &str) -> Result<pest::iterators::Pairs<Rule>, ParseError> {
    let pairs = CowParser::parse(Rule::program, source)?;
    Ok(pairs)
}

pub fn get_commands(source: &str) -> Result<Vec<String>, ParseError> {
    let pairs = CowParser::parse(Rule::program, source)?;

    let commands = pairs
        .flatten()
        .filter(|pair| {
            matches!(
                pair.as_rule(),
                Rule::increment | Rule::decrement | Rule::move_left | Rule::move_right | Rule::other_command
            )
        })
        .map(|pair| pair.as_str().to_string())
        .collect();

    Ok(commands)
}

pub fn optimize_cow_source(source: &str) -> Result<Vec<String>, ParseError> {
    // Reuse the existing parser helper to get a flat list of commands.
    let commands = get_commands(source)?;

    let mut optimized = Vec::new();

    let mut inc: usize = 0;
    let mut dec: usize = 0;
    let mut left_moves: usize = 0;
    let mut right_moves: usize = 0;

    let mut flush_counts = |inc: &mut usize, dec: &mut usize, out: &mut Vec<String>| {
        if *inc > 0 {
            out.push(format!("increment({})", *inc));
            *inc = 0;
        }
        if *dec > 0 {
            out.push(format!("decrement({})", *dec));
            *dec = 0;
        }
    };

    for cmd in commands {
        match cmd.as_str() {
            "MoO" => {
                if dec > 0 {
                    flush_counts(&mut inc, &mut dec, &mut optimized);
                }
                inc += 1;
            }
            "MOo" => {
                if inc > 0 {
                    flush_counts(&mut inc, &mut dec, &mut optimized);
                }
                dec += 1;
            }
            "mOo" => {
                left_moves += 1;
            }
            "moO" => {
                right_moves += 1;
            }
            _ => {
                flush_counts(&mut inc, &mut dec, &mut optimized);
                optimized.push(cmd);
            }
        }
    }
    flush_counts(&mut inc, &mut dec, &mut optimized);

    if right_moves > left_moves {
        for _ in 0..(right_moves - left_moves) {
            optimized.push("moO".to_string());
        }
    } else if left_moves > right_moves {
        for _ in 0..(left_moves - right_moves) {
            optimized.push("mOo".to_string());
        }
    }

    Ok(optimized)
}
