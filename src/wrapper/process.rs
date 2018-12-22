use error::*;
use std::io::Read;
use std::process;

const ERROR_NAME: &str = "process";

pub(crate) struct Command {
    command: process::Command,
}

impl Command {
    pub(crate) fn new(program: &str, args: &[&str]) -> Self {
        let mut command = process::Command::new(program);
        for arg in args {
            command.arg(arg);
        }

        Self { command }
    }

    pub(crate) fn output(mut self) -> Result<String> {
        self.command
            .output()
            .map(|o| String::from(String::from_utf8_lossy(&o.stdout).trim()))
            .wrap_error(ERROR_NAME, "reading process output failed")
    }

    pub(crate) fn listen_stdout<D, S>(
        mut self,
        success_handler: S,
        default_handler: D,
    ) -> Result<()>
    where
        S: Fn() -> Result<()>,
        D: Fn(),
    {
        let mut monitor = self
            .command
            .stdout(process::Stdio::piped())
            .spawn()
            .wrap_error(ERROR_NAME, "failed to start process")?
            .stdout
            .wrap_error(ERROR_NAME, "failed to pipe process output")?;

        let mut buffer = [0; 1024];
        loop {
            if let Ok(bytes) = monitor.read(&mut buffer) {
                // reader has reached end-of-life -> thread gets killed
                if bytes == 0 {
                    break Ok(());
                }

                success_handler()?;
            }

            default_handler();
        }
    }
}
