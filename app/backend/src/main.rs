#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate syslog;
extern crate serde_json;
extern crate web_view;

use syslog::{Facility, Formatter3164, BasicLogger};
use log::{LevelFilter};
//use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
//use std::thread;
use std::process::{Command};

use std::panic;

use web_view::*;

const APP_NAME: &str = "MyApp";

/// This struct should contain any state that you
/// need to remember from one button-press to another.
/// You may need to change this to store your own
/// data.
struct HeadlightState {
    left_indicator_state: bool,
    right_indicator_state: bool,
    day_running_lights_state: bool,
    low_beam_state: bool,
    high_beam_state: bool,
}

/// These are the way that the frontend communicates with
/// the backend. Each entry in this allows the frontend
/// to communicate with the backend. You may need to add
/// more, or rename these.
#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init, //i
    LeftIndicator, //li
    RightIndicator, //ri
    DaylightRunningLights, //do
    LowBeam, //lb
    HighBeam, //hb
    AllOff, //cl
} 

impl Default for HeadlightState {
    fn default () -> HeadlightState {
        HeadlightState {
            left_indicator_state : false,
            right_indicator_state : false,
            day_running_lights_state : false,
            low_beam_state : false,
            high_beam_state : false,
        }
    }
}

/// This actually creates the frontend. This already contains the
/// settings you'll need from the webview::builder, you can find
/// more details in the webview documentation
fn create_webview(apps: Arc<Mutex<HeadlightState>>) -> web_view::WebView<'static, ()> {
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
                Cmd::Init => {
                    println!("____INIT____");
                    HeadlightState::default();
                    println!(HeadlightState{});
                    let process = Command::new("python3 src/embd_headlights_tool/audi_cmd_line.py")
                            //.arg("/dev/cu.Bluetooth-Incoming-Port")
                            .arg("i")
                            .spawn()
                            .expect("failed to execute process 'Initialise Headlights'");
                    Some(process);
                    let mut cur_headlight_state = apps.lock().unwrap();
                    cur_headlight_state.init = true;
                    println!("hello there!");
                },
                Cmd::LeftIndicator => {
                    println!("__LEFT__");
                    let process = Command::new("python3 src/embd_headlights_tool/audi_cmd_line.py")
                            //.arg("/dev/cu.Bluetooth-Incoming-Port")
                            .arg("li")
                            .spawn()
                            .expect("failed to execute process 'Left Indicator'");
                    Some(process);
                    let mut cur_headlight_state = apps.lock().unwrap();
                    cur_headlight_state.left_indicator_state = true;
                }
                Cmd::RightIndicator => {
                    println!("__RIGHT__");
                    let process = Command::new("python3 src/embd_headlights_tool/audi_cmd_line.py")
                            .arg("/dev/cu.Bluetooth-Incoming-Port")
                            .arg("ri")
                            .spawn()
                            .expect("failed to execute process 'Right indicator'");
                    Some(process);
                    let mut cur_headlight_state = apps.lock().unwrap();
                    cur_headlight_state.right_indicator_state = true;
                },
                Cmd::DaylightRunningLights => {
                    println!("__DAY__");
                    let process = Command::new("python3 src/embd_headlights_tool/audi_cmd_line.py")
                            .arg("/dev/cu.Bluetooth-Incoming-Port")
                            .arg("do")
                            .spawn()
                            .expect("failed to execute process 'Daylight running lights'");
                    Some(process);                    
                    let mut cur_headlight_state = apps.lock().unwrap();
                    cur_headlight_state.day_running_lights_state = true;
                },
                Cmd::LowBeam => {
                    println!("__LOW__");
                    let process = Command::new("python3 src/embd_headlights_tool/audi_cmd_line.py")
                            .arg("/dev/cu.Bluetooth-Incoming-Port")
                            .arg("lb")
                            .spawn()
                            .expect("failed to execute process 'low beam lights'");
                    Some(process);
                    let mut cur_headlight_state = apps.lock().unwrap();
                    cur_headlight_state.low_beam_state = true;
                },
                Cmd::HighBeam => {
                    println!("__HIGH__");
                    let process = Command::new("python3 src/embd_headlights_tool/audi_cmd_line.py")
                            .arg("/dev/cu.Bluetooth-Incoming-Port")
                            .arg("hb")
                            .spawn()
                            .expect("failed to execute process 'high beam lights'");
                    Some(process);
                    let mut cur_headlight_state = apps.lock().unwrap();
                    cur_headlight_state.high_beam_state = true;
                },
                Cmd::AllOff => {
                    println!("__OFF__");
                    let process = Command::new("python3 src/embd_headlights_tool/audi_cmd_line.py")
                            .arg("/dev/cu.Bluetooth-Incoming-Port")
                            .arg("cl")
                            .spawn()
                            .expect("failed to execute process 'all off'");
                    Some(process);
                    let mut cur_headlight_state = apps.lock().unwrap();
                    cur_headlight_state.all_off_state = true;
                },
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

    let data = Arc::new(Mutex::new(HeadlightState::default()));

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
