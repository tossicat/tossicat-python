use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;


#[pyfunction]
pub fn postfix(word: &str, tossi: &str) -> String {
    ::tossicat::postfix(word, tossi)
}

#[pyfunction]
pub fn pick(word: &str, tossi: &str) -> String {
    ::tossicat::pick(word, tossi)
}

/// ## 변환하기 전에 입력된 것들이 변환가능한 것인지 검사하는 함수
/// 위에서부터 아래 조건 문을 순서대로 살펴 보겠다.
///
/// 1. 단어는 마지막 글자가 한글이나 숫자이면 된다.
/// 2. 토시는 한글이면 된다.
/// 3. 변환할 수 있는 토시인지 아닌지 파악한다.
/// 4. 단어의 길이가 50자를 넘으면 처리하지 않도록 처리한다.
///
/// 이 4가지를 만족하면 본 작업인 글자에 맞게 토시를 변환하게 된다.
#[pyfunction]
pub fn verifiers<'a>(word: &'a str, tossi: &'a str) -> PyResult<()> {
    match ::tossicat::verifiers(word, tossi) {
        Ok(()) => Ok(()),
        Err(error) => {
            Err(PyValueError::new_err(format!("{}",error)))
        }
    }
}

#[pymodule]
fn tossicat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pick, m)?)?;
    m.add_function(wrap_pyfunction!(postfix, m)?)?;
    m.add_function(wrap_pyfunction!(verifiers, m)?)?;
    Ok(())
}