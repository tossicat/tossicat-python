import tossicat as tc

name1 = "한국어"
name2 = "토씨캣"

print(f"안녕하세요, {tc.postfix(name1, '은')} 바로 앞 글자에 따라 조사가 변합니다.")
print(f"따라서 {tc.postfix(name2, '은')} 조사를 알맞게 변경해줍니다.")
