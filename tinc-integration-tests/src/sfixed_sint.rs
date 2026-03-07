use tinc::__private::{TincValidate, TrackerFor, TrackerSharedState, deserialize_tracker_target};

mod pb {
    #![allow(clippy::all)]
    tinc::include_proto!("sfixed_sint");
}

#[test]
fn test_sfixed_sint_valid() {
    let mut state = TrackerSharedState::default();
    let valid = pb::SFixedSIntExpressions {
        sfixed32_field: 50,
        sfixed64_field: 100,
        sint32_field: 5,
        sint64_field: 42,
        fixed32_field: 200,
        fixed64_field: 500,
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
fn test_sfixed_sint_invalid() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::SFixedSIntExpressions {
        sfixed32_field: 150,
        sfixed64_field: -10,
        sint32_field: 7,
        sint64_field: 0,
        fixed32_field: 50,
        fixed64_field: 2000,
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must be less than or equal to `100`",
                },
                fatal: true,
                path: "sfixed32_field",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be greater than `0`",
                },
                fatal: true,
                path: "sfixed64_field",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be one of `[-10, -5, 0, 5, 10]`",
                },
                fatal: true,
                path: "sint32_field",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must not be one of `[0]`",
                },
                fatal: true,
                path: "sint64_field",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be greater than or equal to `100`",
                },
                fatal: true,
                path: "fixed32_field",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be less than `1000`",
                },
                fatal: true,
                path: "fixed64_field",
            },
        ],
    }
    "#);
}

#[test]
fn test_sfixed_sint_boundary_values() {
    let mut state = TrackerSharedState::default();
    let boundary = pb::SFixedSIntExpressions {
        sfixed32_field: -100,
        sfixed64_field: 1,
        sint32_field: -10,
        sint64_field: 1,
        fixed32_field: 100,
        fixed64_field: 999,
    };

    state.in_scope(|| boundary.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r"
    TrackerSharedState {
        fail_fast: false,
        errors: [],
    }
    ");
}

#[test]
fn test_repeated_fixed_types_valid() {
    let mut state = TrackerSharedState::default();
    let valid = pb::RepeatedFixedTypes {
        sfixed32_list: vec![0, 5, 10],
        fixed64_list: vec![100, 200],
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
fn test_repeated_fixed_types_invalid() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::RepeatedFixedTypes {
        sfixed32_list: vec![-5, 0, 10],
        fixed64_list: vec![1, 2, 3, 4],
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must be greater than or equal to `0`",
                },
                fatal: true,
                path: "sfixed32_list[0]",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must have at most `3` elements",
                },
                fatal: true,
                path: "fixed64_list",
            },
        ],
    }
    "#);
}

#[test]
fn test_repeated_fixed_types_empty_list() {
    let mut state = TrackerSharedState::default();
    let empty = pb::RepeatedFixedTypes {
        sfixed32_list: vec![],
        fixed64_list: vec![],
    };

    state.in_scope(|| empty.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must have at least `1` elements",
                },
                fatal: true,
                path: "sfixed32_list",
            },
        ],
    }
    "#);
}

#[test]
fn test_sfixed_sint_deserialize() {
    let mut message = pb::SFixedSIntExpressions::default();
    let mut tracker = <pb::SFixedSIntExpressions as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "sfixed32_field": -50,
        "sfixed64_field": 999,
        "sint32_field": 0,
        "sint64_field": -100,
        "fixed32_field": 500,
        "fixed64_field": 100
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
    insta::assert_debug_snapshot!(message, @r"
    SFixedSIntExpressions {
        sfixed32_field: -50,
        sfixed64_field: 999,
        sint32_field: 0,
        sint64_field: -100,
        fixed32_field: 500,
        fixed64_field: 100,
    }
    ");
}

#[test]
fn test_sfixed_sint_serialization() {
    let message = pb::SFixedSIntExpressions {
        sfixed32_field: -100,
        sfixed64_field: 9223372036854775807,
        sint32_field: -10,
        sint64_field: -9223372036854775808,
        fixed32_field: 4294967295,
        fixed64_field: 999,
    };

    insta::assert_json_snapshot!(message, @r#"
    {
      "sfixed32_field": -100,
      "sfixed64_field": 9223372036854775807,
      "sint32_field": -10,
      "sint64_field": -9223372036854775808,
      "fixed32_field": 4294967295,
      "fixed64_field": 999
    }
    "#);
}
