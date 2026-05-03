#!/usr/bin/env python3
import re
import os

FILES_TO_FIX = [
    "gateway.rs",
    "module_rpc.rs",
    "observability.rs",
    "ws.rs",
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


def add_imports(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    if "use crate::java::exception::throw_illegal_argument" in content:
        return content
    
    # Find the last use statement
    lines = content.split('\n')
    last_use_idx = -1
    
    for i, line in enumerate(lines):
        if line.strip().startswith('use ') and line.strip().endswith(';'):
            last_use_idx = i
    
    if last_use_idx != -1:
        lines.insert(last_use_idx + 1, 'use crate::java::exception::throw_illegal_argument;')
        lines.insert(last_use_idx + 2, 'use crate::java::{register_jni_ptr, unregister_jni_ptr, is_jni_ptr_valid};')
    else:
        # Find end of header
        for i, line in enumerate(lines):
            if line.strip() and not line.strip().startswith('//'):
                lines.insert(i, 'use crate::java::exception::throw_illegal_argument;')
                lines.insert(i + 1, 'use crate::java::{register_jni_ptr, unregister_jni_ptr, is_jni_ptr_valid};')
                break
    
    return '\n'.join(lines)


def fix_constructor(content):
    # Fix patterns like:
    # let gateway = Box::new(RiGateway::new());
    # Box::into_raw(gateway) as jlong
    
    # Look for: let <var> = Box::new(...);
    #           Box::into_raw(<var>) as jlong
    
    # First, fix simple cases
    def fix_case1(match):
        var = match.group(1)
        expr = match.group(2)
        return f'    let {var}_boxed = {expr};\n    let {var} = Box::into_raw({var}_boxed);\n    register_jni_ptr({var} as usize);\n    {var} as jlong'
    
    # Pattern: let <var> = Box::into_raw(...); (wait, no, original is: let x = Box::new(...); Box::into_raw(x) as jlong)
    
    # Let's do a line by line approach
    lines = content.split('\n')
    new_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        # Check if this line is a let statement with Box::new
        let_match = re.match(r'\s*let\s+(\w+)\s*=\s*Box::new\((.*)\);', line)
        if let_match:
            var_name = let_match.group(1)
            expr = let_match.group(2)
            # Look ahead for Box::into_raw(var_name)
            if i + 1 < len(lines):
                next_line = lines[i + 1]
                if f'Box::into_raw({var_name})' in next_line and 'as jlong' in next_line:
                    # Found our pattern!
                    new_lines.append(f'    let {var_name}_boxed = Box::new({expr});')
                    new_lines.append(f'    let {var_name} = Box::into_raw({var_name}_boxed);')
                    new_lines.append(f'    register_jni_ptr({var_name} as usize);')
                    # Replace the Box::into_raw line
                    indent = next_line[:len(next_line) - len(next_line.lstrip())]
                    new_lines.append(f'{indent}{var_name} as jlong')
                    i += 2  # skip 2 lines
                    continue
        new_lines.append(line)
        i += 1
    
    return '\n'.join(new_lines)


def fix_free_funcs(content):
    # Fix free functions: replace "if ptr != 0 {" with "if ptr != 0 && is_jni_ptr_valid(ptr as usize) {\n        unregister_jni_ptr(ptr as usize);"
    
    return content.replace(
        'if ptr != 0 {', 
        'if ptr != 0 && is_jni_ptr_valid(ptr as usize) {\n        unregister_jni_ptr(ptr as usize);'
    )


def fix_file(filename):
    filepath = os.path.join(CLASSES_DIR, filename)
    if not os.path.exists(filepath):
        print(f"Missing: {filename}")
        return False
    
    print(f"Fixing: {filename}")
    
    content = add_imports(filepath)
    content = fix_constructor(content)
    content = fix_free_funcs(content)
    
    # Write back
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"Done: {filename}")
    return True


def main():
    print("Simple fixing of JNI files...")
    for filename in FILES_TO_FIX:
        fix_file(filename)
    print("All done!")


if __name__ == "__main__":
    main()
