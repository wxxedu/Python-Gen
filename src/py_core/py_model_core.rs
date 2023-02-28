use crate::py_model::PyModel;

pub trait PyModelCore {
    fn to_model(&self) -> PyModel;
}
