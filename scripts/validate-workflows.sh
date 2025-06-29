#!/bin/bash
# Validate GitHub workflows locally

set -e

echo "🔍 Validating GitHub workflows..."

# Check if we're in the right directory
if [ ! -d ".github/workflows" ]; then
    echo "❌ Error: .github/workflows directory not found"
    echo "Run this script from the project root directory"
    exit 1
fi

# Check workflow files exist
WORKFLOWS=(".github/workflows/ci.yml" ".github/workflows/release.yml" ".github/workflows/dependencies.yml")

for workflow in "${WORKFLOWS[@]}"; do
    if [ ! -f "$workflow" ]; then
        echo "❌ Error: Workflow file not found: $workflow"
        exit 1
    else
        echo "✅ Found: $workflow"
    fi
done

# Basic YAML syntax validation (if yamllint is available)
if command -v yamllint >/dev/null 2>&1; then
    echo "🔍 Checking YAML syntax..."
    for workflow in "${WORKFLOWS[@]}"; do
        if yamllint "$workflow"; then
            echo "✅ YAML syntax valid: $workflow"
        else
            echo "❌ YAML syntax error in: $workflow"
            exit 1
        fi
    done
else
    echo "⚠️  yamllint not found, skipping YAML syntax validation"
    echo "   Install with: pip install yamllint"
fi

# Validate that key sections exist in workflows
echo "🔍 Checking workflow structure..."

# Check CI workflow
if grep -q "on:" ".github/workflows/ci.yml" && \
   grep -q "jobs:" ".github/workflows/ci.yml" && \
   grep -q "cargo test" ".github/workflows/ci.yml"; then
    echo "✅ CI workflow structure looks good"
else
    echo "❌ CI workflow missing required sections"
    exit 1
fi

# Check release workflow
if grep -q "tags:" ".github/workflows/release.yml" && \
   grep -q "create-release" ".github/workflows/release.yml" && \
   grep -q "build-release" ".github/workflows/release.yml"; then
    echo "✅ Release workflow structure looks good"
else
    echo "❌ Release workflow missing required sections"
    exit 1
fi

# Check if example trigger conditions work
echo "🔍 Checking trigger conditions..."

# CI should trigger on main branch pushes
if grep -A 5 "on:" ".github/workflows/ci.yml" | grep -q "main"; then
    echo "✅ CI triggers on main branch"
else
    echo "❌ CI doesn't trigger on main branch"
    exit 1
fi

# Release should trigger on tags
if grep -A 5 "on:" ".github/workflows/release.yml" | grep -q "v\*"; then
    echo "✅ Release triggers on version tags"
else
    echo "❌ Release doesn't trigger on version tags"
    exit 1
fi

echo ""
echo "🎉 All workflow validations passed!"
echo ""
echo "💡 Next steps:"
echo "   1. Commit and push workflows to GitHub"
echo "   2. Check workflow runs in GitHub Actions tab"
echo "   3. Create a test tag to verify release workflow"
echo ""
echo "📚 Useful commands:"
echo "   git add .github/"
echo "   git commit -m 'add GitHub workflows'"
echo "   git push origin main"
echo "   git tag v0.1.0 && git push origin v0.1.0"
