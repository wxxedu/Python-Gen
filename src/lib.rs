pub mod py_model;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::py_model::{
        PyClosure, PyCond, PyFunc, PyIf, PyLine, PyModelCore, PY_TRUE,
    };
    use std::fs::File;

    #[test]
    fn write_if() {
        let mut file = File::create("./generated/if.py").unwrap();

        let mut py_print = PyFunc::default();
        py_print.rename("print");
        py_print.add_param("'hello'");

        let mut iff = PyIf::default();
        iff.set_cond(PY_TRUE);
        iff.add(py_print.invoke());
        iff.add(iff.clone().to_model());
        iff.to_model().write_to_file(&mut file).unwrap();
    }
}
