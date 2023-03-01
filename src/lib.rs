pub mod py_constants;
pub mod py_core;
pub mod py_model;

pub use py_constants::*;
pub use py_core::py_closure::*;
pub use py_core::py_cond::*;
pub use py_core::py_model_core::*;
pub use py_model::py_doc::*;
pub use py_model::py_elif::*;
pub use py_model::py_for::*;
pub use py_model::py_func::*;
pub use py_model::py_if::*;
pub use py_model::py_line::*;
pub use py_model::py_while::*;

#[cfg(test)]
mod tests {
    use super::*;
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
