use js_sys::Function;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sugars::{
    document::{get_document, get_element_by_id, get_body},
    error::WebSysSugarsError,
};
use web_sys::{Element, Node};

use crate::err_to_sugar;

use fastrand;

#[wasm_bindgen]
pub async fn on_finish_animation() -> Result<(), WebSysSugarsError> {
    let bird = get_element_by_id("bird")?;
    match (bird.get_attribute("goal").unwrap_or("no_goal".to_string())).as_str() {
        "land" => {
            err_to_sugar(bird.set_attribute("animation", "rest"))?;
        }
        _ => {
            err_to_sugar(bird.set_attribute("animation", "flying"))?;
        }
    }

    Ok(())
}

#[wasm_bindgen]
pub async fn startle_bird() -> Result<(), WebSysSugarsError> {
    let bird = get_element_by_id("bird")?;

    

    let body = get_body()?;

    let bird_rect = bird.get_bounding_client_rect();

    let bird_x = bird_rect.x();
    let bird_y = bird_rect.y();

    


    let x = ((fastrand::f64() as f64 - 0.5) * 300.0 + bird_x).min(body.client_width() as f64 - 20.0).max(20.0);
    let y = ((fastrand::f64() as f64 - 0.5) * 300.0 + bird_y).min(body.client_height() as f64 - 20.0).max(20.0);

    

    err_to_sugar(bird.set_attribute("animation", "flying"))?;
    err_to_sugar(bird.set_attribute("goal", "hover"))?;
    err_to_sugar(bird.set_attribute("startle", "2"))?;
    

    if bird_x > x {
        err_to_sugar(bird.set_attribute("flip", "true"))?;
    }else {
        err_to_sugar(bird.set_attribute("flip", "false"))?;
    }

        let direction = (bird_x - x).abs() / (bird_y - y).abs();

    if bird_y < y {
        if direction < 1.0 {
            err_to_sugar(bird.set_attribute("animation", "float"))?;
        } else if direction < 1.9 {
            err_to_sugar(bird.set_attribute("animation", "soar"))?;

        }
    }

    let time = ((bird_x - x).abs().powi(2) + (bird_y - y).abs().powi(2)).sqrt()/50.0;

    err_to_sugar(bird.set_attribute("style", 
    &format!(
        "top: calc( {y}px - 1.5em ); left: {x}px;transition: transform .3s ease-in-out, top {time}s linear, left {time}s linear;"
    )))?;

    // let x = landing_node



    return Ok(());
}

#[wasm_bindgen]
pub async fn update_bird_target_location() -> Result<(), WebSysSugarsError> {
    let bird = get_element_by_id("bird")?;
    let document = get_document()?;

    let targets_node_list = err_to_sugar(document.query_selector_all("#content .link , #nav .link, h1, h2,h3,#content p,#content li, img, iframe"))?;

    let nodes = (0..targets_node_list.length())
        .map(|x| targets_node_list.get(x))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<Node>>();

    // nodes.retain()

    let landing_node: Node = match fastrand::choice(nodes) {
        Some(n) => n,
        None => return Ok(()),
    };

    let element = match landing_node.dyn_into::<Element>() {
        Ok(element) => element,
        Err(e) => return err_to_sugar(Err(e)),
    };
    let rect = element.get_bounding_client_rect();

    let x = rect.x() + fastrand::f64() * rect.width() * 0.5;
    // let x = rect.x();
    let y = rect.y();


    // err_to_sugar(element.set_attribute("style", "border: 1px black solid"))?;

    err_to_sugar(bird.set_attribute("animation", "flying"))?;
    err_to_sugar(bird.set_attribute("goal", "land"))?;

    let bird_rect = bird.get_bounding_client_rect();

    let bird_x = bird_rect.x();
    let bird_y = bird_rect.y();

    

    let body = get_body()?;
    
    if x >= body.client_width() as f64 || y >= body.client_height() as f64 || x <=0.0 || y <= 0.0 {
        startle_bird().await?;
        return Ok(()); 
    }
    if bird_x > x {
        err_to_sugar(bird.set_attribute("flip", "true"))?;
    }else {
        err_to_sugar(bird.set_attribute("flip", "false"))?;
    }

    let time = ((bird_x - x).abs().powi(2) + (bird_y - y).abs().powi(2)).sqrt()/70.0;

    let direction = (bird_x - x).abs() / (bird_y - y).abs();

    if bird_y < y {
        if direction < 1.0 {
            err_to_sugar(bird.set_attribute("animation", "float"))?;
        } else if direction < 1.9 {
            err_to_sugar(bird.set_attribute("animation", "soar"))?;
        }
    }

    err_to_sugar(bird.set_attribute("style", 
    &format!(
        "top: calc( {y}px - 4em ); left: calc( {x}px );transition:top {time}s linear, left {time}s linear;"
    )))?;

    // let x = landing_node

    Ok(())
}
