
document.getElementById("content").scrollIntoView({
  behavior: "smooth",
  block: "end",
  inline: "nearest",
});

// var collapsed=[]

function stackTrace() {
  var err = new Error();
  return err.stack;
}

var looking_at_page = true;

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

function getTouches(evt) {
  return (
    evt.touches || // browser API
    evt.originalEvent.touches
  ); // jQuery
}

function handleTouchStart(evt) {
  const firstTouch = getTouches(evt)[0];
  xDown = firstTouch.clientX;
  yDown = firstTouch.clientY;
}

function handleTouchMove(evt) {
  if (!xDown || !yDown) {
    return;
  }

  var xUp = evt.touches[0].clientX;
  var yUp = evt.touches[0].clientY;

  var xDiff = xDown - xUp;
  var yDiff = yDown - yUp;

  if (Math.abs(xDiff) > Math.abs(yDiff)) {
    /*most significant*/
    if (xDiff > 0) {
      show_content();
    } else {
      show_nav();
    }
  } else {
    if (yDiff > 0) {
      /* down swipe */
    } else {
      /* up swipe */
    }
  }
  /* reset values */
  xDown = null;
  yDown = null;
}

function show_content() {
  document.getElementById("content").scrollIntoView({
    behavior: "smooth",
    block: "start",
    inline: "nearest",
  });
  document.getElementById("swipe_info").innerText = "<- menu";
}

function show_nav() {
  document.getElementById("nav").scrollIntoView({
    behavior: "smooth",
    block: "start",
    inline: "end",
  });
  document.getElementById("swipe_info").innerText = "document ->";
}

// document.addEventListener("DOMContentLoaded", function () {
//   console.log(location.hash);
//   let hash = location.hash
//     .replaceAll("%20", " ")
//     .replace("#", "")
//     .replaceAll("/", "\\");
//   if (location.hash.includes(".md")) {
//     console.log(hash);
//     document.getElementById("md_block").src = hash;
//   }
// });

var xDown = null;
var yDown = null;

document.addEventListener("touchstart", handleTouchStart, false);
document.addEventListener("touchmove", handleTouchMove, false);

function clicked_scroll() {
  if (document.getElementById("swipe_info").innerText == "<- menu") {
    show_nav();
  } else {
    show_content();
  }
}



function load_md(file) {
  if (file == null) {
    return;
  }

  collapsed = collapsed.filter((a) => !file.includes(a.replaceAll("\\", "/")));

  if (file.includes(".md")) {
    // show_nav()
    current_file = file;
    location.hash = file.replaceAll("\\", "/");

    load_gzip(file).then((text) => {
      document.getElementById("md_block").mdContent = text;
    }, (text)=>{
      document.getElementById("md_block").mdContent = text;
    })

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


window.addEventListener("popstate", function (event) {
  let hash = location.hash
    .replaceAll("%20", " ")
    .replace("#", "")
    .replaceAll("/", "\\");
  if (location.hash.includes(".md")) {
    console.log("lodeing page",hash);
    load_md(hash);
  }
});


console.log(location.hash);
let hash = location.hash
  .replaceAll("%20", " ")
  .replace("#", "")
  .replaceAll("/", "\\");
if (hash.includes(".md")) {
  load_md(hash);
}else{
  load_md("md_files\\home.md");
}

// update_nav();

