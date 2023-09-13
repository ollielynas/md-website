import os
from pathlib import Path

p = Path('./md_files/')

paths = list(p.glob('**/*.md'))

print(paths)
text = ""

for t in paths:
    if "\\".join(f"{t}".split("\\")[:-1])+"\n" not in text:
        text += ("\\".join(f"{t}".split("\\")[:-1])+"\n")
    text += (f"{t}"+"\n")



with open('tree.txt', 'w') as f:
   f.write(text)
