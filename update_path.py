import datetime
import os
from pathlib import Path
import gzip
import markdown
from io import BytesIO

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib

p = Path('./md_files/')

paths = list(p.glob('**/*.md'))

bottom = ["report bug.md"]

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

with open('src/tree.txt', 'w') as f:
    f.write(text)


csv = ""

for i in text.split("\n"):
    if ".md" not in i:continue
    csv += i.split("\\")[-1]+","

with open('tree.csv', 'w') as f:
    f.write(csv)


resources = ["favicon.ico"]

resources.append("src/tree.txt")
resources.append("css/main.css")


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
    if ".md" in i or ".html" in i:
        sitemap += "\n  <url>"
        i = i.replace('\\', '/')
        sitemap += f"\n      <loc>https://ollielynas.github.io/md-website/{i}</loc>"
        sitemap += "\n  </url>"
sitemap += "\n</urlset>"

with open('sitemap.xml', 'w') as f:
    f.write(sitemap)




# index_inner=""


# no_js = ""

# for i in resources:
#     if ".md" not in i:continue
#     with open(i, "r") as f:
#         a="\\"
#         index_inner += f"\n<a target='_parent' href='#{i}'>{i.split(a)[-1]}</a><br>"
#         no_js += f"<h3 name = '{i}'>{i.split(a)[-1]}</h3>"
#         no_js += f.read()

# print(no_js)



# with open("no_js.html","w") as f:
#     f.write(index_inner+"\n"+no_js)
text=""
with open("md_files/site/website stats.md", "r") as f:
    lines = f.readlines()
    for i in range(len(lines)):
        if "<td>last compiled</td><td>" in lines[i]:
            lines[i] = "<td>last compiled</td><td>"+date_time.strftime("%Y-%m-%d %H:%M:%S")+"</td>\n"
    text="".join(lines)
    
with open("md_files/site/website stats.md", "w") as f:
    if text=="":raise ValueError
    f.write(text)

def compress(a):
    
    output_file = Path('gz/'+a.replace("\\","/")+'.gz')
    output_file.parent.mkdir(exist_ok=True, parents=True)
    
    if ".md" in a:
    
    
        with open(a, 'r') as f_in:
            f_in = markdown.markdown(f_in.read())
            f_in = BytesIO(bytes(f_in, 'utf-8'))
            
            with gzip.open('gz/'+a.replace("\\","/")+'.gz', 'wb') as f_out:
                f_out.writelines(f_in)
    else:
        with open(a, 'rb') as f_in, gzip.open('gz/'+a.replace("\\", "/")+'.gz', 'wb') as f_out:
            f_out.writelines(f_in)
for i in resources:
    compress(i)


# timeline = tomllib.loads(Path("md_files/about me/timeline.toml").read_text(encoding="utf-8"))


