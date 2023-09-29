if (window.XMLHttpRequest)
  // code for IE7+, Firefox, Chrome, Opera, Safari
  xmlhttp = new XMLHttpRequest();
// code for IE6, IE5
else xmlhttp = new ActiveXObject("Microsoft.XMLHTTP");

xmlhttp.onreadystatechange = function (oEvent) {
  if (xmlhttp.readyState === 4) {
    if (xmlhttp.status === 200) {
      console.log(xmlhttp.responseText);
    } else {
      console.log("Error", xmlhttp.statusText);
    }
  }
};

if (console.everything === undefined) {
  console.everything = [];

  console.defaultLog = console.log.bind(console);
  console.log = function () {
    let date = new Date();
    console.everything.push({
      type: "log",
      datetime: date.toLocaleDateString("en-GB"),
      value: Array.from(arguments),
      stack: "stack trace disabled for non-error messages",
    });
    console.defaultLog.apply(console, arguments);
  };
  console.defaultError = console.error.bind(console);
  console.error = function () {
    let date = new Date();
    console.everything.push({
      type: "error",
      datetime: date.toLocaleDateString("en-GB"),
      value: Array.from(arguments),
      stack: stackTrace(),
    });
    console.defaultError.apply(console, arguments);
  };
  console.defaultWarn = console.warn.bind(console);
  console.warn = function () {
    let date = new Date();
    console.everything.push({
      type: "warn",
      datetime: date.toLocaleDateString("en-GB"),
      value: Array.from(arguments),
      stack: "stack trace disabled for non-error messages",
    });
    console.defaultWarn.apply(console, arguments);
  };
  console.defaultDebug = console.debug.bind(console);
  console.debug = function () {
    let date = new Date();
    console.everything.push({
      type: "debug",
      datetime: date.toLocaleDateString("en-GB"),
      value: Array.from(arguments),
      stack: "stack trace disabled for non-error messages",
    });
    console.defaultDebug.apply(console, arguments);
  };
}
// get a list of all the file in the md_files directory
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

var saved_cookies = getCookie("saved cookies");
if (saved_cookies == null) {
  saved_cookies = "";
}
var collapsed = [];
var first_time = true;


var current_file = getCookie("current page");
if (current_file == null) {
  current_file = "md_files\\home.md";
}

let url_pg = decodeURIComponent(new URLSearchParams(window.location.search).get("page"));
if (url_pg != null) {
  console.log("urlpg", url_pg)
  current_file = url_pg.replaceAll("\\","/")
}


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

function load_md(file) {
  if (file == null) {
    return;
  }

  
  collapsed = collapsed.filter((a) => !file.includes(a.replaceAll("\\", "/")));

  current_file = file;

  if (file.includes(".md")) {
    let md_block = document.getElementById("md_block");
    if (md_block == null) {
      console.log("md block not yet loaded");
      return;
    }
    md_block.src = file;
    md_block.onchange = () => {
      console.log("change");
    };

    let scripts = md_block.querySelectorAll("script");
    console.log(scripts);
    for (var i = 0; i < scripts.length; i++) {
      console.log("fakeImage: ", scripts[i]);
    }
  }
  setTimeout(() => {
    update_cookies();
  }, 1000);
  update_nav();
}

function setCookie(name, value, days) {
  var expires = "";
  if (days) {
    var date = new Date();
    date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000);
    expires = "; expires=" + date.toUTCString();
  }
  document.cookie = name + "=" + (value || "") + expires + "; path=/";
}
function getCookie(name) {
  var nameEQ = name + "=";
  var ca = document.cookie.split(";");
  for (var i = 0; i < ca.length; i++) {
    var c = ca[i];
    while (c.charAt(0) == " ") c = c.substring(1, c.length);
    if (c.indexOf(nameEQ) == 0) return c.substring(nameEQ.length, c.length);
  }
  return null;
}
var first_loop = true;

function update_cookies() {
  if (saved_cookies.includes("current page" + ",")) {
    setCookie("current page", current_file, 600);
  }

  // code

  if (!current_file.includes("cookies.md")) {
    return;
  }
  let saved_cookies2 = "";
  for (i of document.querySelectorAll("ul li input").values()) {
    let name = i.nextSibling.nodeValue;

    name = name.replaceAll(" : ", "");
    console.log(name);
    name = name.replace(" ", "", 1);
    i.disabled = false;
    if (first_loop) {
      if (saved_cookies.includes(name + ",")) {
        i.checked = true;
      } else {
        i.checked = false;
      }
      i.onclick = () => {
        update_cookies();
      };
    }
    if (i.checked) {
      saved_cookies2 += name + ",";
    } else {
      setCookie(name, "null", 0);
    }
    let sibling = i.nextElementSibling;
    let value = getCookie(name);
    if (value == null) {
      value = "null";
    }
    sibling.innerText = value;
  }

  first_loop = false;

  setCookie("saved cookies", saved_cookies2, 600);
  saved_cookies = saved_cookies2;

  for (i of document.querySelectorAll("ul li").values()) {
    i.style = "list-style-type: none; margin-left:-1em";
  }
}

setInterval(() => {
  update_cookies();
}, 3000);

function stackTrace() {
  var err = new Error();
  return err.stack;
}

function get_meta() {
  let string = "-----------------------\n";

  string += stackTrace();

  string += "\n";

  for (i of console.everything) {
    if (i.type != "log") {
      string += "--------------\n";
      string += i.datetime + " " + i.type + "\n" + i.value;
      string += "\n" + i.stack;
      string += "\n";
    }
  }

  return string;
}


