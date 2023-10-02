
var current_file = "md_files\\home.md";

var files = "";
fetch("tree.txt")
  .then((response) => response.text())
  .then((data) => {
    // Do something with your data
    // files = data;
    files = data;
    update_nav();
  });

  collapsed = files.split("\n");


var collapsed = [];
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
      if (f == current_file.replaceAll("/","\\")) {
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
  if (first_time) {
    console.log(collapsed);
    new_c.splice(0, 2);
    new_c.splice(new_c.indexOf(""), 1);
    collapsed = new_c;
    console.log(collapsed);
    first_time = false;
    update_nav();
  }
}

var e = document.createElement("base");
e.target = "_blank";
document.head.appendChild(e);

var first_loop = true;
