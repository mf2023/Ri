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

"""
DMS (Dunimd Middleware Service) - A high-performance Rust middleware framework with modular architecture.

This Python library provides bindings to the DMS Rust core, allowing you to leverage DMS functionality
in Python applications.
"""

__version__ = "0.1.0"
__author__ = "Dunimd Team"
__license__ = "Apache-2.0"

# Import the Rust extension
from .dms_core import new_app_builder

# Submodules are created in the Rust bindings
# They can be imported as from dms_core.log import ...

# Expose submodules for direct access
__all__ = ['new_app_builder', 'log', 'config', 'device', 'cache', 'fs', 'hooks', 'observability']
