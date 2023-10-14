use console_error_panic_hook;
use js_sys::Function;
use js_sys::Uint8Array;
use std::str;
use wasm_bindgen::prelude::*;
use web_sugars::prelude::*;
use web_sugars::window;
use zune_inflate::DeflateDecoder;

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
    let list = files.iter().filter(|x|!x.contains(".md") && x!=&&"md_files").map(|x|String::from(*x)).collect::<Vec<String>>();

    let mut collapsed = get_collapsed().await?;

    for i in &collapsed {
        files.retain(|x| !x.replace("\\", "/").contains(&format!("{}/", i)));
    }

    let nav = get_element_by_id("nav")?;
    let inner_html = nav.inner_html();

    nav.set_inner_html("");

    for i in 0..files.len() {
        let f = files[i];
        let name = option_to_sugar(f.split("\\").last())?;
        let md = f.contains(".md");
        let horizontal = err_to_sugar(document.create_element("div"))?;
        horizontal.set_class_name(&("horizontal".to_owned() + if inner_html.contains(
            &format!("\"{}\"",f)) {""} else {
                " new"
            })
        );
        let new_element = err_to_sugar(document.create_element("p"))?;
        new_element.set_id(&f);
        new_element.set_text_content(Some(&name.replace(".md","")));

        new_element.set_class_name(&if md {
            "link".to_owned()
        } else {
            "folder ".to_owned()
                + if collapsed.contains(&f.replace("\\", "/")) {
                    "open"
                } else {
                    "close"
                }
        });

        err_to_sugar(new_element
            .add_event_listener_with_callback_and_bool(
                "click",
                &if md {
                    Function::new_no_args("window.load_md(this.id);")
                } else {
                    Function::new_no_args(&format!("window.collapse('{}');", f.replace("\\", "/")))
                },
                true,
            ))?;
        let mut tree_text = " ".repeat((f.split("\\").count() as i32 - 1).max(0) as usize);

        let current_len = f.split("\\").count();
        let next_len = files.get(i + 1).unwrap_or(&"").split("\\").count();

        // check out this gross match statement
        tree_text += match (current_len < next_len, current_len > next_len) {
            _ if current_len == 1 => "",
            (true, true) | (false, false) => "├─",
            (false, true) => "└─",
            (true, false) => "└┬",
        };

        let path_text_element = err_to_sugar(document.create_element("p"))?;
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

pub async fn get_collapsed() -> Result<Vec<String>, WebSysSugarsError> {
    let window = get_window()?;

    let collapsed = window.get("collapsed");


    let list = if let Some(collapsed_string) = collapsed {
        option_to_sugar(collapsed_string.as_string())?
        .as_str().replace("\\", "/")
            .to_owned()
            .split(";")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
        
    }else {
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
    if loc.hash().unwrap_or("x".to_owned()) != file {
        // window.location().set_hash(&file.replace("\\","/"));
        loc.replace(&format!(
            "{}#{}",
            loc.pathname().unwrap_or("error!".to_string()),
            file.replace("\\", "/")
        ));
    }

    let text = match load_gzip(&file).await {
        Ok(a) => a,
        Err(a) => format!("{:?}", a),
    };

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
    }else {
        link.set_inner_html("open external ->");
    }
    md_block.set_inner_html(&text);

    // window()
    update_nav().await?;

    show_content().await?;

    get_element_by_id(&file.replace("/", "\\"))?.set_class_name("link ur-here");
    // md

    return Ok(());
}

#[wasm_bindgen]
pub async fn load_style() -> Result<(), WebSysSugarsError> {
    let style = match load_gzip("css/main.css").await {
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

    err_to_sugar(get_window()?.add_event_listener_with_callback(
        "hashchange",
        &Function::new_with_args(
            "event",
            "console.log('hashchange');window.update_from_hash()",
        ),
    ))?;

    

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
