use super::*;

#[test]
fn test_parse_object() {
    let s = r#"{"a":{"b":"c"}}"#;
    let o = object! {
        "a" => object! {
            "b" => "c"
        }
    };
    assert_eq!(o, parse_value(s, false));
}

#[test]
fn test_parse_str() {
    let s = "a";
    assert_eq!(JsonValue::String(s.to_owned()), parse_value(s, false));
}

#[test]
fn test_parse_true() {
    let s = "true";
    assert_eq!(JsonValue::Boolean(true), parse_value(s, false));
}

#[test]
fn test_parse_number() {
    let s = "123";
    assert_eq!(JsonValue::Number(123.into()), parse_value(s, false));
}

#[test]
fn test_do_object_with_no_arguments() {
    let args = [];

    let result = do_object(&args, false);
    let expected = object! {};
    assert_eq!(expected, result.unwrap());
}

#[test]
fn test_do_object() {
    let args = [
        "a=b".to_string(),
        "b=true".to_string(),
        "c=1".to_string(),
        "d=-1".to_string(),
    ];

    let result = do_object(&args, false);
    let expected = object! {
        "a" => "b",
        "b" => true,
        "c" => 1,
        "d" => -1,
    };
    assert_eq!(expected, result.unwrap());
}

#[test]
fn test_do_object_with_warning() {
    let args = [
        "a=b".to_string(),
        "b=true".to_string(),
        "c=1".to_string(),
        "d=-1".to_string(),
        "d".to_string(),
        "=d".to_string(),
    ];

    let result = do_object(&args, false);
    let expected = object! {
        "a" => "b",
        "b" => true,
        "c" => 1,
        "d" => -1,
    };
    assert_eq!(expected, result.unwrap());
}

#[test]
fn test_do_array() {
    let args = [
        "b".to_string(),
        "true".to_string(),
        "1".to_string(),
        "-1".to_string(),
    ];

    let result = do_array(&args, false);
    let expected = array!["b", true, 1, -1];
    assert_eq!(expected, result.unwrap());
}

#[test]
fn test_array() {
    let args = vec![crate_name!(), "-a", "b", "true", "1", "-1"];
    let matches = get_app().get_matches_from(args);
    let config = configure(&matches);

    assert_eq!(true, run(config).unwrap());
}

#[test]
fn test_object() {
    let args = vec![crate_name!(), "a=b", "b=true", "c=1", "d=-1"];
    let matches = get_app().get_matches_from(args);
    let config = configure(&matches);

    assert_eq!(true, run(config).unwrap());
}

#[test]
fn test_object_with_pretty_print() {
    let args = vec![crate_name!(), "-p", "a=b", "b=true", "c=1", "d=-1"];
    let matches = get_app().get_matches_from(args);
    let config = configure(&matches);

    assert_eq!(true, run(config).unwrap());
}

#[test]
fn test_disable_boolean() {
    let args = [
        "b".to_string(),
        "true".to_string(),
        "false".to_string(),
        "1".to_string(),
        "-1".to_string(),
    ];
    let disable_boolean = true;

    let result = do_array(&args, disable_boolean);
    let expected = array!["b", "true", "false", 1, -1];
    assert_eq!(expected, result.unwrap());
}

#[test]
fn test_disable_boolean_run() {
    let args = vec![crate_name!(), "-a", "-B", "b", "true", "1", "-1"];
    let matches = get_app().get_matches_from(args);
    let config = configure(&matches);

    run(config).unwrap();
}
