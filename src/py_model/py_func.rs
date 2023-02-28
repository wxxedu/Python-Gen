use crate::constants::PY_INDENT;
use crate::py_core::py_closure::PyClosure;
use crate::py_core::py_model_core::PyModelCore;
use crate::py_model::PyModel;
use crate::py_model::PyModel::{Func, FuncInvoke};

#[derive(Debug, Clone, Default)]
pub struct PyFunc {
    ident: String,
    params: Vec<String>,
    body: Vec<PyModel>,
    return_val: Option<String>,
}

impl PyFunc {
    pub fn rename(&mut self, name: &str) {
        self.ident = name.to_string();
    }

    pub fn set_return(&mut self, val: &str) {
        self.return_val = Some(val.to_string());
    }

    pub fn clear_return(&mut self) {
        self.return_val = None;
    }

    pub fn add_param(&mut self, param: &str) {
        self.params.push(param.to_string());
    }

    pub fn clear_params(&mut self) {
        self.params.clear();
    }

    pub fn invoke(&self) -> PyModel {
        FuncInvoke(self.clone())
    }

    pub(crate) fn invoke_str(&self) -> String {
        format!("{}({})", self.ident, self.params.join(", "))
    }
}

impl PyClosure for PyFunc {
    fn add(&mut self, child: PyModel) {
        self.body.push(child);
    }

    fn get_signature(&self) -> String {
        format!("def {}({}):", self.ident, self.params.join(", "))
    }

    fn get_body(&self) -> &Vec<PyModel> {
        &self.body
    }

    fn get_closing_lines(&self) -> Vec<String> {
        let mut res = vec![];
        match &self.return_val {
            None => {}
            Some(rtn) => res.push(format!("{}return {}", PY_INDENT, rtn)),
        }
        res
    }
}

impl PyModelCore for PyFunc {
    fn to_model(&self) -> PyModel {
        Func(self.clone())
    }
}
