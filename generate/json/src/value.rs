use splutter::Output;

pub trait JsonValue {
    fn json_value(self) -> impl Output;
}

macro_rules! json_value_surround_with_quotes {
    ($type:ty) => {
        impl JsonValue for $type {
            fn json_value(self) -> impl Output {
                ('"', self, '"')
            }
        }
    };
}

json_value_surround_with_quotes!(u8);
json_value_surround_with_quotes!(i8);
json_value_surround_with_quotes!(u16);
json_value_surround_with_quotes!(i16);
json_value_surround_with_quotes!(u32);
json_value_surround_with_quotes!(i32);
json_value_surround_with_quotes!(u64);
json_value_surround_with_quotes!(i64);

impl<'a> JsonValue for &'a str {
    fn json_value(self) -> impl Output {
        JsonValueStr(self)
    }
}

pub struct JsonValueStr<'a>(&'a str);

impl<'a> Output for JsonValueStr<'a> {
    fn output(self, output: &mut String) {
        output.push('"');
        for char in self.0.chars() {
            if char == '"' {
                output.push('\\');
            }
            output.push(char);
        }
        output.push('"');
    }
}
