use tinc::__private::{TrackerFor, TrackerSharedState, TincValidate, deserialize_tracker_target};

mod pb {
    #![allow(clippy::all)]
    tinc::include_proto!("optional_fields");
}

#[test]
fn test_optional_primitives_all_some() {
    let mut message = pb::OptionalPrimitives::default();
    let mut tracker = <pb::OptionalPrimitives as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "opt_string": "hello",
        "opt_int32": 42,
        "opt_int64": 100,
        "opt_uint32": 200,
        "opt_uint64": 300,
        "opt_float": 0.5,
        "opt_double": 1.5,
        "opt_bool": true,
        "opt_bytes": "aGVsbG8="
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
    insta::assert_json_snapshot!(message);
}

#[test]
fn test_optional_primitives_all_none() {
    let mut message = pb::OptionalPrimitives::default();
    let mut tracker = <pb::OptionalPrimitives as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(r#"{}"#);

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
    insta::assert_json_snapshot!(message);
}

#[test]
fn test_optional_primitives_invalid_string() {
    let mut message = pb::OptionalPrimitives::default();
    let mut tracker = <pb::OptionalPrimitives as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "opt_string": ""
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        let _ = message.validate(Some(&tracker));
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_optional_primitives_invalid_int32() {
    let mut message = pb::OptionalPrimitives::default();
    let mut tracker = <pb::OptionalPrimitives as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "opt_int32": -5
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        let _ = message.validate(Some(&tracker));
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_optional_primitives_invalid_float() {
    let mut message = pb::OptionalPrimitives::default();
    let mut tracker = <pb::OptionalPrimitives as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "opt_float": 1.5
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        let _ = message.validate(Some(&tracker));
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_wrapper_types_all_set() {
    let mut message = pb::WrapperTypes::default();
    let mut tracker = <pb::WrapperTypes as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "string_value": "wrapped",
        "int32_value": 42,
        "int64_value": 100,
        "uint32_value": 200,
        "uint64_value": 300,
        "float_value": 3.14,
        "double_value": 2.718,
        "bool_value": true,
        "bytes_value": "d29ybGQ="
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
    insta::assert_json_snapshot!(message);
}

#[test]
fn test_wrapper_types_all_omitted() {
    let mut message = pb::WrapperTypes::default();
    let mut tracker = <pb::WrapperTypes as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(r#"{}"#);

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
    insta::assert_json_snapshot!(message);
}

#[test]
fn test_json_omittable_required_field_missing() {
    let mut message = pb::JsonOmittableOptions::default();
    let mut tracker = <pb::JsonOmittableOptions as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(r#"{}"#);

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        let _ = message.validate(Some(&tracker));
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_json_omittable_all_provided() {
    let mut message = pb::JsonOmittableOptions::default();
    let mut tracker = <pb::JsonOmittableOptions as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "required_field": "required",
        "omittable_true": "value1",
        "omittable_false": "value2",
        "omittable_true_but_serialize": "value3",
        "optional_default": "value4",
        "optional_omittable_true": "value5"
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
    insta::assert_json_snapshot!(message);
}

#[test]
fn test_json_omittable_serialization() {
    let message = pb::JsonOmittableOptions {
        required_field: "required".to_string(),
        omittable_true: String::new(),
        omittable_false: String::new(),
        omittable_true_but_serialize: String::new(),
        optional_default: None,
        optional_omittable_true: None,
    };

    insta::assert_json_snapshot!(message);
}

#[test]
fn test_nested_optional_with_inner() {
    let mut message = pb::NestedOptional::default();
    let mut tracker = <pb::NestedOptional as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "inner": {
            "name": "test",
            "value": 42
        }
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
    insta::assert_json_snapshot!(message);
}

#[test]
fn test_nested_optional_inner_invalid() {
    let mut message = pb::NestedOptional::default();
    let mut tracker = <pb::NestedOptional as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "inner": {
            "name": ""
        }
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        let _ = message.validate(Some(&tracker));
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_nested_optional_repeated() {
    let mut message = pb::NestedOptional::default();
    let mut tracker = <pb::NestedOptional as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "items": [
            {"name": "first"},
            {"name": "second", "value": 10}
        ]
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
    insta::assert_json_snapshot!(message);
}

#[test]
fn test_nested_optional_map() {
    let mut message = pb::NestedOptional::default();
    let mut tracker = <pb::NestedOptional as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "map_items": {
            "key1": {"name": "value1"},
            "key2": {"name": "value2", "value": 99}
        }
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
    insta::assert_json_snapshot!(message);
}

#[test]
fn test_optional_with_constraints_valid_email() {
    let mut message = pb::OptionalWithConstraints::default();
    let mut tracker = <pb::OptionalWithConstraints as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "email": "test@example.com"
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_optional_with_constraints_invalid_email() {
    let mut message = pb::OptionalWithConstraints::default();
    let mut tracker = <pb::OptionalWithConstraints as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "email": "not-an-email"
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        let _ = message.validate(Some(&tracker));
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_optional_with_constraints_valid_uuid() {
    let mut message = pb::OptionalWithConstraints::default();
    let mut tracker = <pb::OptionalWithConstraints as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "uuid": "550e8400-e29b-41d4-a716-446655440000"
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_optional_with_constraints_invalid_uuid() {
    let mut message = pb::OptionalWithConstraints::default();
    let mut tracker = <pb::OptionalWithConstraints as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "uuid": "not-a-uuid"
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        let _ = message.validate(Some(&tracker));
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_optional_with_constraints_valid_ipv4() {
    let mut message = pb::OptionalWithConstraints::default();
    let mut tracker = <pb::OptionalWithConstraints as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "ipv4": "192.168.1.1"
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_optional_with_constraints_invalid_ipv4() {
    let mut message = pb::OptionalWithConstraints::default();
    let mut tracker = <pb::OptionalWithConstraints as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "ipv4": "999.999.999.999"
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        let _ = message.validate(Some(&tracker));
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_optional_with_constraints_percentage_valid() {
    let mut message = pb::OptionalWithConstraints::default();
    let mut tracker = <pb::OptionalWithConstraints as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "percentage": 50.5
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        message.validate(Some(&tracker)).unwrap();
    });

    insta::assert_debug_snapshot!(state);
}

#[test]
fn test_optional_with_constraints_percentage_invalid() {
    let mut message = pb::OptionalWithConstraints::default();
    let mut tracker = <pb::OptionalWithConstraints as TrackerFor>::Tracker::default();
    let mut state = TrackerSharedState::default();

    let mut de = serde_json::Deserializer::from_str(
        r#"{
        "percentage": 150.0
    }"#,
    );

    deserialize_tracker_target(&mut state, &mut de, &mut tracker, &mut message).unwrap();
    state.in_scope(|| {
        let _ = message.validate(Some(&tracker));
    });

    insta::assert_debug_snapshot!(state);
}
