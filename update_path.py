import os
from pathlib import Path

p = Path('./md_files/')

paths = list(p.glob('**/*.md'))

bottom = ["cookies.md", "report bug.md"]

print(paths)
text = ""
text_last = ""
for t in paths:
    if any([n in f"{t}" for n in bottom]):
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


csv = ""

for i in text.split("\n"):
    if ".md" not in i:continue
    csv += i.split("\\")[-1]+","

with open('tree.csv', 'w') as f:
   f.write(csv)


resources = ["/favicon.ico"]

resources.append("/tree.txt")
resources.append("/main.js")
resources.append("/main.css")


for i in text.split("\n"):
    if ".md" not in i:
        continue
    resources.append(i)

resources_str = ""

for i in resources:
    resources_str += "\n["+i+"](" + \
        i.replace(" ", "%20").replace("\\", "/")+")"+"\n"

with open('md_files/site/resources.md', 'w') as f:
   f.write(resources_str)
