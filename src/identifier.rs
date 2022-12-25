//! # 입력된 토시(tossi)가 어떤 것인지 알아내 저장하는 토시 구조체를 구현한 모듈
//!
//! 사용자가 입력한 토시를 변환해서 저장하고,
//! 변환한 값을 토대로 어떤 종류인지 분류한 다음 분류한 결과를 저장한다.
//! 사용법은 아래와 같다.
//!
//! ```rust, ignore
//! let test_tossi = "으로";
//! let temp = Tossi::new(test_tossi);
//! println!("입력된 토시: {:?}", temp.modified);
//! println!("토시 종류: {:?}", temp.kind);
//! ```
//!
//! 실행 결과는 다음과 같다.
//!
//! ```ignore
//! 입력된 토시: ['으', '로']
//! 토시 종류: Ro
//! ```
//!
//! 여기서 주의할 점은 반환된 값 중 첫 번째 값은, 위에서`temp.modified`은,
//! `filter_only_significant()` 함수를 통해서
//! 변환되어 오기 때문에 이 함수의 처리 방법에 따라 변형되어 반환된다는 점이다.
//! 그리고 반환된 값은 `Vec<char>`으로 되어 있기 때문에 한 글자씩 분리되어 있다.
//!
//! 두 번째 값은 프로그램이 판단한 토시의 종류를 열거자를 가지고 표현하고 있는데
//! 이 '토시 종류(TossiKind)'의 토시 영어명 표시명은
//! 이 프로젝트 안에 있는 다음 파일을 참고한다.
//!
//! - docs/terms.md

use crate::filter::filter_only_significant;

pub enum TossiKind {
    Deun,
    Deunji,
    Deunka,
    Eul,
    Ida,
    Ka,
    Ko,
    Myeo,
    Na,
    Nama,
    Neun,
    Ni,
    Rado,
    Ran,
    Rang,
    Raya,
    Yamalro,
    Yeo,
    Wa,
    Ro,
    Robuteo,
    Roseo,
    Rosseo,
    Others,
}
pub enum TransTossiWhen {
    Blank,
    RiEulAndBlank,
    Nothing,
}

pub struct Tossi {
    pub modified: Vec<char>,
    pub kind: TossiKind,
    pub when: TransTossiWhen,
}

impl Tossi {
    pub fn new(raw: &str) -> Self {
        let temp_modified = filter_only_significant(raw);
        // 앞에서 변환 것을 토대로 글자 수에 따라 조사 종류를 찾는다.
        let temp_kind = match temp_modified.len() {
            1 => one_letter(temp_modified[0]),
            2 => two_letters(&temp_modified),
            3 => three_letters(&temp_modified),
            4 => four_letters(&temp_modified),
            _ => TossiKind::Others,
        };
        let temp_trans_tossi_when = match temp_kind {
            TossiKind::Deun => TransTossiWhen::Blank,
            TossiKind::Deunka => TransTossiWhen::Blank,
            TossiKind::Deunji => TransTossiWhen::Blank,
            TossiKind::Eul => TransTossiWhen::Blank,
            TossiKind::Ida => TransTossiWhen::Blank,
            TossiKind::Ka => TransTossiWhen::Blank,
            TossiKind::Ko => TransTossiWhen::Blank,
            TossiKind::Myeo => TransTossiWhen::Blank,
            TossiKind::Na => TransTossiWhen::Blank,
            TossiKind::Nama => TransTossiWhen::Blank,
            TossiKind::Neun => TransTossiWhen::Blank,
            TossiKind::Ni => TransTossiWhen::Blank,
            TossiKind::Rado => TransTossiWhen::Blank,
            TossiKind::Ran => TransTossiWhen::Blank,
            TossiKind::Rang => TransTossiWhen::Blank,
            TossiKind::Raya => TransTossiWhen::Blank,
            TossiKind::Yamalro => TransTossiWhen::Blank,
            TossiKind::Yeo => TransTossiWhen::Blank,
            TossiKind::Wa => TransTossiWhen::Blank,
            TossiKind::Ro => TransTossiWhen::RiEulAndBlank,
            TossiKind::Robuteo => TransTossiWhen::RiEulAndBlank,
            TossiKind::Roseo => TransTossiWhen::RiEulAndBlank,
            TossiKind::Rosseo => TransTossiWhen::RiEulAndBlank,
            TossiKind::Others => TransTossiWhen::Nothing,
        };
        Self {
            modified: temp_modified,
            kind: temp_kind,
            when: temp_trans_tossi_when,
        }
    }
}

/// ## 한 글자로 된 토시를 분류하는 함수
/// 한 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
fn one_letter(element: char) -> TossiKind {
    match element {
        '은' | '는' => TossiKind::Neun,
        '이' | '가' => TossiKind::Ka,
        '을' | '를' => TossiKind::Eul,
        '와' | '과' => TossiKind::Wa,
        '로' => TossiKind::Ro,
        '다' => TossiKind::Ida,
        '나' => TossiKind::Na,
        '랑' => TossiKind::Rang,
        '란' => TossiKind::Ran,
        '며' => TossiKind::Myeo,
        '고' => TossiKind::Ko,
        '니' => TossiKind::Ni,
        '든' => TossiKind::Deun,
        '여' => TossiKind::Yeo,
        _ => TossiKind::Others,
    }
}

/// ## 두 글자로 된 토시를 분류하는 함수
/// 두 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
fn two_letters(elements: &[char]) -> TossiKind {
    match (elements[0], elements[1]) {
        ('으', '로') => TossiKind::Ro,
        ('로', '서') => TossiKind::Roseo,
        ('로', '써') => TossiKind::Rosseo,
        ('이', '다') => TossiKind::Ida,
        ('이', '나') => TossiKind::Na,
        ('이', '랑') => TossiKind::Rang,
        ('이', '란') => TossiKind::Ran,
        ('나', '마') => TossiKind::Nama,
        ('이', '며') => TossiKind::Myeo,
        ('이', '고') => TossiKind::Ko,
        ('이', '니') => TossiKind::Ni,
        ('이', '든') => TossiKind::Deun,
        ('든', '지') => TossiKind::Deunji,
        ('든', '가') => TossiKind::Deunka,
        ('이', '여') => TossiKind::Yeo,
        ('라', '야') => TossiKind::Raya,
        ('라', '도') => TossiKind::Rado,
        (_, _) => TossiKind::Others,
    }
}

/// ## 세 글자로 된 토시를 분류하는 함수
/// 세 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
fn three_letters(elements: &[char]) -> TossiKind {
    match (elements[0], elements[1], elements[2]) {
        ('으', '로', '서') => TossiKind::Roseo,
        ('으', '로', '써') => TossiKind::Rosseo,
        ('로', '부', '터') => TossiKind::Robuteo,
        ('이', '나', '마') => TossiKind::Nama,
        ('야', '말', '로') => TossiKind::Yamalro,
        ('이', '든', '지') => TossiKind::Deunji,
        ('이', '든', '가') => TossiKind::Deunka,
        ('이', '라', '야') => TossiKind::Raya,
        ('이', '라', '도') => TossiKind::Rado,
        (_, _, _) => TossiKind::Others,
    }
}

/// ## 네 글자로 된 토시를 분류하는 함수
/// 네 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
fn four_letters(elements: &[char]) -> TossiKind {
    match (elements[0], elements[1], elements[2], elements[3]) {
        ('으', '로', '부', '터') => TossiKind::Robuteo,
        ('이', '야', '말', '로') => TossiKind::Yamalro,
        (_, _, _, _) => TossiKind::Others,
    }
}
