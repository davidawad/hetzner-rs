use hetzner_rs::client::types::{Action, GetLocationResponseLocation};

#[test]
fn action_fixture_deserializes() {
    let action: Action = serde_json::from_str(include_str!("fixtures/action.json")).unwrap();
    assert_eq!(action.id.get(), 42);
    assert_eq!(action.progress, 100);
    assert_eq!(action.resources[0].type_, "server");
}

#[test]
fn location_fixture_deserializes() {
    let location: GetLocationResponseLocation =
        serde_json::from_str(include_str!("fixtures/location.json")).unwrap();
    assert_eq!(location.id.get(), 2);
    assert_eq!(&*location.name, "nbg1");
    assert_eq!(location.country, "DE");
}
