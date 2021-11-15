#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate syslog;
extern crate serde_json;
extern crate web_view;

use syslog::{Facility, Formatter3164, BasicLogger};
use log::{LevelFilter};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;

use std::panic;

use web_view::*;

const APP_NAME: &str = "MyApp";

/// This struct should contain any state that you
/// need to remember from one button-press to another.
/// You may need to change this to store your own
/// data.
struct AppState {
    your_data_here: ()
}


/// These are the way that the frontend communicates with
/// the backend. Each entry in this allows the frontend
/// to communicate with the backend. You may need to add
/// more, or rename these.
// The derive here means we can convert this enum from a
// JSON object to a Rust struct.
#[derive(Deserialize)]
// This means that if we get an object like
// {cmd: "someAction"}, we convert that into
// an enum Cmd::SomeAction
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    /// Here is an action you can take from the frontend.
    Init,
    /// Here is another actino.
    SomeAction,
    /// Here is an action, but you can also pass along
    /// a string with it.
    Log { text: String },
}


/// This actually creates the frontend. This already contains the
/// settings you'll need from the webview::builder, you can find
/// more details in the webview documentation
fn create_webview(apps: Arc<Mutex<AppState>>) -> web_view::WebView<'static, ()> {
    let mut webview = web_view::builder()
        .title(APP_NAME)
        .content(Content::Html(get_html()))
        .size(1000, 1500)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(move |_, arg| {
            // Whenever webview does an "rpc" (remote procedure call)
            // it will eventually call this handler.
            // Your job here is to deal with the relevant
            // RPC.
            match serde_json::from_str(arg).unwrap() {
                Cmd::Init => (),
                Cmd::SomeAction => {
                    info!("This will perform some action.");
                },
                Cmd::Log { text } => info!("{}", text),
            };
            Ok(())
        })
        .build()
        .unwrap();

    webview.set_color((0, 0, 0));

    return webview;
}

/// This function just sets up logging. You will need to run
/// `tail /var/log/syslog` to see errors this function outputs.
fn create_formatter() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: APP_NAME.into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
            .map(|()| log::set_max_level(LevelFilter::Info)).expect("Logger was not setup.");

    // Whenever a panic occurs, this will be executed.
    // You might need to run the app differently to see
    // the exact error.
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            error!("panic occurred: {:?}", s);
        } else {
            error!("panic occurred");
        }
    }));

}

fn main() {
    create_formatter();

    let data = Arc::new(Mutex::new(AppState {
        your_data_here: ()
    }));

    create_webview(data).run().unwrap();

}


/// This function returns a default html wrapper; but it
/// grabs the compiled css and js from the frontend, and
/// embeds it in the binary. Whenever you build, it grabs
/// the frontend, and puts it into the wrapper. This is what
/// the webview displays.
fn get_html() -> String {
    return format!(
        r#"
		<!doctype html>
		<html>
			<head>
				{styles}
			</head>
			<body>
				<!--[if lt IE 9]>
				<div class="ie-upgrade-container">
					<p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
					<a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
				</div>
				<![endif]-->
				<!--[if gte IE 9 | !IE ]> <!-->
				<div id="app"></div>
				{scripts}
				<![endif]-->
			</body>
		</html>
		"#,
        styles = inline_style(include_str!("frontend_build/bundle.css")),
        scripts = inline_script(include_str!("frontend_build/bundle.js"))
    );
}
fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
