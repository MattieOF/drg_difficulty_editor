use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Difficulty {
    pub(crate) name: String,
    pub(crate) description: String,
}

impl Default for Difficulty
{
    fn default() -> Self {
        Self {
            name: "Custom Difficulty".to_string(),
            description: "Default Difficulty".to_string(),
        }
    }
}

impl Difficulty {
    pub fn from_name(new_name: &str) -> Self {
        let mut diff = Difficulty::default();
        diff.name = new_name.to_string();
        return diff;
    }

    pub fn with_name(&mut self, new_name: &str) -> &mut Self {
        self.name = new_name.to_string();
        return self;
    }
}
