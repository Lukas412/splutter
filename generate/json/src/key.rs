use splutter::Output;

pub trait JsonKey {
    fn json_key(self) -> impl Output;
}

macro_rules! json_key_surround_with_quotes {
    ($type:ty) => {
        impl JsonKey for $type {
            fn json_key(self) -> impl Output {
                ('"', self, '"')
            }
        }
    };
}

json_key_surround_with_quotes!(u8);
json_key_surround_with_quotes!(i8);
json_key_surround_with_quotes!(u16);
json_key_surround_with_quotes!(i16);
json_key_surround_with_quotes!(u32);
json_key_surround_with_quotes!(i32);
json_key_surround_with_quotes!(u64);
json_key_surround_with_quotes!(i64);

impl<'a> JsonKey for &'a str {
    fn json_key(self) -> impl Output {
        JsonKeyStr(self)
    }
}

pub struct JsonKeyStr<'a>(&'a str);

impl<'a> Output for JsonKeyStr<'a> {
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
