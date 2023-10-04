use console_error_panic_hook;
use js_sys::Function;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Location;
use web_sys::Response;
use web_sys::window;
use web_sys::{Request, RequestInit, RequestMode};
use zune_inflate::DeflateDecoder;
use web_sugars::prelude::*;
use std::str;


// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn console_error_panic_hook_set() {
    console_error_panic_hook::set_once();
    
}

#[wasm_bindgen]
pub fn collapse(mut path: String) {
    if path == "" {
        return;
    }
    path = path.replace("\\", "/");
    let window = get_window().unwrap();
    let mut collapsed = window.get("collapsed").unwrap().to_string().as_string().unwrap_or("unset".to_owned());
    if collapsed.contains(&path) {
        collapsed = collapsed.split(";").filter(|x| !x.contains(path)).collect::<Vec<&str>>().join(";")
    } else {
        collapsed += &format!(";{}", path)
    }
    js_sys::eval(&format!("window.collapsed='{}'",collapsed));
    
    update_nav();
}

/// todo: make this function return a result
#[wasm_bindgen]
pub fn update_nav() -> Option<bool> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let mut files: Vec<&str> = include_str!(r"tree.txt").lines().collect();
    let mut collapsed;
    if window.get("collapsed").is_none() ||  window.get("collapsed")?.to_string().as_string() == Some("unset".to_owned()) {
        collapsed = files.iter().filter(|x|!x.contains(".md")).cloned().collect::<Vec<&str>>().join(";").replace("\\", "/");
        
    }else {
        collapsed = window.get("collapsed")?.to_string().as_string().unwrap_or("unset".to_owned());
    }

    if collapsed == "" {
        collapsed = files.join(";").replace("\\", "/");
    }

    for i in collapsed.split(";") {
        if i == ""{
            continue;
        }
        js_sys::eval(&format!("console.log('{:?} {:?}')",files, i));
        files.retain(|x| !x.replace("\\","/").contains(i));
    }
    
    let nav = document.get_element_by_id("nav")?;
    js_sys::eval(&format!("window.collapsed='{}'",collapsed));
    
    nav.set_inner_html("");

    

    for i in 0..files.len() {
        let f = files[i];
        let name = f.split("\\").last()?;
        let md = f.contains(".md");
        let horizontal = document.create_element("div").ok()?;
        horizontal.set_class_name("horizontal");
        let new_element = document.create_element("p").ok()?;
        new_element.set_id(&f);
        new_element.set_text_content(Some(name));
        
        new_element.set_class_name(&if md {
            "link".to_owned()
        } else {
            "folder ".to_owned()
                + if collapsed.contains(f) {
                    "open"
                } else {
                    "close"
                }
        });

        new_element
            .add_event_listener_with_callback(
                "click",
                &if md {
                    Function::new_no_args("window.update_nav();this.className+=' ur-here';window.load_md(this.id);")
                } else {
                    Function::new_no_args("window.collapse(this.id);")
                },
            )
            .ok()?;
        let mut tree_text = "".to_owned();
        tree_text += &" ".repeat(f.split("\\").count());

        tree_text += match (
            name.contains(".md"),
            if i + 1 < files.len() {
                files[i + 1].contains(".md")
            } else {
                false
            },
        ) {
            (true, true) | (false, false) => "├─",
            (true, false) => "└─",
            (false, true) => "└┬",
        };

        let path_text_element = document.create_element("p").ok()?;
        path_text_element.set_text_content(Some(&tree_text));

        horizontal.append_child(&path_text_element).ok()?;
        horizontal.append_child(&new_element).ok()?;
        nav.append_child(&horizontal).ok()?;
    }

    Some(true)
}

#[wasm_bindgen]
pub async fn load_gzip(file: String) -> Result<String, JsValue> {

    let url = format!("gz\\{}.gz", file);

    let buffer = fetch_as_array_buffer(&url).await?;
    
    let array = Uint8Array::new(&buffer);
    let bytes: Vec<u8> = array.to_vec();
    let mut decoder = DeflateDecoder::new( & bytes);

    let data = decoder.decode_gzip().unwrap();
    let string = str::from_utf8(&data).unwrap();

    return Ok(string.to_owned());
}

#[wasm_bindgen]
pub async fn load_md(file: String) -> Result<(), WebSysSugarsError> {

    if !file.contains(".md") {
        return Err(WebSysSugarsError::NotImplemented);
    }
    let window = get_window()?;
    
    let mut collapsed = window.get("collapsed").unwrap().to_string().as_string().unwrap_or("unset".to_owned());

    collapsed = collapsed.split(";").filter(|a|!a.contains(&file)).collect::<Vec<&str>>().join(";").to_owned();

    
    
    js_sys::eval(&format!("window.collapsed='{}'",collapsed));

    let md_block = get_element_by_id("md_block")?;

    let window = get_window()?;
    if window.location().hash().unwrap_or("x".to_owned()) != file {
        window.location().set_hash(&file.replace("\\","/"));
    }

    let text = match load_gzip(file).await {
        Ok(a) => a,
        Err(a) => format!("{:?}",a),
    };
    
    md_block.set_inner_html(&text);

    // window()
    

    // md

    return Ok(());
}


#[wasm_bindgen]
pub async fn rs_onload() -> Result<(), WebSysSugarsError> {
    
    let hash = get_window()?.location().hash().unwrap_or("md_files\\home.md".to_owned());

    if hash.contains(".md") {
        load_md(hash);
    }

    update_nav();

    return Ok(());
}


#[wasm_bindgen]
pub async fn clicked_scroll() -> Result<(), WebSysSugarsError> {

    let button = get_element_by_id("swipe_info")?;
    if button.text_content() == Some("document ->".to_owned()) {
        show_content().await?;
    }else{
        show_nav().await?;
    }

    Ok(())
}


#[wasm_bindgen]
pub async fn show_nav() -> Result<(), WebSysSugarsError> {

    let slider = get_element_by_id("slider")?;
    slider.set_attribute("style", "animation: show_nav 0.1s ease-in-out forwards");
    let button = get_element_by_id("swipe_info")?;
    button.set_text_content(Some("document ->"));
    Ok(())
}


#[wasm_bindgen]
pub async fn show_content() -> Result<(), WebSysSugarsError> {
    let slider = get_element_by_id("slider")?;
    slider.set_attribute("style", "animation: show_content 0.1s ease-in-out forwards");
    let button = get_element_by_id("swipe_info")?;
    button.set_text_content(Some("<- menu"));

    Ok(())
}
