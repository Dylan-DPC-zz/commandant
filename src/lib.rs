#[macro_use]
pub mod macros;

#[macro_use]
extern crate failure;

use std::process::{ Command, Output, ExitStatus };
use failure::Error;
use std::borrow::Cow;
use std::fmt::{ Display, Formatter};
use std::ffi::OsString;


#[derive(Clone, Debug)]
pub struct Commandant<T : Into<OsString> + Clone> {
    command: T,
    args: Vec<T>
}

impl<T> Commandant<T>
where T: Into<OsString> + Clone
{
    pub fn new(command: T, args: &[T]) -> Commandant<T> {
            Commandant {
            command,
            args: args.to_vec()
        }
    }

    pub fn run(&self) -> Result<OutputBag, Error> {
    }
}


pub struct OutputBag {
    output: Result<Output, CommandError>
}

impl OutputBag
{
    pub fn from<T: std::convert::AsRef<std::ffi::OsStr>>(command: T, args: &[T]) -> OutputBag {
        OutputBag {
            output: Command::new(&command)
                .args(args)
                .output()
        }.into()
    }

    pub fn status(&self) -> ExitStatus {
        self.output.status
    }

    pub fn output(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.output.stdout)
    }

    pub fn errors(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.output.stderr)
    }
}

impl<'a> Into<Result<Cow<'a, str>, CommandError>> for OutputBag
{
    fn into(self) -> Result<Cow<'a, str>, CommandError> {
        if self.output.status().success {
           Ok(self.output())
        } else {
            Err(CommandError { status: self.output.status, message: self.errors()})
        }

    }
}

#[derive(Clone, Debug, Fail)]
pub struct RunError {
    status: ExitStatus,
    message: String
}

#[derive(Fail, Debug)]
#[fail(display = "command has encountered an error")]
pub struct CommandError(#[fail(cause)] RunError, #[fail(cause)] std::io::Error);


impl Display for RunError{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{}",  self.status, self.message)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
