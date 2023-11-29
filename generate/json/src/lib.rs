pub use {
    key::JsonKey,
    object::{JsonKeyValue, JsonObject},
    value::JsonValue,
};

mod key;
mod object;
mod value;

#[cfg(test)]
mod tests {
    use super::*;
    use splutter::Output;

    #[test]
    fn test_json_object_from_tuples() {
        let obj = (("value", 8), ("text", "hallo")).json_object();
        let mut buffer = String::new();
        obj.output(&mut buffer);
        assert_eq!(buffer, "{\"value\":8,\"text\":\"hallo\"}")
    }
}
