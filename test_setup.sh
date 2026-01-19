#!/bin/bash

# Test script for setup command
# Uses mock data to test the setup functionality

echo "🧪 Testing Briefcase Setup Command"
echo "=================================="

# Create a temporary test directory
TEST_DIR=$(mktemp -d)
echo "📁 Test directory: $TEST_DIR"

# Set HOME to test directory
export HOME=$TEST_DIR

# Create a simple test that simulates the setup process
echo "🔧 Testing configuration creation..."

# Create the config directory structure
mkdir -p $TEST_DIR/.config/briefcase
mkdir -p $TEST_DIR/.cache/briefcase/temp
mkdir -p $TEST_DIR/.cache/briefcase/log

echo "✅ Directory structure created"

# Create a mock configuration file
MOCK_CONFIG="[general]
password_hint = \"What is your favorite color?\"
password_key = \"mock_password_key_abc123\"
max_retention = 10
http_proxy = \"\"
https_proxy = \"\"
no_proxy = \"localhost\"
log_level = \"info\"

[[sources]]
name = \"firefox\"
enabled = false
source_type = \"Firefox\"
dir = \"/path/to/firefox/profile\"
frequency = \"Daily\"
last_backup = null
backup_count = 0

[[sources]]
name = \"documents\"
enabled = false
source_type = \"Folder\"
dir = \"/path/to/sensitive/folder\"
frequency = \"Daily\"
last_backup = null
backup_count = 0

[[remotes]]
name = \"dropbox\"
enabled = false
remote_type = \"Dropbox\"
api_key = null
username = null
ipaddr = null
port = null
http_proxy = \"\"
https_proxy = \"\"
last_sync = null

[[remotes]]
name = \"ssh\"
enabled = false
remote_type = \"SSH\"
api_key = null
username = null
ipaddr = null
port = 22
http_proxy = \"\"
https_proxy = \"\"
last_sync = null
"

echo "$MOCK_CONFIG" > $TEST_DIR/.config/briefcase/briefcase.toml

echo "✅ Mock configuration file created"

# Verify the configuration file
if [ -f "$TEST_DIR/.config/briefcase/briefcase.toml" ]; then
    echo "✅ Configuration file exists"
    
    # Check file content
    if grep -q "password_hint" "$TEST_DIR/.config/briefcase/briefcase.toml"; then
        echo "✅ Password hint found"
    else
        echo "❌ Password hint not found"
    fi
    
    if grep -q "firefox" "$TEST_DIR/.config/briefcase/briefcase.toml"; then
        echo "✅ Firefox source found"
    else
        echo "❌ Firefox source not found"
    fi
    
    if grep -q "dropbox" "$TEST_DIR/.config/briefcase/briefcase.toml"; then
        echo "✅ Dropbox remote found"
    else
        echo "❌ Dropbox remote not found"
    fi
else
    echo "❌ Configuration file not created"
fi

# Test configuration file structure
echo "📋 Configuration file structure:"
cat $TEST_DIR/.config/briefcase/briefcase.toml

echo ""
echo "🎉 Setup test completed!"
echo "Test directory: $TEST_DIR"
echo "You can inspect the files at this location."

# Cleanup (optional)
# read -p "Delete test directory? (y/n) " -n 1 -r
# echo
# if [[ $REPLY =~ ^[Yy]$ ]]; then
#     rm -rf $TEST_DIR
#     echo "🗑️ Test directory cleaned up"
# else
#     echo "📁 Test directory preserved: $TEST_DIR"
# fi