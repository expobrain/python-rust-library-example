#[macro_use] extern crate cpython;

use cpython::{PyString, Python, PyResult};

// A trivial function which returns a static string to the caller
fn hello(py: Python) -> PyResult<PyString> {
    Ok(PyString::new(py, "Rust says: Hello world"))
}

// A more complete function which accept a Python string as argument and return a formatted string
fn greetings(py: Python, name: PyString) -> PyResult<PyString> {
    // Python string can be unicode or byte string on Python < 3.x
    // We need to handle the case that the Python string contains invalid Unicode code points
    match name.to_string(py) {
        Ok(name_str) => {
            // Format output string
            let greetings = format!("Rust says: Greetings {} !", name_str);

            // Convert Rust's String into Python String
            // On Python < 3.x the type will be byte string if all the characters are ASCII
            // otherwise it will be Unicode; on Python 3.x strings are always Unicode
            let greetings_py = PyString::new(py, &greetings);

            Ok(greetings_py)
        }
        Err(e) => {
            // Unicode conversion errors are returned as is
            Err(e)
        }
    }
}

// To build a Python compatible module we need an intialiser which expose the public interface
py_module_initializer!(example, initexample, PyInit_example, |py, m| {
    // Expose our two functions hello() and greetings() as `extern "C"`
    try!(m.add(py, "hello", py_fn!(py, hello())));
    try!(m.add(py, "greetings", py_fn!(py, greetings(name: PyString))));

    // Initialiser's macro needs a Result<> as return value
    Ok(())
});
