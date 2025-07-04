# CircuitPython Deploy (cpd) - Agent Commands

## Build Commands

### Development Build
```bash
cargo check          # Quick syntax/type check
cargo build           # Debug build
cargo test            # Run tests
```

### Release Build
```bash
cargo build --release    # Optimized build for distribution
```

### Binary Location
- Debug: `target/debug/cpd.exe` (Windows) / `target/debug/cpd` (Unix)
- Release: `target/release/cpd.exe` (Windows) / `target/release/cpd` (Unix)

## Testing Commands

### Basic Functionality Tests
```bash
# Show help
./target/release/cpd.exe --help

# List detected boards (no deployment)
./target/release/cpd.exe --list-boards

# Dry run (shows what would be deployed without copying)
./target/release/cpd.exe --dry-run

# Verbose dry run
./target/release/cpd.exe --dry-run --verbose

# Deploy with manual board specification
./target/release/cpd.exe --board F:\

# Deploy with backup
./target/release/cpd.exe --backup ./backup_dir
```

### Test Project Structure
Located in `test_project/`:
- `code.py` - Main CircuitPython file
- `lib/helper.py` - Helper library  
- `.cpdignore` - Ignore patterns
- `README.md` - Documentation

### Test Commands from test_project directory:
```bash
cd test_project
..\target\release\cpd.exe --dry-run
..\target\release\cpd.exe --verbose --dry-run
..\target\release\cpd.exe --list-boards
```

## Code Structure

### Main Modules
- `src/main.rs` - Entry point and orchestration
- `src/cli.rs` - Command-line argument parsing  
- `src/error.rs` - Error types and handling
- `src/board.rs` - CircuitPython board detection
- `src/file_ops.rs` - File copying and backup operations
- `src/ignore.rs` - .cpdignore/.gitignore pattern matching

### Key Dependencies
- `clap` - CLI argument parsing
- `sysinfo` - System/disk information
- `walkdir` - Directory traversal
- `ignore` - Pattern matching for ignore files
- `indicatif` - Progress bars
- `winapi` (Windows) - Volume label detection
- `filetime` - Timestamp preservation

## Development Phases

### ✅ Phase 1: Core Infrastructure (Completed)
- Project setup with dependencies
- CLI argument parsing
- Error handling framework  
- Basic file operations
- Module structure

### ✅ Phase 2: Board Detection (Completed)
- Multi-platform disk enumeration
- CircuitPython board identification
- Volume label detection (Windows/Unix)
- Interactive board selection
- Enhanced board detection logic

### ✅ Phase 3: File Operations (Completed)
- .cpdignore pattern implementation with full gitignore syntax support
- Progress tracking during copy with indicatif progress bars
- Backup functionality with automatic directory structure preservation
- File filtering and exclusion with comprehensive pattern matching
- End-to-end deployment testing with mock boards

### ✅ Phase 4: Integration & Testing (Completed)
- End-to-end deployment testing with real CircuitPython boards
- Comprehensive integration test suite with 7 test scenarios
- Enhanced error messages with helpful troubleshooting guidance
- Performance testing with large projects (24 files in 68ms)
- Cross-platform validation and compatibility testing
- Interactive board selection testing

### ✅ Phase 5: Polish & Documentation (Completed)
- Comprehensive README.md with installation and usage instructions
- Example CircuitPython projects (LED blink, sensor dashboard)
- Optimized CLI help text with examples and feature highlights
- Complete CHANGELOG.md documenting all features and development
- Release preparation scripts and documentation
- MIT license and proper Cargo.toml metadata

### ✅ Bonus: GitHub Actions & CI/CD (Completed)
- Complete CI/CD pipeline with multi-platform testing
- Automated release workflow with cross-platform binaries
- Dependency update automation with security audits
- Code quality checks (formatting, linting, security)
- Issue templates and PR templates for contributions
- Workflow validation scripts for local testing

## Project Status

**Current Status**: Phase 5 Complete - Production-ready deployment tool with comprehensive documentation

**Key Features Working**:
- CLI argument parsing with clap
- Cross-platform board detection with enhanced CircuitPython identification
- Volume label reading (Windows/Unix)
- .cpdignore file support with gitignore-style patterns
- Progress tracking with visual progress bars
- Backup functionality with directory structure preservation
- Dry-run mode for safe testing
- Verbose output for debugging
- Board listing functionality
- End-to-end deployment with comprehensive error handling
- Interactive board selection for multiple boards
- Enhanced user experience with helpful error messages
- High performance (24 files deployed in 68ms)

**Status**: Complete! Ready for release and distribution to the CircuitPython community.
