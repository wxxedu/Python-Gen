use crate::py_core::py_closure::PyClosure;
use crate::py_core::py_cond::PyCond;
use crate::py_core::py_model_core::PyModelCore;
use crate::py_model::PyModel;
use crate::py_model::PyModel::If;

#[derive(Debug, Clone, Default)]
pub struct PyIf {
    cond: String,
    body: Vec<PyModel>,
}

impl PyClosure for PyIf {
    fn add(&mut self, child: PyModel) {
        self.body.push(child);
    }

    fn get_signature(&self) -> String {
        format!("if {}:", self.cond)
    }

    fn get_body(&self) -> &Vec<PyModel> {
        &self.body
    }
}

impl PyModelCore for PyIf {
    fn to_model(&self) -> PyModel {
        If(self.clone())
    }
}

impl PyCond for PyIf {
    fn set_cond(&mut self, name: &str) {
        self.cond = name.to_string();
    }
}
