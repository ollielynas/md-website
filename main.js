


var current_file = "md_files\\home.md";

async function load_gzip(a) {
  let blob = await fetch("gz\\"+a+".gz").then(r => r.blob());
  
  let stream = blob.stream();
  const compressedReadableStream = stream.pipeThrough(
    new DecompressionStream("gzip")
  );

    const resp = new Response(compressedReadableStream);
    return resp.text();
}

collapsed = [];


function load_md(file) {
  if (file == null) {
    return;
  }

  collapsed = collapsed.filter((a) => !file.includes(a.replaceAll("\\", "/")));

  if (file.includes(".md")) {
    // show_nav()
    current_file = file;
    location.hash = file.replaceAll("\\", "/");

    load_gzip(file).then(
      (text) => {
        document.getElementById("md_block").mdContent = text;
      },
      (text) => {
        document.getElementById("md_block").mdContent = text;
      }
    );

    show_content();

    // let scripts = md_block.querySelectorAll("script");
    // console.log(scripts);
    // for (var i = 0; i < scripts.length; i++) {
    //   console.log("fakeImage: ", scripts[i]);
    // }
    // md_block.src = file;
  }

  update_nav();
}



var files = "";
console.log("this is decompressed", load_gzip("tree.txt").then((data) => {
  files = data;
  let file_list = files.replaceAll("\r", "").split("\n");
  collapsed = file_list;
  load_md(current_file);
  update_nav();
}));

// fetch("tree.txt")
//   .then((response) => response.text())
//   .then((data) => {
//     // Do something with your data
//     // files = data;
//   });

var first_time = true;

function update_nav() {
  let new_c = [];
  let nav = document.getElementById("nav");
  if (nav == null) {
    return;
  }
  nav.innerHTML = "";

  let file_list = files.replaceAll("\r", "").split("\n");


  for (f of file_list) {
    f = f.trim();
    let do_collapse = false;

    for (i of collapsed) {
      if (f.includes(i) && f != i) {
        do_collapse = true;
        break;
      }
    }
    if (do_collapse) {
      continue;
    }
    let v_div = document.createElement("div");
    v_div.className = "horizontal";
    let f2 = f.split("\\");
    let name = f2[f2.length - 1];
    let new_element_name = document.createElement("p");
    new_element_name.innerText = name;
    new_element_name.id = f;

    if (f.includes(".md")) {
      new_element_name.className = "link";
      new_element_name.addEventListener("click", function () {
        load_md(this.id);
        // this.scrollIntoView();
      });
      if (f == current_file.replaceAll("/", "\\")) {
        new_element_name.className += " ur-here";
      }
    } else {
      if (first_time) {
        new_c.push(f);
      }
      new_element_name.className = "folder";
      new_element_name.addEventListener("click", function () {
        if (collapsed.includes(this.id)) {
          collapsed.splice(collapsed.indexOf(this.id), 1);
        } else {
          collapsed.push(this.id);
        }
      });
      if (collapsed.includes(f)) {
        new_element_name.className += " open";
      } else {
        new_element_name.className += " close";
      }
    }
    let last_child = nav.lastChild;
    let path_text = document.createElement("p");
    path_text.innerText =
      " ".repeat(Math.max(f2.length - 1, 0)) +
      "├─ ".repeat(Math.min(Math.max(f2.length - 1, 0), 1));

    if (
      last_child != null &&
      last_child.firstChild.innerText.length + 1 != path_text.innerText.length
    ) {
      let path_text_child = last_child.firstChild;
      let new_text = "└─";
      if (
        last_child.firstChild.innerText.length + 1 <
        path_text.innerText.length
      ) {
        new_text = "└┬";
      }

      path_text_child.innerText = path_text_child.innerText.replace(
        "├─",
        new_text
      );
    }

    v_div.appendChild(path_text);
    v_div.appendChild(new_element_name);
    nav.appendChild(v_div);
  }

}

var e = document.createElement("base");
e.target = "_blank";
document.head.appendChild(e);

var first_loop = true;
