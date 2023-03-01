use crate::py_model::PyModel;
use crate::{PyClosure, PyFunc, PyModelCore};

#[derive(Debug, Clone, Default)]
pub struct PyClass {
    ident: String,
    init: Option<PyFunc>,
    body: Vec<PyFunc>,
}

impl PyClass {}

impl PyClosure for PyClass {
    fn add(&mut self, child: PyModel) {
        match child {
            PyModel::Func(val) => {
                self.body.push(val);
            }
            _ => {
                println!("Ignored insert: {} because you can only insert a function into a class.", child)
            }
        }
    }

    fn get_signature(&self) -> String {
        format!("class {}:", self.ident)
    }

    fn get_body(&self) -> &Vec<PyModel> {
        let clone = self.clone();
        let mut res = match clone.init {
            None => vec![],
            Some(val) => vec![val.to_model()],
        };
        res.extend(
            clone
                .body
                .iter()
                .map(|x| x.to_model())
                .collect::<Vec<PyModel>>(),
        );
        &res
    }
}
