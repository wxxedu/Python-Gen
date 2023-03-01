use crate::py_constants::PY_INDENT;
use crate::py_core::py_closure::PyClosure;
use crate::py_core::py_model_core::PyModelCore;
use crate::py_model::PyModel;
use crate::py_model::PyModel::{Func, FuncInvoke};

#[macro_export]
macro_rules! py_func {
    ($x: expr, $($p: expr), *) => {
        PyFunc::new($x, vec![$(format!("{}", $p)), *])
    };
}

#[derive(Debug, Clone, Default)]
pub struct PyFunc {
    ident: String,
    is_method: bool,
    params: Vec<String>,
    body: Vec<PyModel>,
    return_val: Option<String>,
}

impl PyFunc {
    pub fn new(ident: &str, params: Vec<String>) -> Self {
        Self {
            ident: ident.to_string(),
            is_method: true,
            params,
            body: vec![],
            return_val: None,
        }
    }

    #[deprecated(note = "this function is replaced by set_name()")]
    pub fn rename(&mut self, name: &str) {
        self.ident = name.to_string();
    }

    /// Makes this function as a method, see [PyFunc::make_func]
    pub fn make_method(&mut self) {
        self.is_method = true;
    }

    /// Makes this function as a function, see [PyFunc::make_method]
    pub fn make_func(&mut self) {
        self.is_method = false;
    }

    pub fn set_name(&mut self, name: &str) {
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
        if self.is_method {
            format!("def {}(self, {}):", self.ident, self.params.join(", "))
        } else {
            format!("def {}({}):", self.ident, self.params.join(", "))
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py_func_macro() {
        let func = py_func!("hello", "h");
        dbg!(func);
    }
}
