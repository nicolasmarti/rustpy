use pyo3::prelude::*;

//////////////

#[pyclass]
pub struct Node{
    #[pyo3(get,set)]
    pub value: i32,

    #[pyo3(get,set)]
    pub left: Option<Py<Node>>,

    #[pyo3(get,set)]
    pub right: Option<Py<Node>>,
}

#[pymethods]
impl Node{

    #[new]
    #[pyo3(signature = (value, left=None, right=None))]
    fn new(value: i32, left: Option<Py<Node>>, right: Option<Py<Node>>) -> Self {
	Node {value, left, right}
    }

    fn sum_tree(&self, py: Python<'_>) -> i32 {
	let mut total = self.value;

	if let Some(ref left_py) = self.left {
	    let left_node = left_py.borrow(py);
	    total += left_node.sum_tree(py);
	}

	if let Some(ref right_py) = self.right {
	    let right_node = right_py.borrow(py);
	    total += right_node.sum_tree(py);
	}

	total
    }
}

//////////////

#[pyclass(from_py_object)]
#[derive(Clone)]
pub enum Expression {
    Number { value: f64 },
    BinaryOp {
	op: String,
	lhs: Py<Expression>,
	rhs: Py<Expression>,
    },
}

#[pymethods]
impl Expression {
    #[staticmethod]
    fn number(value: f64) -> Self {
	Expression::Number {value}
    }

    #[staticmethod]
    fn binaryop(op: String, lhs: Py<Expression>, rhs: Py<Expression>,) -> Self {
	Expression::BinaryOp {op, lhs, rhs}
    }

    fn eval(&self, py: Python<'_>) -> PyResult<f64> {
	match self {
	    Expression::Number {value} => Ok(*value),
	    Expression::BinaryOp {op, lhs, rhs} => {
		let left_val = lhs.borrow(py).eval(py)?;
		let right_val = rhs.borrow(py).eval(py)?;
		match op.as_str() {
		    "+" => Ok(left_val + right_val),
		    "-" => Ok(left_val - right_val),
		    "*" => Ok(left_val * right_val),
		    "/" => Ok(left_val / right_val),
		    _ => Err(pyo3::exceptions::PyValueError::new_err(format!("Unknown op: {}", op))),		    
		}
		
	    }
	}
    }
}

//////////////

#[pymodule]
fn rustpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Node>()?;
    m.add_class::<Expression>()?;
    Ok(())
}
