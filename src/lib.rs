pub mod py_model;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::py_model::{PyClosure, PyCond, PyIf, PyLine, PyModelCore};
    use std::fs::File;

    #[test]
    fn write_if() {
        let mut file = File::create("./generated/if.py").unwrap();
        let line = py_line!("{}", "print('hello')");
        let mut iff = PyIf::default();
        iff.set_cond("True");
        iff.add(line.to_model());
        iff.add(iff.clone().to_model());
        iff.to_model().write_to_file(&mut file).unwrap();
    }
}
