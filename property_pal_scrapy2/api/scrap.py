import re

pattern = re.compile(r"<.*?>")
string = "</p><p>Bathroom</p><p>"
clean = re.sub(pattern, " ", string)
print(clean)