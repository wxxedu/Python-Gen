use crate::py_core::py_to_lines::PyToLines;
use crate::py_model::PyModel;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Clone, Debug, Default)]
pub struct PyDoc {
    name: String,
    components: Vec<PyModel>,
}

impl PyDoc {
    /// Creates a new pydoc with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: vec![],
        }
    }

    /// Adds a component.
    pub fn add(&mut self, model: PyModel) {
        self.components.push(model);
    }

    /// Gets the file name.
    pub fn file_name(&self) -> String {
        format!("{}.py", self.name)
    }

    /// Writes this file
    pub fn write(&self) -> io::Result<()> {
        let mut file = File::create(self.file_name())?;
        let content = self.to_lines().join("\n");
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl PyToLines for PyDoc {
    fn to_lines(&self) -> Vec<String> {
        self.components
            .iter()
            .flat_map(|x| {
                let mut lines = x.to_lines();
                lines.push("".to_string());
                lines.push("".to_string());
                lines
            })
            .collect::<Vec<String>>()
    }
}
