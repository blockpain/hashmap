use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


const DEFAULT_MAX_SIZE: u64 = 256;

pub struct HashMap<T, V> {
    curr_size: usize,
    arr: [Option<KeyValue<T, V>>; DEFAULT_MAX_SIZE as usize],
}

pub struct KeyValue<T, V> {
    key: T,
    value: V,
    next: Option<Box<KeyValue<T, V>>>,
}

impl<T:Clone + Hash + PartialEq, V: Copy> HashMap<T, V> {

    fn insert_new_value(&mut self, key: T, val: V, position: usize) {
        let new_entry = KeyValue::new(key, val);
    
        self.arr[position] = Some(new_entry);
        self.curr_size += 1;
    }

    fn update_or_link_new_val(&mut self, key: T, val: V, position: usize) -> Option<V> {
        // traverse linked list until either find value (update)
        // or stick a new value on the end
    
        // can safely unwrap as we've already checked this pos exists
        let key_val = self.arr[position].as_mut().unwrap();
        if key_val.key == key {
            let old_val = key_val.value;
            key_val.value = val;
            // return the old value
            return Some(old_val);
        }
    
        let mut current = key_val;
        while current.next.is_some() {
            let node = current.next.as_mut().unwrap();
    
            if node.key == key {
                // update the value
                let old_val = node.value;
                node.value = val;
                return Some(old_val);
            }
    
            current = node;
        }
    
        // append the new value to the end of the linked list
        let new_key_val = KeyValue::new(key, val);
    
        current.next = Some(Box::new(new_key_val));
        self.curr_size += 1;
    
        None
    }

    pub fn put(&mut self, key: T, val: V) -> Option<V> {
        let hash_val: u64 = hash_key(key.clone());
        let position = hash_val % DEFAULT_MAX_SIZE;
    
        match &self.arr[position as usize] {
            Some(_) => self.update_or_link_new_val(key, val, position as usize),
            None => {
                self.insert_new_value(key, val, position as usize);
                None
            }
        }
    }
    pub fn get(&self, key: T) -> Option<V> {
        todo!()
    }
    pub fn remove(&self, key: T) -> Option<V> {
        todo!()
    }
    pub fn clear(&mut self) {
        todo!()
    }
}

impl<T, V> KeyValue<T, V> {
    pub fn new(key: T, value: V) -> KeyValue<T, V> {
        KeyValue {
            key,
            value,
            next: None,
        }
    }
}

fn hash_key<T: Hash>(key: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let hash_val = hasher.finish();
    hash_val
}
