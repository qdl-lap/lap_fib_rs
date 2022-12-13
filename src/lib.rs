use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod fib_calcs;

use fib_calcs::fib_number::fibonacci_number;
use fib_calcs::fib_numbers::fibonacci_numbers;

#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust!");
}

#[pymodule]
fn lap_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    // match m.add_wrapped(wrap_pyfunction!(say_hello)) {
    //     Err(e) => println!("{:?}", e),
    //     _ => ()
    // }
    m.add_wrapped(wrap_pyfunction!(say_hello)).unwrap();
    m.add_wrapped(wrap_pyfunction!(fibonacci_number)).unwrap();
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers)).unwrap();
    Ok(())
}
