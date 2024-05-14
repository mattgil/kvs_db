pub struct KvStore {}

impl KvStore {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set(&mut self, _key: String, _val: String) {
        panic!("unimplemented")
    }

    pub fn get(&mut self, _key: String) -> Option<String> {
        panic!("unimplemented")
    }

    pub fn remove(&mut self, _key: String) {
        panic!("unimplemented")
    }
}
