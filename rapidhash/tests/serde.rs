#![cfg(feature = "serde")]

use rapidhash::{
    inner::RapidSecrets as InnerRapidSecrets, v1::RapidSecrets as V1RapidSecrets,
    v2::RapidSecrets as V2RapidSecrets, v3::RapidSecrets as V3RapidSecrets,
};

fn json_round_trip<T>(value: T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + core::fmt::Debug,
{
    let serialized = serde_json::to_string(&value).expect("serialize RapidSecrets");
    let deserialized: T = serde_json::from_str(&serialized).expect("deserialize RapidSecrets");
    assert_eq!(value, deserialized);
}

#[test]
fn rapid_secrets_round_trip_through_json() {
    json_round_trip(V1RapidSecrets::seed(0x1234_5678));
    json_round_trip(V2RapidSecrets::seed(0x1234_5678));
    json_round_trip(V3RapidSecrets::seed(0x1234_5678));
    json_round_trip(InnerRapidSecrets::seed(0x1234_5678));
}
