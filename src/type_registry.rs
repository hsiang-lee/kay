use std::any::{TypeId, type_name};
use std::collections::HashMap;
use std::convert::From;
use std::num::NonZeroU16;

#[cfg_attr(feature = "serde-serialization", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct ShortTypeId(NonZeroU16);

impl ShortTypeId {
    pub fn new(id: u16) -> Option<Self> {
        NonZeroU16::new(id).map(ShortTypeId)
    }

    pub fn as_usize(&self) -> usize {
        self.0.get() as usize
    }

    pub fn as_u16(&self) -> u16 {
        self.0.get()
    }
}

impl From<ShortTypeId> for u16 {
    fn from(id: ShortTypeId) -> Self {
        id.0.get()
    }
}

pub struct TypeRegistry {
    next_short_id: ShortTypeId,
    long_to_short_ids: HashMap<TypeId, ShortTypeId>,
    pub short_ids_to_names: HashMap<ShortTypeId, String>,
}

impl TypeRegistry {
    pub fn new() -> TypeRegistry {
        TypeRegistry {
            next_short_id: ShortTypeId::new(1).unwrap(), // Non nullable optimization
            long_to_short_ids: HashMap::new(),
            short_ids_to_names: HashMap::new(),
        }
    }

    pub fn register_new<T: 'static>(&mut self) -> ShortTypeId {
        let short_id = self.next_short_id;
        let long_id = TypeId::of::<T>();
        assert!(self.long_to_short_ids.get(&long_id).is_none());
        self.long_to_short_ids.insert(long_id, short_id);
        self.short_ids_to_names
            .insert(short_id, type_name::<T>().into());
        self.next_short_id = ShortTypeId::new(u16::from(self.next_short_id) + 1).unwrap();
        short_id
    }

    pub fn get<T: 'static>(&self) -> ShortTypeId {
        if let Some(&short_id) = self.long_to_short_ids.get(&TypeId::of::<T>()) {
            short_id
        } else {
            panic!("{:?} not known.", &type_name::<T>())
        }
    }

    pub fn get_or_register<T: 'static>(&mut self) -> ShortTypeId {
        self.long_to_short_ids
            .get(&TypeId::of::<T>())
            .cloned()
            .unwrap_or_else(|| self.register_new::<T>())
    }

    pub fn get_name(&self, short_id: ShortTypeId) -> &String {
        &self.short_ids_to_names[&short_id]
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}
