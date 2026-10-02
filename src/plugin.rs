use std::collections::HashMap;

pub trait Plugin {
    fn name(&self) -> &str;
    fn initialize(&mut self) {}
}

#[derive(Default)]
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        let name = plugin.name().to_string();
        self.plugins.insert(name, Box::new(plugin));
    }

    pub fn list(&self) -> Vec<&str> {
        self.plugins.keys().map(String::as_str).collect()
    }
}
