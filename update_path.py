import datetime
import os
from pathlib import Path
import gzip
import markdown
from io import BytesIO
import os.path, time
from html2image import Html2Image
from PIL import Image
import threading

from css_html_js_minify import html_minify, js_minify, css_minify

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib
    
total_words = 0

schema = """<script type="application/ld+json">
[
"""

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
resources.append("css/bird.css")

portfolio_project_names_and_catagories = []

for i in text.split("\n"):
    if ".md" not in i:
        continue
    resources.append(i)
    if "portfolio" in i and "index.md" not in i :
        portfolio_project_names_and_catagories.append((i.split("\\")[-2], i.split("\\")[-1].replace(".md","")))

print(portfolio_project_names_and_catagories)

resources_str = ""

for i in resources:
    resources_str += "\n["+i.replace("\\", "/")+"](" + i.replace(" ", "%20").replace("\\", "/")+")"+"\n"

old_resources = ""

with open('md_files/site/resources.md', 'r', encoding="utf-8") as f:
    old_resources = f.read()
if resources_str != old_resources:
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
    
    


    last_edit = round(os.path.getmtime(file_name))
    
    print(text.split("\n")[-1] , f"<!-- LAST EDITED {last_edit} LAST EDITED-->")
    has_been_modified = text.split("\n")[-1] != f"<!-- LAST EDITED {last_edit} LAST EDITED-->"
    
    
#                                                                      o8o  oooo                           oooo  oooo  
#                                                                      `"'  `888                           `888  `888  
# oooo d8b  .ooooo.   .ooooo.   .ooooo.  ooo. .oo.  .oo.   oo.ooooo.  oooo   888   .ooooo.        .oooo.    888   888  
# `888""8P d88' `88b d88' `"Y8 d88' `88b `888P"Y88bP"Y88b   888' `88b `888   888  d88' `88b      `P  )88b   888   888  
#  888     888ooo888 888       888   888  888   888   888   888   888  888   888  888ooo888       .oP"888   888   888  
#  888     888    .o 888   .o8 888   888  888   888   888   888   888  888   888  888    .o      d8(  888   888   888  
# d888b    `Y8bod8P' `Y8bod8P' `Y8bod8P' o888o o888o o888o  888bod8P' o888o o888o `Y8bod8P'      `Y888""8o o888o o888o 
#                                                           888                                                        
#                                                          o888o                                                       

    # has_been_modified = True
    
    current_time = round(time.time())
    
    if has_been_modified:
        new_text = ""
        first = True
        for line in text.split("\n"):
            if "LAST EDITED" in line:
                ...
                # new_text += f"\n<!-- LAST EDITED {current_time} LAST EDITED-->"
            elif first:
                new_text += f"{line}"
                first = False
            else:
                new_text += f"\n{line}"
        new_text += f"\n<!-- LAST EDITED {current_time} LAST EDITED-->"
        print(len(new_text))
        text = new_text
        with open(file_name, 'w', encoding='utf-8') as file_writer:
            file_writer.write(new_text)
    
    
    for i in replace:
        text = text.replace(i, replace[i])
    text = markdown.markdown(text)
    for i in replace:
        text = text.replace(f"<p>{replace[i]}</p>", replace[i])

    # text = text.replace("Ollie Lynas", """<span id="_author6" itemprop="author" itemscope itemtype="http://schema.org/Person">Ollie Lynas</span>""")
    
    # for p in portfolio_project_names_and_catagories:
    #     text = text.replace(p[1], f"""<span itemscope itemtype="http://schema.org/SoftwareApplication" itemref="_author6">{p[1]}</span>""")
        
    
    return (text, has_been_modified)

favorite = ""

def html_template(path, html, has_been_modified):
    global sitemap, favorite, sitemap
    path2 = path.replace('\\', '/')
    sitemap += "\n  <url>"
    sitemap += f"\n      <loc>https://ollielynas.github.io/md-website/sub/{path2.replace('.md','.html').replace(' ', '%20')}</loc>"
    sitemap += "\n  </url>"
    
    #  only do if edited
    
    meta = html.split("META")
    template = ""
    with open("sub/template.html", encoding="utf-8") as t:
        template = t.read()
    if "no index" in html:return
    template = template.replace("CONTENT", html)
    folder , name = path.split("\\")[-2:]
    folder = folder.replace("md_files","")
    name = name.replace(".md","")
    markup = ""
    if "portfolio" in path and "books" not in path:
        category = []
        
        # GameApplication
        # SocialNetworkingApplication
        # TravelApplication
        # ShoppingApplication
        # SportsApplication
        # LifestyleApplication
        # BusinessApplication
        # DesignApplication
        # DeveloperApplication
        # DriverApplication
        # EducationalApplication
        # HealthApplication
        # FinanceApplication
        # SecurityApplication
        # BrowserApplication
        # CommunicationApplication
        # DesktopEnhancementApplication
        # EntertainmentApplication
        # MultimediaApplication
        # HomeApplication
        # UtilitiesApplication
        # ReferenceApplication
        if "itch.io" in path2 or "game" in path2 or "golf" in path2:
            category += ["GameApplication"]
        if "web" in path2 or "desmos" in path2:
            category += ["BrowserApplication"]
        if "desmos" in path2 or "esolangs" in path2:
            category += ["EducationalApplication"]
        
        if category == []:
            category += [""]
    
        markup = f"""
        {{
        "@context": "https://schema.org",
        "@type": "SoftwareApplication",
        "author": {{
        "@type": "Person",
        "name": "Ollie Lynas"
        }},
        "image": "https://ollielynas.github.io/md-website/IMAGE_PATH"
        "name": "{name}",
        "applicationCategory": {category},
        "offers": {{
        "@type": "Offer",
        "price": "0"
        }}
        }}""".replace("'", "\"")
    # if "books" in path:
    #     markup = f"""
    #     {{
    #     "@context": "https://schema.org",
    #     "@type": "SoftwareApplication",
    #     "name": "{name}",
    #     <!-- "operatingSystem": "ANDROID", -->
    #     "applicationCategory": "{" ".join(category)}",
    #     "offers": {
    #     "@type": "Offer",
    #     "price": "0"
    #     }
    #     }}"""
        
        

    template = template.replace("MARKUP", markup)
        
    
    template = template.replace("THISPAGE", path2)
    template = template.replace("TITLE",f"Ollie Lynas - {folder} - {name}")
    if len(meta) > 2:
        _, inner_meta, _ = meta
    else: inner_meta = name + " by Ollie Lynas"
    if "<!-- STAR ICON -->" in html:
        favorite += f"{path}\n"
        favorite += f"{inner_meta}\n"
    template=template.replace("META_DESCRIPTION", inner_meta)
    template=template.replace("THIS_URL", "sub/"+path2.replace(".md",".html").replace(" ","%20"))
    
    if not has_been_modified:
        return
    
    output_file = Path('sub/'+path2)
    output_file.parent.mkdir(exist_ok=True, parents=True)

    output_file2 = Path('og-img/'+path2)
    output_file2.parent.mkdir(exist_ok=True, parents=True)

    # if sum(img.convert("L").getextrema()) in (0, 2):
        # csv += f"https://ollielynas.github.io/md-website/sub/#{path.replace('.md','.html')}"


    hti = Html2Image(size=(1000, 1800), custom_flags=['--virtual-time-budget=50000', '--hide-scrollbars', '--enable-gpu'])
    
    hti.output_path = str(output_file2.parent)
    print("adding image to", hti.output_path)
    
    template = template.replace("IMAGE_PATH", "og-img/"+path2.replace('.md','.png'))
    
    im_diff = 0
    
    
#     
# ooooo                                                  
# `888'                                                  
#  888  ooo. .oo.  .oo.    .oooo.    .oooooooo  .ooooo.  
#  888  `888P"Y88bP"Y88b  `P  )88b  888' `88b  d88' `88b 
#  888   888   888   888   .oP"888  888   888  888ooo888 
#  888   888   888   888  d8(  888  `88bod8P'  888    .o 
# o888o o888o o888o o888o `Y888""8o `8oooooo.  `Y8bod8P' 
#                                   d"     YD            
#                                   "Y88888P'            

    im_loop = 0
    # im_loop = 31
    
    while im_diff < 10 and im_loop < 30:
        
        try:
            hti.screenshot(url=f'https://ollielynas.github.io/md-website/#{path2}', save_as=(name+".png"))
            # hti.screenshot(html_str = template, css_file='css\main.css', save_as=(name+".png"))
        except:
            print("An exception occurred") 
        
        
        
        print("finished screenshot for", name)
        
        im_loop += 1
        
        img_path = hti.output_path + "\\" + name + ".png"
        im = Image.open(img_path)
        im = im.crop((0, 0, 1000, 1000))
        ex = im.convert("L").getextrema()
        im_diff = abs(ex[1] - ex[0])
        print(ex, img_path)
        im.save(img_path, quality=95)
        im.close()
        
        
    
    
    with open("sub/"+path.replace(".md",".html"), "w", encoding = "utf-8") as f:
        f.write(template)
    
def compress(file_name):
    # global total_words
    output_file = Path('gz/'+file_name.replace("\\","/")+'.gz')
    output_file.parent.mkdir(exist_ok=True, parents=True)
    
    if ".md" in file_name:
        with open(file_name, 'r', encoding = "utf-8") as f_in:
            # total_words += len([i for i in f_in.read().split(" ") if i.isalnum()])
            # print(total_words)
            # if 
            f_in, has_been_modified = process_and_detect_edit(f_in.read(),file_name)
            print(has_been_modified)
            
            html_template(file_name,f_in, has_been_modified)
            
            f_in = BytesIO(bytes(f_in, 'utf-8'))
            if has_been_modified:
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

# for i in resources:
#     compress(i)

def run_item(f, item):
    result_info = [threading.Event(), None]
    def runit():
        result_info[1] = f(item)
        result_info[0].set()
        print("finished processing", item)
        
    threading.Thread(target=runit).start()
    
    return result_info

def gather_results(result_infos):
    results = [] 
    for i in range(len(result_infos)):
        result_infos[i][0].wait()
        results.append(result_infos[i][1])
        print("results: ", len(results))
    return results

gather_results([run_item(compress, item) for item in resources])

# with open('tree.csv', 'w') as f:
#     f.write(csv)

# timeline = tomllib.loads(Path("md_files/about me/timeline.toml").read_text(encoding="utf-8"))



sitemap += "\n</urlset>"
with open('sitemap.xml', 'w', encoding = "utf-8") as f:
    f.write(sitemap)
    

with open('src/favorite.txt', 'w', encoding = "utf-8") as f:
    f.write(favorite)