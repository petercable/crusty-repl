#[derive(Eq, PartialEq, Debug)]
pub enum HandleResult {
    Success,
    Result(String),
    Failure(String),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Read(String),
    Write(String, String),
    Delete(String),
    Start,
    Abort,
    Commit,
    Quit,
    ParseError(String),
}