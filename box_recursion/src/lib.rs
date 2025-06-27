#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        Self { grade: None }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let new = Worker {
            role: role,
            name: name,
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(new));
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        let mut next_worker = self.grade.take().unwrap();
        let name = next_worker.name;
        self.grade = next_worker.next.take();
        Some(name)
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        if let Some(curr) = &self.grade {
            Some((curr.name.clone(), curr.role.clone()))
        } else {
            None
        }
    }
}
