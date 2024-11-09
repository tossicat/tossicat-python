<img width="1441" alt="스크린샷 2023-01-02 오후 12 56 27" src="https://user-images.githubusercontent.com/42489770/210288424-d89542a2-0def-4e8c-b301-411658b1568b.png">

# tossicat-python

[![PyPI](https://img.shields.io/pypi/v/tossicat-python)](https://pypi.org/project/tossicat-python/)
[![GitHub license](https://img.shields.io/github/license/tossicat/tossicat-python)](https://github.com/tossicat/tossicat-python/blob/main/LICENSE)

tossicat-python은 현재 PyPi에 올라가 있는 파이썬 라이브러리입니다.  
해당 소스코드는 PyO3를 사용하여 [tossicat-core](https://github.com/tossicat/tossicat-core)를 파이썬에서 사용할 수 있도록 바인딩 합니다.

# 사용방법

먼저 pip를 사용하여 라이브러리를 설치합니다.

```bash
pip install tossicat-python
```

해당 패키지에서는 크게 2가지의 함수를 지원합니다.

## postfix(word, tossi)  

이 함수는 단어와 토씨(조사)를 파라미터로 받으며 앞 단어에 맞는 토씨로 변환 후 문자열로 합쳐 반환합니다.  
예시 코드는 아래와 같습니다.

```python
import tossicat as tc

word = "테스트"
tossi = "은"

result = tc.postfix(word, tossi)

print(result)
테스트는
```

## modify_sentence(sentence)

이 함수는 문장 문자열 중에서 중괄호 `{단어, 토씨}`로 구성된 값을 파싱하여 올바른 토씨로 반환 후 다시 반환합니다.  
예시 코드는 아래와 같습니다.

```python
import tossicat as tc

sentence = "{한국어, 은} 정말 좋은 언어입니다. {커피, 을} 정말 좋아해요"

result = tc.modify_sentence(sentence)

print(result)
한국어는 정말 좋은 언어입니다. 커피를 정말 좋아해요
```

https://www.youtube.com/watch?v=H9Fj___zmkg

## transform(word, tossi)  

이 함수는 단어와 토씨(조사)를 파라미터로 받으며 앞 단어에 맞는 토씨로 변환하여 각각 반환합니다.  
예시 코드는 아래와 같습니다.

```python
import tossicat as tc

word = "테스트"
tossi = "은"

result = tc.transform(word, tossi)

print(result)
("테스트", "는")
```