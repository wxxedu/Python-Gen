use crate::constants::PY_INDENT;
use crate::py_core::py_to_lines::PyToLines;
use crate::py_model::PyModel;

pub trait PyClosure {
    fn add(&mut self, child: PyModel);

    fn get_signature(&self) -> String;

    fn get_body(&self) -> &Vec<PyModel>;

    fn get_closing_lines(&self) -> Vec<String> {
        vec!["".to_string()]
    }

    fn to_lines(&self) -> Vec<String> {
        let mut res = vec![self.get_signature()];
        res.extend(
            self.get_body()
                .iter()
                .flat_map(|x| x.to_lines())
                .map(|line| format!("{}{}", PY_INDENT, line))
                .collect::<Vec<String>>(),
        );
        res.extend(self.get_closing_lines());
        res
    }
}
