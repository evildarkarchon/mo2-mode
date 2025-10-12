use std::path::{Path, PathBuf};
use std::process::Command;

/// Builder for constructing Mod Organizer 2 command strings.
///
/// This builder helps create properly escaped command strings for running
/// programs through MO2's `ModOrganizer.exe run` command.
///
/// # Example
///
/// ```
/// use mo2_mode::MO2Command;
///
/// let cmd = MO2Command::new(
///     r"C:\Modding\MO2\ModOrganizer.exe",
///     r"d:\programs\xedit\xedit64.exe"
/// )
/// .arg("-sse")
/// .arg("-autoexit")
/// .arg("-autoload")
/// .arg(r#""-MyPlugin.esp""#)
/// .build();
/// ```
#[derive(Debug, Clone)]
pub struct MO2Command {
    mo2_path: PathBuf,
    program_path: PathBuf,
    arguments: Vec<String>,
}

impl MO2Command {
    /// Creates a new MO2 command builder.
    ///
    /// # Arguments
    ///
    /// * `mo2_path` - Path to ModOrganizer.exe
    /// * `program_path` - Path to the program to run through MO2
    pub fn new(mo2_path: impl AsRef<Path>, program_path: impl AsRef<Path>) -> Self {
        Self {
            mo2_path: mo2_path.as_ref().to_path_buf(),
            program_path: program_path.as_ref().to_path_buf(),
            arguments: Vec::new(),
        }
    }

    /// Adds a single argument to be passed to the program.
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.arguments.push(arg.into());
        self
    }

    /// Adds multiple arguments to be passed to the program.
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.arguments.extend(args.into_iter().map(|s| s.into()));
        self
    }

    /// Builds the final command string for MO2.
    ///
    /// The command will be in the format:
    /// `"<mo2_path>" run "<program_path>" -a "<arguments>"`
    ///
    /// Arguments are properly escaped for Windows command line.
    pub fn build(&self) -> String {
        let mo2_path = quote_path(&self.mo2_path);
        let program_path = quote_path(&self.program_path);

        if self.arguments.is_empty() {
            format!(r#"{} run {}"#, mo2_path, program_path)
        } else {
            let args_string = self.arguments.join(" ");
            // Escape quotes in the arguments string for the -a parameter
            let escaped_args = escape_for_mo2_args(&args_string);
            format!(r#"{} run {} -a "{}""#, mo2_path, program_path, escaped_args)
        }
    }

    /// Creates a `std::process::Command` ready to execute.
    ///
    /// This is the recommended way to run the MO2 command as it properly
    /// constructs the Command without needing to parse the string.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mo2_mode::MO2Command;
    ///
    /// let mut cmd = MO2Command::new(
    ///     r"C:\Modding\MO2\ModOrganizer.exe",
    ///     r"d:\programs\xedit\xedit64.exe"
    /// )
    /// .arg("-sse")
    /// .arg("-autoexit")
    /// .execute();
    ///
    /// // Run and wait for completion
    /// let status = cmd.status().expect("Failed to run MO2");
    /// println!("Exit status: {}", status);
    /// ```
    pub fn execute(&self) -> Command {
        let mut cmd = Command::new(&self.mo2_path);
        cmd.arg("run");
        cmd.arg(&self.program_path);

        if !self.arguments.is_empty() {
            cmd.arg("-a");
            // Join and escape the arguments for the -a parameter
            let args_string = self.arguments.join(" ");
            cmd.arg(args_string);
        }

        cmd
    }
}

/// Wraps a path in quotes for Windows command line.
fn quote_path(path: &Path) -> String {
    format!(r#""{}""#, path.display())
}

/// Escapes a string for use within MO2's -a argument.
///
/// This handles the nested quoting scenario where the -a argument itself
/// is quoted, and may contain quotes within it.
fn escape_for_mo2_args(args: &str) -> String {
    // In Windows cmd.exe, within a quoted string, quotes need to be escaped with backslash
    // However, MO2 may have its own parsing rules. Based on the Python example,
    // it appears that quotes within the -a string need backslash escaping.
    args.replace('"', r#"\""#)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_command_without_args() {
        let cmd = MO2Command::new(
            r"C:\Modding\MO2\ModOrganizer.exe",
            r"d:\programs\xedit\xedit64.exe"
        ).build();

        assert_eq!(
            cmd,
            r#""C:\Modding\MO2\ModOrganizer.exe" run "d:\programs\xedit\xedit64.exe""#
        );
    }

    #[test]
    fn test_command_with_simple_args() {
        let cmd = MO2Command::new(
            r"C:\Modding\MO2\ModOrganizer.exe",
            r"d:\programs\xedit\xedit64.exe"
        )
        .arg("-sse")
        .arg("-autoexit")
        .build();

        assert_eq!(
            cmd,
            r#""C:\Modding\MO2\ModOrganizer.exe" run "d:\programs\xedit\xedit64.exe" -a "-sse -autoexit""#
        );
    }

    #[test]
    fn test_command_with_quoted_plugin_name() {
        let cmd = MO2Command::new(
            r"C:\Modding\MO2\ModOrganizer.exe",
            r"d:\programs\xedit\xedit64.exe"
        )
        .arg("-sse")
        .arg("-autoexit")
        .arg("-autoload")
        .arg(r#""MyPlugin.esp""#)
        .build();

        assert_eq!(
            cmd,
            r#""C:\Modding\MO2\ModOrganizer.exe" run "d:\programs\xedit\xedit64.exe" -a "-sse -autoexit -autoload \"MyPlugin.esp\"""#
        );
    }

    #[test]
    fn test_command_with_multiple_args_at_once() {
        let cmd = MO2Command::new(
            r"C:\Modding\MO2\ModOrganizer.exe",
            r"d:\programs\xedit\xedit64.exe"
        )
        .args(["-sse", "-autoexit", "-autoload"])
        .build();

        assert_eq!(
            cmd,
            r#""C:\Modding\MO2\ModOrganizer.exe" run "d:\programs\xedit\xedit64.exe" -a "-sse -autoexit -autoload""#
        );
    }

    #[test]
    fn test_xedit_cleaning_command() {
        // Simulates the Python example for cleaning a plugin
        let plugin_name = "MyPlugin.esp";
        let args = format!(r#"-qac -autoexit -autoload "{}""#, plugin_name);

        let cmd = MO2Command::new(
            r"C:\Modding\MO2\ModOrganizer.exe",
            r"d:\programs\xedit\SSEEdit64.exe"
        )
        .arg(args)
        .build();

        assert_eq!(
            cmd,
            r#""C:\Modding\MO2\ModOrganizer.exe" run "d:\programs\xedit\SSEEdit64.exe" -a "-qac -autoexit -autoload \"MyPlugin.esp\"""#
        );
    }

    #[test]
    fn test_escaping_complex_quotes() {
        let cmd = MO2Command::new(
            r"C:\MO2\ModOrganizer.exe",
            r"C:\tools\program.exe"
        )
        .arg(r#"-flag "value with spaces""#)
        .arg(r#""another quoted value""#)
        .build();

        assert_eq!(
            cmd,
            r#""C:\MO2\ModOrganizer.exe" run "C:\tools\program.exe" -a "-flag \"value with spaces\" \"another quoted value\"""#
        );
    }

    #[test]
    fn test_execute_creates_proper_command() {
        use std::ffi::OsStr;

        let mo2_cmd = MO2Command::new(
            r"C:\Modding\MO2\ModOrganizer.exe",
            r"d:\programs\xedit\xedit64.exe"
        )
        .arg("-sse")
        .arg("-autoexit")
        .arg(r#""MyPlugin.esp""#);

        let cmd = mo2_cmd.execute();

        // Verify the program is correct
        assert_eq!(cmd.get_program(), OsStr::new(r"C:\Modding\MO2\ModOrganizer.exe"));

        // Verify the arguments
        let args: Vec<&OsStr> = cmd.get_args().collect();
        assert_eq!(args.len(), 4);
        assert_eq!(args[0], OsStr::new("run"));
        assert_eq!(args[1], OsStr::new(r"d:\programs\xedit\xedit64.exe"));
        assert_eq!(args[2], OsStr::new("-a"));
        assert_eq!(args[3], OsStr::new(r#"-sse -autoexit "MyPlugin.esp""#));
    }

    #[test]
    fn test_execute_without_args() {
        use std::ffi::OsStr;

        let mo2_cmd = MO2Command::new(
            r"C:\Modding\MO2\ModOrganizer.exe",
            r"C:\tools\notepad++.exe"
        );

        let cmd = mo2_cmd.execute();

        let args: Vec<&OsStr> = cmd.get_args().collect();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], OsStr::new("run"));
        assert_eq!(args[1], OsStr::new(r"C:\tools\notepad++.exe"));
    }
}
