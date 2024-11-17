use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(text_signature = "(word, tossi)")]
/// The transform function takes two strings (word, tossi) as input and returns a tuple containing the word 
/// and tossi transformed according to Korean grammar rules.
pub fn transform(word: &str, tossi: &str) -> PyResult<(String, String)> {
    match ::tossicat::transform(word, tossi) {
        Ok(result) => Ok(result),
        Err(error) => {
            Err(PyValueError::new_err(format!("{}",error)))
        }
    }
}

#[pyfunction]
#[pyo3(text_signature = "(word, tossi)")]
/// The postfix function takes two strings (word, tossi) as input and returns the appropriate tossi.
pub fn postfix(word: &str, tossi: &str) -> PyResult<String> {
    match ::tossicat::postfix(word, tossi) {
        Ok(result) => Ok(result),
        Err(error) => {
            Err(PyValueError::new_err(format!("{}",error)))
        }
    }
}

#[pyfunction]
#[pyo3(text_signature = "(sentence)")]
/// The modify_sentence function takes a sentence containing {word, tossi} templates as input 
/// and returns a modified sentence with the templates properly adjusted.
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
