import os
from pathlib import Path

p = Path('./md_files/')

paths = list(p.glob('**/*.md'))

print(paths)
text = ""

for t in paths:
    if "\\".join(f"{t}".split("\\")[:-2])+"\n" not in text:
        text += ("\\".join(f"{t}".split("\\")[:-2])+"\n")
    if "\\".join(f"{t}".split("\\")[:-1])+"\n" not in text:
        text += ("\\".join(f"{t}".split("\\")[:-1])+"\n")
    text += (f"{t}"+"\n")

# text = text.replace("md_files\\","")
# text = text.replace("md_files", "")

with open('tree.txt', 'w') as f:
   f.write(text)
