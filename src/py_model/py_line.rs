use crate::py_core::py_model_core::PyModelCore;
use crate::py_core::py_to_lines::PyToLines;
use crate::py_model::PyModel;
use crate::py_model::PyModel::Line;

#[derive(Debug, Clone)]
pub struct PyLine(String);

impl PyLine {
    pub fn new(line: &str) -> Self {
        Self(line.to_string())
    }
}

impl PyToLines for PyLine {
    fn to_lines(&self) -> Vec<String> {
        vec![self.0.to_string()]
    }
}

impl PyModelCore for PyLine {
    fn to_model(&self) -> PyModel {
        Line(self.clone())
    }
}

#[macro_export]
macro_rules! py_line {
    ($fmt:expr, $($x:expr),*) => {
        PyLine::new(&format!($fmt, $($x),*))
    }
}
