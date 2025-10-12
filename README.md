# mo2-mode

A Rust library for building properly escaped command strings to run programs through Mod Organizer 2's `ModOrganizer.exe run` command.

## Overview

Mod Organizer 2 (MO2) is a mod manager primarily used for Bethesda games. When running programs through MO2, they inherit the virtual filesystem that MO2 creates, allowing tools to see installed mods as if they were actually in the game directory.

The command format is:
```
"C:\Path\To\ModOrganizer.exe" run "path\to\program.exe" -a "arguments for the program"
```

This library handles the tricky nested quoting that occurs when arguments themselves contain quoted values.

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
mo2-mode = "0.1.0"
```

### Basic Example - Building a Command String

```rust
use mo2_mode::MO2Command;

let cmd = MO2Command::new(
    r"C:\Modding\MO2\ModOrganizer.exe",
    r"d:\programs\xedit\xedit64.exe"
)
.arg("-sse")
.arg("-autoexit")
.build();

// Result: "C:\Modding\MO2\ModOrganizer.exe" run "d:\programs\xedit\xedit64.exe" -a "-sse -autoexit"
```

### Executing a Command

The recommended way is to use the `execute()` method which returns a `std::process::Command`:

```rust
use mo2_mode::MO2Command;

let mut cmd = MO2Command::new(
    r"C:\Modding\MO2\ModOrganizer.exe",
    r"d:\programs\xedit\xedit64.exe"
)
.arg("-sse")
.arg("-autoexit")
.execute();

// Run and wait for completion
let status = cmd.status().expect("Failed to run MO2");
println!("Process exited with: {}", status);

// Or spawn without waiting
let child = cmd.spawn().expect("Failed to spawn MO2");
println!("Started MO2 with PID: {}", child.id());
```

### Running xEdit for Plugin Cleaning

```rust
use mo2_mode::MO2Command;

let plugin_name = "MyPlugin.esp";
let mut cmd = MO2Command::new(
    r"C:\Modding\MO2\ModOrganizer.exe",
    r"C:\Tools\SSEEdit\SSEEdit64.exe"
)
.arg("-sse")
.arg("-qac")  // Quick Auto Clean
.arg("-autoexit")
.arg("-autoload")
.arg(format!(r#""{}""#, plugin_name))  // Plugin name needs to be quoted
.execute();

// Run and wait for it to finish
let status = cmd.status().expect("Failed to clean plugin");
if status.success() {
    println!("Plugin {} cleaned successfully", plugin_name);
} else {
    eprintln!("Cleaning failed with status: {}", status);
}
```

### Capturing Output

If you need to capture stdout/stderr from the executed program:

```rust
use mo2_mode::MO2Command;
use std::process::Stdio;

let output = MO2Command::new(
    r"C:\Modding\MO2\ModOrganizer.exe",
    r"C:\Tools\xEdit\xEdit64.exe"
)
.arg("-sse")
.arg("-dumpinfo")
.execute()
.stdout(Stdio::piped())
.stderr(Stdio::piped())
.output()
.expect("Failed to execute");

if output.status.success() {
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Output: {}", stdout);
} else {
    let stderr = String::from_utf8_lossy(&output.stderr);
    eprintln!("Error: {}", stderr);
}
```

### Adding Multiple Arguments at Once

```rust
use mo2_mode::MO2Command;

let cmd = MO2Command::new(
    r"C:\Modding\MO2\ModOrganizer.exe",
    r"C:\Tools\xEdit\xEdit64.exe"
)
.args(["-sse", "-autoexit", "-autoload"])
.arg(r#""Skyrim.esm""#)
.build();
```

### Mimicking the Python Implementation

This is equivalent to the Python code from your example:

**Python:**
```python
mo2_path = r"C:\Modding\MO2\ModOrganizer.exe"
xedit_path = r"d:\programs\xedit\xedit64.exe"
plugin_name = "MyPlugin.esp"
game_type = "sse"
cleaning_flag = "-qac"

args = f'{cleaning_flag} -autoexit -autoload "{plugin_name}"'
command = f'"{mo2_path}" run "{xedit_path}" -a "{args}"'
```

**Rust:**
```rust
use mo2_mode::MO2Command;

let mo2_path = r"C:\Modding\MO2\ModOrganizer.exe";
let xedit_path = r"d:\programs\xedit\xedit64.exe";
let plugin_name = "MyPlugin.esp";
let game_type = "sse";
let cleaning_flag = "-qac";

let command = MO2Command::new(mo2_path, xedit_path)
    .arg(format!("-{}", game_type))
    .arg(cleaning_flag)
    .arg("-autoexit")
    .arg("-autoload")
    .arg(format!(r#""{}""#, plugin_name))
    .build();
```

### Running Without Arguments

If you just need to run a program through MO2 without any arguments:

```rust
use mo2_mode::MO2Command;

let cmd = MO2Command::new(
    r"C:\Modding\MO2\ModOrganizer.exe",
    r"C:\Program Files\Notepad++\notepad++.exe"
).build();

// Result: "C:\Modding\MO2\ModOrganizer.exe" run "C:\Program Files\Notepad++\notepad++.exe"
```

## How Escaping Works

The library handles the nested quoting problem:

1. **Outer level**: The MO2 paths and program path are quoted
2. **-a argument**: The entire argument string is quoted
3. **Inner quotes**: Any quotes within the argument string are escaped with backslashes

Example:
```rust
.arg(r#""MyPlugin.esp""#)  // You provide: "MyPlugin.esp"
                           // Library escapes to: \"MyPlugin.esp\"
                           // Final in command: -a "... \"MyPlugin.esp\""
```

## API Reference

### `MO2Command`

Main builder struct for constructing MO2 commands.

#### Methods

- `new(mo2_path, program_path)` - Create a new command builder
- `arg(arg)` - Add a single argument
- `args(args)` - Add multiple arguments from an iterator
- `build()` - Build the final command string

## Testing

Run the test suite:

```bash
cargo test
```

## License

GPL-3.0 License. See `LICENSE.md` file for details.

## Contributing

Contributions welcome! The main areas for improvement:

- Support for different shells (PowerShell, bash on Windows)
- Additional helper methods for common xEdit operations
- More comprehensive escaping tests
