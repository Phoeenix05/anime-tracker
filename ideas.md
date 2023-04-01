```rust

lazy_static::lazy_static! {
    pub static Mutex<HashMap<String, Box<dyn Api + Send + Sync>>> 
}

fn get_api(name: String) -> Option<&Box<dyn Api + Send + Sync>> {
    // Check if hashmap has a implementation with the given name.
    // If it has then return the implementation, if not return `None`
}

```