import os
from pathlib import Path

p = Path('./md_files/')

paths = list(p.glob('**/*.md'))

print(paths)
text = ""
text_last = ""
for t in paths:
    if "cookies.md" in f"{t}":
        text_last += (f"{t}"+"\n")
    else:
        if "\\".join(f"{t}".split("\\")[:-2])+"\n" not in text:
            text += ("\\".join(f"{t}".split("\\")[:-2])+"\n")
        if "\\".join(f"{t}".split("\\")[:-1])+"\n" not in text:
            text += ("\\".join(f"{t}".split("\\")[:-1])+"\n")
        text += (f"{t}"+"\n")

text+=text_last



# text = text.replace("md_files\\","")
# text = text.replace("md_files", "")

with open('tree.txt', 'w') as f:
   f.write(text)
