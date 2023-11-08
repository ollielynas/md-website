import datetime
import os
from pathlib import Path
import gzip
import markdown
from io import BytesIO
import os.path, time

from css_html_js_minify import html_minify, js_minify, css_minify

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib
    
total_words = 0

p = Path('./md_files/')

paths = list(p.glob('**/*.md'))

bottom = ["report bug.md"]

text = ""
text_last = ""
for t in paths:
    print(t)
    if any([n in f"{t}" for n in bottom]) or "\\site" in f"{t}":
        if "\\".join(f"{t}".split("\\")[:-2])+"\n" not in text_last+text:
            text_last += ("\\".join(f"{t}".split("\\")[:-2])+"\n")
        if "\\".join(f"{t}".split("\\")[:-1])+"\n" not in text_last+text:
            text_last += ("\\".join(f"{t}".split("\\")[:-1])+"\n")
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

with open('src/tree.txt', 'w', encoding="utf-8") as f:
    f.write(text)


csv = ""

for i in text.split("\n"):
    if ".md" not in i:continue
    csv += i.split("\\")[-1]+","

with open('tree.csv', 'w', encoding="utf-8") as f:
    f.write(csv)


#  does tree + csv ^


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

with open('md_files/site/resources.md', 'w', encoding="utf-8") as f:
    f.write(resources_str)


date_time = datetime.datetime.now()
sitemap = ""

sitemap += "<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">"

for i in ["index.html"]:        
    sitemap += "\n  <url>"
    i = i.replace('\\', '/')
    i = i.replace(" ", "%20")
    sitemap += f"\n      <loc>https://ollielynas.github.io/md-website/{i}</loc>"
    sitemap += "\n  </url>"




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

def sizeof_fmt(num, suffix="B"):
    for unit in ("", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi"):
        if abs(num) < 1024.0:
            return f"{num:3.1f}{unit}{suffix}"
        num /= 1024.0
    return f"{num:.1f}Yi{suffix}"


def get_size(start_path='.'):
    total_size = 0
    for dirpath, dirnames, filenames in os.walk(start_path):
        for f in filenames:
            fp = os.path.join(dirpath, f)
            if "target" in fp: continue
            # skip if it is symbolic link
            if not os.path.islink(fp):
                total_size += os.path.getsize(fp)

    return total_size
# with open("no_js.html","w") as f:
#     f.write(index_inner+"\n"+no_js)
text=""
with open("md_files/site/website stats.md", "r", encoding="utf-8") as f:
    lines = f.readlines()
    for i in range(len(lines)):
        if "<td>last compiled</td><td>" in lines[i]:
            lines[i] = "<td>last compiled</td><td>"+date_time.strftime("%Y-%m-%d %H:%M:%S")+"</td>\n"
        if "<td>number of pages</td><td>" in lines[i]:
            lines[i] = "<td>number of pages</td><td>"+str(len([i for i in resources if ".md" in i]))+"</td>\n"
        if "<td>project size</td><td>" in lines[i]:
            lines[i] = "<td>project size</td><td>"+sizeof_fmt(get_size())+"</td>\n"
        # if "<td>word count</td><td>" in lines[i]:
        #     lines[i] = "<td>word count</td><td>"+str(total_words)+"</td>\n"
    text="".join(lines)


with open("md_files/site/website stats.md", "w", encoding="utf-8") as f:
    if text=="":raise ValueError
    f.write(text)
    


def process_and_detect_edit(text,file_name):
    replace = {
        "- [ ]": "<input type=\"checkbox\"></input>",
        "- [x]": "<input type=\"checkbox\" checked></input>"
    }


    last_edit = time.ctime(os.path.getmtime(file_name))
    has_been_modified = text.split("\n")[-1] == f"<!-- LAST EDITED {last_edit} LAST EDITED-->"

    
    if has_been_modified:
        new_text = ""
        first = True
        for line in text.split("\n"):
            if "LAST EDITED" in line:
                new_text += f"\n<!-- LAST EDITED {last_edit} LAST EDITED-->"
            elif first:
                new_text += f"{line}"
                first = False
            else:
                new_text += f"\n{line}"
        if f"\n<!-- LAST EDITED {last_edit} LAST EDITED-->" not in new_text:
            new_text += f"\n<!-- LAST EDITED {last_edit} LAST EDITED-->"
        print(len(new_text))
        text = new_text
        with open(file_name, 'w', encoding='utf-8') as file_writer:
            file_writer.write(new_text)
    
    
    for i in replace:
        text = text.replace(i, replace[i])
    text = markdown.markdown(text)
    for i in replace:
        text = text.replace(f"<p>{replace[i]}</p>", replace[i])

    # if "home.md" in a:
    #     with open("index.html", "r") as f_index:
    #         f_index_list = f_index.read().split("<!-- START-STOP -->")
    #     with open("index.html", "w") as f_index_w:
    #         f_index_w.write(f_index_list[0]+"<!-- START-STOP -->" +
    #                         text+"<!-- START-STOP -->"+f_index_list[2])

    
    return (text, has_been_modified)

favorite = ""

def html_template(path, html, has_been_modified):
    global sitemap, favorite, sitemap
    path2 = path.replace('\\', '/')
    sitemap += "\n  <url>"
    sitemap += f"\n      <loc>https://ollielynas.github.io/md-website/sub/{path2.replace('.md','.html').replace(' ', '%20')}</loc>"
    sitemap += "\n  </url>"
    
    #  only do if edited
    if not has_been_modified:
        return
    
    meta = html.split("META")
    template = ""
    with open("sub/template.html", encoding="utf-8") as t:
        template = t.read()
    if "no index" in html:return
    template = template.replace("CONTENT", html)
    folder , name = path.split("\\")[-2:]
    folder = folder.replace("md_files","")
    name = name.replace(".md","")
    template = template.replace("TITLE",f"Ollie Lynas - {folder} - {name}")
    template = template.replace("THISPAGE", path2)
    if len(meta) > 2:
        _, inner_meta, _ = meta
    else: inner_meta = name + " by Ollie Lynas"
    if "<!-- STAR ICON -->" in html:
        favorite += f"{path}\n"
        favorite += f"{inner_meta}\n"
    template=template.replace("META_DESCRIPTION", inner_meta)
    output_file = Path('sub/'+path2)
    output_file.parent.mkdir(exist_ok=True, parents=True)

    
    with open("sub/"+path.replace(".md",".html"), "w", encoding = "utf-8") as f:
        f.write(template)
        # csv += f"https://ollielynas.github.io/md-website/sub/#{path.replace('.md','.html')}"
    
def compress(file_name):
    # global total_words
    output_file = Path('gz/'+file_name.replace("\\","/")+'.gz')
    output_file.parent.mkdir(exist_ok=True, parents=True)
    
    if ".md" in file_name:
        with open(file_name, 'r', encoding = "utf-8") as f_in:
            # total_words += len([i for i in f_in.read().split(" ") if i.isalnum()])
            # print(total_words)
            # if 
            print("error here", file_name)
            f_in, has_been_modified = process_and_detect_edit(f_in.read(),file_name)

            
            html_template(file_name,f_in, has_been_modified)
            
            f_in = BytesIO(bytes(f_in, 'utf-8'))
            
            with gzip.open('gz/'+file_name.replace("\\", "/")+'.gz', 'wb') as f_out:
                f_out.writelines(f_in)
    elif ".js" in file_name:
        with open(file_name, 'r', encoding = "utf-8") as f_in:
            f_in = js_minify(f_in.read())
            f_in = BytesIO(bytes(f_in, 'utf-8'))
            with gzip.open('gz/'+file_name.replace("\\", "/")+'.gz', 'wb') as f_out:
                f_out.writelines(f_in)
    elif ".css" in file_name:
        with open(file_name, 'r', encoding="utf-8") as f_in:
            f_in = css_minify(f_in.read())
            f_in = BytesIO(bytes(f_in, 'utf-8'))
            with gzip.open('gz/'+file_name.replace("\\", "/")+'.gz', 'wb') as f_out:
                f_out.writelines(f_in)
    else:
        with open(file_name, 'rb') as f_in, gzip.open('gz/'+file_name.replace("\\", "/")+'.gz', 'wb') as f_out:
            f_out.writelines(f_in)

for i in resources:
    compress(i)



# with open('tree.csv', 'w') as f:
#     f.write(csv)

# timeline = tomllib.loads(Path("md_files/about me/timeline.toml").read_text(encoding="utf-8"))



sitemap += "\n</urlset>"
with open('sitemap.xml', 'w', encoding = "utf-8") as f:
    f.write(sitemap)

with open('src/favorite.txt', 'w', encoding = "utf-8") as f:
    f.write(favorite)