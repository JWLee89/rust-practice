#[derive(Debug)]
pub struct Student {
    id: u32,
    pub age: u8,
    pub name: String,
}

impl Student {
    pub fn new(std_name: String) -> Result<Self, String> {
        if std_name.chars().all(|c| matches!(c, 'a'..='z')) {
            Ok(Student {
                id: 0,
                age: 20,
                name: std_name,
            })
        } else {
            Err("Name should only contain lowercase alphabets".to_string())
        }
    }
}

impl Default for Student {
    fn default() -> Self {
        Student {
            id: 0,
            age: 20,
            name: "John Doe".to_string(),
        }
    }
}