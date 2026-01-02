<div align="center">

# Basic Application Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to build a basic DMSC Python web application with configuration, logging, and HTTP services.

## Example Overview

We will create a simple user management API with the following features:
- User registration
- User login
- Get user information
- User list

## Complete Code Example

```python
from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCHTTPConfig, DMSCConfig, DMSCError
)
import asyncio
from datetime import datetime
from typing import Dict, List, Optional

# Simulated user database
users_db = {}
sessions = {}

# User data model
class User:
    def __init__(self, username: str, email: str, password: str):
        self.username = username
        self.email = email
        self.password = password  # In production, should be encrypted
        self.created_at = datetime.now()
        self.last_login = None
    
    def to_dict(self) -> Dict:
        return {
            "username": self.username,
            "email": self.email,
            "created_at": self.created_at.isoformat(),
            "last_login": self.last_login.isoformat() if self.last_login else None
        }

# User service
class UserService:
    def __init__(self, context: DMSCServiceContext):
        self.context = context
        self.logger = context.logger
    
    def register(self, username: str, email: str, password: str) -> Dict:
        if username in users_db:
            raise DMSCError("Username already exists", "USERNAME_EXISTS")
        
        user = User(username, email, password)
        users_db[username] = user
        
        self.logger.info("user_service", f"User registered: {username}")
        return user.to_dict()
    
    def login(self, username: str, password: str) -> Optional[str]:
        if username not in users_db:
            raise DMSCError("User not found", "USER_NOT_FOUND")
        
        user = users_db[username]
        if user.password != password:
            raise DMSCError("Invalid password", "INVALID_PASSWORD")
        
        user.last_login = datetime.now()
        session_id = f"session_{username}_{datetime.now().timestamp()}"
        sessions[session_id] = username
        
        self.logger.info("user_service", f"User logged in: {username}")
        return session_id
    
    def get_user(self, username: str) -> Optional[Dict]:
        if username not in users_db:
            return None
        
        return users_db[username].to_dict()
    
    def list_users(self) -> List[Dict]:
        return [user.to_dict() for user in users_db.values()]

# HTTP request handler
async def handle_request(context: DMSCServiceContext):
    request = context.http.request
    path = request.path
    method = request.method
    
    user_service = UserService(context)
    
    try:
        if path == "/users" and method == "POST":
            # Register user
            data = await request.json()
            user = user_service._register(
                data["username"],
                data["email"],
                data["password"]
            )
            return {"status": "success", "data": user}
        
        elif path == "/login" and method == "POST":
            # User login
            data = await request.json()
            session_id = user_service.login(
                data["username"],
                data["password"]
            )
            return {"status": "success", "session_id": session_id}
        
        elif path.startswith("/users/") and method == "GET":
            # Get user information
            username = path.split("/")[-1]
            user = user_service.get_user(username)
            if user:
                return {"status": "success", "data": user}
            else:
                return {"status": "error", "message": "User not found"}, 404
        
        elif path == "/users" and method == "GET":
            # List all users
            users = user_service.list_users()
            return {"status": "success", "data": users}
        
        else:
            return {"status": "error", "message": "Not found"}, 404
    
    except DMSCError as e:
        return {"status": "error", "message": e.message}, 400
    except Exception as e:
        context.logger.error("handler", f"Error: {e}")
        return {"status": "error", "message": "Internal server error"}, 500

# Main application
async def main():
    # Create application builder
    app = DMSCAppBuilder()
    
    # Configure logging
    app.with_logging(DMSCLogConfig(
        level="INFO",
        format="json"
    ))
    
    # Configure HTTP
    app.with_http(DMSCHTTPConfig(
        host="0.0.0.0",
        port=8080,
        workers=4
    ))
    
    # Build application
    dms_app = app.build()
    
    # Add routes
    dms_app.router.add_route("POST", "/users", handle_request)
    dms_app.router.add_route("POST", "/login", handle_request)
    dms_app.router.add_route("GET", "/users/{username}", handle_request)
    dms_app.router.add_route("GET", "/users", handle_request)
    
    # Run application
    await dms_app.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## Code Analysis

### Application Structure

The example demonstrates a typical DMSC Python application structure:

1. **Application Builder**: Uses `DMSCAppBuilder` to configure and build the application
2. **Service Context**: Provides access to all DMSC features (logger, HTTP, etc.)
3. **Service Layer**: Implements business logic in separate service classes
4. **HTTP Handlers**: Process incoming HTTP requests

### Key Components

- **DMSCAppBuilder**: Configures and builds the application
- **DMSCServiceContext**: Provides access to framework features
- **DMSCLogConfig**: Configures logging behavior
- **DMSCHTTPConfig**: Configures HTTP server settings
- **DMSCError**: Structured error handling

## Running Steps

1. Save the code to `app.py`
2. Install DMSC Python:
   ```bash
   pip install dmsc
   ```
3. Run the application:
   ```bash
   python app.py
   ```
4. Test the API endpoints:

   ```bash
   # Register a user
   curl -X POST http://localhost:8080/users \
     -H "Content-Type: application/json" \
     -d '{"username": "john", "email": "john@example.com", "password": "password123"}'
   
   # User login
   curl -X POST http://localhost:8080/login \
     -H "Content-Type: application/json" \
     -d '{"username": "john", "password": "password123"}'
   
   # Get user information
   curl http://localhost:8080/users/john
   
   # List all users
   curl http://localhost:8080/users
   ```

## Expected Output

### Register User Response

```json
{
  "status": "success",
  "data": {
    "username": "john",
    "email": "john@example.com",
    "created_at": "2024-01-15T10:30:00",
    "last_login": null
  }
}
```

### Login Response

```json
{
  "status": "success",
  "session_id": "session_john_1705313400.123"
}
```

### User Information Response

```json
{
  "status": "success",
  "data": {
    "username": "john",
    "email": "john@example.com",
    "created_at": "2024-01-15T10:30:00",
    "last_login": "2024-01-15T10:31:00"
  }
}
```

### User List Response

```json
{
  "status": "success",
  "data": [
    {
      "username": "john",
      "email": "john@example.com",
      "created_at": "2024-01-15T10:30:00",
      "last_login": "2024-01-15T10:31:00"
    }
  ]
}
```

## Console Output

```
[2024-01-15T10:30:00] [INFO] [user_service] User registered: john
[2024-01-15T10:31:00] [INFO] [user_service] User logged in: john
```
