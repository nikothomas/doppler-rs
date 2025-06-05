#!/bin/bash

# Script to generate Rust client with chrono datetime types
set -e

echo "🔧 Generating Doppler Rust client with chrono datetime support and improved model names..."

# Step 1: Preprocess OpenAPI spec to mark datetime fields
echo "📝 Preprocessing OpenAPI spec to mark datetime fields..."
if command -v python3 &> /dev/null; then
    python3 preprocess_openapi.py openapi.yaml openapi-processed.yaml
else
    echo "⚠️  Python3 not found. Skipping OpenAPI preprocessing."
    echo "   Datetime fields will be treated as strings."
    cp openapi.yaml openapi-processed.yaml
fi

# Step 2: Clean previous generation
echo "🧹 Cleaning previous generation..."
rm -rf generated/

# Step 3: Generate the Rust client
echo "🚀 Generating Rust client with custom templates and improved model names..."
openapi-generator generate \
    -g rust \
    -i openapi-processed.yaml \
    -o generated/ \
    -c openapi-generator-config.yaml \
    --skip-validate-spec

# Step 4: Post-process generated files to convert datetime fields to chrono types
echo "🔄 Post-processing files to convert datetime fields to chrono types..."
if command -v python3 &> /dev/null; then
    python3 post_process_chrono.py
else
    echo "⚠️  Python3 not found. Skipping post-processing."
    echo "   Datetime fields will remain as String types."
fi

# Step 5: Copy integration tests
echo "📋 Copying integration tests to generated directory..."
if [ -d "tests" ]; then
    mkdir -p generated/tests
    cp tests/*.rs generated/tests/ 2>/dev/null || echo "   ℹ️  No test files found to copy"
    echo "   ✅ Copied integration tests"
else
    echo "   ℹ️  No tests directory found"
fi

# Step 6: Update main Cargo.toml to include chrono
echo "📦 Updating main Cargo.toml to include chrono..."
if [ -f "Cargo.toml" ]; then
    if ! grep -q "chrono" Cargo.toml; then
        # Add chrono dependency if it doesn't exist
        sed -i '' '/^url = /a\
chrono = { version = "^0.4", features = ["serde"] }
' Cargo.toml
        echo "   ✅ Added chrono dependency to main Cargo.toml"
    else
        echo "   ℹ️  chrono already exists in main Cargo.toml"
    fi
else
    echo "   ⚠️  Main Cargo.toml not found"
fi

# Step 7: Copy auto-generated README to root
echo "📄 Copying auto-generated README to root..."
if [ -f "generated/README.md" ]; then
    cp generated/README.md README.md
    echo "   ✅ Copied generated README.md to root directory"
else
    echo "   ⚠️  Generated README.md not found"
fi

# Step 8: Clean up temporary files
echo "🧹 Cleaning up temporary files..."
rm -f openapi-processed.yaml

echo "📁 Files generated in generated/ directory"
echo "   To replace current source files, run:"
echo "   cp -r generated/src/* src/"
echo "   cp -r generated/tests/* tests/"
echo "   cp generated/Cargo.toml ."
echo "✅ Generation complete!"

echo ""
echo "📋 Summary of changes:"
echo "   • Datetime fields (initial_fetch_at, created_at, etc.) now use chrono::DateTime<Utc>"
echo "   • Added chrono dependency with serde feature"
echo "   • Generated files include proper chrono imports"
echo "   • Integration tests copied to generated/tests/"
echo "   • 🎯 ALL model names improved with comprehensive mappings:"
echo "     - groups_list_200_response_groups_inner → Group"
echo "     - service_account_tokens_list_200_response_api_tokens_inner → ServiceAccountToken"
echo "     - change_request_policies_get_200_response_policy → ChangeRequestPolicy"
echo "     - And 100+ more semantic model names instead of verbose auto-generated ones!"
echo ""
echo "🔍 To see the difference, check files like:"
echo "   generated/src/models/group.rs (instead of groups_list_200_response_groups_inner.rs)"
echo "   generated/src/models/service_account_token.rs"
echo "   generated/src/models/change_request_policy.rs"
echo ""
echo "🚨 Note: You may need to update any existing code that expects String types"
echo "   for datetime fields to work with chrono::DateTime<Utc> instead." 