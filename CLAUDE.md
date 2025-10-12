# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# Project Context: mo2-mode

## Project Purpose

This is a Rust library that builds command strings for running programs through Mod Organizer 2 (MO2). MO2 is a mod manager for Bethesda games that creates a virtual filesystem, allowing tools to see installed mods without physically modifying the game directory.

## Core Problem

Running programs through MO2 requires a specific command format:
```
"C:\Path\To\ModOrganizer.exe" run "path\to\program.exe" -a "arguments"
```

The challenge is **nested quoting**: the `-a` parameter takes a quoted string that may itself contain quoted values. For example:
```
-a "-sse -autoexit -autoload \"MyPlugin.esp\""
```

## Implementation Details

### Main API: `MO2Command`

A builder pattern that:
1. Takes MO2 path and target program path
2. Collects arguments via `.arg()` or `.args()`
3. Builds the final command string with proper escaping

### Escaping Strategy

- **Paths**: Wrapped in double quotes
- **Inner quotes**: Escaped with backslashes for the `-a` argument
- **Windows-specific**: Follows Windows command-line quoting rules

### Code Location

- `src/lib.rs`: Main implementation with `MO2Command` struct
- Tests included in the same file
- `README.md`: User-facing documentation with examples

## Migration from Python

This project replicates functionality from a Python implementation that used:
```python
command = f'"{mo2_path}" run "{xedit_path}" -a "{args}"'
```

The Python version required `shell=True` in subprocess calls. The Rust version produces the properly escaped string that can be used directly.

## Common Use Cases

### xEdit Plugin Cleaning
Running SSEEdit/FO4Edit/TES5Edit to clean plugin files:
```rust
MO2Command::new(mo2_path, xedit_path)
    .arg("-sse")
    .arg("-qac")
    .arg("-autoexit")
    .arg("-autoload")
    .arg(format!(r#""{}""#, plugin_name))
    .build()
    .execute();
```

### Generic Tool Execution
Running any tool through MO2 with various flags and arguments.

## Testing Philosophy

- Test basic commands without arguments
- Test simple flags
- Test quoted plugin names (the tricky case)
- Test complex nested quoting scenarios
- Verify escaping matches expected output

## Future Enhancements

Potential improvements:
- Helper methods for common xEdit operations (clean, patch, etc.)
- Support for other shells (PowerShell syntax)
- Validation of paths before building

## Development Commands

### Building and Testing
```bash
# Build the library
cargo build

# Build with release optimizations
cargo build --release

# Run all tests
cargo test

# Run a specific test
cargo test test_command_with_quoted_plugin_name

# Run tests with output
cargo test -- --nocapture

# Run doc tests only
cargo test --doc

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy
```

### Documentation
```bash
# Build and open documentation locally
cargo doc --open

# Build documentation including private items
cargo doc --document-private-items
```

## Architecture

### Single-Module Design
All code resides in `src/lib.rs` - this is intentional for a simple library. The module contains:
- Public `MO2Command` struct (lines 25-118)
- Private helper functions `quote_path()` and `escape_for_mo2_args()` (lines 120-134)
- Comprehensive test module (lines 136-279)

### Builder Pattern Flow
1. `new()` initializes with two paths and an empty argument vector
2. `.arg()` or `.args()` accumulates arguments (builder pattern returns `Self`)
3. `.build()` produces a command string OR `.execute()` produces `std::process::Command`

### Escaping Logic Critical Points
The `escape_for_mo2_args()` function (line 129) handles nested quoting by replacing `"` with `\"`. This is the core of solving the nested quoting problem. The `.execute()` method bypasses string escaping by directly constructing `Command` arguments, which is safer and recommended.

## Development Notes

- **Windows-focused**: This is specifically for Windows command-line syntax
- **Library, not executable**: Produces strings for external execution
- **No external dependencies**: Uses only std library
- **Rust 2024 edition**: Modern Rust idioms

## Common Patterns

When adding features:
1. Add the API to `MO2Command`
2. Add corresponding test cases
3. Update README with examples
4. Run `cargo test` to verify

## Known Limitations

- Assumes Windows command-line escaping rules
- No runtime validation of paths
- Executes commands via execute method which uses `std::process::Command`
- Limited to the `run` command (MO2 has other commands too)
