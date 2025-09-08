use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use crate::mcp::types::{JsonRpcMessage, MessageContent, error_codes};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub name: String,       // User name
    pub email: Option<String>,
    pub roles: Vec<String>, // User roles/permissions
    pub exp: u64,          // Expiration time
    pub iat: u64,          // Issued at
    pub iss: String,       // Issuer
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub roles: Vec<String>,
    pub api_key: Option<String>,
    pub created_at: u64,
    pub last_login: Option<u64>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub user_id: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub last_used: Option<u64>,
    pub is_active: bool,
}

pub struct AuthManager {
    jwt_secret: String,
    api_keys: HashMap<String, ApiKey>,
    users: HashMap<String, User>,
    sessions: HashMap<String, SessionInfo>,
    require_auth: bool,
}

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub session_id: String,
    pub user_id: String,
    pub created_at: u64,
    pub last_activity: u64,
    pub expires_at: u64,
    pub permissions: Vec<String>,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "fitness-advisor-ai-secret-key".to_string()),
            api_keys: HashMap::new(),
            users: HashMap::new(),
            sessions: HashMap::new(),
            require_auth: false, // Default to no auth required for development
        }
    }

    pub fn with_auth_required(mut self, required: bool) -> Self {
        self.require_auth = required;
        self
    }

    pub fn with_jwt_secret(mut self, secret: String) -> Self {
        self.jwt_secret = secret;
        self
    }

    // Create a new user
    pub fn create_user(&mut self, name: String, email: Option<String>, roles: Vec<String>) -> Result<User> {
        let user_id = Uuid::new_v4().to_string();
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        
        let user = User {
            id: user_id.clone(),
            name,
            email,
            roles,
            api_key: None,
            created_at: now,
            last_login: None,
            is_active: true,
        };

        self.users.insert(user_id.clone(), user.clone());
        Ok(user)
    }

    // Generate API key for a user
    pub fn generate_api_key(&mut self, user_id: &str, name: String, permissions: Vec<String>, expires_in_days: Option<u64>) -> Result<ApiKey> {
        if !self.users.contains_key(user_id) {
            return Err(anyhow!("User not found"));
        }

        let api_key = format!("fai_{}", Uuid::new_v4().to_simple());
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let expires_at = expires_in_days.map(|days| now + (days * 24 * 60 * 60));

        let key = ApiKey {
            key: api_key.clone(),
            user_id: user_id.to_string(),
            name,
            permissions,
            created_at: now,
            expires_at,
            last_used: None,
            is_active: true,
        };

        self.api_keys.insert(api_key.clone(), key.clone());

        // Update user with API key
        if let Some(user) = self.users.get_mut(user_id) {
            user.api_key = Some(api_key);
        }

        Ok(key)
    }

    // Generate JWT token
    pub fn generate_jwt(&self, user_id: &str) -> Result<String> {
        let user = self.users.get(user_id)
            .ok_or_else(|| anyhow!("User not found"))?;

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let exp = now + (24 * 60 * 60); // 24 hours

        let claims = Claims {
            sub: user.id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            roles: user.roles.clone(),
            exp,
            iat: now,
            iss: "fitness-advisor-ai".to_string(),
        };

        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(self.jwt_secret.as_ref());
        
        encode(&header, &claims, &encoding_key)
            .map_err(|e| anyhow!("Failed to generate JWT: {}", e))
    }

    // Validate JWT token
    pub fn validate_jwt(&self, token: &str) -> Result<Claims> {
        let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_ref());
        let validation = Validation::new(Algorithm::HS256);

        decode::<Claims>(token, &decoding_key, &validation)
            .map(|token_data| token_data.claims)
            .map_err(|e| anyhow!("Invalid JWT: {}", e))
    }

    // Validate API key
    pub fn validate_api_key(&mut self, api_key: &str) -> Result<&User> {
        let key = self.api_keys.get_mut(api_key)
            .ok_or_else(|| anyhow!("Invalid API key"))?;

        if !key.is_active {
            return Err(anyhow!("API key is disabled"));
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        // Check expiration
        if let Some(expires_at) = key.expires_at {
            if now > expires_at {
                return Err(anyhow!("API key has expired"));
            }
        }

        // Update last used
        key.last_used = Some(now);

        // Get user
        let user = self.users.get(&key.user_id)
            .ok_or_else(|| anyhow!("User not found for API key"))?;

        if !user.is_active {
            return Err(anyhow!("User account is disabled"));
        }

        Ok(user)
    }

    // Create session
    pub fn create_session(&mut self, user_id: &str, permissions: Vec<String>, duration_hours: u64) -> Result<SessionInfo> {
        if !self.users.contains_key(user_id) {
            return Err(anyhow!("User not found"));
        }

        let session_id = Uuid::new_v4().to_string();
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let expires_at = now + (duration_hours * 60 * 60);

        let session = SessionInfo {
            session_id: session_id.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            last_activity: now,
            expires_at,
            permissions,
        };

        self.sessions.insert(session_id.clone(), session.clone());
        Ok(session)
    }

    // Validate session
    pub fn validate_session(&mut self, session_id: &str) -> Result<&SessionInfo> {
        let session = self.sessions.get_mut(session_id)
            .ok_or_else(|| anyhow!("Invalid session"))?;

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        // Check expiration
        if now > session.expires_at {
            self.sessions.remove(session_id);
            return Err(anyhow!("Session has expired"));
        }

        // Update last activity
        session.last_activity = now;

        Ok(session)
    }

    // Main authentication method for MCP messages
    pub async fn authenticate(&mut self, message: &JsonRpcMessage) -> Result<Option<String>> {
        if !self.require_auth {
            return Ok(None); // No authentication required
        }

        // Extract authentication information from the message
        let auth_info = self.extract_auth_from_message(message)?;

        match auth_info {
            AuthInfo::ApiKey(api_key) => {
                let user = self.validate_api_key(&api_key)?;
                Ok(Some(user.id.clone()))
            }
            AuthInfo::JWT(token) => {
                let claims = self.validate_jwt(&token)?;
                
                // Check if user still exists and is active
                let user = self.users.get(&claims.sub)
                    .ok_or_else(|| anyhow!("User not found"))?;
                
                if !user.is_active {
                    return Err(anyhow!("User account is disabled"));
                }

                Ok(Some(claims.sub))
            }
            AuthInfo::Session(session_id) => {
                let session = self.validate_session(&session_id)?;
                Ok(Some(session.user_id.clone()))
            }
            AuthInfo::None => {
                Err(anyhow!("Authentication required but not provided"))
            }
        }
    }

    fn extract_auth_from_message(&self, message: &JsonRpcMessage) -> Result<AuthInfo> {
        // Check for authentication in different places:
        // 1. Custom headers (if supported by transport)
        // 2. Parameters in initialize request
        // 3. Special auth parameters in requests

        match &message.content {
            MessageContent::Request(request) => {
                if let Some(params) = &request.params {
                    // Check for auth in params
                    if let Some(auth) = params.get("auth") {
                        if let Some(api_key) = auth.get("api_key").and_then(|v| v.as_str()) {
                            return Ok(AuthInfo::ApiKey(api_key.to_string()));
                        }
                        if let Some(token) = auth.get("jwt_token").and_then(|v| v.as_str()) {
                            return Ok(AuthInfo::JWT(token.to_string()));
                        }
                        if let Some(session_id) = auth.get("session_id").and_then(|v| v.as_str()) {
                            return Ok(AuthInfo::Session(session_id.to_string()));
                        }
                    }

                    // Check for API key in top-level params
                    if let Some(api_key) = params.get("api_key").and_then(|v| v.as_str()) {
                        return Ok(AuthInfo::ApiKey(api_key.to_string()));
                    }
                }
            }
            _ => {}
        }

        Ok(AuthInfo::None)
    }

    // Check if user has permission
    pub fn check_permission(&self, user_id: &str, permission: &str) -> Result<bool> {
        let user = self.users.get(user_id)
            .ok_or_else(|| anyhow!("User not found"))?;

        // Check user roles for permission
        if user.roles.contains(&"admin".to_string()) {
            return Ok(true); // Admin has all permissions
        }

        // Check specific permission
        let permission_granted = match permission {
            "tools:call" => user.roles.contains(&"user".to_string()),
            "resources:read" => user.roles.contains(&"user".to_string()),
            "prompts:get" => user.roles.contains(&"user".to_string()),
            "admin:*" => user.roles.contains(&"admin".to_string()),
            _ => false,
        };

        Ok(permission_granted)
    }

    // Cleanup expired sessions and keys
    pub fn cleanup_expired(&mut self) -> Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        // Remove expired sessions
        self.sessions.retain(|_, session| session.expires_at > now);

        // Mark expired API keys as inactive
        for key in self.api_keys.values_mut() {
            if let Some(expires_at) = key.expires_at {
                if now > expires_at {
                    key.is_active = false;
                }
            }
        }

        Ok(())
    }

    // Get user by ID
    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    // List all users (admin only)
    pub fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    // Disable user
    pub fn disable_user(&mut self, user_id: &str) -> Result<()> {
        let user = self.users.get_mut(user_id)
            .ok_or_else(|| anyhow!("User not found"))?;
        
        user.is_active = false;
        
        // Disable all user's API keys
        for key in self.api_keys.values_mut() {
            if key.user_id == user_id {
                key.is_active = false;
            }
        }

        // Remove all user's sessions
        self.sessions.retain(|_, session| session.user_id != user_id);

        Ok(())
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum AuthInfo {
    ApiKey(String),
    JWT(String),
    Session(String),
    None,
}

// Helper function to create default users for development
impl AuthManager {
    pub fn create_default_users(&mut self) -> Result<()> {
        // Create admin user
        let admin = self.create_user(
            "Admin User".to_string(),
            Some("admin@fitness-advisor.ai".to_string()),
            vec!["admin".to_string(), "user".to_string()],
        )?;

        // Generate API key for admin
        let _admin_key = self.generate_api_key(
            &admin.id,
            "Admin API Key".to_string(),
            vec!["*".to_string()],
            Some(365), // 1 year
        )?;

        // Create regular user
        let user = self.create_user(
            "Test User".to_string(),
            Some("user@fitness-advisor.ai".to_string()),
            vec!["user".to_string()],
        )?;

        // Generate API key for user
        let _user_key = self.generate_api_key(
            &user.id,
            "User API Key".to_string(),
            vec!["tools:call".to_string(), "resources:read".to_string(), "prompts:get".to_string()],
            Some(90), // 90 days
        )?;

        Ok(())
    }
}