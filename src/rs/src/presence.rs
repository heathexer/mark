use crate::config::PresenceOptions;
use std::collections::HashMap;
use std::fs::File;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rpi_led_matrix::{LedCanvas, LedFont, LedMatrix, LedMatrixOptions, LedRuntimeOptions};
use scraper::{Html, Selector};
use ureq;

pub struct PresenceWidget {
    position: (usize, usize),
    size: (usize, usize),
    font: LedFont,
    username: Arc<String>,
    password: Arc<String>,
    active_users: Arc<Mutex<Vec<String>>>,
}

impl PresenceWidget {
    pub fn new(position: (usize, usize), size: (usize, usize), config: &PresenceOptions) -> Self {
        PresenceWidget {
            position: position,
            size: size,
            font: LedFont::new(Path::new(&config.font_path)).expect("Failed to load font"),
            username: Arc::new(config.username.clone()),
            password: Arc::new(config.password.clone()),
            active_users: Default::default(),
        }
    }

    pub fn render(&self, canvas: &mut LedCanvas) {}

    pub fn start_thread(&self, user_devices: HashMap<String, String>) {
        thread::spawn({
            let username = Arc::clone(&self.username);
            let password = Arc::clone(&self.password);
            let active_users = Arc::clone(&self.active_users);
            move || {
                let mut cookie = Self::get_cookie(&username, &password);

                let mut loops: u32 = 0;
                loop {
                    if loops == 30 {
                        loops = 0;
                        cookie = Self::get_cookie(&username, &password);
                    }

                    let mut out = vec![];
                    for device in Self::get_devices(&cookie) {
                        if let Some(user) = user_devices.get(&device) {
                            out.push(user.clone());
                        }
                    }

                    println!("\nactive users: {:?}", out);

                    let mut active_users = active_users.lock().unwrap();

                    *active_users.deref_mut() = out;

                    loops += 1;
                    thread::sleep(Duration::new(10, 0));
                }
            }
        });
    }

    fn get_cookie(router_user: &str, router_pass: &str) -> String {
        let resp = ureq::builder()
            .redirects(0)
            .build()
            .post("http://10.0.0.1/check.jst")
            .send_form(&[("username", router_user), ("password", router_pass)])
            .unwrap();

        let cookie = resp
            .header("Set-Cookie")
            .unwrap()
            .strip_suffix(";")
            .unwrap();

        eprintln!("Got cookie: {}", cookie);

        cookie.to_string()
    }

    fn get_devices(cookie: &str) -> Vec<String> {
        let resp = ureq::builder()
            .redirects(0)
            .build()
            .get("http://10.0.0.1/at_a_glance.jst")
            .set("Cookie", &cookie)
            .call()
            .unwrap();

        let resp = resp.into_string().unwrap();

        //io::stderr().write_all(&resp.clone().into_bytes());

        let document = Html::parse_document(&resp);

        let selector = Selector::parse("#internet-usage .readonlyLabel").unwrap();

        document
            .select(&selector)
            .map(|element| element.inner_html())
            .collect()
    }
}
