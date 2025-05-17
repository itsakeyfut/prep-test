use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct HashTable {
    size: usize,
    table: Vec<Vec<(String, String)>>,
}

impl HashTable {
    fn new(size: usize) -> Self {
        let table = vec![Vec::new(); size];
        Self { size, table }
    }

    fn hash(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize)  % self.size
    }

    fn add(&mut self, key: &str, value: &str) {
        let idx = self.hash(key);

        for pair in &mut self.table[idx] {
            if pair.0 == key {
                pair.1 = value.to_string();
                return;
            }
        }

        self.table[idx].push((key.to_string(), value.to_string()));
    }

    fn get(&self, key: &str) -> Option<&str> {
        let idx = self.hash(key);

        for pair in &self.table[idx] {
            if pair.0 == key {
                return Some(&pair.1);
            }
        }
        None
    }

    fn print(&self) {
        for (idx, bucket) in self.table.iter().enumerate() {
            print!("{} ", idx);
            for pair in bucket {
                print!("--> {:?}", pair);
            }
            println!();
        }
    }

    fn remove(&mut self, key: &str) -> bool {
        let idx = self.hash(key);
        let bucket = &mut self.table[idx];

        if let Some(pos) = bucket.iter().position(|(k, _)| k == key) {
            bucket.remove(pos);
            true
        } else {
            false
        }
    }

    fn resize(&mut self, new_size: usize) {
        let mut new_table = vec![Vec::new(); new_size];
        for bucket in &self.table {
            for (k, v) in bucket {
                let mut hasher = DefaultHasher::new();
                k.hash(&mut hasher);
                let idx = (hasher.finish() as usize) % new_size;
                new_table[idx].push((k.clone(), v.clone()));
            }
        }

        self.size = new_size;
        self.table = new_table;
    }
}

fn main() {
    let mut ht = HashTable::new(5);

    // [[], [], [], [], [], [("router", "IODATA")], [], [], [], []]
    ht.add("router", "IODATA");
    println!("{:?}", ht.table);

    // [[], [], [], [], [], [("router", "NTT")], [], [], [], []]
    ht.add("router", "NTT");
    println!("{:?}", ht.table);

    // [[], [], [], [], [], [("router", "NTT")], [], [], [], [("mouse", "Logicool")]]
    ht.add("mouse", "Logicool");
    println!("{:?}", ht.table);

    // [[], [], [("GPU", "NVIDIA")], [], [], [("router", "NTT")], [], [], [], [("mouse", "Logicool")]]
    ht.add("GPU", "NVIDIA");
    println!("{:?}", ht.table);


    /*
     * 0
     * 1
     * 2 --> ("GPU", "NVIDIA")
     * 3
     * 4
     * 5 --> ("router", "NTT")
     * 6
     * 7
     * 8
     * 9 --> ("mouse", "Logicool")
     */
    ht.print();

    println!("{:?}", ht.get("GPU")); // Some("NVIDIA")
    println!("{:?}", ht.get("memory")); // None

    println!("{}", ht.remove("GPU"));
    println!("{}", ht.remove("TPU"));

    ht.resize(3);
    ht.print();
}
