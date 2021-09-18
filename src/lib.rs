// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use chrono;
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {  	
		content: String::new(),
		date:String::new(),
		
		
	}
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    content: String,
	date: String,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
	Hello(String),
	Date(String),
			
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
	Msg::Hello(content) => model.content = content,
	Msg::Date (date) => model.date = date,
    }

}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
	
    vec![
	div![
		div![
		h3![format!("Pre Operative Diagnosis: {}", model.content)],],
		
		div![
		h3![format!("{}", model.date)],],
	],
	div![
	C!["counter"],
        div![
	h3!["Pre Operative Diagnosis"],

	input![
		attrs!{At::Placeholder => "Pre Operative Diagnosis",
		       },
		input_ev(Ev::Input,  Msg::Hello)],],
	br![],
	div![
	input![
		attrs!{At::Type =>"date"},
		input_ev(Ev::Input, Msg::Date)		
			],],



],
	
]
			
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
