use tinc::__private::{TincValidate, TrackerFor, TrackerSharedState, deserialize_tracker_target};

mod pb {
    #![allow(clippy::all)]
    tinc::include_proto!("bool_expressions");
}

#[test]
fn test_bool_expressions_valid() {
    let mut state = TrackerSharedState::default();
    let valid = pb::BoolExpressions {
        must_be_true: true,
        must_be_false: false,
        optional_bool: Some(true),
        bool_list: vec![true, false, true],
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
fn test_bool_expressions_invalid() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::BoolExpressions {
        must_be_true: false,
        must_be_false: true,
        optional_bool: None,
        bool_list: vec![],
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must be true",
                },
                fatal: true,
                path: "must_be_true",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must be false",
                },
                fatal: true,
                path: "must_be_false",
            },
            TrackedError {
                kind: InvalidField {
                    message: "value must have at least `1` elements",
                },
                fatal: true,
                path: "bool_list",
            },
        ],
    }
    "#);
}

#[test]
fn test_bool_list_too_long() {
    let mut state = TrackerSharedState::default();
    let invalid = pb::BoolExpressions {
        must_be_true: true,
        must_be_false: false,
        optional_bool: None,
        bool_list: vec![true, false, true, false, true, false],
    };

    state.in_scope(|| invalid.validate(None)).unwrap();

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: InvalidField {
                    message: "value must have at most `5` elements",
                },
                fatal: true,
                path: "bool_list",
            },
        ],
    }
    "#);
}

#[test]
fn test_bool_with_defaults_deserialize() {
    let mut message = pb::BoolWithDefaults::default();
    let mut tracker = <pb::BoolWithDefaults as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "required_bool": true
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
    BoolWithDefaults {
        enabled: false,
        disabled: false,
        required_bool: true,
    }
    ");
}

#[test]
fn test_bool_with_defaults_all_provided() {
    let mut message = pb::BoolWithDefaults::default();
    let mut tracker = <pb::BoolWithDefaults as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "enabled": true,
        "disabled": false,
        "required_bool": true
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
    BoolWithDefaults {
        enabled: true,
        disabled: false,
        required_bool: true,
    }
    ");
}

#[test]
fn test_bool_with_defaults_missing_required() {
    let mut message = pb::BoolWithDefaults::default();
    let mut tracker = <pb::BoolWithDefaults as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "enabled": true
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        TincValidate::validate(&message, Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state, @r#"
    TrackerSharedState {
        fail_fast: false,
        errors: [
            TrackedError {
                kind: MissingField,
                fatal: true,
                path: "required_bool",
            },
        ],
    }
    "#);
}

#[test]
fn test_bool_serialization() {
    let message = pb::BoolExpressions {
        must_be_true: true,
        must_be_false: false,
        optional_bool: Some(true),
        bool_list: vec![true, false],
    };

    insta::assert_json_snapshot!(message, @r#"
    {
      "must_be_true": true,
      "must_be_false": false,
      "optional_bool": true,
      "bool_list": [
        true,
        false
      ]
    }
    "#);
}

#[test]
fn test_bool_serialization_optional_none() {
    let message = pb::BoolExpressions {
        must_be_true: true,
        must_be_false: false,
        optional_bool: None,
        bool_list: vec![],
    };

    insta::assert_json_snapshot!(message, @r#"
    {
      "must_be_true": true,
      "must_be_false": false,
      "optional_bool": null,
      "bool_list": []
    }
    "#);
}
