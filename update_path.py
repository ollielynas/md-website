import datetime
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

# index = "";

# with open("index.html","r") as f:
#     index = f.read()

# for i in text.split("\n"):
#     if ".md" not in i: continue
#     with open(i.replace(".md",".html"),"x") as f:
#         f.write(index.replace("md_files\\home.md", f"\\{i}"))
        


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


resources = ["favicon.ico"]

resources.append("tree.txt")
resources.append("main.js")
resources.append("main.css")


for i in text.split("\n"):
    if ".md" not in i:
        continue
    resources.append(i)


resources_str = ""

for i in resources:
    resources_str += "\n["+i.replace("\\", "/")+"](" + i.replace(" ", "%20").replace("\\", "/")+")"+"\n"

with open('md_files/site/resources.md', 'w') as f:
   f.write(resources_str)


date_time = datetime.datetime.now()
sitemap = ""

sitemap += "<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">"

for i in ["index.html"]+resources:
    print(i)
    if ".md" in i or ".html" in i:
        sitemap += "\n  <url>"
        i = i.replace('\\', '/')
        sitemap += f"\n      <loc>https://ollielynas.github.io/md-website/{i}</loc>"
        sitemap += "\n  </url>"
sitemap += "\n</urlset>"

with open('sitemap.xml', 'w') as f:
   f.write(sitemap)
