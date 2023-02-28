pub mod py_model;

pub mod constants;
pub mod py_core;

#[cfg(test)]
mod tests {
    use crate::constants::PY_TRUE;
    use crate::py_core::py_closure::PyClosure;
    use crate::py_core::py_cond::PyCond;
    use crate::py_core::py_model_core::PyModelCore;
    use crate::py_model::py_func::PyFunc;
    use crate::py_model::py_if::PyIf;
    use std::fs::File;

    #[test]
    fn write_if() {
        let mut file = File::create("./generated/if.py").unwrap();

        let mut py_print = PyFunc::default();
        py_print.set_name("print");
        py_print.add_param("'hello'");

        let mut iff = PyIf::default();
        iff.set_cond(PY_TRUE);
        iff.add(py_print.invoke());
        iff.add(iff.clone().to_model());
        iff.to_model().write_to_file(&mut file).unwrap();
    }
}
