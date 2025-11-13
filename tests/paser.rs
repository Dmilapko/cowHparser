use anyhow::Result;
use cow_parser::{get_commands, optimize_cow_source, parse_cow_source, Rule};

#[test]
fn test_parse_cow_source_success() -> Result<()> {
    let result = parse_cow_source("MoO MOo");
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_parse_cow_source_failure() {
    let result = parse_cow_source("Invalid Command");
    assert!(result.is_ok()); 
}

#[test]
fn test_get_commands_simple() -> Result<()> {
    let commands = get_commands("MoO")?;
    assert_eq!(commands, vec!["MoO"]);
    Ok(())
}

#[test]
fn test_get_commands_with_comments() -> Result<()> {
    let source = "Comment MoO another mOo";
    let commands = get_commands(source)?;
    assert_eq!(commands, vec!["MoO", "mOo"]);
    Ok(())
}

#[test]
fn test_optimize_increments() -> Result<()> {
    let source = "MoO MoO MoO";
    let optimized = optimize_cow_source(source)?;
    assert_eq!(optimized, vec!["increment(3)"]);
    Ok(())
}

#[test]
fn test_optimize_decrements() -> Result<()> {
    let source = "MOo MOo";
    let optimized = optimize_cow_source(source)?;
    assert_eq!(optimized, vec!["decrement(2)"]);
    Ok(())
}

#[test]
fn test_optimize_cancelling_moves() -> Result<()> {
    let source = "mOo moO";
    let optimized = optimize_cow_source(source)?;
    assert!(optimized.is_empty());
    Ok(())
}

#[test]
fn test_optimize_mixed_commands() -> Result<()> {
    let source = "mOo MMM moO MOo MOo MoO";
    let optimized = optimize_cow_source(source)?;
    assert_eq!(optimized, vec!["MMM", "decrement(2)", "increment(1)"]);
    Ok(())
}