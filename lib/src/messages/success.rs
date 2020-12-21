use crate::types::*;
use neo4rs_macros::BoltStruct;

pub const MARKER: u8 = 0xB1;
pub const SIGNATURE: u8 = 0x70;

#[derive(Debug, PartialEq, Clone, BoltStruct)]
#[signature(0xB1, 0x70)]
pub struct Success {
    metadata: BoltMap,
}

impl Success {
    pub fn new(metadata: BoltMap) -> Success {
        Success { metadata }
    }
}

impl Success {
    pub fn get<T: std::convert::TryFrom<BoltType>>(&self, key: &str) -> Option<T> {
        self.metadata.get(key)
    }

    pub fn fields(&self) -> Vec<String> {
        match self.metadata.get("fields").unwrap() {
            BoltType::List(list) => list.into(),
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::convert::TryInto;
    use std::rc::Rc;

    #[test]
    fn should_deserialize_success() {
        let data = Bytes::from_static(&[
            0xB1, 0x70, 0xA2, 0x86, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x8B, 0x4E, 0x65, 0x6F,
            0x34, 0x6A, 0x2F, 0x34, 0x2E, 0x31, 0x2E, 0x34, 0x8D, 0x63, 0x6F, 0x6E, 0x6E, 0x65,
            0x63, 0x74, 0x69, 0x6F, 0x6E, 0x5F, 0x69, 0x64, 0x87, 0x62, 0x6F, 0x6C, 0x74, 0x2D,
            0x33, 0x31,
        ]);

        let success: Success = Rc::new(RefCell::new(data)).try_into().unwrap();

        assert_eq!(success.get::<String>("server").unwrap(), "Neo4j/4.1.4");
        assert_eq!(success.get::<String>("connection_id").unwrap(), "bolt-31");
    }
}
