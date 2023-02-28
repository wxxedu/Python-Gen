use crate::py_core::py_closure::PyClosure;
use crate::py_core::py_cond::PyCond;
use crate::py_core::py_model_core::PyModelCore;
use crate::py_model::PyModel;
use crate::py_model::PyModel::While;

#[derive(Debug, Clone)]
pub struct PyWhile {
    cond: String,
    body: Vec<PyModel>,
}

impl PyClosure for PyWhile {
    fn add(&mut self, child: PyModel) {
        self.body.push(child);
    }

    fn get_signature(&self) -> String {
        format!("while {}:", &self.cond)
    }

    fn get_body(&self) -> &Vec<PyModel> {
        &self.body
    }
}

impl PyCond for PyWhile {
    fn set_cond(&mut self, name: &str) {
        self.cond = name.to_string();
    }
}

impl PyModelCore for PyWhile {
    fn to_model(&self) -> PyModel {
        While(self.clone())
    }
}
