pub use {
    array::JsonArray,
    key::JsonKey,
    object::{JsonKeyValue, JsonObject},
    value::JsonValue,
};

mod array;
mod key;
mod object;
mod value;

#[cfg(test)]
mod tests {
    use super::*;
    use splutter::Output;

    #[test]
    fn test_json_object_from_tuples() {
        let obj = (("works", true), ("value", 8), ("text", "hallo")).json_object();
        let mut buffer = String::new();
        obj.output(&mut buffer);
        assert_eq!(buffer, "{\"works\":true,\"value\":8,\"text\":\"hallo\"}");
    }

    #[test]
    fn test_json_array_from_tuple() {
        let array = ("works", 9, true).json_array();
        let mut buffer = String::new();
        array.output(&mut buffer);
        assert_eq!(buffer, "[\"works\",9,true]");
    }
}
