use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust!");
}


#[pymodule]
fn lap_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    match m.add_wrapped(wrap_pyfunction!(say_hello)) {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
    Ok(())
}

