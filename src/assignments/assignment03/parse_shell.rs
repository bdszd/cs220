//! Parsing a shell command.
//!
//! Shell commands are text-based instructions that you can enter in a command-line interface (CLI)
//! to interact with operating systems (e.g. Linux) and others. For example, you can use the `ls`
//! command to list files in a directory.
//!
//! You will parse a given string consists of a small number of shell commands.

/// Parse the string as a shell command.
///
/// Usually, a shell command is whitespace-separated array of strings.
///
/// ```text
/// cat file  -->  ["cat", "file"]
/// ```
///
/// But sometimes, you may want to include whitespaces in each argument.  In that case, you can use
/// quotes.
///
/// ```text
/// ls 'VirtualBox VMs'  -->  ["ls", 'VirtualBox VMs']
/// ls VirtualBox' 'VMs  -->  ["ls", 'VirtualBox VMs']
/// ```
///
/// For simplicity, you may assume that the string only contains alphanumeric characters, spaces
/// (" "), and single quotes ("'").
///
/// See `test_shell` for more examples.
pub fn parse_shell_command(command: &str) -> Vec<String> {
    let mut ret = Vec::new();
    let mut s = String::new();
    let mut in_quotes = false;

    for c in command.chars() {
        match c {
            ' ' if !in_quotes => {
                if !s.is_empty() {
                    ret.push(s);
                    s = String::new();
                }
            }
            '\'' => {
                in_quotes = !in_quotes;
            }
            _ => {
                s.push(c);
            }
        }
    }
    if !s.is_empty() {
        ret.push(s);
    }
    ret
}
