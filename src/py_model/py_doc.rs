use crate::py_core::py_to_lines::PyToLines;
use crate::py_model::PyModel;

#[derive(Copy, Clone, Debug, Default)]
pub struct PyDoc {
    name: String,
    components: Vec<PyModel>,
}

impl PyDoc {
    /// Creates a new pydoc with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name,
            components: vec![],
        }
    }

    /// Adds a component.
    pub fn add(&mut self, model: PyModel) {
        self.components.push(model);
    }
}

impl PyToLines for PyDoc {}
