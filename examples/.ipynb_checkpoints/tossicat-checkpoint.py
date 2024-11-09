import tossicat as tc

name1 = "한국어"
name2 = "토씨캣"

print(f"안녕하세요, {tc.postfix(name1, '은')} 바로 앞 글자에 따라 조사가 변합니다.")
print(f"따라서 {tc.postfix(name2, '은')} 조사를 알맞게 변경해줍니다.")


sentence = "{한국어, 은} 정말 좋은 언어입니다. {커피, 을} 정말 좋아해요"

tc.modify_sentence(sentence)

result = tc.transform(name2, '(은)는')

print(f"<strong>{result[0]}</strong>{result[1]} 사용하기 정말 편해요.")
