use crate::py_model::PyModel::{Elif, For, Func, FuncInvoke, If, Line, While};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;

pub const PY_INDENT: &str = "    ";

pub const PY_TRUE: &str = "True";

pub const PY_FALSE: &str = "False";

pub const PY_NONE: &str = "None";

pub trait PyToLines {
    fn to_lines(&self) -> Vec<String>;
}

pub trait PyModelCore {
    fn to_model(&self) -> PyModel;
}

pub trait PyCond {
    fn set_cond(&mut self, cond: &str);
}

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
/// The enum representing a Python model
#[derive(Debug, Clone)]
pub enum PyModel {
    Line(PyLine),
    If(PyIf),
    Elif(PyElif),
    For(PyFor),
    While(PyWhile),
    Func(PyFunc),
    FuncInvoke(PyFunc),
}

impl Display for PyModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.to_lines() {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl PyModel {
    pub fn write_to_file(&self, file: &mut File) -> std::io::Result<()> {
        file.write_all(self.to_string().as_bytes())
    }
}

impl PyToLines for PyModel {
    fn to_lines(&self) -> Vec<String> {
        match self {
            Line(val) => val.to_lines(),
            If(val) => val.to_lines(),
            Elif(val) => val.to_lines(),
            For(val) => val.to_lines(),
            While(val) => val.to_lines(),
            Func(val) => val.to_lines(),
            FuncInvoke(val) => vec![val.invoke_str()],
        }
    }
}

#[derive(Debug, Clone)]
pub struct PyLine(String);

impl PyLine {
    pub fn new(line: &str) -> Self {
        Self(line.to_string())
    }
}

#[macro_export]
macro_rules! py_line {
    ($fmt:expr, $($x:expr),*) => {
        PyLine::new(&format!($fmt, $($x),*))
    }
}

impl PyToLines for PyLine {
    fn to_lines(&self) -> Vec<String> {
        vec![self.0.to_string()]
    }
}

impl PyModelCore for PyLine {
    fn to_model(&self) -> PyModel {
        Line(self.clone())
    }
}

#[derive(Debug, Clone, Default)]
pub struct PyIf {
    cond: String,
    body: Vec<PyModel>,
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

#[derive(Debug, Clone, Default)]
pub struct PyElif {
    cond: String,
    body: Vec<PyModel>,
}

impl PyClosure for PyElif {
    fn add(&mut self, child: PyModel) {
        self.body.push(child);
    }

    fn get_signature(&self) -> String {
        format!("elif {}:", self.cond)
    }

    fn get_body(&self) -> &Vec<PyModel> {
        &self.body
    }
}

impl PyModelCore for PyElif {
    fn to_model(&self) -> PyModel {
        Elif(self.clone())
    }
}

impl PyCond for PyElif {
    fn set_cond(&mut self, name: &str) {
        self.cond = name.to_string();
    }
}

#[derive(Debug, Clone, Default)]
pub struct PyFor {
    cond: String,
    body: Vec<PyModel>,
}

impl PyClosure for PyFor {
    fn add(&mut self, child: PyModel) {
        self.body.push(child);
    }

    fn get_signature(&self) -> String {
        format!("for {}:", &self.cond)
    }

    fn get_body(&self) -> &Vec<PyModel> {
        &self.body
    }
}

impl PyModelCore for PyFor {
    fn to_model(&self) -> PyModel {
        For(self.clone())
    }
}

impl PyCond for PyFor {
    fn set_cond(&mut self, name: &str) {
        self.cond = name.to_string();
    }
}

#[derive(Debug, Clone)]
pub struct PyWhile {
    cond: String,
    body: Vec<PyModel>,
}

impl PyClosure for PyWhile {
    fn add(&mut self, child: PyModel) {
        self.body.push(child);
    }

    fn get_signature(&self) -> String {
        format!("while {}:", &self.cond)
    }

    fn get_body(&self) -> &Vec<PyModel> {
        &self.body
    }
}

impl PyCond for PyWhile {
    fn set_cond(&mut self, name: &str) {
        self.cond = name.to_string();
    }
}

impl PyModelCore for PyWhile {
    fn to_model(&self) -> PyModel {
        While(self.clone())
    }
}

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

    fn invoke_str(&self) -> String {
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
        res.push("".to_string());
        res
    }
}

impl PyModelCore for PyFunc {
    fn to_model(&self) -> PyModel {
        Func(self.clone())
    }
}
