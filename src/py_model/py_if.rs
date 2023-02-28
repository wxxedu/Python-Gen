use crate::py_core::py_closure::PyClosure;
use crate::py_core::py_cond::PyCond;
use crate::py_core::py_model_core::PyModelCore;
use crate::py_model::PyModel;
use crate::py_model::PyModel::If;

#[macro_export]
macro_rules! py_if {
    ($cond: expr) => {
        PyIf::new(&format!("{}", $cond))
    };
}

#[derive(Debug, Clone, Default)]
pub struct PyIf {
    cond: String,
    body: Vec<PyModel>,
}

impl PyIf {
    pub fn new(cond: &str) -> Self {
        Self {
            cond: cond.to_string(),
            body: vec![],
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py_if_expansion() {
        let if_ = py_if!("hello");
        dbg!(if_);
    }
}
