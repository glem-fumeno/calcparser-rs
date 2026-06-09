use std::collections::HashMap;

#[derive(Default)]
pub struct VariableStore {
    max_id: usize,
    pub mapping: HashMap<String, usize>,
    inverse_mapping: HashMap<usize, String>,
}

impl VariableStore {
    pub fn register(&mut self, name: String) -> usize {
        if let Some(id) = self.mapping.get(&name) {
            return *id;
        }
        self.max_id += 1;
        self.mapping.insert(name.clone(), self.max_id);
        self.inverse_mapping.insert(self.max_id, name);
        self.max_id
    }

    pub fn get_name(&self, id: usize) -> &str {
        self.inverse_mapping
            .get(&id)
            .unwrap()
            .trim_start_matches("${")
            .trim_start_matches("#{")
            .trim_end_matches("}")
    }
}
