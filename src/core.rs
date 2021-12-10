use crate::client::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
// use std::thread::sleep;
// use std::time::Duration;

#[derive(Debug)]
pub struct BitMap {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug)]
pub struct Inner {
    values: HashMap<String, Vec<u8>>,
    clients: HashMap<usize, Client>,
}

impl Inner {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            clients: HashMap::new(),
        }
    }
}

impl BitMap {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner::new())),
        }
    }

    // 获取inner
    fn inner(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().unwrap()
    }

    // 设置bit位
    pub fn set(&mut self, key: String, offset: usize, value: u8) -> usize {
        // 获取到这个key对应的db，也就是这个key的所有字节
        let mut db = self.get_storage(&key).unwrap();

        // // 这个offset是在db中的第几个下标
        let index = self.get_index_by_offset(offset);

        // // 补齐数据
        while (offset / 8) > db.len() {
            db.push(0);
        }

        // 获取当前下标的数据
        if let None = db.get(index) {
            db.push(0);
        }

        let byte = db.get(index).unwrap().clone();

        // 1byte有8bit，下标从0 ~ 7，一共8个位置
        let bit_offset = offset % 8;
        //
        // 设置状态
        let number: u8;

        if value == 1 {
            number = byte | (1 << bit_offset);
        } else {
            number = byte & !(1 << bit_offset);
        }

        // 保存数据
        db[index] = number;

        // 保存数据
        self.inner().values.insert(key, db.clone());

        db.len()
    }

    // 获取当前offset是在第几个位置
    fn get_index_by_offset(&self, offset: usize) -> usize {
        offset / 8
    }

    // 获取这个key的所有数据
    fn get_storage(&mut self, key: &str) -> Result<Vec<u8>, anyhow::Error> {
        let mut inner = self.inner();

        if let Some(db) = inner.values.get(key) {
            Ok(db.clone())
        } else {
            // 设置一个新的数据，key不存在
            inner.values.insert(key.to_string(), Vec::new());

            Ok(inner.values.get(&key.to_string()).unwrap().clone())
        }
    }

    // 获取bit位是0还是1
    pub fn get(&self, key: String, offset: usize) -> u8 {
        let inner = self.inner();

        let db = inner.values.get(&key).unwrap();
        let index = self.get_index_by_offset(offset);

        if let Some(value) = db.get(index) {
            let bit_offset = offset % 8;
            if (1 & (value >> bit_offset)) >= 1 {
                1
            } else {
                0
            }
        } else {
            0
        }
    }
}

// 实现clone，共享数据
impl Clone for BitMap {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
