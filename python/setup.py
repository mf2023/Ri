#!/usr/bin/env python3

# Copyright © 2025 Wenze Wei. All Rights Reserved.
#
# This file is part of DMS.
# The DMS project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="dms-core",
    version="0.1.2",
    author="Dunimd Team",
    author_email="dunimd@outlook.com",
    description="Dunimd Middleware Service - A high-performance Rust middleware framework with modular architecture",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://gitee.com/dunimd/dms",
    project_urls={
        "Bug Tracker": "https://gitee.com/dunimd/dms/issues",
        "Documentation": "https://gitee.com/dunimd/dms",
        "Source Code": "https://gitee.com/dunimd/dms",
    },
    packages=find_packages(),
    rust_extensions=[
        RustExtension(
            "dms_core.dms_core",
            path="../Cargo.toml",
            binding=Binding.PyO3,
            features=["pyo3"],
        )
    ],
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Rust",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Topic :: System :: Networking",
        "Topic :: Internet :: WWW/HTTP :: HTTP Servers",
    ],
    keywords="middleware framework distributed async microservices rust performance",
    python_requires=">=3.7",
    install_requires=[],
    extras_require={
        "dev": [
            "pytest>=6.0",
            "pytest-asyncio>=0.18.0",
            "black>=22.0",
            "isort>=5.0",
            "mypy>=0.950",
        ],
    },
    zip_safe=False,
    include_package_data=True,
)