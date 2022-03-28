use crate::config::PresenceOptions;
use std::collections::HashMap;
use std::fs::File;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::sync::{
    mpsc::{Receiver, SyncSender},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

use rpi_led_matrix::{LedCanvas, LedColor, LedFont};
use scraper::{Html, Selector};
use ureq;

pub struct PresenceWidget {
    position: (i32, i32),
    size: (i32, i32),
    font: LedFont,
    username: Arc<String>,
    password: Arc<String>,
    active_users: Vec<String>,
    user_colors: HashMap<String, LedColor>,
    main_color: LedColor,
}

impl PresenceWidget {
    pub fn new(position: (i32, i32), size: (i32, i32), config: &PresenceOptions) -> Self {
        PresenceWidget {
            position: position,
            size: size,
            font: LedFont::new(Path::new(&config.font_path)).expect("Failed to load font"),
            username: Arc::new(config.username.clone()),
            password: Arc::new(config.password.clone()),
            active_users: Default::default(),
            user_colors: config
                .user_colors
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            main_color: config.main_color.into(),
        }
    }

    pub fn render(&mut self, canvas: &mut LedCanvas, chan: &Receiver<Vec<String>>) {
        // let active_names: String = self.active_users.lock().unwrap().join("\n");
        let (sx, sy) = self.size;
        let (px, py) = self.position;

        if let Ok(active_users) = chan.try_recv() {
            self.active_users = active_users;
        }

        for (i, user) in self.active_users.iter().enumerate() {
            canvas.draw_text(
                &self.font,
                &user,
                px,
                py + 7 * (i as i32 + 1),
                self.user_colors.get(user).unwrap_or(&self.main_color),
                0,
                false,
            );
        }
    }

    pub fn start_thread(
        &self,
        user_devices: HashMap<String, String>,
        chan: SyncSender<Vec<String>>,
    ) {
        thread::spawn({
            let username = Arc::clone(&self.username);
            let password = Arc::clone(&self.password);
            move || {
                let mut cookie = Self::get_cookie(&username, &password);
                let mut loops: u32 = 0;

                loop {
                    if loops == 60 {
                        loops = 0;
                        cookie = Self::get_cookie(&username, &password);
                    }

                    let mut out = vec![];
                    for device in Self::get_devices(&cookie) {
                        if let Some(user) = user_devices.get(&device) {
                            out.push(user.clone());
                        }
                    }

                    // println!("\nactive users: {:?}", out);
                    chan.send(out).unwrap();

                    loops += 1;
                    thread::sleep(Duration::new(5, 0));
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
