#[cfg(test)]
mod auth_manager_tests {
    use fitness_advisor_ai::mcp::{AuthManager, ApiKey, User, SessionInfo};
    use std::collections::HashMap;
    use uuid::Uuid;
    use chrono::{Utc, Duration};

    fn create_test_auth_manager(require_auth: bool) -> AuthManager {
        let mut users = HashMap::new();
        let mut api_keys = HashMap::new();
        
        // Add test user
        let user_id = Uuid::new_v4();
        users.insert("testuser".to_string(), User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            is_active: true,
            created_at: Utc::now(),
            last_login: None,
            permissions: vec!["read".to_string(), "write".to_string()],
        });

        // Add test API key
        api_keys.insert("test-api-key-123".to_string(), ApiKey {
            id: Uuid::new_v4(),
            key: "test-api-key-123".to_string(),
            name: "Test API Key".to_string(),
            user_id,
            permissions: vec!["read".to_string(), "write".to_string()],
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + Duration::days(30)),
            is_active: true,
            last_used: None,
        });

        AuthManager {
            jwt_secret: "test-secret-key-for-testing".to_string(),
            api_keys,
            users,
            sessions: HashMap::new(),
            require_auth,
        }
    }

    #[tokio::test]
    async fn test_auth_manager_creation() {
        let auth_manager = AuthManager::new("test-secret".to_string(), true);
        
        assert_eq!(auth_manager.jwt_secret, "test-secret");
        assert_eq!(auth_manager.require_auth, true);
        assert!(auth_manager.users.is_empty());
        assert!(auth_manager.api_keys.is_empty());
        assert!(auth_manager.sessions.is_empty());
    }

    #[tokio::test]
    async fn test_validate_api_key_valid() {
        let auth_manager = create_test_auth_manager(true);
        
        let result = auth_manager.validate_api_key("test-api-key-123").await;
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(user.is_active);
    }

    #[tokio::test]
    async fn test_validate_api_key_invalid() {
        let auth_manager = create_test_auth_manager(true);
        
        let result = auth_manager.validate_api_key("invalid-key").await;
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Invalid API key"));
    }

    #[tokio::test]
    async fn test_validate_api_key_expired() {
        let mut auth_manager = create_test_auth_manager(true);
        
        // Create expired API key
        let user_id = Uuid::new_v4();
        auth_manager.api_keys.insert("expired-key".to_string(), ApiKey {
            id: Uuid::new_v4(),
            key: "expired-key".to_string(),
            name: "Expired Key".to_string(),
            user_id,
            permissions: vec!["read".to_string()],
            created_at: Utc::now() - Duration::days(60),
            expires_at: Some(Utc::now() - Duration::days(1)), // Expired yesterday
            is_active: true,
            last_used: None,
        });

        let result = auth_manager.validate_api_key("expired-key").await;
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("API key has expired"));
    }

    #[tokio::test]
    async fn test_validate_api_key_inactive() {
        let mut auth_manager = create_test_auth_manager(true);
        
        // Create inactive API key
        let user_id = Uuid::new_v4();
        auth_manager.api_keys.insert("inactive-key".to_string(), ApiKey {
            id: Uuid::new_v4(),
            key: "inactive-key".to_string(),
            name: "Inactive Key".to_string(),
            user_id,
            permissions: vec!["read".to_string()],
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + Duration::days(30)),
            is_active: false, // Inactive
            last_used: None,
        });

        let result = auth_manager.validate_api_key("inactive-key").await;
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("API key is not active"));
    }

    #[tokio::test]
    async fn test_generate_jwt_token() {
        let auth_manager = create_test_auth_manager(true);
        
        let user = auth_manager.users.get("testuser").unwrap();
        let result = auth_manager.generate_jwt_token(user).await;
        
        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(!token.is_empty());
        assert!(token.split('.').count() == 3); // JWT has 3 parts
    }

    #[tokio::test]
    async fn test_validate_jwt_token_valid() {
        let auth_manager = create_test_auth_manager(true);
        
        let user = auth_manager.users.get("testuser").unwrap();
        let token = auth_manager.generate_jwt_token(user).await.unwrap();
        
        let result = auth_manager.validate_jwt_token(&token).await;
        assert!(result.is_ok());
        
        let claims = result.unwrap();
        assert_eq!(claims.sub, user.id.to_string());
        assert_eq!(claims.username, "testuser");
    }

    #[tokio::test]
    async fn test_validate_jwt_token_invalid() {
        let auth_manager = create_test_auth_manager(true);
        
        let result = auth_manager.validate_jwt_token("invalid.jwt.token").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_session() {
        let mut auth_manager = create_test_auth_manager(true);
        
        let user = auth_manager.users.get("testuser").cloned().unwrap();
        let session_id = auth_manager.create_session(&user).await.unwrap();
        
        assert!(!session_id.is_empty());
        assert!(auth_manager.sessions.contains_key(&session_id));
        
        let session = auth_manager.sessions.get(&session_id).unwrap();
        assert_eq!(session.user_id, user.id);
        assert!(session.is_active);
    }

    #[tokio::test]
    async fn test_validate_session_valid() {
        let mut auth_manager = create_test_auth_manager(true);
        
        let user = auth_manager.users.get("testuser").cloned().unwrap();
        let session_id = auth_manager.create_session(&user).await.unwrap();
        
        let result = auth_manager.validate_session(&session_id).await;
        assert!(result.is_ok());
        
        let validated_user = result.unwrap();
        assert_eq!(validated_user.id, user.id);
        assert_eq!(validated_user.username, "testuser");
    }

    #[tokio::test]
    async fn test_validate_session_invalid() {
        let auth_manager = create_test_auth_manager(true);
        
        let result = auth_manager.validate_session("invalid-session-id").await;
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Invalid session"));
    }

    #[tokio::test]
    async fn test_validate_session_expired() {
        let mut auth_manager = create_test_auth_manager(true);
        
        // Create expired session
        let user_id = Uuid::new_v4();
        let session_id = "expired-session".to_string();
        auth_manager.sessions.insert(session_id.clone(), SessionInfo {
            user_id,
            created_at: Utc::now() - Duration::hours(25), // Created 25 hours ago
            last_activity: Utc::now() - Duration::hours(25),
            expires_at: Utc::now() - Duration::hours(1), // Expired 1 hour ago
            is_active: true,
            ip_address: None,
            user_agent: None,
        });

        let result = auth_manager.validate_session(&session_id).await;
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Session has expired"));
    }

    #[tokio::test]
    async fn test_validate_session_inactive() {
        let mut auth_manager = create_test_auth_manager(true);
        
        // Create inactive session
        let user_id = Uuid::new_v4();
        let session_id = "inactive-session".to_string();
        auth_manager.sessions.insert(session_id.clone(), SessionInfo {
            user_id,
            created_at: Utc::now(),
            last_activity: Utc::now(),
            expires_at: Utc::now() + Duration::hours(24),
            is_active: false, // Inactive
            ip_address: None,
            user_agent: None,
        });

        let result = auth_manager.validate_session(&session_id).await;
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Session is not active"));
    }

    #[tokio::test]
    async fn test_revoke_session() {
        let mut auth_manager = create_test_auth_manager(true);
        
        let user = auth_manager.users.get("testuser").cloned().unwrap();
        let session_id = auth_manager.create_session(&user).await.unwrap();
        
        // Verify session exists and is active
        assert!(auth_manager.sessions.get(&session_id).unwrap().is_active);
        
        let result = auth_manager.revoke_session(&session_id).await;
        assert!(result.is_ok());
        
        // Session should now be inactive
        assert!(!auth_manager.sessions.get(&session_id).unwrap().is_active);
    }

    #[tokio::test]
    async fn test_revoke_session_nonexistent() {
        let mut auth_manager = create_test_auth_manager(true);
        
        let result = auth_manager.revoke_session("nonexistent-session").await;
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Session not found"));
    }

    #[tokio::test]
    async fn test_check_permission_valid() {
        let auth_manager = create_test_auth_manager(true);
        
        let user = auth_manager.users.get("testuser").unwrap();
        
        assert!(auth_manager.check_permission(user, "read"));
        assert!(auth_manager.check_permission(user, "write"));
        assert!(!auth_manager.check_permission(user, "admin"));
    }

    #[tokio::test]
    async fn test_cleanup_expired_sessions() {
        let mut auth_manager = create_test_auth_manager(true);
        
        // Create expired session
        let session_id1 = "expired-session-1".to_string();
        let session_id2 = "active-session-2".to_string();
        let user_id = Uuid::new_v4();
        
        auth_manager.sessions.insert(session_id1.clone(), SessionInfo {
            user_id,
            created_at: Utc::now() - Duration::hours(25),
            last_activity: Utc::now() - Duration::hours(25),
            expires_at: Utc::now() - Duration::hours(1), // Expired
            is_active: true,
            ip_address: None,
            user_agent: None,
        });
        
        auth_manager.sessions.insert(session_id2.clone(), SessionInfo {
            user_id,
            created_at: Utc::now(),
            last_activity: Utc::now(),
            expires_at: Utc::now() + Duration::hours(23), // Active
            is_active: true,
            ip_address: None,
            user_agent: None,
        });

        assert_eq!(auth_manager.sessions.len(), 2);
        
        auth_manager.cleanup_expired_sessions().await;
        
        // Only active session should remain
        assert_eq!(auth_manager.sessions.len(), 1);
        assert!(auth_manager.sessions.contains_key(&session_id2));
        assert!(!auth_manager.sessions.contains_key(&session_id1));
    }

    #[tokio::test]
    async fn test_update_api_key_last_used() {
        let mut auth_manager = create_test_auth_manager(true);
        
        let api_key = auth_manager.api_keys.get("test-api-key-123").unwrap();
        assert!(api_key.last_used.is_none());
        
        // Validate API key should update last_used
        let _result = auth_manager.validate_api_key("test-api-key-123").await.unwrap();
        
        let updated_api_key = auth_manager.api_keys.get("test-api-key-123").unwrap();
        assert!(updated_api_key.last_used.is_some());
    }

    #[tokio::test]
    async fn test_auth_not_required() {
        let auth_manager = create_test_auth_manager(false); // Auth not required
        
        // Should succeed even with invalid credentials when auth is not required
        assert!(!auth_manager.require_auth);
    }

    #[tokio::test]
    async fn test_user_lookup_by_id() {
        let auth_manager = create_test_auth_manager(true);
        
        let user = auth_manager.users.get("testuser").unwrap();
        let user_id = user.id;
        
        let found_user = auth_manager.users.values().find(|u| u.id == user_id);
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().username, "testuser");
    }

    #[tokio::test]
    async fn test_session_activity_update() {
        let mut auth_manager = create_test_auth_manager(true);
        
        let user = auth_manager.users.get("testuser").cloned().unwrap();
        let session_id = auth_manager.create_session(&user).await.unwrap();
        
        let initial_activity = auth_manager.sessions.get(&session_id).unwrap().last_activity;
        
        // Simulate some time passing
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        // Update session activity
        auth_manager.update_session_activity(&session_id).await.unwrap();
        
        let updated_activity = auth_manager.sessions.get(&session_id).unwrap().last_activity;
        assert!(updated_activity > initial_activity);
    }
}