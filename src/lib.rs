//! # TossiCat Core
//!
//! 토시를 같이 입력된 단어에 맞춰 변환해 주는 모듈
//!
//! 한글은 단어와 토시를 합칠 때 입력한 토시를 변환해서 저장하고,
//! 변환한 값을 토대로 어떤 종류인지 분류한 다음 분류한 결과를 저장합니다.
//!
//! 이 프로젝트에서 구현하고 있는 함수는 다음과 같습니다.
//!
//! - `postfix()`: 입력된 토시를 같이 입력된 단어에 맞게 변환해, 입력된 단어와 합쳐 반환하는 함수
//! - `pick()` : 입력된 토시를 같이 입력된 단어에 맞게 변환해, 변환된 토시만 반환하는 함수
//!
//! 자세한 내용은 각 함수 설명을 참고하세요.
 
mod filter;
mod hangeul;
mod identifier;
mod number;
mod transfer;
mod verifier;
mod error;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use identifier::{Tossi, TossiKind};

// hangeul 모듈
// tests/hangeul.rs 에서 test 한다.
pub fn join_phonemes(word: [char; 3]) -> char {
    hangeul::join_phonemes(word)
}

// tests/hangeul.rs 에서 test 한다.
pub fn split_phonemes(word: char) -> [char; 3] {
    hangeul::split_phonemes(word)
}

// filter 모듈
// tests/filter.rs 에서 test 한다.
pub fn find_last_letter(word: &str) -> char {
    filter::find_last_letter(word)
}

// tests/filter.rs 에 test 가 한다.
pub fn guess_final_letter(word: &str) -> char {
    filter::guess_final_letter(word)
}

// number 모듈
// tests/number.rs 에서 test 한다.
pub fn change_num_to_hangeul(word: &str) -> String {
    number::change_num_to_hangeul(word)
}

/// ## 입력된 토시를 같이 입력된 단어에 맞게 변환해, 입력된 단어와 변환한 토시를 같이 반환하는 실질적인 함수
///
/// 아래 `postfix()`, `pick()`, 두 함수 안에서 실질적인 역할을 하는 함수입니다.
/// 아래와 같은 형식으로 입력된 것 중 `tossi` 인수로 입력된 것을 어떤 토시인지 파악해서
/// 그 종류를 열거자로 변환한 다음, 토시를 붙이고자 하는 `word` 인수로 받은 단어와 함께
/// 해당 토시를 변환하는 함수로 보내서, 그 단어에 적절한 적절한 토시를 받아서
/// 해당 단어와 변환된 토시를 각각 반환합니다.
fn postfix_raw(word: &str, tossi: &str) -> (String, String) {
    //파라미터에 올바른 규격의 값이 들어왔는지 확인하기
    let temp = Tossi::new(tossi);
    let result = match temp.kind {
        TossiKind::Others => tossi.to_string(),
        _ => transfer::tossi(word, temp).to_string(),
    };

    let front = word.to_string();
    (front, result)
}
/// ## 입력된 토시를 같이 입력된 단어에 맞게 변환해, 입력된 단어와 합쳐 반환하는 함수
///
/// 아래와 같은 형식으로 입력된 것 중 두 번째로 입력된 토시를
/// 첫 번째로 입력된 단어에 맞쳐 적절하게 변형해 입력된 단어와 합쳐서
/// 반환하는 함수
///
/// ```rust
/// use tossicat::postfix;
/// postfix("집", "으로");
/// postfix("집", "로");
/// postfix("집", "(으)로");
/// ```
#[pyfunction]
pub fn postfix(word: &str, tossi: &str) -> String {
    let temp = postfix_raw(word, tossi);
    temp.0 + &temp.1
}

/// ## 입력된 토시를 같이 입력된 단어에 맞게 변환해, 변환된 토시만 반환하는 함수
///
/// `postfix()`와는 다르게 이 함수는 변환된 토시만 반환합니다.
///
/// ```rust
/// use tossicat::pick;
/// pick("집", "으로");
/// pick("집", "로");
/// pick("집", "(으)로");
/// ```
#[pyfunction]
pub fn pick(word: &str, tossi: &str) -> String {
    let temp = postfix_raw(word, tossi);
    temp.1
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
    match verifier::verifier(word, tossi) {
        Ok(()) => Ok(()),
        Err(error::ValueError::InvalidTossi) => Err(PyValueError::new_err("This value is not correct tossi.")),
        Err(error::ValueError::LimitLength) => Err(PyValueError::new_err("The length has been exceeded. Set the word length to less than 50."))
    }
}


#[pymodule]
fn tossicat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pick, m)?)?;
    m.add_function(wrap_pyfunction!(postfix, m)?)?;
    m.add_function(wrap_pyfunction!(verifiers, m)?)?;
    Ok(())
}