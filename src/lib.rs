use console_error_panic_hook;
use js_sys::Function;
use js_sys::Uint8Array;
use rust_fuzzy_search::*;
use std::str;
use urlencoding::encode;
use wasm_bindgen::prelude::*;
use web_sugars::prelude::*;
use zune_inflate::DeflateDecoder;

pub mod bird;
pub mod speech;
pub mod voicelines;

use bird::*;
use speech::*;
use voicelines::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn console_error_panic_hook_set() {
    console_error_panic_hook::set_once();
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub async fn collapse(mut path: String) -> Result<(), WebSysSugarsError> {
    console_log!("{}", path);

    if path == "" {
        return Ok(());
    }

    path = path.replace("\\", "/");
    let mut collapsed_list = get_collapsed().await?;

    if collapsed_list.contains(&path) {
        collapsed_list.retain(|x| x != &path);
    } else {
        collapsed_list.push(path);
    }

    set_collapsed(collapsed_list);

    update_nav().await?;

    Ok(())
}

/// todo: make this function return a result
#[wasm_bindgen]
pub async fn update_nav() -> Result<(), WebSysSugarsError> {
    let window = option_to_sugar(web_sys::window())?;
    let document = option_to_sugar(window.document())?;

    let mut files: Vec<&str> = include_str!(r"tree.txt").lines().collect();
    // let list = files
    //     .iter()
    //     .filter(|x| !x.contains(".md") && x != &&"md_files")
    //     .map(|x| String::from(*x))
    //     .collect::<Vec<String>>();

    let collapsed = get_collapsed().await?;

    for i in &collapsed {
        files.retain(|x| !x.replace("\\", "/").contains(&format!("{}/", i)));
    }

    let nav = get_element_by_id("nav")?;
    let inner_html = nav.inner_html();

    nav.set_inner_html(include_str!("search_bar.html"));

    let mut indent_number = (0, 0);

    for i in 0..files.len() {
        let f = files[i];
        if f == "md_files" {
            continue;
        }
        let name = option_to_sugar(f.split("\\").last())?;
        let md = f.contains(".md");
        let horizontal = err_to_sugar(document.create_element("div"))?;
        horizontal.set_class_name(
            &("horizontal".to_owned()
                + if inner_html.contains(&format!("\"{}\"", f)) {
                    ""
                } else {
                    " new"
                }),
        );
        let new_element = err_to_sugar(document.create_element("p"))?;
        new_element.set_id(&f);
        new_element.set_text_content(Some(&name.replace(".md", "")));

        new_element.set_class_name(
            &(if md {
                if include_str!("favorite.txt").contains(&(f.to_owned())) {
                    "favorite link".to_owned()
                } else {
                    "link".to_owned()
                }
            } else {
                "folder ".to_owned()
                    + if collapsed.contains(&f.replace("\\", "/")) {
                        "open"
                    } else {
                        "close"
                    }
            }),
        );

        err_to_sugar(new_element.add_event_listener_with_callback_and_bool(
            "click",
            &if md {
                Function::new_no_args("window.load_md(this.id);")
            } else {
                Function::new_no_args(&format!("window.collapse('{}');", f.replace("\\", "/")))
            },
            true,
        ))?;

        let indent = (f.split("\\").count() as i32 - 1).max(0) as usize;
        let mut tree_text = " ".repeat(indent);

        let current_len = f.split("\\").count();
        let next_len = files.get(i + 1).unwrap_or(&"").split("\\").count();

        tree_text += match (current_len < next_len, current_len > next_len) {
            _ if current_len <= 2 => "",
            (true, true) | (false, false) => "├─",
            (false, true) => "└─",
            (true, false) => "└┬",
        };
        // if indent_number.0 != indent {
        //     indent_number.0 = indent;
        //     indent_number.1 = 1;
        // } else {
        //     indent_number.1 += 1;
        // }
        // tree_text += &format!("{}.{}.",indent, indent_number.1);
        // tree_text += match (current_len < next_len, current_len > next_len) {
        //     _ if current_len <= 2 => "",
        //     (true, true) | (false, false) => "",
        //     (false, true) => "",
        //     (true, false) => "",
        // };

        let path_text_element = err_to_sugar(document.create_element("p"))?;
        path_text_element.set_class_name(&format!("path path-{current_len}"));
        path_text_element.set_text_content(Some(&tree_text));

        err_to_sugar(horizontal.append_child(&path_text_element))?;
        err_to_sugar(horizontal.append_child(&new_element))?;
        err_to_sugar(nav.append_child(&horizontal))?;
    }

    Ok(())
}

#[wasm_bindgen]
pub async fn load_gzip(file: &str) -> Result<String, JsValue> {
    let url = format!("gz\\{}.gz", file);

    let buffer = fetch_as_array_buffer(&url).await?;

    let array = Uint8Array::new(&buffer);
    let bytes: Vec<u8> = array.to_vec();
    let mut decoder = DeflateDecoder::new(&bytes);

    let data = decoder.decode_gzip().unwrap_or(vec![]);
    let string = str::from_utf8(&data).unwrap_or("invalid data");

    return Ok(string.to_owned());
}

fn err_to_sugar<A, T: std::fmt::Debug>(a: Result<A, T>) -> Result<A, WebSysSugarsError> {
    match a {
        Ok(b) => Ok(b),
        Err(b) => Err(WebSysSugarsError::ElementIdIsNotFound(format!(
            "---JS ERROR---\n js error:{:?} ---JS ERROR---",
            b
        ))),
    }
}
fn option_to_sugar<T>(a: Option<T>) -> Result<T, WebSysSugarsError> {
    match a {
        Some(b) => Ok(b),
        None => Err(WebSysSugarsError::ElementIdIsNotFound(format!(
            "---Null value error---"
        ))),
    }
}

pub async fn get_show_search() -> Result<Vec<String>, WebSysSugarsError> {
    let window = get_window()?;

    let collapsed = window.get("show_search");

    let list = if let Some(collapsed_string) = collapsed {
        option_to_sugar(collapsed_string.as_string())?
            .as_str()
            .replace("\\", "/")
            .to_owned()
            .split(";")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
    } else {
        vec![]
    };

    return Ok(list);
}

pub async fn get_collapsed() -> Result<Vec<String>, WebSysSugarsError> {
    let window = get_window()?;

    let collapsed = window.get("collapsed");

    let list = if let Some(collapsed_string) = collapsed {
        option_to_sugar(collapsed_string.as_string())?
            .as_str()
            .replace("\\", "/")
            .to_owned()
            .split(";")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
    } else {
        let files: Vec<&str> = include_str!(r"tree.txt").lines().collect();

        files
            .into_iter()
            .filter(|x| !x.contains(".md") && x != &"md_files")
            .map(|x| String::from(x).replace("\\", "/"))
            .collect::<Vec<String>>()
    };

    return Ok(list);
}

// #[wasm_bindgen]
pub fn set_collapsed(collapsed: Vec<String>) {
    js_sys::eval(&format!("window.collapsed='{}'", collapsed.join(";")));
}

#[wasm_bindgen]
pub async fn load_md(mut file: String) -> Result<(), WebSysSugarsError> {
    if !file.contains(".md") {
        return Err(WebSysSugarsError::NotImplemented);
    }

    file = file.replace("/", "\\");

    // let window = get_window()?;

    let mut collapsed_list = get_collapsed().await?;

    let path = file
        .split("\\")
        .filter(|x| !x.contains(".md"))
        .collect::<Vec<&str>>()
        .join("/");

    collapsed_list.retain(|x| !path.replace("\\", "/").contains(x) && x != &"");

    set_collapsed(collapsed_list);

    let md_block = get_element_by_id("md_block")?;

    let window = get_window()?;
    let loc = window.location();

    let location = loc.hash().unwrap_or("".to_string());

    add_history(location).await?;

    if loc.hash().unwrap_or("x".to_owned()) != file {
        // window.location().set_hash(&file.replace("\\","/"));
        loc.replace(&format!(
            "{}#{}",
            loc.pathname().unwrap_or("error!".to_string()),
            file.replace("\\", "/")
        ));
    }

    let mut text = match load_gzip(&file).await {
        Ok(a) => a,
        Err(a) => format!("{:?}", a),
    };

    let name = file
    .split("\\")
    .last()
    .unwrap_or("invalid file")
    .replace(".md", "");

// console_log!("You have opened {name}");

    let link = get_element_by_id("link_to_external")?;

    err_to_sugar(link.set_attribute(
        "href",
        &format!(
            "https://ollielynas.github.io/md-website/sub/{}",
            file.replace("\\", "/").replace(".md", ".html")
        ),
    ))?;
    if text.contains("no index") {
        link.set_inner_html("");
        link.set_attribute("style", "display: none");
    } else {
        link.set_attribute("style", "display: auto");
        link.set_inner_html(include_str!("open_in_external.svg"));
    }

    let local_link = if text.contains("no index") {
        format!(
            "https://ollielynas.github.io/md-website/#{}",
            file.replace("\\", "/").replace(" ", "%20")
        )
    } else {
        format!(
            "https://ollielynas.github.io/md-website/sub/{}?redirect=true",
            file.replace("\\", "/")
                .replace(".md", ".html")
                .replace(" ", "%20")
        )
    };

    let link_internal = get_element_by_id("link_to_internal")?;

    err_to_sugar(link_internal.set_attribute(
        "onclick",
        &format!("navigator.clipboard.writeText('{local_link}');alert('copied link!');"),
    ))?;
    let link_twitter = get_element_by_id("link_to_twitter")?;
    err_to_sugar(link_twitter.set_attribute(
        "href",
        &format!(
            "https://twitter.com/intent/tweet?prefiltext&url={}",
            encode(&local_link)
        ),
    ))?;
    let link_reddit = get_element_by_id("link_to_reddit")?;
    err_to_sugar(link_reddit.set_attribute(
        "href",
        &format!(
            "https://www.reddit.com/r/test/submit?title={name}&url={}",
            encode(&local_link)
        ),
    ))?;

    // let bookmark = include_str!("bookmark.html");
    match file.as_str() {
        "md_files\\home.md" | "md_files\\portfolio\\index.md" => {
            let fav = include_str!("favorite.txt")
        .lines()
        .map(|x| {
            if x.ends_with(".md") {
                format!(
                    "<li><a id = '{}' class='link' onclick = 'window.load_md(this.id);'>{}</a></li>",
                    x.replace("\\", "/"),
                    x.split("\\").last().unwrap_or("error!").replace(".md", ""),
                )
            }else {format!("<p>{x}<p>\n",)}
        })
        .collect::<String>();

            text = text.replace("loading starred projects...", &format!("<ul>{fav}</ul>"));
        }
        _ => {}
    }
    if text.starts_with("JsValue") {
        text = format!(
            "<h1>{}</h1>\n<i>an error occured while trying to load page</i>\n<br><br>",
            name
        ) + &text;
    }
    md_block.set_inner_html(&text);

    // window()
    update_nav().await?;

    show_content().await?;

    let link = get_element_by_id(&file.replace("/", "\\"))?;
    let class = link.class_name();
    link.set_class_name(
        &class
            .replace("link ur-here", "link")
            .replace("link", "link ur-here"),
    );
    // md

    // let bookmarks = md_block.get_elements_by_class_name("bookmark");

    // startle_bird().await?;
    read_message(&format!("You Have Opened {name}")).await?;

    js_sys::eval("renderMathInElement(document.getElementById('md_block'))");
    return Ok(());
}

#[wasm_bindgen]
pub async fn load_style() -> Result<(), WebSysSugarsError> {
    let style = match load_gzip("css/main.css").await {
        Ok(a) => a,
        Err(a) => format!("body:after {{content: '{:?}'}}", a),
    } + &match load_gzip("css/bird.css").await {
        Ok(a) => a,
        Err(a) => format!("body:after {{content: '{:?}'}}", a),
    };

    get_element_by_id("main-style")?.set_inner_html(&style);

    Ok(())
}

#[wasm_bindgen]
pub async fn update_from_hash() -> Result<(), WebSysSugarsError> {
    let mut hash = get_window()?
        .location()
        .hash()
        .unwrap_or("md_files/home.md".to_owned());

    if hash.contains(".md") {
        hash = hash.replace("#", "").replace("%20", " ");
        load_md(hash).await?;
    } else {
        load_md("md_files/home.md".to_owned()).await?;
    }

    Ok(())
}

#[wasm_bindgen]
pub async fn rs_onload() -> Result<(), WebSysSugarsError> {
    // collapse("md_files".to_owned());
    update_nav().await?;
    update_from_hash().await?;

    // err_to_sugar(get_window()?.add_event_listener_with_callback(
    //     "hashchange",
    //     &Function::new_with_args(
    //         "event",
    //         "console.log('hashchange');window.update_from_hash()",
    //     ),
    // ))?;

    return Ok(());
}

#[wasm_bindgen]
pub async fn clicked_scroll() -> Result<(), WebSysSugarsError> {
    let button = get_element_by_id("swipe_info")?;
    if button.text_content() == Some("document ->".to_owned()) {
        show_content().await?;
    } else {
        show_nav().await?;
    }

    Ok(())
}

#[wasm_bindgen]
pub async fn show_nav() -> Result<(), WebSysSugarsError> {
    let slider = get_element_by_id("slider")?;
    err_to_sugar(slider.set_attribute("style", "animation: show_nav 0.1s ease-in-out forwards"))?;
    let button = get_element_by_id("swipe_info")?;
    button.set_text_content(Some("document ->"));
    Ok(())
}

#[wasm_bindgen]
pub async fn show_content() -> Result<(), WebSysSugarsError> {
    let slider = get_element_by_id("slider")?;
    err_to_sugar(
        slider.set_attribute("style", "animation: show_content 0.1s ease-in-out forwards"),
    )?;
    let button = get_element_by_id("swipe_info")?;
    button.set_text_content(Some("<- menu"));

    Ok(())
}

#[wasm_bindgen]
pub async fn search_results(mut input: String) {
    input = input.to_lowercase();
    let mut search_results = "<ul>".to_owned();

    let mut results = include_str!("tree.txt")
        .lines()
        .filter(|s| s.ends_with(".md"))
        .map(|s| {
            (
                s.split("\\")
                    .last()
                    .unwrap_or("error no \\\\ found")
                    .replace(".md", ""),
                s.to_owned(),
                fuzzy_compare(
                    &s.replace(".md", "").replace("\\", " ").to_lowercase(),
                    &input,
                ),
            )
        })
        .collect::<Vec<(String, String, f32)>>();

    results.sort_by(|a, b| b.2.total_cmp(&a.2));
    let mut show: Vec<String> = vec![];
    for i in 0..10.min(results.len()) {
        if results[i].2 == 0.0 {
            break;
        }
        if results[i].2 < results[0].2 * 0.5 {
            break;
        }
        show.push(results[i].1.replace("\\", "/"));

        search_results.push_str(&format!(
            "<li><a aria-label='view this content as a standalone page' id = '{}' class='link' onclick = 'window.load_md(this.id);'>{}</a></li>",
            results[i].1.replace("\\", "/"),
            results[i].0,
        ));
    }
    search_results.push_str("</ul>");

    // js_sys::eval(&format!("window.show_search='{}'", show.join(";")));
    get_element_by_id("results")
        .unwrap()
        .set_inner_html(&(search_results));
}
#[wasm_bindgen]
pub async fn search_results_big(mut input: String) {
    input = input.to_lowercase();
    let mut search_results = "<ul>".to_owned();

    let mut results = include_str!("tree.txt")
        .lines()
        .filter(|s| s.ends_with(".md"))
        .map(|s| {
            (
                s.split("\\")
                    .last()
                    .unwrap_or("error no \\\\ found")
                    .replace(".md", ""),
                s.to_owned(),
                fuzzy_compare(
                    &s.replace(".md", "").replace("\\", " ").to_lowercase(),
                    &input,
                ),
            )
        })
        .collect::<Vec<(String, String, f32)>>();

    results.sort_by(|a, b| b.2.total_cmp(&a.2));
    let mut show: Vec<String> = vec![];
    for i in 0..20.min(results.len()) {
        if results[i].2 == 0.0 {
            break;
        }
        show.push(results[i].1.replace("\\", "/"));

        search_results.push_str(&format!(
            "<h2 style='font-size: 2em; margin-bottom: 0'>{}</h2><a class='link' href=\"https://ollielynas.github.io/md-website/#{}\">{}</a><p class='search-res' id=\"{}\"><p>",
            results[i].0,
            results[i].1.replace("\\", "/"),
            results[i].1.replace("\\", "/").replace(".md", ""),
            results[i].1,
        ));
    }
    search_results.push_str("</ul>");

    // js_sys::eval(&format!("window.show_search='{}'", show.join(";")));
    get_element_by_id("results-big")
        .unwrap()
        .set_inner_html(&(search_results));
}

pub async fn get_history() -> Result<(usize, Vec<String>), WebSysSugarsError> {
    let body = get_body()?;

    let mut history: Vec<String> = match body.get_attribute("history") {
        Some(a) => a.split(",").map(|x| x.to_owned()).collect::<Vec<String>>(),
        None => {
            body.set_attribute("history", "");
            vec![]
        }
    };
    history.retain(|x| x != "");

    let index = body
        .get_attribute("history-index")
        .unwrap_or("0".to_string())
        .parse::<usize>()
        .unwrap_or(0);

    return Ok((index, history));
}
pub async fn add_history(location: String) -> Result<(), WebSysSugarsError> {
    let body = get_body()?;
    let (mut index, mut history) = get_history().await?;
    history.truncate(index);
    history.push(location);
    history.dedup();
    index = history.len();

    if index <= 1 {
        get_element_by_id("back-arrow")?.set_attribute("disabled", "true");
    } else {
        get_element_by_id("back-arrow")?.set_attribute("disabled", "false");
    }
    if index >= history.len() {
        get_element_by_id("forward-arrow")?.set_attribute("disabled", "true");
    } else {
        get_element_by_id("forward-arrow")?.set_attribute("disabled", "false");
    }

    body.set_attribute("history", &history.join(","));
    body.set_attribute("history-index", &index.to_string());

    return Ok(());
}

#[wasm_bindgen]
pub async fn back_arrow() -> Result<(), WebSysSugarsError> {
    let body = get_body()?;
    let (mut index, history) = get_history().await?;

    if index > 1 {
        index -= 1;
    } else {
        return Ok(());
    }

    let _ = load_md(history[index - 1].clone().replace("#", "")).await;

    if index <= 1 {
        get_element_by_id("back-arrow")?.set_attribute("disabled", "true");
    } else {
        get_element_by_id("back-arrow")?.set_attribute("disabled", "false");
    }
    if index >= history.len() {
        get_element_by_id("forward-arrow")?.set_attribute("disabled", "true");
    } else {
        get_element_by_id("forward-arrow")?.set_attribute("disabled", "false");
    }
    body.set_attribute("history-index", &index.to_string());
    body.set_attribute("history", &history.join(","));

    return Ok(());
}
#[wasm_bindgen]
pub async fn forward_arrow() -> Result<(), WebSysSugarsError> {
    let body = get_body()?;
    let (mut index, mut history) = get_history().await?;

    if index >= history.len() {
        return Ok(());
    }

    load_md(history[index].clone().replace("#", "").replace("%20", " ")).await?;

    if index < history.len() {
        index += 1;
    }

    if index <= 1 {
        get_element_by_id("back-arrow")?.set_attribute("disabled", "true");
    } else {
        get_element_by_id("back-arrow")?.set_attribute("disabled", "false");
    }
    if index >= history.len() {
        get_element_by_id("forward-arrow")?.set_attribute("disabled", "true");
    } else {
        get_element_by_id("forward-arrow")?.set_attribute("disabled", "false");
    }

    body.set_attribute("history", &history.join(","));
    body.set_attribute("history-index", &index.to_string());
    return Ok(());
}
