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
