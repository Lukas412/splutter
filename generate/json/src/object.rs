use crate::key::JsonKey;
use crate::value::JsonValue;
use splutter::{Output, Separated};

pub trait JsonObject {
    fn json_object(self) -> impl Output;
}

impl<T> JsonObject for (T,)
where
    T: JsonKeyValue,
{
    fn json_object(self) -> impl Output {
        ('{', self.0.json_key_value(), '}')
    }
}

impl<T1, T2> JsonObject for (T1, T2)
where
    T1: JsonKeyValue,
    T2: JsonKeyValue,
{
    fn json_object(self) -> impl Output {
        let json_key_values = (self.0.json_key_value(), self.1.json_key_value());
        ('{', json_key_values.separated(','), '}')
    }
}

impl<T1, T2, T3> JsonObject for (T1, T2, T3)
where
    T1: JsonKeyValue,
    T2: JsonKeyValue,
    T3: JsonKeyValue,
{
    fn json_object(self) -> impl Output {
        let json_key_values = (
            self.0.json_key_value(),
            self.1.json_key_value(),
            self.2.json_key_value(),
        );
        ('{', json_key_values.separated(','), '}')
    }
}

impl<T1, T2, T3, T4> JsonObject for (T1, T2, T3, T4)
where
    T1: JsonKeyValue,
    T2: JsonKeyValue,
    T3: JsonKeyValue,
    T4: JsonKeyValue,
{
    fn json_object(self) -> impl Output {
        let json_key_values = (
            self.0.json_key_value(),
            self.1.json_key_value(),
            self.2.json_key_value(),
            self.3.json_key_value(),
        );
        ('{', json_key_values.separated(','), '}')
    }
}

impl<T1, T2, T3, T4, T5> JsonObject for (T1, T2, T3, T4, T5)
where
    T1: JsonKeyValue,
    T2: JsonKeyValue,
    T3: JsonKeyValue,
    T4: JsonKeyValue,
    T5: JsonKeyValue,
{
    fn json_object(self) -> impl Output {
        let json_key_values = (
            self.0.json_key_value(),
            self.1.json_key_value(),
            self.2.json_key_value(),
            self.3.json_key_value(),
            self.4.json_key_value(),
        );
        ('{', json_key_values.separated(','), '}')
    }
}

pub trait JsonKeyValue {
    fn json_key_value(self) -> impl Output;
}

impl<K, V> JsonKeyValue for (K, V)
where
    K: JsonKey,
    V: JsonValue,
{
    fn json_key_value(self) -> impl Output {
        (self.0.json_key(), ':', self.1.json_value())
    }
}
