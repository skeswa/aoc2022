use anyhow::{anyhow, Result};

/// One call-and-response within a terminal shell session.
#[derive(Debug)]
pub(crate) struct ShellExchange {
    /// Line of text interpreted by the shell.
    pub(crate) input: String,
    /// Lines of text printed by the shell.
    pub(crate) output: Vec<String>,
}

impl ShellExchange {
    /// Interprets `encoded_shell_exchange` as a [ShellExchange], returning an [Err] if
    /// that is impossible.
    pub(crate) fn parse(encoded_shell_exchange: &str) -> Result<ShellExchange> {
        let mut lines = encoded_shell_exchange
            .lines()
            .map(|line| line.trim().to_owned())
            .collect::<Vec<String>>();

        if lines.is_empty() {
            return Err(anyhow!(
                "\"{}\" is not a valid shell exchange: not enough lines",
                encoded_shell_exchange
            ));
        }

        let first_line = lines.remove(0);

        Ok(ShellExchange {
            input: first_line,
            output: lines,
        })
    }
}
