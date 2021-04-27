use crate::enums::Command;
use regex::Regex;


pub struct CommandParser {
    split_regex: Regex,
}

impl CommandParser {
    pub fn new() -> Self {
        let regex = Regex::new(r"\W+").unwrap();
        CommandParser { split_regex: regex }
    }

    pub fn parse_line(&self, input: &str) -> Command {
        // Assumptions:
        //  input is whitespace delimited
        //  keys cannot contain whitespace
        //  values may contain whitespace
        //  commands are case-insensitive
        //  keys and values are case sensitive
        //  leading/trailing whitespace is ignored
        let mut parts = self.split_regex.splitn(input.trim(), 3);
        let command = parts.next().unwrap();

        // check that command is valid and required arguments are provided
        // if so, return a command, otherwise return ParseError
        let command = match command.to_lowercase().as_str() {
            "read" => {
                match parts.next() {
                    None | Some("") => Command::ParseError("READ must supply key".to_owned()),
                    Some(key) => Command::Read(key.to_owned()),
                }
            },
            "write" => {
                match (parts.next(), parts.next()) {
                    (None, None) | (Some(""), None) | (Some(_), Some("")) => Command::ParseError("WRITE must supply key and value".to_owned()),
                    (Some(key), Some(value)) => Command::Write(key.to_owned(), value.to_owned()),
                    _ => Command::ParseError("WRITE must supply key and value".to_owned()),
                }
            },
            "delete" => {
                match parts.next() {
                    Some(key) => Command::Delete(key.to_owned()),
                    None => Command::ParseError("DELETE must supply key".to_owned())
                }
            },
            "start" => Command::Start,
            "commit" => Command::Commit,
            "abort" => Command::Abort,
            "quit" => Command::Quit,
            "" => Command::ParseError("Must supply command".to_owned()),
            _ => Command::ParseError(format!("Invalid Command <{}>", command))
        };

        match command {
            // if we already captured an error, return that
            Command::ParseError(_) => command,
            _ => {
                // no other tokens should remain at this point
                match parts.next() {
                    None => command,
                    Some(_) => Command::ParseError("Unexpected input after command".to_owned()),
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let parser = CommandParser::new();
        assert_eq!(Command::Read("foo".to_owned()), parser.parse_line("read foo"));
        assert_eq!(Command::Read("foo".to_owned()), parser.parse_line(" READ foo "));
        assert_eq!(Command::Read("foo".to_owned()), parser.parse_line("READ    foo"));
        assert_eq!(Command::Read("foo".to_owned()), parser.parse_line("rEAd foo"));
        assert_eq!(Command::Read("foo".to_owned()), parser.parse_line("rEAd foo "));
        assert_eq!(Command::ParseError("READ must supply key".to_owned()), parser.parse_line("read"));
        assert_eq!(Command::ParseError("READ must supply key".to_owned()), parser.parse_line("read  "));
    }

    #[test]
    fn test_write() {
        let parser = CommandParser::new();
        assert_eq!(Command::Write("foo".to_owned(), "bar".to_owned()), parser.parse_line("write foo bar"));
        assert_eq!(Command::Write("foo".to_owned(), "bar".to_owned()), parser.parse_line("WRITE foo bar "));
        assert_eq!(Command::Write("foo".to_owned(), "bar".to_owned()), parser.parse_line("WRITE    foo  bar"));
        assert_eq!(Command::Write("foo".to_owned(), "bar baz".to_owned()), parser.parse_line("WRITE foo bar baz"));
        assert_eq!(Command::ParseError("WRITE must supply key and value".to_owned()), parser.parse_line("WRITE"));
        assert_eq!(Command::ParseError("WRITE must supply key and value".to_owned()), parser.parse_line("WRITE foo"));
        assert_eq!(Command::ParseError("WRITE must supply key and value".to_owned()), parser.parse_line("WRITE foo  "));
    }

    #[test]
    fn test_delete() {
        let parser = CommandParser::new();
        assert_eq!(Command::Delete("foo".to_owned()), parser.parse_line("delete foo"));
        assert_eq!(Command::Delete("foo".to_owned()), parser.parse_line("DELETE  foo"));
        assert_eq!(Command::ParseError("Unexpected input after command".to_owned()), parser.parse_line("DELETE  foo bar"));
    }

    #[test]
    fn test_start_commit_abort_quit() {
        let parser = CommandParser::new();
        assert_eq!(Command::Start, parser.parse_line("start"));
        assert_eq!(Command::Commit, parser.parse_line("commit"));
        assert_eq!(Command::Abort, parser.parse_line("abort"));
        assert_eq!(Command::Quit, parser.parse_line("quit"));
    }

    #[test]
    fn test_empty_command() {
        let parser = CommandParser::new();
        assert_eq!(Command::ParseError("Must supply command".to_owned()), parser.parse_line(""));
        assert_eq!(Command::ParseError("Must supply command".to_owned()), parser.parse_line("  "));
    }
}