use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
pub fn transform(word: &str, tossi: &str) -> PyResult<(String, String)> {
    match ::tossicat::transform(word, tossi) {
        Ok(result) => Ok(result),
        Err(error) => {
            Err(PyValueError::new_err(format!("{}",error)))
        }
    }
}

#[pyfunction]
pub fn postfix(word: &str, tossi: &str) -> PyResult<String> {
    match ::tossicat::postfix(word, tossi) {
        Ok(result) => Ok(result),
        Err(error) => {
            Err(PyValueError::new_err(format!("{}",error)))
        }
    }
}

#[pyfunction]
pub fn modify_sentence(sentence: &str) -> PyResult<String> {
    match ::tossicat::modify_sentence(sentence) {
        Ok(result) => Ok(result),
        Err(error) => Err(PyValueError::new_err(format!("{}", error))),
    }
}

#[pymodule]
fn tossicat(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(transform, m)?)?;
    m.add_function(wrap_pyfunction!(postfix, m)?)?;
    m.add_function(wrap_pyfunction!(modify_sentence, m)?)?;
    Ok(())
}
