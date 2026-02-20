// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package com.dunimd.dmsc;

/**
 * DMSC error exception.
 * 
 * This exception is thrown when a DMSC operation fails.
 */
public class DMSCError extends RuntimeException {
    private static final long serialVersionUID = 1L;
    
    private String errorCode;
    
    /**
     * Create a new DMSCError with a message.
     * 
     * @param message the error message
     */
    public DMSCError(String message) {
        super(message);
    }
    
    /**
     * Create a new DMSCError with a message and cause.
     * 
     * @param message the error message
     * @param cause the cause of the error
     */
    public DMSCError(String message, Throwable cause) {
        super(message, cause);
    }
    
    /**
     * Create a new DMSCError with a message and error code.
     * 
     * @param message the error message
     * @param errorCode the error code
     */
    public DMSCError(String message, String errorCode) {
        super(message);
        this.errorCode = errorCode;
    }
    
    /**
     * Get the error code.
     * 
     * @return the error code, or null if not set
     */
    public String getErrorCode() {
        return errorCode;
    }
}
