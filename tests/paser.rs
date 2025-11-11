use anyhow::Result;
use cow_parser::{Rule, parse_cow_source, get_commands};

#[test]
fn test_parse_single_command() -> Result<()> {
    let commands = get_commands("MoO").unwrap();
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].as_str(), "MoO");
    Ok(())
}

#[test]
fn test_parse_multiple_commands_with_comments() -> Result<()> {
    let source = "This is a comment MoO and another mOo";
    let commands = get_commands(source).unwrap();
    assert_eq!(commands.len(), 2);
    assert_eq!(commands[0].as_str(), "MoO");
    assert_eq!(commands[1].as_str(), "mOo");
    Ok(())
}