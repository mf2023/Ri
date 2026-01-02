<div align="center">

# File System 使用指南

**Version: 0.0.3**

**Last modified date: 2026-01-01**

本文档提供 DMSC File System 模块的使用示例。

</div>

## 基础文件操作

```python
from dmsc import DMSCFS

async def basic_file_example():
    """基础文件操作示例"""
    fs = DMSCFS()
    
    # 读取文本文件
    content = await fs.read_file("data/config.txt")
    print(f"配置内容: {content}")
    
    # 写入文本文件
    await fs.write_file("data/output.txt", "Hello, DMSC!")
    
    # 读取二进制文件
    binary_data = await fs.read_binary("images/photo.png")
    
    # 写入二进制文件
    await fs.write_binary("images/copy.png", binary_data)
```

## 文件信息

```python
from dmsc import DMSCFS

async def file_info_example():
    """文件信息示例"""
    fs = DMSCFS()
    
    # 检查文件是否存在
    exists = await fs.exists("data/file.txt")
    print(f"文件存在: {exists}")
    
    # 检查路径是否为文件
    is_file = await fs.is_file("data/file.txt")
    print(f"是文件: {is_file}")
    
    # 检查路径是否为目录
    is_dir = await fs.is_dir("data/")
    print(f"是目录: {is_dir}")
```

## 目录操作

```python
from dmsc import DMSCFS

async def directory_example():
    """目录操作示例"""
    fs = DMSCFS()
    
    # 列出目录内容
    files = await fs.list_dir("data/")
    print(f"文件列表: {files}")
    
    # 创建目录
    await fs.create_dir("data/new_folder", parents=True)
    
    # 删除目录
    await fs.remove_dir("data/old_folder", recursive=True)
```

## 路径安全

```python
from dmsc import DMSCFS

async def path_safety_example():
    """路径安全示例"""
    fs = DMSCFS(root_path="/project")
    
    # 所有路径都相对于 root_path
    # 只会访问 /project 内的文件
    content = await fs.read_file("data/config.txt")
    
    # 尝试访问父目录会被阻止
    try:
        # 这会失败 - 尝试访问 /etc/passwd
        content = await fs.read_file("../../etc/passwd")
    except Exception as e:
        print(f"访问被拒绝: {e}")
```

## 完整示例

```python
from dmsc import DMSCFS

async def complete_file_example():
    """完整文件系统示例"""
    fs = DMSCFS()
    
    # 创建项目结构
    await fs.create_dir("src", parents=True)
    await fs.create_dir("tests", parents=True)
    await fs.create_dir("docs", parents=True)
    
    # 写入源文件
    await fs.write_file("src/main.py", "# 主应用代码")
    await fs.write_file("src/utils.py", "# 工具函数")
    
    # 写入测试文件
    await fs.write_file("tests/test_main.py", "# 主测试")
    
    # 读取并处理文件
    main_code = await fs.read_file("src/main.py")
    print(f"主文件大小: {len(main_code)} 字节")
    
    # 列出 src 下所有文件
    src_files = await fs.list_dir("src/")
    print(f"源文件: {src_files}")
    
    # 清理
    await fs.remove_dir("tests", recursive=True)
```
