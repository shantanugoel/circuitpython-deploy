# CircuitPython Deploy (cpd) - Project Specification

## Overview
A command-line tool written in Rust for deploying CircuitPython projects from a local development environment to CircuitPython boards. This tool addresses the limitation of developing directly on the board by enabling version-controlled development on the host system with seamless deployment.

## Requirements

### Functional Requirements

#### Core Functionality
- **F1**: Deploy files from local directory to CircuitPython board
- **F2**: Exclude `.git` directory and files by default
- **F3**: Support `.cpdignore` file for custom exclusions (follows `.gitignore` syntax)
- **F4**: Auto-detect CircuitPython board drives
- **F5**: Allow manual board drive specification via command-line parameter
- **F6**: Interactive board selection when multiple boards detected
- **F7**: Optional backup of existing board files before deployment
- **F8**: Preserve file timestamps during deployment
- **F9**: Display deployment progress and summary

#### Command-Line Interface
- **CLI1**: Primary command: `cpd` (deploy current directory)
- **CLI2**: Board specification: `cpd --board <drive>` or `cpd -b <drive>`
- **CLI3**: Backup option: `cpd --backup <path>` or `cpd -B <path>`
- **CLI4**: Dry-run mode: `cpd --dry-run` or `cpd -n`
- **CLI5**: Verbose output: `cpd --verbose` or `cpd -v`
- **CLI6**: Help and version: `cpd --help`, `cpd --version`

### Non-Functional Requirements
- **NF1**: Cross-platform support (Windows, macOS, Linux)
- **NF2**: Fast deployment for typical project sizes
- **NF3**: Graceful error handling with informative messages
- **NF4**: Minimal external dependencies
- **NF5**: Memory efficient for large projects

## Architecture

### High-Level Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CLI Parser    │    │  Board Detector │    │ File Operations │
│                 │    │                 │    │                 │
│ - Argument      │    │ - Drive         │    │ - Copy          │
│   parsing       │    │   enumeration   │    │ - Backup        │
│ - Validation    │    │ - Board         │    │ - Filtering     │
│                 │    │   identification│    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                │
                    ┌─────────────────┐
                    │ Deployment      │
                    │ Orchestrator    │
                    │                 │
                    │ - Coordinates   │
                    │   all operations│
                    │ - Progress      │
                    │   reporting     │
                    └─────────────────┘
```

### Module Structure

#### 1. CLI Module (`src/cli.rs`)
- Command-line argument parsing using `clap`
- Configuration validation
- Help and version display

#### 2. Board Detection Module (`src/board.rs`)
- Drive enumeration for each platform
- CircuitPython board identification
- Interactive board selection

#### 3. File Operations Module (`src/file_ops.rs`)
- File copying with progress tracking
- Backup functionality
- `.cpdignore` parsing and filtering

#### 4. Ignore Parser Module (`src/ignore.rs`)
- `.gitignore`-style pattern matching
- File and directory exclusion logic

#### 5. Error Handling Module (`src/error.rs`)
- Custom error types
- User-friendly error messages
- Error propagation

#### 6. Main Orchestrator (`src/main.rs`)
- Coordinates all modules
- Progress reporting
- Deployment workflow

## Design Decisions

### Board Detection Strategy
1. **Enumerate all removable drives** on the system
2. **Identify CircuitPython boards** by checking for:
   - `CIRCUITPY` volume label
   - Presence of `boot_out.txt` file
   - Presence of `code.py` or `main.py` files
3. **Handle multiple boards**:
   - If none found: Error with helpful message
   - If one found: Use automatically
   - If multiple found: Interactive selection

### File Filtering Strategy
1. **Default exclusions**: `.git/`, `.gitignore`, `target/`, `node_modules/`
2. **Custom exclusions**: Parse `.cpdignore` file if present
3. **Pattern matching**: Use glob-style patterns with `**` for recursive matching

### Backup Strategy
1. **Optional feature** activated with `--backup` flag
2. **Backup location**: User-specified directory
3. **Backup structure**: Mirror the board's file structure
4. **Timestamp preservation**: Maintain original file timestamps

### Error Handling Strategy
1. **Graceful degradation**: Continue deployment even if some files fail
2. **Detailed reporting**: Show which files succeeded/failed and why
3. **Atomic operations**: Where possible, use temporary files and rename

## Implementation Plan

### Phase 1: Core Infrastructure
- [ ] Set up Rust project with necessary dependencies
- [ ] Implement CLI argument parsing
- [ ] Create error handling framework
- [ ] Implement basic file operations

### Phase 2: Board Detection
- [ ] Implement drive enumeration for each platform
- [ ] Add CircuitPython board identification logic
- [ ] Create interactive board selection

### Phase 3: File Operations
- [ ] Implement file copying with progress
- [ ] Add `.cpdignore` parsing
- [ ] Create backup functionality

### Phase 4: Integration & Testing
- [ ] Integrate all components
- [ ] Add comprehensive error handling
- [ ] Test on multiple platforms and boards

### Phase 5: Polish & Documentation
- [ ] Optimize performance
- [ ] Add comprehensive documentation
- [ ] Create usage examples

## Dependencies

### Core Dependencies
- `clap` - Command-line argument parsing
- `sysinfo` - System information and drive enumeration
- `walkdir` - Directory traversal
- `ignore` - `.gitignore`-style pattern matching
- `indicatif` - Progress bars and spinners

### Platform-Specific Dependencies
- `winapi` (Windows) - Windows-specific drive operations
- `libc` (Unix) - Unix-specific file operations

## Testing Strategy
- Unit tests for each module
- Integration tests for end-to-end workflows
- Cross-platform testing on Windows, macOS, and Linux
- Testing with various CircuitPython boards and project structures

## Future Enhancements
- Watch mode for automatic deployment on file changes
- Configuration file support (`.cpdconfig`)
- Template support for new CircuitPython projects
- Integration with CircuitPython package managers
- Remote deployment over network connections
