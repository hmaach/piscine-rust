mod err;

pub use err::{ParseErr, ReadErr};
use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let contents = fs::read_to_string(path).map_err(|e| ReadErr {
            child_err: Box::new(e),
        })?;

        let contents = json::parse(&contents).map_err(|e| ParseErr::Malformed(Box::new(e)))?;
        if contents["tasks"].is_empty() {
            return Err(ParseErr::Empty.into());
        }

        let title = contents["title"]
            .as_str()
            .ok_or_else(|| ParseErr::Malformed("Missing or invalid title".into()))?
            .to_owned();

        let tasks = contents["tasks"]
            .members()
            .map(|m| {
                Ok(Task {
                    id: m["id"]
                        .as_u32()
                        .ok_or_else(|| ParseErr::Malformed("Invalid or missing id".into()))?,
                    description: m["description"]
                        .as_str()
                        .ok_or_else(|| {
                            ParseErr::Malformed("Invalid or missing description".into())
                        })?
                        .to_owned(),
                    level: m["level"]
                        .as_u32()
                        .ok_or_else(|| ParseErr::Malformed("Invalid or missing level".into()))?,
                })
            })
            .collect::<Result<Vec<Task>, ParseErr>>()?;

        Ok(Self { title, tasks })
    }
}
