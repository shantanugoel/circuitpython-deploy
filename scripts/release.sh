#!/bin/bash
# Release preparation script for CircuitPython Deploy

set -e

echo "🚀 Preparing CircuitPython Deploy for release..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Must be run from project root directory"
    exit 1
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Run tests
echo "🧪 Running tests..."
cargo test
cargo test --test integration_test

# Build release binary
echo "🔨 Building release binary..."
cargo build --release

# Check binary
if [ ! -f "target/release/cpd" ] && [ ! -f "target/release/cpd.exe" ]; then
    echo "❌ Error: Release binary not found"
    exit 1
fi

# Run basic functionality tests
echo "✅ Testing basic functionality..."

# Test help output
echo "Testing help output..."
if command -v ./target/release/cpd >/dev/null 2>&1; then
    BINARY="./target/release/cpd"
elif command -v ./target/release/cpd.exe >/dev/null 2>&1; then
    BINARY="./target/release/cpd.exe"
else
    echo "❌ Error: Could not find release binary"
    exit 1
fi

$BINARY --help > /dev/null
$BINARY --version > /dev/null

# Test board listing (should not fail even if no boards)
$BINARY --list-boards > /dev/null || true

echo "✅ All tests passed!"

# Create release directory
RELEASE_DIR="release"
mkdir -p $RELEASE_DIR

# Copy binary
if [ -f "target/release/cpd.exe" ]; then
    cp target/release/cpd.exe $RELEASE_DIR/
    BINARY_NAME="cpd.exe"
else
    cp target/release/cpd $RELEASE_DIR/
    BINARY_NAME="cpd"
fi

# Copy documentation
cp README.md $RELEASE_DIR/
cp LICENSE $RELEASE_DIR/
cp CHANGELOG.md $RELEASE_DIR/

# Copy examples
cp -r examples $RELEASE_DIR/

# Create archive
VERSION=$(grep "^version" Cargo.toml | cut -d'"' -f2)
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

ARCHIVE_NAME="circuitpython-deploy-v${VERSION}-${PLATFORM}-${ARCH}"

if command -v tar >/dev/null 2>&1; then
    tar -czf "${ARCHIVE_NAME}.tar.gz" -C $RELEASE_DIR .
    echo "📦 Created release archive: ${ARCHIVE_NAME}.tar.gz"
fi

if command -v zip >/dev/null 2>&1; then
    (cd $RELEASE_DIR && zip -r "../${ARCHIVE_NAME}.zip" .)
    echo "📦 Created release archive: ${ARCHIVE_NAME}.zip"
fi

echo ""
echo "🎉 Release preparation complete!"
echo ""
echo "📁 Release contents:"
ls -la $RELEASE_DIR/
echo ""
echo "Binary size: $(du -h $RELEASE_DIR/$BINARY_NAME | cut -f1)"
echo "Version: $VERSION"
echo ""
echo "Next steps:"
echo "1. Test the binary on different platforms"
echo "2. Create GitHub release with the archive"
echo "3. Update package managers"
echo "4. Announce the release"
