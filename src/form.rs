use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn init_form(name: &str, history: &JsValue) {
    let history: Vec<String> = history.into_serde().unwrap();
    let window = web_sys::window().expect("global `window` should exist");
    let document = window.document().expect("should have a `document` on `window`");

    let root = document
        .get_element_by_id("root")
        .expect("page `root` exists");
    root.set_inner_html("");

    let form_node: web_sys::Element = document
        .create_element("form")
        .expect("DOM element to have been created");

    let template: &str = &gen_template(name, &history);
    form_node.set_inner_html(template);
    
    let form_node = add_submit_handler(form_node, history);

    root.append_child(&form_node).expect("`form` to have been appended to `root` node");
}

fn add_submit_handler(form_node: web_sys::Element, mut history: Vec<String>) -> web_sys::Element {
    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        event.prevent_default();

        let target = event.current_target().unwrap();
        let form = target.dyn_ref::<web_sys::HtmlFormElement>().unwrap();
        let data = web_sys::FormData::new_with_form(form).unwrap();

        let name: String = data
            .get("name")
            .as_string()
            .expect("`name` to exist in form data");


        history.push(String::from(&name));
        let js_val_history = &JsValue::from_serde(&history).unwrap();

        init_form(&name, js_val_history);
    }) as Box<dyn FnMut(_)>);

    let cb = closure.as_ref().unchecked_ref();

    form_node
        .add_event_listener_with_callback("submit", cb)
        .expect("`submit_handler` to have been added");
    closure.forget();
    form_node
}

fn gen_template(name: &str, history: &Vec<String>) -> String {
    let history_template: String = history
        .iter()
        .fold(String::new(), |acc, curr| {
            format!("{}<p>{}</p>", acc, curr)
        });

    format!(
        "
            <h1>User Form</h1>
            <label for=\"name\">Name</label>
            <input type=\"text\" id=\"name\" name=\"name\" value=\"{}\">
            <input id=\"submit\" type=\"submit\" value=\"Submit\">
            <section id=\"user-history\">
                {}
            </section>
        ",
        name, history_template,
    )
}