#!/usr/bin/env python3
"""
Post-process generated Rust files to convert datetime string fields to chrono types.
"""

import os
import re
from pathlib import Path

# Known datetime field names that should use chrono
DATETIME_FIELDS = {
    'initial_fetch_at', 'created_at', 'updated_at', 'last_fetch_at', 
    'last_seen_at', 'expires_at', 'expire_at', 'lastSyncedAt'
}

def process_rust_file(file_path: Path):
    """Process a single Rust file to convert datetime fields."""
    print(f"Processing {file_path}")
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    changes_made = False
    
    # Convert datetime fields from Option<String> to Option<chrono::DateTime<chrono::Utc>>
    for field_name in DATETIME_FIELDS:
        # Pattern to match field definitions like:
        # pub field_name: Option<String>,
        pattern = rf'(pub\s+{re.escape(field_name)}\s*:\s*Option<)String(\s*>)'
        replacement = r'\1chrono::DateTime<chrono::Utc>\2'
        
        new_content = re.sub(pattern, replacement, content)
        if new_content != content:
            print(f"  ✅ Updated field: {field_name}")
            content = new_content
            changes_made = True
    
    # Ensure chrono is imported if we made changes
    if changes_made:
        # Check if any chrono import already exists
        has_chrono_import = bool(re.search(r'use chrono::', content))
        
        if not has_chrono_import:
            # Find the import section and add chrono
            import_pattern = r'(use crate::models;\nuse serde::\{[^}]+\};)'
            if re.search(import_pattern, content):
                content = re.sub(
                    import_pattern, 
                    r'\1\nuse chrono::{DateTime, Utc};', 
                    content, 
                    count=1
                )
                print(f"  ✅ Added chrono import")
            else:
                # Fallback: add after the last import line
                lines = content.split('\n')
                last_import_idx = -1
                
                for i, line in enumerate(lines):
                    if line.startswith('use '):
                        last_import_idx = i
                
                if last_import_idx >= 0:
                    lines.insert(last_import_idx + 1, 'use chrono::{DateTime, Utc};')
                    content = '\n'.join(lines)
                    print(f"  ✅ Added chrono import (fallback)")
        else:
            print(f"  ℹ️  Chrono import already exists")
    
    # Write back if changes were made
    if changes_made:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"  💾 Saved changes to {file_path}")
    else:
        print(f"  ⏭️  No datetime fields found in {file_path}")

def main():
    """Process all Rust model files in the generated directory."""
    generated_dir = Path('generated')
    models_dir = generated_dir / 'src' / 'models'
    
    if not models_dir.exists():
        print(f"❌ Models directory not found: {models_dir}")
        return
    
    print("🔧 Post-processing generated Rust files to use chrono for datetime fields...")
    
    # Find all .rs files in the models directory
    rust_files = list(models_dir.glob('*.rs'))
    
    if not rust_files:
        print(f"❌ No Rust files found in {models_dir}")
        return
    
    processed_count = 0
    changed_count = 0
    
    for rust_file in rust_files:
        if rust_file.name == 'mod.rs':
            continue  # Skip module files
            
        original_size = rust_file.stat().st_size
        process_rust_file(rust_file)
        new_size = rust_file.stat().st_size
        
        processed_count += 1
        if new_size != original_size:
            changed_count += 1
    
    print(f"\n✅ Post-processing complete!")
    print(f"   📁 Processed: {processed_count} files")
    print(f"   🔄 Changed: {changed_count} files")
    print(f"\n🎯 Datetime fields now use chrono::DateTime<chrono::Utc> instead of String")

if __name__ == '__main__':
    main() 