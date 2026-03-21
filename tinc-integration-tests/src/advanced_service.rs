use http_body_util::BodyExt;
use tinc::__private::{TincValidate, TrackerSharedState};
use tinc::TincService;
use tower::Service;

mod pb {
    #![allow(clippy::all)]
    tinc::include_proto!("advanced_service");
}

struct Svc {}

#[tonic::async_trait]
impl pb::advanced_service_server::AdvancedService for Svc {
    async fn create_user(
        &self,
        request: tonic::Request<pb::CreateUserRequest>,
    ) -> tonic::Result<tonic::Response<pb::CreateUserResponse>> {
        let req = request.get_ref();
        Ok(pb::CreateUserResponse {
            user_id: "550e8400-e29b-41d4-a716-446655440000".into(),
            username: req.username.clone(),
            email: req.email.clone(),
        }
        .into())
    }

    async fn get_user(
        &self,
        request: tonic::Request<pb::GetUserRequest>,
    ) -> tonic::Result<tonic::Response<pb::GetUserResponse>> {
        Ok(pb::GetUserResponse {
            user_id: request.get_ref().user_id.clone(),
            username: "testuser".into(),
            email: "test@example.com".into(),
            display_name: Some("Test User".into()),
        }
        .into())
    }

    async fn update_user(
        &self,
        request: tonic::Request<pb::UpdateUserRequest>,
    ) -> tonic::Result<tonic::Response<pb::UpdateUserResponse>> {
        let req = request.get_ref();
        Ok(pb::UpdateUserResponse {
            user_id: req.user_id.clone(),
            username: "testuser".into(),
            email: req
                .email
                .clone()
                .unwrap_or_else(|| "test@example.com".into()),
            display_name: req.display_name.clone(),
        }
        .into())
    }

    async fn delete_user(
        &self,
        _request: tonic::Request<pb::DeleteUserRequest>,
    ) -> tonic::Result<tonic::Response<pb::DeleteUserResponse>> {
        Ok(pb::DeleteUserResponse { success: true }.into())
    }

    async fn list_users(
        &self,
        request: tonic::Request<pb::ListUsersRequest>,
    ) -> tonic::Result<tonic::Response<pb::ListUsersResponse>> {
        let req = request.get_ref();
        Ok(pb::ListUsersResponse {
            users: vec![pb::GetUserResponse {
                user_id: "550e8400-e29b-41d4-a716-446655440000".into(),
                username: "testuser".into(),
                email: "test@example.com".into(),
                display_name: None,
            }],
            total_count: 1,
            page: req.page.unwrap_or(1),
            page_size: req.page_size.unwrap_or(10),
        }
        .into())
    }

    async fn upload_avatar(
        &self,
        _request: tonic::Request<pb::UploadAvatarRequest>,
    ) -> tonic::Result<tonic::Response<pb::UploadAvatarResponse>> {
        Ok(pb::UploadAvatarResponse {
            avatar_url: "https://example.com/avatars/user.png".into(),
        }
        .into())
    }

    async fn download_avatar(
        &self,
        _request: tonic::Request<pb::DownloadAvatarRequest>,
    ) -> tonic::Result<tonic::Response<pb::DownloadAvatarResponse>> {
        Ok(pb::DownloadAvatarResponse {
            data: vec![0x89, 0x50, 0x4E, 0x47],
            content_type: "image/png".into(),
        }
        .into())
    }
}

#[test]
fn test_create_user_request_valid() {
    let mut state = TrackerSharedState::default();
    let valid = pb::CreateUserRequest {
        username: "john_doe".into(),
        email: "john@example.com".into(),
        display_name: Some("John Doe".into()),
    };

    state.in_scope(|| valid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");
}

#[test]
fn test_create_user_request_invalid() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::CreateUserRequest {
        username: "J".into(),
        email: "not-an-email".into(),
        display_name: Some("A".repeat(60)),
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must be at least `3` characters long",
                },
                fatal: true,
                path: "username",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must match the pattern `^[a-z][a-z0-9_]*$`",
                },
                fatal: true,
                path: "username",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be a valid email address",
                },
                fatal: true,
                path: "email",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be at most `50` characters long",
                },
                fatal: true,
                path: "display_name",
            },
        ],
    }
    "#);
}

#[test]
fn test_create_user_username_pattern() {
    let mut state = TrackerSharedState::default();

    let invalid_start_number = pb::CreateUserRequest {
        username: "1user".into(),
        email: "test@example.com".into(),
        display_name: None,
    };
    state
        .in_scope(|| invalid_start_number.validate(None))
        .unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must match the pattern `^[a-z][a-z0-9_]*$`",
                },
                fatal: true,
                path: "username",
            },
        ],
    }
    "#);

    let mut state = TrackerSharedState::default();
    let invalid_uppercase = pb::CreateUserRequest {
        username: "UserName".into(),
        email: "test@example.com".into(),
        display_name: None,
    };
    state.in_scope(|| invalid_uppercase.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must match the pattern `^[a-z][a-z0-9_]*$`",
                },
                fatal: true,
                path: "username",
            },
        ],
    }
    "#);
}

#[test]
fn test_get_user_request_uuid_validation() {
    let mut state = TrackerSharedState::default();
    let valid = pb::GetUserRequest {
        user_id: "550e8400-e29b-41d4-a716-446655440000".into(),
    };

    state.in_scope(|| valid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");

    let mut state = TrackerSharedState::default();
    let invalid = pb::GetUserRequest {
        user_id: "not-a-uuid".into(),
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must be a valid uuid",
                },
                fatal: true,
                path: "user_id",
            },
        ],
    }
    "#);
}

#[test]
fn test_list_users_request_pagination() {
    let mut state = TrackerSharedState::default();
    let valid = pb::ListUsersRequest {
        page: Some(1),
        page_size: Some(50),
        filter: Some("active".into()),
    };

    state.in_scope(|| valid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");

    let mut state = TrackerSharedState::default();
    let invalid = pb::ListUsersRequest {
        page: Some(0),
        page_size: Some(200),
        filter: None,
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must be greater than or equal to `1`",
                },
                fatal: true,
                path: "page",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be less than or equal to `100`",
                },
                fatal: true,
                path: "page_size",
            },
        ],
    }
    "#);
}

#[test]
fn test_upload_avatar_request_validation() {
    let mut state = TrackerSharedState::default();
    let valid = pb::UploadAvatarRequest {
        user_id: "550e8400-e29b-41d4-a716-446655440000".into(),
        data: vec![0x89, 0x50, 0x4E, 0x47],
        content_type: "image/png".into(),
    };

    state.in_scope(|| valid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");

    let mut state = TrackerSharedState::default();
    let invalid = pb::UploadAvatarRequest {
        user_id: "invalid".into(),
        data: vec![],
        content_type: "text/plain".into(),
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must be a valid uuid",
                },
                fatal: true,
                path: "user_id",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be at least `1` bytes long",
                },
                fatal: true,
                path: "data",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be one of `[image/png, image/jpeg, image/gif, image/webp]`",
                },
                fatal: true,
                path: "content_type",
            },
        ],
    }
    "#);
}

#[tokio::test]
async fn test_advanced_service_create_user() {
    let mut client = pb::advanced_service_tinc::AdvancedServiceTinc::new(Svc {}).into_router();

    let req = http::Request::builder()
        .uri("/api/v1/users")
        .method("POST")
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(http_body_util::Full::new(bytes::Bytes::from_static(
            r#"{"username": "testuser", "email": "test@example.com"}"#.as_bytes(),
        )))
        .unwrap();

    let resp = client.call(req).await.unwrap();
    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["username"], "testuser");
    assert_eq!(response["email"], "test@example.com");
}

#[tokio::test]
async fn test_advanced_service_get_user() {
    let mut client = pb::advanced_service_tinc::AdvancedServiceTinc::new(Svc {}).into_router();

    let req = http::Request::builder()
        .uri("/api/v1/users/550e8400-e29b-41d4-a716-446655440000")
        .method("GET")
        .body(http_body_util::Empty::<bytes::Bytes>::new())
        .unwrap();

    let resp = client.call(req).await.unwrap();
    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["user_id"], "550e8400-e29b-41d4-a716-446655440000");
    assert_eq!(response["username"], "testuser");
}

#[tokio::test]
async fn test_advanced_service_list_users_query() {
    let mut client = pb::advanced_service_tinc::AdvancedServiceTinc::new(Svc {}).into_router();

    let req = http::Request::builder()
        .uri("/api/v1/users?page=1&page_size=20")
        .method("GET")
        .body(http_body_util::Empty::<bytes::Bytes>::new())
        .unwrap();

    let resp = client.call(req).await.unwrap();
    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["page"], 1);
    assert_eq!(response["page_size"], 20);
}

#[tokio::test]
async fn test_advanced_service_delete_user() {
    let mut client = pb::advanced_service_tinc::AdvancedServiceTinc::new(Svc {}).into_router();

    let req = http::Request::builder()
        .uri("/api/v1/users/550e8400-e29b-41d4-a716-446655440000")
        .method("DELETE")
        .body(http_body_util::Empty::<bytes::Bytes>::new())
        .unwrap();

    let resp = client.call(req).await.unwrap();
    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["success"], true);
}

#[test]
fn test_advanced_service_openapi_schema() {
    let svc = pb::advanced_service_tinc::AdvancedServiceTinc::new(Svc {});

    insta::assert_json_snapshot!(svc.openapi_schema());
}
