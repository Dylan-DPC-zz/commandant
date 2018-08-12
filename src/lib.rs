#[macro_use]
pub mod macros;

use std::process::{ Command, Output, ExitStatus };
use std::ffi::{OsString, OsStr};
use std::io::Error;

#[derive(Clone, Debug)]
pub struct Commandant {
    command: OsString,
    args: Vec<OsString>
}

impl Commandant {
    pub fn new<T>(command: T, args: &[T]) -> Commandant
        where T: Into<OsString> + Clone
    {
        Commandant {
            command: command.into(),
            args: args.into_iter().map(|arg| arg.clone().into()).collect()
        }
    }

    pub fn run(&self) -> OutputBag {
        OutputBag::from(&self.command, &self.args)
    }
}

#[derive(Debug)]
pub struct OutputBag {
    output: Result<Output, Error>
}

impl OutputBag
{
    pub fn from<T: AsRef<OsStr>>(command: &T, args: &[T]) -> OutputBag {
        OutputBag {
            output: Command::new(command)
                .args(args)
                .output()
        }
    }

    pub fn status(self) -> Result<ExitStatus, Error> {
        Ok(self.output?.status)
    }

    pub fn output(self) -> Result<String, Error> {
        let stdout = self.output?.stdout;
        Ok(String::from_utf8_lossy(&stdout).into_owned())
    }

    pub fn errors(self) -> Result<String, Error> {
        let stderr = self.output?.stderr;
        Ok(String::from_utf8_lossy(&stderr).into_owned())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_commandant() {
        let commandant = super::Commandant::new("echo", &vec!["1"]).run();
        assert_eq!(commandant.output().unwrap(), "1\n");
    }
}
