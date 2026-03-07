use tinc::__private::{TincValidate, TrackerFor, TrackerSharedState, deserialize_tracker_target};

mod pb {
    #![allow(clippy::all)]
    tinc::include_proto!("message_cel");
}

#[test]
fn test_password_change_valid() {
    let mut state = TrackerSharedState::default();
    let valid = pb::PasswordChange {
        old_password: "oldpassword123".into(),
        new_password: "newpassword456".into(),
        confirm_password: "newpassword456".into(),
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
fn test_password_change_same_password() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::PasswordChange {
        old_password: "samepassword".into(),
        new_password: "samepassword".into(),
        confirm_password: "samepassword".into(),
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "new_password must be different from old_password",
                },
                fatal: true,
                path: "",
            },
        ],
    }
    "#);
}

#[test]
fn test_password_change_mismatch_confirm() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::PasswordChange {
        old_password: "oldpassword123".into(),
        new_password: "newpassword456".into(),
        confirm_password: "differentpassword".into(),
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "confirm_password must match new_password",
                },
                fatal: true,
                path: "",
            },
        ],
    }
    "#);
}

#[test]
fn test_password_change_too_short() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::PasswordChange {
        old_password: "short".into(),
        new_password: "short".into(),
        confirm_password: "short".into(),
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must be at least `8` characters long",
                },
                fatal: true,
                path: "old_password",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be at least `8` characters long",
                },
                fatal: true,
                path: "new_password",
            },
            TrackedError {
                kind: InvalidField {
                    message: "new_password must be different from old_password",
                },
                fatal: true,
                path: "",
            },
        ],
    }
    "#);
}

#[test]
fn test_date_range_valid() {
    let mut state = TrackerSharedState::default();
    let valid = pb::DateRange {
        start_year: 2024,
        start_month: 1,
        start_day: 15,
        end_year: 2024,
        end_month: 12,
        end_day: 31,
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
fn test_date_range_same_date() {
    let mut state = TrackerSharedState::default();
    let valid = pb::DateRange {
        start_year: 2024,
        start_month: 6,
        start_day: 15,
        end_year: 2024,
        end_month: 6,
        end_day: 15,
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
fn test_date_range_end_before_start() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::DateRange {
        start_year: 2024,
        start_month: 12,
        start_day: 31,
        end_year: 2024,
        end_month: 1,
        end_day: 1,
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "end_date must be after start_date",
                },
                fatal: true,
                path: "",
            },
        ],
    }
    "#);
}

#[test]
fn test_date_range_invalid_values() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::DateRange {
        start_year: 1800,
        start_month: 13,
        start_day: 32,
        end_year: 2200,
        end_month: 0,
        end_day: 0,
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must be greater than or equal to `1900`",
                },
                fatal: true,
                path: "start_year",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be less than or equal to `12`",
                },
                fatal: true,
                path: "start_month",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be less than or equal to `31`",
                },
                fatal: true,
                path: "start_day",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be less than or equal to `2100`",
                },
                fatal: true,
                path: "end_year",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be greater than or equal to `1`",
                },
                fatal: true,
                path: "end_month",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be greater than or equal to `1`",
                },
                fatal: true,
                path: "end_day",
            },
        ],
    }
    "#);
}

#[test]
fn test_conditional_required_phone() {
    let mut state = TrackerSharedState::default();
    let valid = pb::ConditionalRequired {
        contact_method: "phone".into(),
        phone_number: "+1234567890".into(),
        email: "".into(),
    };

    state.in_scope(|| valid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");

    let mut state = TrackerSharedState::default();
    let invalid = pb::ConditionalRequired {
        contact_method: "phone".into(),
        phone_number: "".into(),
        email: "".into(),
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "phone_number is required when contact_method is 'phone'",
                },
                fatal: true,
                path: "",
            },
        ],
    }
    "#);
}

#[test]
fn test_conditional_required_email() {
    let mut state = TrackerSharedState::default();
    let valid = pb::ConditionalRequired {
        contact_method: "email".into(),
        phone_number: "".into(),
        email: "test@example.com".into(),
    };

    state.in_scope(|| valid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");

    let mut state = TrackerSharedState::default();
    let invalid = pb::ConditionalRequired {
        contact_method: "email".into(),
        phone_number: "".into(),
        email: "".into(),
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "email is required when contact_method is 'email'",
                },
                fatal: true,
                path: "",
            },
        ],
    }
    "#);
}

#[test]
fn test_conditional_required_mail() {
    let mut state = TrackerSharedState::default();
    let valid = pb::ConditionalRequired {
        contact_method: "mail".into(),
        phone_number: "".into(),
        email: "".into(),
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
fn test_at_least_one_of_valid() {
    let mut state = TrackerSharedState::default();
    let valid = pb::AtLeastOneOf {
        username: "testuser".into(),
        email: "".into(),
        phone: "".into(),
    };

    state.in_scope(|| valid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");

    let mut state = TrackerSharedState::default();
    let valid_all = pb::AtLeastOneOf {
        username: "testuser".into(),
        email: "test@example.com".into(),
        phone: "+1234567890".into(),
    };

    state.in_scope(|| valid_all.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");
}

#[test]
fn test_at_least_one_of_invalid() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::AtLeastOneOf {
        username: "".into(),
        email: "".into(),
        phone: "".into(),
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "at least one of username, email, or phone must be provided",
                },
                fatal: true,
                path: "",
            },
        ],
    }
    "#);
}

#[test]
fn test_mutually_exclusive_valid() {
    let mut state = TrackerSharedState::default();
    let valid = pb::MutuallyExclusive {
        credit_card: "4111111111111111".into(),
        bank_account: "".into(),
        paypal: "".into(),
    };

    state.in_scope(|| valid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");

    let mut state = TrackerSharedState::default();
    let valid_none = pb::MutuallyExclusive {
        credit_card: "".into(),
        bank_account: "".into(),
        paypal: "".into(),
    };

    state.in_scope(|| valid_none.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");
}

#[test]
fn test_mutually_exclusive_invalid() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::MutuallyExclusive {
        credit_card: "4111111111111111".into(),
        bank_account: "123456789".into(),
        paypal: "".into(),
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "only one of credit_card, bank_account, or paypal can be provided",
                },
                fatal: true,
                path: "",
            },
        ],
    }
    "#);

    let mut state = TrackerSharedState::default();
    let invalid_all = pb::MutuallyExclusive {
        credit_card: "4111111111111111".into(),
        bank_account: "123456789".into(),
        paypal: "user@paypal.com".into(),
    };

    state.in_scope(|| invalid_all.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "only one of credit_card, bank_account, or paypal can be provided",
                },
                fatal: true,
                path: "",
            },
        ],
    }
    "#);
}

#[test]
fn test_password_change_deserialize() {
    let mut message = pb::PasswordChange::default();
    let mut tracker = <pb::PasswordChange as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "old_password": "oldpassword123",
        "new_password": "newpassword456",
        "confirm_password": "newpassword456"
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        TincValidate::validate(&message, Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");
    insta::assert_debug_snapshot!(message, @r#"
    PasswordChange {
        old_password: "oldpassword123",
        new_password: "newpassword456",
        confirm_password: "newpassword456",
    }
    "#);
}

#[test]
fn test_at_least_one_of_serialization() {
    let message = pb::AtLeastOneOf {
        username: "testuser".into(),
        email: "test@example.com".into(),
        phone: "".into(),
    };

    insta::assert_json_snapshot!(message, @r#"
    {
      "username": "testuser",
      "email": "test@example.com",
      "phone": ""
    }
    "#);
}
