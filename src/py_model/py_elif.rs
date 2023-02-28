use crate::py_core::py_closure::PyClosure;
use crate::py_core::py_cond::PyCond;
use crate::py_core::py_model_core::PyModelCore;
use crate::py_model::PyModel;
use crate::py_model::PyModel::Elif;

#[derive(Debug, Clone, Default)]
pub struct PyElif {
    cond: String,
    body: Vec<PyModel>,
}

impl PyClosure for PyElif {
    fn add(&mut self, child: PyModel) {
        self.body.push(child);
    }

    fn get_signature(&self) -> String {
        format!("elif {}:", self.cond)
    }

    fn get_body(&self) -> &Vec<PyModel> {
        &self.body
    }
}

impl PyModelCore for PyElif {
    fn to_model(&self) -> PyModel {
        Elif(self.clone())
    }
}

impl PyCond for PyElif {
    fn set_cond(&mut self, name: &str) {
        self.cond = name.to_string();
    }
}
