use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use std::marker::PhantomData;

// A simple implementation of a map in Rust
#[derive(Debug, Clone, Default)]
pub struct Map<Key, Value> {
    entries: Vec<(Key, Value)>,
}

impl<Key, Value> Map<Key, Value>
where
    Key: Eq,
{
    // Inserts a key-value pair
    pub fn insert(
        &mut self,
        key: Key,
        value: Value,
    ) {
        self.entries.push((key, value));
    }

    // Returns the length of the map
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    // Returns a value of a key in the map
    pub fn get(
        &self,
        key: &Key,
    ) -> Option<&Value> {
        self.entries
            .iter()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }
}

impl<'de, Key, Value> Deserialize<'de> for Map<Key, Value>
where
    Key: Deserialize<'de> + Eq,
    Value: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MapVisitor<Key, Value> {
            phantom: PhantomData<(Key, Value)>,
        }

        impl<'de, Key, Value> Visitor<'de> for MapVisitor<Key, Value>
        where
            Key: Deserialize<'de> + Eq,
            Value: Deserialize<'de>,
        {
            type Value = Map<Key, Value>;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("key-value pair")
            }

            fn visit_map<A>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut new_map = Map {
                    entries: Vec::new(),
                };

                while let Some((key, value)) = map.next_entry()? {
                    new_map.insert(key, value);
                }

                Ok(new_map)
            }
        }

        let visitor = MapVisitor::<Key, Value> {
            phantom: PhantomData,
        };
        deserializer.deserialize_map(visitor)
    }
}
