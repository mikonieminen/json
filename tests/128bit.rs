use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct Data {
    value: u128,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum UntaggedMessage {
    TestMessage(Data),
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", content = "msg")]
enum TaggedMessage {
    TestMessage(Data)
}

#[test]
fn deserialize_tagged_message_containing_128bit_value() {
    let json = String::from(r#"{ "type": "TestMessage", "msg": { "value": 1000000000000000000000000 } }"#);
    if let TaggedMessage::TestMessage(Data { value }) = serde_json::from_str(&json).unwrap() {
        assert_eq!(1000000000000000000000000, value)
    } else {
        panic!("fail")
    }
}

#[test]
fn deserialize_untagged_message_containing_128bit_value() {
    let json = String::from(r#"{ "value": 1000000000000000000000000 }"#);
    if let UntaggedMessage::TestMessage(Data { value }) = serde_json::from_str(&json).unwrap() {
        assert_eq!(1000000000000000000000000, value)
    } else {
        panic!("fail")
    }
}
