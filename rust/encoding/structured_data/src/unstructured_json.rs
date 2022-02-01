use serde_json::{json, Error, Value};

pub fn serialize_and_deserialize_unstructured_json() -> Result<(), Error> {
    let j = r#"{
                    "userid": 103609,
                    "verified": true,
                    "access_privileges": [
                        "user",
                        "admin"
                    ]
    }"#;
    let parsed: Value = serde_json::from_str(j)?;
    let expected = json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });
    assert_eq!(parsed, expected);
    Ok(())
}
