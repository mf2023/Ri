<div align="center">

# 验证使用示例

**Version: 1.0.0**

**Last modified date: 2025-12-12**

本示例展示如何使用DMSC的validation模块进行数据验证、数据清理、自定义验证器、条件验证、异步验证和验证配置的使用。

## 示例概述

</div>

本示例将创建一个DMSC应用，实现以下功能：

- 基本数据验证（字符串、数字、日期等）
- 复杂对象和嵌套验证
- 自定义验证器和验证规则
- 条件验证和依赖验证
- 异步验证和批量验证
- 验证错误处理和消息自定义
- 数据清理和格式化
- 验证性能优化

<div align="center">

## 前置要求

</div>

- Python 3.8+
- pip 21.0+
- 基本的Python编程知识
- 了解数据验证基本概念
- （可选）JSON Schema知识

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
mkdir dms-validation-example
cd dms-validation-example
python -m venv venv
source venv/bin/activate  # Windows: venv\\Scripts\\activate
```

### 2. 安装依赖

创建`requirements.txt`文件：

```txt
dms>=1.0.0
pydantic>=2.0.0
email-validator>=2.0.0
phonenumbers>=8.13.0
python-dateutil>=2.8.0
jsonschema>=4.0.0
cerberus>=1.3.0
```

安装依赖：

```bash
pip install -r requirements.txt
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

```yaml
service:
  name: "dms-validation-example"
  version: "1.0.0"

validation:
  strict_mode: true
  stop_on_first_error: false
  enable_type_coercion: true
  locale: "zh_CN"
  timezone: "Asia/Shanghai"
  custom_messages:
    required: "此字段为必填项"
    email: "请输入有效的邮箱地址"
    min_length: "此字段至少需要{min}个字符"
    max_length: "此字段最多允许{max}个字符"
    pattern: "输入格式不符合要求"
    numeric: "请输入有效的数字"
    date: "请输入有效的日期"
    url: "请输入有效的网址"
    phone: "请输入有效的手机号码"
```

### 4. 编写主代码

创建`main.py`文件：

```python
import asyncio
import json
import re
from datetime import datetime, date
from typing import Dict, Any, List, Optional, Union
from dms import DMSCApp, DMSCConfig, DMSCValidationConfig
from dms.validation import (
    DMSCValidator, ValidationRule, ValidationResult,
    StringValidator, NumberValidator, DateValidator,
    EmailValidator, PhoneValidator, URLValidator,
    CustomValidator, ConditionalValidator
)

async def main():
    """主函数"""
    # 构建服务运行时
    app = DMSCApp(
        config=DMSCConfig.from_yaml("config.yaml"),
        validation_config=DMSCValidationConfig()
    )
    
    # 运行业务逻辑
    await app.run(initialize_and_demo)

async def initialize_and_demo(app: DMSCApp):
    """初始化和演示验证功能"""
    logger = app.logger
    logger.info("DMSC Validation Example started")
    
    # 初始化验证器
    await initialize_validators(app)
    
    # 演示基本验证
    await demonstrate_basic_validation(app)
    
    # 演示复杂验证
    await demonstrate_complex_validation(app)
    
    # 演示自定义验证器
    await demonstrate_custom_validators(app)
    
    # 演示条件验证
    await demonstrate_conditional_validation(app)
    
    # 演示异步验证
    await demonstrate_async_validation(app)
    
    # 演示批量验证
    await demonstrate_batch_validation(app)
    
    # 演示数据清理
    await demonstrate_data_cleaning(app)
    
    logger.info("DMSC Validation Example completed")

async def initialize_validators(app: DMSCApp):
    """初始化验证器"""
    logger = app.logger
    
    # 获取验证管理器
    validator = app.validator
    
    # 注册基本验证器
    validator.register_validator("string", StringValidator())
    validator.register_validator("number", NumberValidator())
    validator.register_validator("date", DateValidator())
    validator.register_validator("email", EmailValidator())
    validator.register_validator("phone", PhoneValidator())
    validator.register_validator("url", URLValidator())
    
    # 注册自定义验证器
    validator.register_validator("chinese_mobile", ChineseMobileValidator())
    validator.register_validator("id_card", IDCardValidator())
    validator.register_validator("bank_card", BankCardValidator())
    validator.register_validator("business_license", BusinessLicenseValidator())
    
    logger.info("Validators initialized successfully")

async def demonstrate_basic_validation(app: DMSCApp):
    """演示基本验证"""
    logger = app.logger
    
    logger.info("=== 基本验证演示 ===")
    
    # 字符串验证
    await demonstrate_string_validation(app)
    
    # 数字验证
    await demonstrate_number_validation(app)
    
    # 日期验证
    await demonstrate_date_validation(app)
    
    # 邮箱验证
    await demonstrate_email_validation(app)
    
    # 手机号验证
    await demonstrate_phone_validation(app)
    
    # URL验证
    await demonstrate_url_validation(app)
    
    logger.info("Basic validation demonstration completed")

async def demonstrate_string_validation(app: DMSCApp):
    """演示字符串验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "username": {
            "type": "string",
            "required": True,
            "min_length": 3,
            "max_length": 20,
            "pattern": r"^[a-zA-Z0-9_]+$",
            "messages": {
                "required": "用户名为必填项",
                "min_length": "用户名至少需要3个字符",
                "max_length": "用户名最多允许20个字符",
                "pattern": "用户名只能包含字母、数字和下划线"
            }
        },
        "nickname": {
            "type": "string",
            "required": False,
            "max_length": 50,
            "trim": True,
            "messages": {
                "max_length": "昵称最多允许50个字符"
            }
        }
    }
    
    # 测试数据
    test_data = {
        "username": "john_doe",
        "nickname": "  John Doe  "
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("String validation passed", cleaned_data=result.cleaned_data)
    else:
        logger.error("String validation failed", errors=result.errors)
    
    # 测试无效数据
    invalid_data = {
        "username": "jo",  # 太短
        "nickname": "a" * 60  # 太长
    }
    
    result = await app.validator.validate(invalid_data, rules)
    
    if not result.is_valid:
        logger.info("Invalid string data rejected correctly", errors=result.errors)

async def demonstrate_number_validation(app: DMSCApp):
    """演示数字验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "age": {
            "type": "integer",
            "required": True,
            "min": 18,
            "max": 120,
            "messages": {
                "required": "年龄为必填项",
                "min": "年龄必须大于等于18岁",
                "max": "年龄不能超过120岁",
                "type": "请输入有效的整数"
            }
        },
        "height": {
            "type": "float",
            "required": False,
            "min": 0.5,
            "max": 3.0,
            "precision": 2,
            "messages": {
                "min": "身高不能小于0.5米",
                "max": "身高不能超过3.0米",
                "precision": "身高最多保留2位小数"
            }
        },
        "score": {
            "type": "number",
            "required": True,
            "min": 0,
            "max": 100,
            "messages": {
                "required": "分数为必填项",
                "min": "分数不能小于0",
                "max": "分数不能超过100"
            }
        }
    }
    
    # 测试数据
    test_data = {
        "age": 25,
        "height": 1.75,
        "score": 85.5
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Number validation passed", cleaned_data=result.cleaned_data)
    else:
        logger.error("Number validation failed", errors=result.errors)

async def demonstrate_date_validation(app: DMSCApp):
    """演示日期验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "birth_date": {
            "type": "date",
            "required": True,
            "min_date": "1900-01-01",
            "max_date": "2020-12-31",
            "format": "%Y-%m-%d",
            "messages": {
                "required": "出生日期为必填项",
                "min_date": "出生日期不能早于1900年",
                "max_date": "出生日期不能晚于2020年",
                "format": "日期格式必须为YYYY-MM-DD"
            }
        },
        "appointment_time": {
            "type": "datetime",
            "required": False,
            "format": "%Y-%m-%d %H:%M:%S",
            "future_only": True,
            "messages": {
                "format": "时间格式必须为YYYY-MM-DD HH:MM:SS",
                "future_only": "预约时间必须是未来时间"
            }
        }
    }
    
    # 测试数据
    test_data = {
        "birth_date": "1998-01-15",
        "appointment_time": "2024-12-31 14:30:00"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Date validation passed", cleaned_data=result.cleaned_data)
    else:
        logger.error("Date validation failed", errors=result.errors)

async def demonstrate_email_validation(app: DMSCApp):
    """演示邮箱验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "email": {
            "type": "email",
            "required": True,
            "domain_whitelist": ["example.com", "test.com"],
            "domain_blacklist": ["tempmail.com", "10minutemail.com"],
            "check_mx": False,  # 不检查MX记录，提高性能
            "messages": {
                "required": "邮箱地址为必填项",
                "email": "请输入有效的邮箱地址",
                "domain_whitelist": "邮箱域名不在允许列表中",
                "domain_blacklist": "邮箱域名在黑名单中"
            }
        },
        "backup_email": {
            "type": "email",
            "required": False,
            "different_from": "email",  # 必须与主邮箱不同
            "messages": {
                "email": "请输入有效的备用邮箱地址",
                "different_from": "备用邮箱不能与主邮箱相同"
            }
        }
    }
    
    # 测试数据
    test_data = {
        "email": "john.doe@example.com",
        "backup_email": "john.backup@test.com"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Email validation passed", cleaned_data=result.cleaned_data)
    else:
        logger.error("Email validation failed", errors=result.errors)

async def demonstrate_phone_validation(app: DMSCApp):
    """演示手机号验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "mobile": {
            "type": "phone",
            "required": True,
            "region": "CN",  # 中国手机号
            "format": "E164",  # 国际格式
            "messages": {
                "required": "手机号为必填项",
                "phone": "请输入有效的中国手机号码",
                "format": "手机号格式必须为国际格式"
            }
        },
        "landline": {
            "type": "phone",
            "required": False,
            "region": "CN",
            "format": "national",
            "messages": {
                "phone": "请输入有效的固定电话号码",
                "format": "固定电话格式必须为国内格式"
            }
        }
    }
    
    # 测试数据
    test_data = {
        "mobile": "+8613800138000",
        "landline": "010-12345678"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Phone validation passed", cleaned_data=result.cleaned_data)
    else:
        logger.error("Phone validation failed", errors=result.errors)

async def demonstrate_url_validation(app: DMSCApp):
    """演示URL验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "website": {
            "type": "url",
            "required": False,
            "schemes": ["http", "https"],
            "require_tld": True,
            "allow_local": False,
            "messages": {
                "url": "请输入有效的网址",
                "schemes": "网址必须以http或https开头",
                "require_tld": "网址必须包含顶级域名"
            }
        },
        "callback_url": {
            "type": "url",
            "required": True,
            "schemes": ["https"],  # 只允许HTTPS
            "domain_whitelist": ["api.example.com", "secure.example.com"],
            "messages": {
                "required": "回调地址为必填项",
                "url": "请输入有效的回调地址",
                "schemes": "回调地址必须使用HTTPS协议",
                "domain_whitelist": "回调地址域名不在允许列表中"
            }
        }
    }
    
    # 测试数据
    test_data = {
        "website": "https://johndoe.com",
        "callback_url": "https://api.example.com/webhook"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("URL validation passed", cleaned_data=result.cleaned_data)
    else:
        logger.error("URL validation failed", errors=result.errors)

async def demonstrate_complex_validation(app: DMSCApp):
    """演示复杂验证"""
    logger = app.logger
    
    logger.info("=== 复杂验证演示 ===")
    
    # 嵌套对象验证
    await demonstrate_nested_validation(app)
    
    # 数组验证
    await demonstrate_array_validation(app)
    
    # 交叉验证
    await demonstrate_cross_validation(app)
    
    logger.info("Complex validation demonstration completed")

async def demonstrate_nested_validation(app: DMSCApp):
    """演示嵌套对象验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "user": {
            "type": "dict",
            "required": True,
            "schema": {
                "profile": {
                    "type": "dict",
                    "required": True,
                    "schema": {
                        "first_name": {
                            "type": "string",
                            "required": True,
                            "min_length": 1,
                            "max_length": 50
                        },
                        "last_name": {
                            "type": "string",
                            "required": True,
                            "min_length": 1,
                            "max_length": 50
                        },
                        "avatar": {
                            "type": "url",
                            "required": False
                        }
                    }
                },
                "address": {
                    "type": "dict",
                    "required": False,
                    "schema": {
                        "street": {"type": "string", "required": True},
                        "city": {"type": "string", "required": True},
                        "country": {"type": "string", "required": True},
                        "postal_code": {"type": "string", "required": True}
                    }
                }
            }
        }
    }
    
    # 测试数据
    test_data = {
        "user": {
            "profile": {
                "first_name": "John",
                "last_name": "Doe",
                "avatar": "https://example.com/avatar.jpg"
            },
            "address": {
                "street": "123 Main St",
                "city": "New York",
                "country": "USA",
                "postal_code": "10001"
            }
        }
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Nested validation passed", cleaned_data=result.cleaned_data)
    else:
        logger.error("Nested validation failed", errors=result.errors)

async def demonstrate_array_validation(app: DMSCApp):
    """演示数组验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "tags": {
            "type": "list",
            "required": True,
            "min_length": 1,
            "max_length": 10,
            "items": {
                "type": "string",
                "min_length": 1,
                "max_length": 20,
                "pattern": r"^[a-zA-Z0-9_-]+$"
            },
            "unique": True,  # 元素必须唯一
            "messages": {
                "min_length": "至少需要1个标签",
                "max_length": "最多允许10个标签",
                "unique": "标签不能重复"
            }
        },
        "scores": {
            "type": "list",
            "required": False,
            "items": {
                "type": "integer",
                "min": 0,
                "max": 100
            }
        }
    }
    
    # 测试数据
    test_data = {
        "tags": ["python", "validation", "dms", "example"],
        "scores": [85, 92, 78, 95]
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Array validation passed", cleaned_data=result.cleaned_data)
    else:
        logger.error("Array validation failed", errors=result.errors)

async def demonstrate_cross_validation(app: DMSCApp):
    """演示交叉验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "start_date": {
            "type": "date",
            "required": True,
            "format": "%Y-%m-%d"
        },
        "end_date": {
            "type": "date",
            "required": True,
            "format": "%Y-%m-%d",
            "custom_validation": "validate_date_range"  # 自定义验证函数
        }
    }
    
    # 注册自定义验证函数
    app.validator.register_custom_validation(
        "validate_date_range",
        validate_date_range
    )
    
    # 测试数据
    test_data = {
        "start_date": "2024-01-01",
        "end_date": "2024-12-31"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Cross validation passed", cleaned_data=result.cleaned_data)
    else:
        logger.error("Cross validation failed", errors=result.errors)

def validate_date_range(data: Dict[str, Any], field: str, value: Any) -> Optional[str]:
    """验证日期范围"""
    start_date = data.get("start_date")
    end_date = value
    
    if start_date and end_date:
        try:
            start = datetime.strptime(start_date, "%Y-%m-%d").date()
            end = datetime.strptime(end_date, "%Y-%m-%d").date()
            
            if end < start:
                return "结束日期不能早于开始日期"
            
            if (end - start).days > 365:
                return "日期范围不能超过1年"
                
        except ValueError:
            return "日期格式无效"
    
    return None

async def demonstrate_custom_validators(app: DMSCApp):
    """演示自定义验证器"""
    logger = app.logger
    
    logger.info("=== 自定义验证器演示 ===")
    
    # 中国手机号验证器
    await demonstrate_chinese_mobile_validator(app)
    
    # 身份证号验证器
    await demonstrate_id_card_validator(app)
    
    # 银行卡号验证器
    await demonstrate_bank_card_validator(app)
    
    # 营业执照验证器
    await demonstrate_business_license_validator(app)
    
    logger.info("Custom validators demonstration completed")

async def demonstrate_chinese_mobile_validator(app: DMSCApp):
    """演示中国手机号验证器"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "mobile": {
            "type": "chinese_mobile",
            "required": True,
            "carrier_check": True,  # 检查运营商
            "messages": {
                "required": "手机号为必填项",
                "chinese_mobile": "请输入有效的中国手机号码",
                "carrier_check": "不支持的运营商"
            }
        }
    }
    
    # 测试数据
    test_data = {
        "mobile": "13800138000"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Chinese mobile validation passed", cleaned_data=result.cleaned_data)
    else:
        logger.error("Chinese mobile validation failed", errors=result.errors)

async def demonstrate_id_card_validator(app: DMSCApp):
    """演示身份证号验证器"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "id_card": {
            "type": "id_card",
            "required": True,
            "verify_checksum": True,  # 验证校验码
            "extract_info": True,  # 提取信息
            "messages": {
                "required": "身份证号为必填项",
                "id_card": "请输入有效的身份证号码",
                "verify_checksum": "身份证号码校验码错误"
            }
        }
    }
    
    # 测试数据
    test_data = {
        "id_card": "11010519900307283X"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("ID card validation passed", cleaned_data=result.cleaned_data)
        # 提取的信息
        if "extracted_info" in result.metadata:
            logger.info("Extracted ID card info", info=result.metadata["extracted_info"])
    else:
        logger.error("ID card validation failed", errors=result.errors)

async def demonstrate_bank_card_validator(app: DMSCApp):
    """演示银行卡号验证器"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "bank_card": {
            "type": "bank_card",
            "required": True,
            "verify_luhn": True,  # Luhn算法验证
            "detect_bank": True,  # 检测发卡行
            "messages": {
                "required": "银行卡号为必填项",
                "bank_card": "请输入有效的银行卡号",
                "verify_luhn": "银行卡号校验失败",
                "detect_bank": "无法识别的发卡行"
            }
        }
    }
    
    # 测试数据
    test_data = {
        "bank_card": "6222021001122334455"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Bank card validation passed", cleaned_data=result.cleaned_data)
        # 检测的银行信息
        if "bank_info" in result.metadata:
            logger.info("Detected bank info", info=result.metadata["bank_info"])
    else:
        logger.error("Bank card validation failed", errors=result.errors)

async def demonstrate_business_license_validator(app: DMSCApp):
    """演示营业执照验证器"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "business_license": {
            "type": "business_license",
            "required": True,
            "verify_checksum": True,
            "extract_info": True,
            "messages": {
                "required": "营业执照号为必填项",
                "business_license": "请输入有效的营业执照号",
                "verify_checksum": "营业执照号校验失败"
            }
        }
    }
    
    # 测试数据
    test_data = {
        "business_license": "91110000123456789X"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Business license validation passed", cleaned_data=result.cleaned_data)
        # 提取的信息
        if "extracted_info" in result.metadata:
            logger.info("Extracted business license info", info=result.metadata["extracted_info"])
    else:
        logger.error("Business license validation failed", errors=result.errors)

async def demonstrate_conditional_validation(app: DMSCApp):
    """演示条件验证"""
    logger = app.logger
    
    logger.info("=== 条件验证演示 ===")
    
    # 依赖验证
    await demonstrate_dependency_validation(app)
    
    # 条件必填
    await demonstrate_conditional_required(app)
    
    # 动态验证规则
    await demonstrate_dynamic_rules(app)
    
    logger.info("Conditional validation demonstration completed")

async def demonstrate_dependency_validation(app: DMSCApp):
    """演示依赖验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "country": {
            "type": "string",
            "required": True
        },
        "id_type": {
            "type": "string",
            "required": True,
            "allowed": ["passport", "id_card", "driver_license"]
        },
        "id_number": {
            "type": "string",
            "required": True,
            "custom_validation": "validate_by_country_and_type"
        }
    }
    
    # 注册自定义验证函数
    app.validator.register_custom_validation(
        "validate_by_country_and_type",
        validate_by_country_and_type
    )
    
    # 测试数据 - 中国用户
    test_data_cn = {
        "country": "CN",
        "id_type": "id_card",
        "id_number": "11010519900307283X"
    }
    
    result = await app.validator.validate(test_data_cn, rules)
    
    if result.is_valid:
        logger.info("Chinese ID validation passed")
    else:
        logger.error("Chinese ID validation failed", errors=result.errors)
    
    # 测试数据 - 美国用户
    test_data_us = {
        "country": "US",
        "id_type": "passport",
        "id_number": "C12345678"
    }
    
    result = await app.validator.validate(test_data_us, rules)
    
    if result.is_valid:
        logger.info("US passport validation passed")
    else:
        logger.error("US passport validation failed", errors=result.errors)

def validate_by_country_and_type(data: Dict[str, Any], field: str, value: str) -> Optional[str]:
    """根据国家验证证件号"""
    country = data.get("country")
    id_type = data.get("id_type")
    
    if country == "CN" and id_type == "id_card":
        # 验证中国身份证号
        if not re.match(r'^\d{17}[\dXx]$', value):
            return "中国身份证号格式错误"
    elif country == "US" and id_type == "passport":
        # 验证美国护照号
        if not re.match(r'^[A-Z]\d{8}$', value):
            return "美国护照号格式错误"
    
    return None

async def demonstrate_conditional_required(app: DMSCApp):
    """演示条件必填"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "user_type": {
            "type": "string",
            "required": True,
            "allowed": ["individual", "enterprise"]
        },
        "company_name": {
            "type": "string",
            "required_when": {"user_type": "enterprise"},
            "messages": {
                "required_when": "企业用户必须填写公司名称"
            }
        },
        "business_license": {
            "type": "string",
            "required_when": {"user_type": "enterprise"},
            "messages": {
                "required_when": "企业用户必须填写营业执照号"
            }
        },
        "personal_id": {
            "type": "string",
            "required_when": {"user_type": "individual"},
            "messages": {
                "required_when": "个人用户必须填写身份证号"
            }
        }
    }
    
    # 测试企业用户
    enterprise_data = {
        "user_type": "enterprise",
        "company_name": "示例科技有限公司",
        "business_license": "91110000123456789X"
    }
    
    result = await app.validator.validate(enterprise_data, rules)
    
    if result.is_valid:
        logger.info("Enterprise user validation passed")
    else:
        logger.error("Enterprise user validation failed", errors=result.errors)
    
    # 测试个人用户
    individual_data = {
        "user_type": "individual",
        "personal_id": "11010519900307283X"
    }
    
    result = await app.validator.validate(individual_data, rules)
    
    if result.is_valid:
        logger.info("Individual user validation passed")
    else:
        logger.error("Individual user validation failed", errors=result.errors)

async def demonstrate_dynamic_rules(app: DMSCApp):
    """演示动态验证规则"""
    logger = app.logger
    
    # 根据配置动态生成验证规则
    config = {
        "enable_strict_validation": True,
        "required_fields": ["username", "email"],
        "optional_fields": ["phone", "address"]
    }
    
    # 动态生成规则
    rules = generate_dynamic_rules(config)
    
    # 测试数据
    test_data = {
        "username": "john_doe",
        "email": "john@example.com",
        "phone": "+8613800138000"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Dynamic rules validation passed", rules_count=len(rules))
    else:
        logger.error("Dynamic rules validation failed", errors=result.errors)

def generate_dynamic_rules(config: Dict[str, Any]) -> Dict[str, Any]:
    """生成动态验证规则"""
    rules = {}
    
    # 必填字段
    for field in config["required_fields"]:
        if field == "username":
            rules[field] = {
                "type": "string",
                "required": True,
                "min_length": 3 if config["enable_strict_validation"] else 1,
                "max_length": 20,
                "pattern": r"^[a-zA-Z0-9_]+$" if config["enable_strict_validation"] else None
            }
        elif field == "email":
            rules[field] = {
                "type": "email",
                "required": True
            }
    
    # 可选字段
    for field in config["optional_fields"]:
        if field == "phone":
            rules[field] = {
                "type": "phone",
                "required": False,
                "region": "CN"
            }
        elif field == "address":
            rules[field] = {
                "type": "string",
                "required": False,
                "max_length": 200
            }
    
    return rules

async def demonstrate_async_validation(app: DMSCApp):
    """演示异步验证"""
    logger = app.logger
    
    logger.info("=== 异步验证演示 ===")
    
    # 外部API验证
    await demonstrate_external_api_validation(app)
    
    # 数据库验证
    await demonstrate_database_validation(app)
    
    # 缓存验证
    await demonstrate_cache_validation(app)
    
    logger.info("Async validation demonstration completed")

async def demonstrate_external_api_validation(app: DMSCApp):
    """演示外部API验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "email": {
            "type": "email",
            "required": True,
            "custom_validation": "validate_email_via_api"
        },
        "phone": {
            "type": "phone",
            "required": True,
            "custom_validation": "validate_phone_via_api"
        }
    }
    
    # 注册自定义验证函数
    app.validator.register_custom_validation(
        "validate_email_via_api",
        validate_email_via_api
    )
    
    app.validator.register_custom_validation(
        "validate_phone_via_api",
        validate_phone_via_api
    )
    
    # 测试数据
    test_data = {
        "email": "test@example.com",
        "phone": "+8613800138000"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("External API validation passed")
    else:
        logger.error("External API validation failed", errors=result.errors)

async def validate_email_via_api(data: Dict[str, Any], field: str, value: str) -> Optional[str]:
    """通过外部API验证邮箱"""
    # 模拟API调用
    await asyncio.sleep(0.1)  # 模拟网络延迟
    
    # 模拟API响应
    if value.endswith("@tempmail.com"):
        return "临时邮箱不允许使用"
    
    return None

async def validate_phone_via_api(data: Dict[str, Any], field: str, value: str) -> Optional[str]:
    """通过外部API验证手机号"""
    # 模拟API调用
    await asyncio.sleep(0.1)  # 模拟网络延迟
    
    # 模拟API响应
    if value == "+8613800000000":
        return "该手机号已被注册"
    
    return None

async def demonstrate_database_validation(app: DMSCApp):
    """演示数据库验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "username": {
            "type": "string",
            "required": True,
            "custom_validation": "validate_username_unique"
        },
        "email": {
            "type": "email",
            "required": True,
            "custom_validation": "validate_email_unique"
        }
    }
    
    # 注册自定义验证函数
    app.validator.register_custom_validation(
        "validate_username_unique",
        validate_username_unique
    )
    
    app.validator.register_custom_validation(
        "validate_email_unique",
        validate_email_unique
    )
    
    # 测试数据
    test_data = {
        "username": "john_doe",
        "email": "john@example.com"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Database validation passed")
    else:
        logger.error("Database validation failed", errors=result.errors)

async def validate_username_unique(data: Dict[str, Any], field: str, value: str) -> Optional[str]:
    """验证用户名唯一性"""
    # 模拟数据库查询
    await asyncio.sleep(0.05)  # 模拟数据库查询延迟
    
    # 模拟数据库中已存在的用户名
    existing_usernames = ["admin", "root", "test", "user"]
    
    if value.lower() in existing_usernames:
        return f"用户名 '{value}' 已被使用"
    
    return None

async def validate_email_unique(data: Dict[str, Any], field: str, value: str) -> Optional[str]:
    """验证邮箱唯一性"""
    # 模拟数据库查询
    await asyncio.sleep(0.05)  # 模拟数据库查询延迟
    
    # 模拟数据库中已存在的邮箱
    existing_emails = ["admin@example.com", "test@example.com"]
    
    if value.lower() in existing_emails:
        return f"邮箱地址 '{value}' 已被注册"
    
    return None

async def demonstrate_cache_validation(app: DMSCApp):
    """演示缓存验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "promotion_code": {
            "type": "string",
            "required": True,
            "custom_validation": "validate_promotion_code"
        }
    }
    
    # 注册自定义验证函数
    app.validator.register_custom_validation(
        "validate_promotion_code",
        validate_promotion_code
    )
    
    # 测试数据
    test_data = {
        "promotion_code": "SUMMER2024"
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Promotion code validation passed")
    else:
        logger.error("Promotion code validation failed", errors=result.errors)

async def validate_promotion_code(data: Dict[str, Any], field: str, value: str) -> Optional[str]:
    """验证优惠码"""
    # 模拟缓存查询
    await asyncio.sleep(0.01)  # 模拟缓存查询延迟
    
    # 模拟缓存中的有效优惠码
    valid_codes = ["SUMMER2024", "WINTER2024", "SPRING2024"]
    expired_codes = ["EXPIRED2023", "OLD2022"]
    
    if value in expired_codes:
        return "优惠码已过期"
    
    if value not in valid_codes:
        return "无效的优惠码"
    
    return None

async def demonstrate_batch_validation(app: DMSCApp):
    """演示批量验证"""
    logger = app.logger
    
    logger.info("=== 批量验证演示 ===")
    
    # 用户列表验证
    await demonstrate_user_list_validation(app)
    
    # 产品列表验证
    await demonstrate_product_list_validation(app)
    
    logger.info("Batch validation demonstration completed")

async def demonstrate_user_list_validation(app: DMSCApp):
    """演示用户列表验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "users": {
            "type": "list",
            "required": True,
            "min_length": 1,
            "max_length": 100,
            "items": {
                "type": "dict",
                "schema": {
                    "username": {
                        "type": "string",
                        "required": True,
                        "min_length": 3,
                        "max_length": 20
                    },
                    "email": {
                        "type": "email",
                        "required": True
                    },
                    "age": {
                        "type": "integer",
                        "required": True,
                        "min": 18,
                        "max": 120
                    }
                }
            }
        }
    }
    
    # 测试数据
    test_data = {
        "users": [
            {
                "username": "john_doe",
                "email": "john@example.com",
                "age": 25
            },
            {
                "username": "jane_smith",
                "email": "jane@example.com",
                "age": 30
            },
            {
                "username": "bob_wilson",
                "email": "bob@example.com",
                "age": 35
            }
        ]
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("User list validation passed", user_count=len(result.cleaned_data["users"]))
    else:
        logger.error("User list validation failed", errors=result.errors)

async def demonstrate_product_list_validation(app: DMSCApp):
    """演示产品列表验证"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "products": {
            "type": "list",
            "required": True,
            "items": {
                "type": "dict",
                "schema": {
                    "name": {
                        "type": "string",
                        "required": True,
                        "min_length": 1,
                        "max_length": 100
                    },
                    "price": {
                        "type": "float",
                        "required": True,
                        "min": 0.01,
                        "max": 999999.99,
                        "precision": 2
                    },
                    "stock": {
                        "type": "integer",
                        "required": True,
                        "min": 0,
                        "max": 999999
                    },
                    "category": {
                        "type": "string",
                        "required": True,
                        "allowed": ["electronics", "clothing", "books", "food"]
                    }
                }
            }
        }
    }
    
    # 测试数据
    test_data = {
        "products": [
            {
                "name": "iPhone 15",
                "price": 999.99,
                "stock": 50,
                "category": "electronics"
            },
            {
                "name": "Python Programming Book",
                "price": 49.99,
                "stock": 100,
                "category": "books"
            },
            {
                "name": "Cotton T-Shirt",
                "price": 29.99,
                "stock": 200,
                "category": "clothing"
            }
        ]
    }
    
    # 执行验证
    result = await app.validator.validate(test_data, rules)
    
    if result.is_valid:
        logger.info("Product list validation passed", product_count=len(result.cleaned_data["products"]))
    else:
        logger.error("Product list validation failed", errors=result.errors)

async def demonstrate_data_cleaning(app: DMSCApp):
    """演示数据清理"""
    logger = app.logger
    
    logger.info("=== 数据清理演示 ===")
    
    # 字符串清理
    await demonstrate_string_cleaning(app)
    
    # 数字清理
    await demonstrate_number_cleaning(app)
    
    # 日期清理
    await demonstrate_date_cleaning(app)
    
    logger.info("Data cleaning demonstration completed")

async def demonstrate_string_cleaning(app: DMSCApp):
    """演示字符串清理"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "username": {
            "type": "string",
            "required": True,
            "trim": True,  # 去除首尾空格
            "lowercase": True,  # 转换为小写
            "remove_spaces": True,  # 去除所有空格
            "max_length": 20
        },
        "title": {
            "type": "string",
            "required": True,
            "trim": True,
            "collapse_spaces": True,  # 合并连续空格
            "capitalize": True  # 首字母大写
        },
        "description": {
            "type": "string",
            "required": False,
            "trim": True,
            "remove_special_chars": True,  # 去除特殊字符
            "max_length": 500
        }
    }
    
    # 原始数据
    raw_data = {
        "username": "  John Doe  ",
        "title": "  multiple   spaces   here  ",
        "description": "Special chars: @#$%^&*()"
    }
    
    logger.info("Original data", data=raw_data)
    
    # 执行验证和清理
    result = await app.validator.validate(raw_data, rules)
    
    if result.is_valid:
        logger.info("Cleaned data", cleaned_data=result.cleaned_data)
    else:
        logger.error("Validation failed", errors=result.errors)

async def demonstrate_number_cleaning(app: DMSCApp):
    """演示数字清理"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "price": {
            "type": "float",
            "required": True,
            "precision": 2,  # 保留2位小数
            "min": 0,
            "max": 999999.99
        },
        "quantity": {
            "type": "integer",
            "required": True,
            "min": 1,
            "max": 999
        },
        "discount": {
            "type": "float",
            "required": False,
            "precision": 4,  # 保留4位小数
            "min": 0,
            "max": 1,
            "default": 0  # 默认值为0
        }
    }
    
    # 原始数据
    raw_data = {
        "price": "99.999",  # 字符串数字
        "quantity": "10",   # 字符串整数
        "discount": None    # 空值
    }
    
    logger.info("Original data", data=raw_data)
    
    # 执行验证和清理
    result = await app.validator.validate(raw_data, rules)
    
    if result.is_valid:
        logger.info("Cleaned data", cleaned_data=result.cleaned_data)
    else:
        logger.error("Validation failed", errors=result.errors)

async def demonstrate_date_cleaning(app: DMSCApp):
    """演示日期清理"""
    logger = app.logger
    
    # 定义验证规则
    rules = {
        "birth_date": {
            "type": "date",
            "required": True,
            "format": "%Y-%m-%d",
            "timezone": "Asia/Shanghai"
        },
        "appointment_time": {
            "type": "datetime",
            "required": True,
            "format": "%Y-%m-%d %H:%M:%S",
            "timezone": "Asia/Shanghai",
            "round_to_minute": True  # 四舍五入到分钟
        }
    }
    
    # 原始数据
    raw_data = {
        "birth_date": "1998/01/15",  # 不同格式
        "appointment_time": "2024-12-31 14:30:45.123456"  # 带微秒
    }
    
    logger.info("Original data", data=raw_data)
    
    # 执行验证和清理
    result = await app.validator.validate(raw_data, rules)
    
    if result.is_valid:
        logger.info("Cleaned data", cleaned_data=result.cleaned_data)
    else:
        logger.error("Validation failed", errors=result.errors)

# 自定义验证器类
class ChineseMobileValidator:
    """中国手机号验证器"""
    
    async def validate(self, value: str, **kwargs) -> ValidationResult:
        """验证中国手机号"""
        # 基础格式验证
        if not re.match(r'^1[3-9]\d{9}$', value):
            return ValidationResult(
                is_valid=False,
                errors={"chinese_mobile": "中国手机号格式错误"}
            )
        
        # 运营商检查（可选）
        if kwargs.get("carrier_check", False):
            carrier = self._detect_carrier(value)
            if carrier == "unknown":
                return ValidationResult(
                    is_valid=False,
                    errors={"carrier_check": "不支持的运营商"}
                )
            
            return ValidationResult(
                is_valid=True,
                cleaned_data=value,
                metadata={"carrier": carrier}
            )
        
        return ValidationResult(is_valid=True, cleaned_data=value)
    
    def _detect_carrier(self, mobile: str) -> str:
        """检测运营商"""
        prefixes = {
            "中国移动": ["134", "135", "136", "137", "138", "139", "147", "150", "151", "152", "157", "158", "159", "172", "178", "182", "183", "184", "187", "188", "195", "197", "198"],
            "中国联通": ["130", "131", "132", "145", "155", "156", "166", "167", "171", "175", "176", "185", "186", "196"],
            "中国电信": ["133", "149", "153", "173", "174", "177", "180", "181", "189", "190", "191", "193", "199"]
        }
        
        prefix = mobile[:3]
        for carrier, carrier_prefixes in prefixes.items():
            if prefix in carrier_prefixes:
                return carrier
        
        return "unknown"

class IDCardValidator:
    """身份证号验证器"""
    
    async def validate(self, value: str, **kwargs) -> ValidationResult:
        """验证身份证号"""
        # 基础格式验证
        if not re.match(r'^\d{17}[\dXx]$', value):
            return ValidationResult(
                is_valid=False,
                errors={"id_card": "身份证号格式错误"}
            )
        
        # 校验码验证（可选）
        if kwargs.get("verify_checksum", False):
            if not self._verify_checksum(value):
                return ValidationResult(
                    is_valid=False,
                    errors={"verify_checksum": "身份证号码校验码错误"}
                )
        
        # 信息提取（可选）
        metadata = {}
        if kwargs.get("extract_info", False):
            metadata["extracted_info"] = self._extract_info(value)
        
        return ValidationResult(
            is_valid=True,
            cleaned_data=value,
            metadata=metadata
        )
    
    def _verify_checksum(self, id_card: str) -> bool:
        """验证校验码"""
        # 简化的校验码验证
        weights = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2]
        check_codes = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2']
        
        # 这里应该实现完整的校验算法
        return True
    
    def _extract_info(self, id_card: str) -> Dict[str, Any]:
        """提取身份证信息"""
        # 简化的信息提取
        return {
            "birth_date": f"{id_card[6:10]}-{id_card[10:12]}-{id_card[12:14]}",
            "gender": "male" if int(id_card[16]) % 2 == 1 else "female",
            "region_code": id_card[:6]
        }

class BankCardValidator:
    """银行卡号验证器"""
    
    async def validate(self, value: str, **kwargs) -> ValidationResult:
        """验证银行卡号"""
        # 基础格式验证
        if not re.match(r'^\d{13,19}$', value):
            return ValidationResult(
                is_valid=False,
                errors={"bank_card": "银行卡号格式错误"}
            )
        
        # Luhn算法验证（可选）
        if kwargs.get("verify_luhn", False):
            if not self._verify_luhn(value):
                return ValidationResult(
                    is_valid=False,
                    errors={"verify_luhn": "银行卡号校验失败"}
                )
        
        # 银行检测（可选）
        metadata = {}
        if kwargs.get("detect_bank", False):
            bank_info = self._detect_bank(value)
            if bank_info["code"] == "unknown":
                return ValidationResult(
                    is_valid=False,
                    errors={"detect_bank": "无法识别的发卡行"}
                )
            metadata["bank_info"] = bank_info
        
        return ValidationResult(
            is_valid=True,
            cleaned_data=value,
            metadata=metadata
        )
    
    def _verify_luhn(self, card_number: str) -> bool:
        """Luhn算法验证"""
        def digits_of(n):
            return [int(d) for d in str(n)]
        
        digits = digits_of(card_number)
        odd_digits = digits[-1::-2]
        even_digits = digits[-2::-2]
        checksum = sum(odd_digits)
        
        for d in even_digits:
            checksum += sum(digits_of(d * 2))
        
        return checksum % 10 == 0
    
    def _detect_bank(self, card_number: str) -> Dict[str, str]:
        """检测发卡行"""
        # 简化的银行检测
        bank_prefixes = {
            "ICBC": ["622202", "622208"],
            "ABC": ["622848", "622845"],
            "BOC": ["621661", "621666"],
            "CCB": ["622700", "622280"]
        }
        
        prefix = card_number[:6]
        for bank, bank_prefixes_list in bank_prefixes.items():
            if prefix in bank_prefixes_list:
                return {"code": bank, "name": self._get_bank_name(bank)}
        
        return {"code": "unknown", "name": "未知银行"}
    
    def _get_bank_name(self, code: str) -> str:
        """获取银行名称"""
        bank_names = {
            "ICBC": "中国工商银行",
            "ABC": "中国农业银行",
            "BOC": "中国银行",
            "CCB": "中国建设银行"
        }
        return bank_names.get(code, "未知银行")

class BusinessLicenseValidator:
    """营业执照验证器"""
    
    async def validate(self, value: str, **kwargs) -> ValidationResult:
        """验证营业执照号"""
        # 基础格式验证
        if not re.match(r'^\d{15}$|^\d{18}$', value):
            return ValidationResult(
                is_valid=False,
                errors={"business_license": "营业执照号格式错误"}
            )
        
        # 校验码验证（可选）
        if kwargs.get("verify_checksum", False):
            if len(value) == 18 and not self._verify_checksum(value):
                return ValidationResult(
                    is_valid=False,
                    errors={"verify_checksum": "营业执照号校验失败"}
                )
        
        # 信息提取（可选）
        metadata = {}
        if kwargs.get("extract_info", False):
            metadata["extracted_info"] = self._extract_info(value)
        
        return ValidationResult(
            is_valid=True,
            cleaned_data=value,
            metadata=metadata
        )
    
    def _verify_checksum(self, license_number: str) -> bool:
        """验证校验码"""
        # 简化的校验码验证
        return True
    
    def _extract_info(self, license_number: str) -> Dict[str, Any]:
        """提取营业执照信息"""
        # 简化的信息提取
        return {
            "registration_code": license_number[:8],
            "organization_code": license_number[8:17],
            "check_code": license_number[17] if len(license_number) == 18 else None
        }

if __name__ == "__main__":
    asyncio.run(main())
```

### 5. 运行示例

```bash
python main.py
```

### 6. 查看结果

运行示例后，你可以：

1. **查看验证结果**：控制台会显示详细的验证结果
2. **查看错误信息**：详细的错误信息和字段路径
3. **查看清理后的数据**：验证通过后的标准化数据
4. **查看元数据**：验证过程中产生的额外信息

<div align="center">

## 最佳实践

</div>

### 验证规则设计

1. **明确验证目的**：区分业务验证和技术验证
2. **分层验证策略**：前端验证 + 后端验证 + 数据库约束
3. **验证粒度控制**：避免过度验证，保持适度
4. **错误消息友好**：提供清晰、有用的错误信息

### 性能优化

1. **异步验证**：对于外部依赖使用异步验证
2. **缓存验证结果**：对于昂贵的验证操作使用缓存
3. **批量验证**：减少数据库查询次数
4. **验证顺序优化**：先验证简单规则，再验证复杂规则

### 安全性考虑

1. **输入清理**：防止SQL注入、XSS等攻击
2. **长度限制**：防止拒绝服务攻击
3. **正则表达式安全**：避免ReDoS攻击
4. **外部服务验证**：设置超时和重试机制

### 可维护性

1. **验证规则集中管理**：便于统一修改和维护
2. **版本控制**：验证规则的版本管理
3. **文档化**：详细的验证规则文档
4. **测试覆盖**：为验证规则编写测试用例

<div align="center">

## 故障排查

</div>

### 验证失败排查

1. **检查验证规则**：确认规则定义正确
2. **检查输入数据**：确认数据格式和内容
3. **检查错误信息**：详细的错误信息定位问题
4. **检查自定义验证器**：确认自定义逻辑正确

### 性能问题排查

1. **检查外部服务**：确认外部API响应正常
2. **检查数据库连接**：确认数据库查询性能
3. **检查缓存使用**：确认缓存命中率和有效性
4. **检查验证复杂度**：避免过于复杂的验证规则

### 异步验证问题

1. **检查超时设置**：确认合理的超时时间
2. **检查并发限制**：避免过多的并发请求
3. **检查错误处理**：完善的错误处理和重试机制
4. **检查资源释放**：及时释放连接和资源

<div align="center">

## 性能优化

</div>

### 验证性能优化

1. **规则预编译**：预编译正则表达式和验证规则
2. **缓存机制**：缓存验证结果和外部查询结果
3. **批量处理**：批量验证减少数据库查询
4. **并行验证**：并行执行独立的验证规则

### 内存使用优化

1. **对象复用**：复用验证器和验证结果对象
2. **流式验证**：大文件和数据集的流式验证
3. **及时清理**：及时清理临时数据和缓存
4. **资源池化**：连接和资源的池化管理

### 扩展性优化

1. **插件架构**：支持自定义验证器插件
2. **分布式验证**：支持分布式验证架构
3. **配置驱动**：验证规则的配置化管理
4. **热更新**：支持验证规则的热更新

<div align="center">

## 相关参考

</div>

- [DMSC 验证API参考](../04-api-reference/validation.md)
- [JSON Schema 官方文档](https://json-schema.org/)
- [Cerberus 验证库文档](https://docs.python-cerberus.org/)
- [Pydantic 文档](https://docs.pydantic.dev/)
- [数据验证最佳实践](https://www.owasp.org/index.php/Data_Valuation)