#!/usr/bin/env python3
import re
import os

# List of files to fix (skip validation.rs)
FILES_TO_FIX = [
    "fs.rs",
    "gateway.rs",
    "module_rpc.rs",
    "observability.rs",
    "ws.rs",
    "config.rs",
    "queue.rs",
    "cache.rs",
    "service_mesh.rs",
    "database.rs",
    "protocol.rs",
    "hooks.rs",
    "auth.rs",
    "grpc.rs",
    "device.rs",
]

CLASSES_DIR = "/workspace/src/java/classes"


def add_imports(content):
    """Add necessary imports to the file"""
    # Check if imports already exist
    if "use crate::java::exception::throw_illegal_argument" in content:
        return content
    
    # Find the use statements section
    # Look for the last use statement and add our imports after
    use_pattern = re.compile(r'^use\s+[\w:]+;', re.MULTILINE)
    matches = list(use_pattern.finditer(content))
    
    if matches:
        last_match = matches[-1]
        insert_pos = last_match.end()
        
        # Add our imports
        new_content = content[:insert_pos] + '\nuse crate::java::exception::throw_illegal_argument;\nuse crate::java::{register_jni_ptr, unregister_jni_ptr, is_jni_ptr_valid};' + content[insert_pos:]
        return new_content
    else:
        # If no use statements, add after copyright header
        header_end = content.find('//!')
        if header_end == -1:
            return content
        
        # Find the end of the header comments
        lines = content.split('\n')
        header_end_line = 0
        for i, line in enumerate(lines):
            if not line.strip().startswith('//') and line.strip():
                header_end_line = i
                break
        
        if header_end_line > 0:
            lines.insert(header_end_line, 'use crate::java::exception::throw_illegal_argument;')
            lines.insert(header_end_line + 1, 'use crate::java::{register_jni_ptr, unregister_jni_ptr, is_jni_ptr_valid};')
            return '\n'.join(lines)
    
    return content


def fix_box_into_raw(content):
    """Fix Box::into_raw(...) as jlong patterns to register the pointer"""
    # Pattern: let <var> = Box::into_raw(<expr>) as jlong;
    # Or: return Box::into_raw(<expr>) as jlong;
    # We need to wrap it to register the pointer
    
    # First, handle let statements
    def fix_let_stmt(match):
        var_name = match.group(1)
        expr = match.group(2)
        return f'    let {var_name}_boxed = {expr};\n    let {var_name} = Box::into_raw({var_name}_boxed);\n    register_jni_ptr({var_name} as usize);\n    {var_name} as jlong'
    
    # Pattern for let statements
    let_pattern = re.compile(r'let\s+(\w+)\s*=\s*Box::into_raw\((.*?)\)\s*as\s+jlong\s*;')
    content = let_pattern.sub(fix_let_stmt, content)
    
    # Handle return statements
    def fix_return_stmt(match):
        expr = match.group(1)
        return f'    let boxed = {expr};\n    let ptr = Box::into_raw(boxed);\n    register_jni_ptr(ptr as usize);\n    return ptr as jlong;'
    
    return_pattern = re.compile(r'return\s+Box::into_raw\((.*?)\)\s*as\s+jlong\s*;')
    content = return_pattern.sub(fix_return_stmt, content)
    
    # Handle simple assignment without let
    def fix_simple_assignment(match):
        pre = match.group(1)
        expr = match.group(2)
        post = match.group(3)
        return f'{pre}let boxed = {expr};\n    let ptr = Box::into_raw(boxed);\n    register_jni_ptr(ptr as usize);\n    ptr as jlong{post}'
    
    simple_pattern = re.compile(r'(\s)(Box::into_raw\(.*?\)\s*as\s+jlong)([\s;])')
    content = simple_pattern.sub(fix_simple_assignment, content)
    
    return content


def fix_free_functions(content):
    """Fix _free0 functions to check validity and unregister"""
    # Pattern:
    # pub extern "system" fn Java_com_dunimd_ri_xxx_yyy_free0(...) {
    #     if ptr != 0 {
    #         unsafe {
    #             let _ = Box::from_raw(ptr as *mut Type);
    #         }
    #     }
    # }
    
    # We need to change to:
    # if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
    #     unregister_jni_ptr(ptr as usize);
    #     unsafe { ... }
    
    # First, find all free functions
    def fix_free_func(match):
        func_def = match.group(1)
        inner = match.group(2)
        # Replace the if condition
        new_inner = inner.replace('if ptr != 0 {', 'if ptr != 0 && is_jni_ptr_valid(ptr as usize) {\n        unregister_jni_ptr(ptr as usize);')
        return f'{func_def}{new_inner}'
    
    free_func_pattern = re.compile(r'(\#\[no_mangle\]\s+pub\s+extern\s+"system"\s+fn\s+Java_com_dunimd_ri_\w+_\w+_free0.*?\{)(.*?)(\n\})', re.DOTALL)
    content = free_func_pattern.sub(fix_free_func, content)
    
    return content


def fix_pointer_access(content):
    """Add is_jni_ptr_valid checks after check_not_null calls"""
    # Pattern: if !check_not_null(&mut env, ptr, "Type") { return ... }
    # We need to add after: if !is_jni_ptr_valid(ptr as usize) { throw_illegal_argument(...); return ... }
    
    def fix_access(match):
        check_line = match.group(1)
        return_stmt = match.group(2)
        rest = match.group(3)
        type_name = re.search(r'"([^"]+)"', check_line).group(1)
        
        return f'{check_line}{return_stmt}\n    if !is_jni_ptr_valid(ptr as usize) {{\n        throw_illegal_argument(&mut env, "Invalid {type_name} pointer");{return_stmt}\n    }}{rest}'
    
    # This is complex, let's handle case by case
    # First, find check_not_null calls followed by return
    access_pattern = re.compile(r'(if\s+!check_not_null\(&mut\s+env,\s*ptr,\s*"[^"]+"\)\s*\{)(.*?\n\s*\})(.*?)(?=\n\s*(let|if|unsafe|return))', re.DOTALL)
    
    # Let's do a simpler approach: find all check_not_null calls
    lines = content.split('\n')
    new_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        new_lines.append(line)
        
        if 'check_not_null' in line and 'ptr' in line:
            # Extract the type name
            type_match = re.search(r'"([^"]+)"', line)
            if type_match:
                type_name = type_match.group(1)
                # Look at the next lines to see the return statement
                j = i + 1
                return_lines = []
                has_return = False
                has_early_exit = False
                
                while j < len(lines) and j < i + 5:
                    next_line = lines[j].strip()
                    if next_line.startswith('return'):
                        has_return = True
                        return_lines.append(lines[j])
                        if '}' in next_line:
                            has_early_exit = True
                            break
                    elif next_line == '}':
                        has_early_exit = True
                        return_lines.append(lines[j])
                        break
                    elif next_line:
                        return_lines.append(lines[j])
                    j += 1
                
                if has_early_exit or has_return:
                    # Add our check
                    indent = line[:len(line) - len(line.lstrip())]
                    new_lines.append(f'{indent}    if !is_jni_ptr_valid(ptr as usize) {{')
                    new_lines.append(f'{indent}        throw_illegal_argument(&mut env, "Invalid {type_name} pointer");')
                    for r_line in return_lines:
                        new_lines.append(r_line)
                    new_lines.append(f'{indent}    }}')
                    i = j  # Skip the lines we already processed
                    continue
        i += 1
    
    return '\n'.join(new_lines)


def fix_file(filename):
    """Fix a single file"""
    filepath = os.path.join(CLASSES_DIR, filename)
    if not os.path.exists(filepath):
        print(f"File not found: {filepath}")
        return False
    
    print(f"Processing: {filename}")
    
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Apply fixes
    original_length = len(content)
    
    # 1. Add imports
    content = add_imports(content)
    
    # 2. Fix Box::into_raw
    content = fix_box_into_raw(content)
    
    # 3. Fix free functions
    content = fix_free_functions(content)
    
    # 4. Fix pointer access
    content = fix_pointer_access(content)
    
    # Check if content changed
    if len(content) != original_length or content != open(filepath, 'r', encoding='utf-8').read():
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"  Updated: {filename}")
        return True
    else:
        print(f"  No changes: {filename}")
        return False


def main():
    print("Starting JNI files batch fix...")
    print(f"Files to process: {FILES_TO_FIX}")
    
    updated_count = 0
    for filename in FILES_TO_FIX:
        if fix_file(filename):
            updated_count += 1
    
    print(f"\nDone! Updated {updated_count} files.")


if __name__ == "__main__":
    main()
