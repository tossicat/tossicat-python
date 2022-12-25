//! # 한글을 처리하는 모듈
//!
//! ## is_hangeul
//! 해당 글자가 한글인지를 체크하는 함수.
//! ```text
//! ex) 글 -> True  
//! ```
//!
//! ## join_phonemes
//! 초,중,종성을 하나의 글자로 합쳐주는 함수.
//! ```text
//! ex) ['ㄱ','ㅡ','ㄹ'] -> '글'  
//! ex) ['ㅈ','ㅏ',' '] -> '자'  
//! ```
//!
//! ## split_phonemes
//! 한글자를 초,중,종성으로 구분하는 함수.
//! 종성이 없는 경우에는 ' '으로 치환된다.  
//! ```text
//! ex) '글' -> ['ㄱ','ㅡ','ㄹ']  
//! ex) '자' -> ['ㅈ','ㅏ',' ']  
//! ```

// 초성, 중성, 종성 배열 정의
const INITIAL: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];
const MEDIAL: [char; 21] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ',
    'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];
const FINAL: [char; 28] = [
    ' ', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ',
    'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];

/// ### 한글인지 체크하는 함수
/// 사용법은 아래 `_is_hangeul()` 참고
pub fn is_hangeul(word: char) -> bool {
    ('가'..='힣').contains(&word)
}

/// 자음인지 체크하는 함수
fn is_consonant(word: char) -> bool {
    ('ㄱ'..='ㅎ').contains(&word)
}

/// 모음인지 체크하는 함수
fn is_medial(word: char) -> bool {
    ('ㅏ'..='ㅣ').contains(&word)
}

/// ## 한글 음절인지 아닌지 체크하는 함수
/// 초,중,종성으로 들어온 것이 합치면 적절하게 한글 음절이 될 수 없는지 있는지를 판단하는 함수
/// 사용법: 이 모둘 아래 tests 모듈, _hangeul.rs 참고

fn is_hangul_syllable(word: [char; 3]) -> bool {
    if is_consonant(word[0]) && is_medial(word[1]) {
        let res = FINAL.iter().position(|&s| s == word[2]);
        res.is_some()
    } else {
        false
    }
}

/// ## 초,중,종성을 하나의 글자로 합쳐주는 함수
/// 이 함수는 기본적으로 입력된 것이 종성까지 가지고 있는다고 가정하고 작성하였다.
/// 사용하기 위해서는 종성이 없는 경우에도 다음과 같이 종성 자리에 ` `를 넣어야 한다.
/// ```rust
///    let temp = ['ㄱ', 'ㅏ', 'ㄴ'];
///    assert_eq!('간', tossicat::join_phonemes(temp));
///    let temp = ['ㄱ', 'ㅏ', ' '];
///    assert_eq!('가', tossicat::join_phonemes(temp));
/// ```
/// 사용법 tests 모듈, /tests/_is_hangul_syllable.rs 참고
pub fn join_phonemes(word: [char; 3]) -> char {
    //한글이 아닌 경우에는 입력된 첫 번째 글자 반환합니다.
    if !is_hangul_syllable(word) {
        return word[0];
    }
    // 파라미터로 받은 초,중,종성 인덱스 추출
    let idx_begin = INITIAL.iter().position(|&x| x == word[0]).unwrap();
    let idx_middle = MEDIAL.iter().position(|&x| x == word[1]).unwrap();
    let idx_end = FINAL.iter().position(|&x| x == word[2]).unwrap();
    // 추가될 값 계산
    let initial = '가' as u32;
    let offset = ((idx_begin * MEDIAL.len() + idx_middle) * FINAL.len() + idx_end) as u32;
    char::from_u32(initial + offset).unwrap()
}

/// ## 입력된 한 글자를 초, 중, 종성으로 구분해 반환하는 함수
/// 이 함수는 기본적으로 입력된 것이 종성이 없는 경우에도 종성을 스페이스, 즉 `' '`으로 반환한다.
/// 사용법은 tests 모듈, /tests/hangeul.rs 참고
pub fn split_phonemes(word: char) -> [char; 3] {
    // 조,중,종성을 담을 배열 정의
    let mut phonemes: [char; 3] = [' '; 3];
    // 받은 문자가 한글인지 확인, 한글이 아닐 경우 배열 첫번째 요소에 그대로 출력
    if !is_hangeul(word) {
        phonemes[0] = word;
        return phonemes;
    }
    //'가'와의 차이값 계산
    let unicode = word as u32;
    let initial = '가' as u32;
    let offset = unicode - initial;
    //초,중,종성 값 계산
    //초성
    let idx_begin: usize = (offset / (21 * 28)) as usize;
    phonemes[0] = INITIAL[idx_begin];
    //중성
    let idx_middle: usize = ((offset / 28) % 21) as usize;
    phonemes[1] = MEDIAL[idx_middle];
    //종성은 있는 경우에만 계산
    if (((unicode - 0xAC00) % (21 * 28)) % 28) != 0 {
        let idx_end: usize = (offset % 28) as usize;
        phonemes[2] = FINAL[idx_end];
    }
    //초,중,종성이 배열로 묶여서 전달
    phonemes
}

/// 비 공개 함수를 테스트합니다.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _is_hangeul() {
        let temp = '똠';
        assert_eq!(true, is_hangeul(temp));

        let temp = 'a';
        assert_eq!(false, is_hangeul(temp));

        let temp = '😀';
        assert_eq!(false, is_hangeul(temp));
    }

    #[test]
    fn _is_hangul_syllable() {
        let temp = ['ㄱ', 'ㅏ', 'ㄴ'];
        assert_eq!(true, is_hangul_syllable(temp));

        let temp = ['ㄱ', 'ㄴ', 'ㄷ'];
        assert_eq!(false, is_hangul_syllable(temp));

        let temp = ['ㅊ', 'ㄴ', 'ㅓ'];
        assert_eq!(false, is_hangul_syllable(temp));

        let temp = ['😀', 'ㄴ', 'ㄷ'];
        assert_eq!(false, is_hangul_syllable(temp));
    }
}
