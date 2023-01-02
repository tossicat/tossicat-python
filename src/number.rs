//! # 숫자에 관련된 모듈
//!
//! 이 프로젝트에서 숫자에 관련된 것들을 처리하는 모듈입니다.
//! 이 모듈은 아래와 같이 2개의 함수를 가지고 있습니다.
//!
//! - `change_int_char()': 숫자 한 글자를 한글 한 글자로 바꿔 줍니다.
//! - `change_nun_to_hangeul()`: 문자열로 된 숫자를 한글 발음으로 바꿔 줍니다.

// 숫자 한 글자를 한글 한 글자로 바꾸기 위한 목록
const DIGITS: [char; 10] = ['영', '일', '이', '삼', '사', '오', '육', '칠', '팔', '구'];

// 10부터 1000까지 한글로 숫자 자리수 읽기 위한 목록
const EXPS_UNTIL_1000: [char; 3] = ['십', '백', '천'];

// 4번째 자리수부터 4의 배수로 48번째 자리수까지 일기 위한 목록
// 양 다음 '구'인데 이게 숫자 '구'와 충돌을 해서 한자 '溝'로 변환
const EXPS: [char; 12] = [
    '만', '억', '조', '경', '해', '자', '양', '溝', '간', '정', '재', '극',
];

/// ## 해당 문자가 숫자인지 아닌지 확인하는 함수
/// 입력된 문자가 숫자이면 `true`, 아니면 `false`를 반환합니다.
pub fn is_digits(num: char) -> bool {
    ('0'..='9').contains(&num)
}

/// ## 숫자 한 글자를 한글 발음으로 변환해주는 함수
fn change_int_char(num: char) -> char {
    let idx: usize = num as usize;
    DIGITS[idx - 48]
}

/// ## 숫자를 한글 발음으로 바꿔주는 함수
/// 입력된 숫자를 단위를 포함한 발음으로 바꿔줍니다.
pub fn change_num_to_hangeul(num: &str) -> String {
    // 입력된 숫자 문자열을 뒤에서부터 읽기 위해서 입력된 숫자 문자열을 뒤집는다.
    let char_vec: Vec<char> = num.chars().rev().collect();

    //한 자리수인 경우 바로 읽어서 도출한다.
    if char_vec.len() == 1 {
        change_int_char(char_vec[0]).to_string()
    //한 자리수 이상인 경우를 처리한다.
    } else {
        let mut temp_result: Vec<char> = Vec::new();
        let mut temp_exps = 0;

        for (i, x) in char_vec.iter().enumerate() {
            let num_char = change_int_char(*x);
            temp_result.push(num_char);
            temp_result.push(' ');

            if ((i + 1) % 4) == 0 {
                temp_result.push(EXPS[temp_exps]);
                temp_exps += 1;
            } else if ((i + 1) % 4) == 1 {
                temp_result.push(EXPS_UNTIL_1000[0]);
            } else if ((i + 1) % 4) == 2 {
                temp_result.push(EXPS_UNTIL_1000[1]);
            } else {
                temp_result.push(EXPS_UNTIL_1000[2]);
            }
        }
        // 맨 마지막에 추가되는 단위떄문에 글자가 들어가는 버그 때문에 들어간 것을 제거한다.
        temp_result.pop();
        // 디버그용 println 삽입, 뒤집기 전에 확인하는 것이 디버그할 때 더 효과적이다.
        // println!("{:?}", temp_result);
        // 뒤집어 입력된 숫자 문자열을 뒤집어 정상으로 되돌려 놓는다.
        temp_result.reverse();

        let mut temp_result: String = temp_result.iter().collect();
        temp_result = temp_result.replace("영천", "");
        temp_result = temp_result.replace("영백", "");
        temp_result = temp_result.replace("영십", "");
        temp_result = temp_result.replace('영', "");
        temp_result = temp_result.replace("  ", " ");
        temp_result = temp_result.replace(' ', "");
        // println!("중간 집계: {:?}", temp_result);

        // 최종 결과물 만들기
        // 아래 for 문은 "억만"과 같은 문장을 제거하기 위한 것입니다.
        // 구현 방법은 억만 과 같인 만 단위 숫자 단위 글자가 같이 붙으면 제거하고 있습니다.
        // println!("만 단위 {:?} 글자가 들어왔습니다!", i); 주석을 풀고 확인하면 알 수 있습니다.

        let mut last_temp: Vec<char> = Vec::new();

        for i in temp_result.chars() {
            if EXPS.iter().any(|&x: &char| x == i) {
                let last_char = last_temp[last_temp.len() - 1];
                if !EXPS.iter().any(|&x: &char| x == last_char) {
                    last_temp.push(i);
                    // println!("만 단위 {:?} 글자가 바로 또 들어왔습니다!", i);
                }
            } else {
                last_temp.push(i);
            }
        }

        // println!("최종 결과:{:?}", last_temp);

        let mut temp_result: String = last_temp.iter().collect();

        // 한글 구와 숫자 단위 수인 구가 충돌하여 한자로 바꾼 것을 되돌린다.
        temp_result = temp_result.replace('溝', "구");
        // 관용적인 표현을 작용한다.
        temp_result = temp_result.replace("일만", "만");
        temp_result = temp_result.replace("일천", "천");
        temp_result = temp_result.replace("일백", "백");
        temp_result = temp_result.replace("일십", "십");
        temp_result = temp_result.replace("  ", " ");
        temp_result = temp_result.replace(' ', "");
        // temp_result.trim_start_matches('일').to_string()
        temp_result
    }
}

/// 비 공개 함수 테스트
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _is_digits() {
        let temp = '5';
        assert_eq!(true, is_digits(temp));

        let temp = '일';
        assert_eq!(false, is_digits(temp));

        let temp = '영';
        assert_eq!(false, is_digits(temp));

        let temp = ' ';
        assert_eq!(false, is_digits(temp));

        let temp = '😀';
        assert_eq!(false, is_digits(temp));
    }

    #[test]
    fn _change_int_char() {
        let temp = '1';
        assert_eq!('일', change_int_char(temp));

        let temp = '2';
        assert_eq!('이', change_int_char(temp));

        let temp = '3';
        assert_eq!('삼', change_int_char(temp));

        let temp = '4';
        assert_eq!('사', change_int_char(temp));

        let temp = '5';
        assert_eq!('오', change_int_char(temp));

        let temp = '6';
        assert_eq!('육', change_int_char(temp));

        let temp = '7';
        assert_eq!('칠', change_int_char(temp));

        let temp = '8';
        assert_eq!('팔', change_int_char(temp));

        let temp = '9';
        assert_eq!('구', change_int_char(temp));

        let temp = '0';
        assert_eq!('영', change_int_char(temp));
    }
}
