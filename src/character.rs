pub struct CharacterCounter {
    pub character: String,
    pub count: i64
}

impl CharacterCounter {
    pub fn get_character(&self) -> String {
        self.character.clone()
    }

    pub fn get_count(&self) -> i64 {
        self.count
    }
} 