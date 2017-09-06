extern crate gtk;

use gtk::prelude::*;
//use gtk::{Button, Window, WindowType, Box};

use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

fn create_profile(profile_name: &str, path: &Path) -> FirefoxProfile {
    let profile = FirefoxProfile::new(
        profile_name,
        &format!("{}/{}", path.display(), profile_name)
    );

    let new_profile_opts = &format!("{} {}", profile.profile_name, profile.profile_dir);
    Command::new("firefox")
        .args(&["-CreateProfile", new_profile_opts])
        .output()
        .expect("firefox not found");
    println!("New profile create at: {}", profile.profile_dir);
    profile
}

fn get_profiles(path: &Path) -> HashSet<FirefoxProfile> {
    let mut profiles: HashSet<FirefoxProfile> = HashSet::new();

    println!("List of avaliable profiles: ");
    for path in fs::read_dir(&path).unwrap() {
        let un_path = path.unwrap();
        profiles.insert(FirefoxProfile::new(
            &un_path.file_name().to_str().unwrap(),
            &un_path.path().to_str().unwrap()
        ));
        println!("{}", &un_path.path().display());
    }
    profiles
}


fn get_custom_firefox_profiles_path() -> PathBuf {
    let mut profiles_path = env::current_dir().unwrap();
    profiles_path.push("firefox_profiles");
    profiles_path
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct FirefoxProfile {
    profile_name: String,
    profile_dir: String,
}
//
//struct FirefoxBrowser<FirefoxProfile, T> -> T{
//firefox_profile: FirefoxProfile,
//}

//impl<F, T> FirefoxBrowser<F, T> where F: Fn() -> T {
//    fn new(firefox_profile: F) -> Self {
//        FirefoxBrowser { firefox_profile }
//    }
//    pub fn open(&self, url: &str) {
//        Command::new("firefox")
//            .args(&[url, "-no-remote", "-P", &self.firefox_profile.profile_name])
//            .spawn();
//    }
//}

impl FirefoxProfile {
    fn new(profile_name: &str, profile_dir: &str) -> FirefoxProfile {
        FirefoxProfile {
            profile_name: profile_name.to_string(),
            profile_dir: profile_dir.to_string()
        }
    }
}

#[derive(Debug)]
struct CreatingProfileUI {
    window: gtk::MessageDialog,
    profile_name_entry: gtk::Entry,
    add_btn: gtk::Button,
    cancel_btn: gtk::Button,
}

impl CreatingProfileUI {
    fn hide_window(self) {
        self.window.hide();
        self.profile_name_entry.set_text("")
    }
}

#[derive(Debug)]
struct DataStore {
    //    firefox_profiles_path: PathBuf,
    profiles: HashSet<FirefoxProfile>,
    model: gtk::ListStore,
}

#[derive(Debug)]
struct UI {
    window: gtk::Window,

    // Header
    url_entry: gtk::Entry,
    open_btn: gtk::Button,

    // Body
    add_profile_btn: gtk::Button,
    remove_profile_btn: gtk::Button,
    profiles_textview: gtk::TextView,

    // Creating profile
    creating_profile: CreatingProfileUI,

    // Statusbar
    statusbar: gtk::Statusbar,

    data: DataStore,
    //    profiles: Option<HashSet<FirefoxProfile>>,
}

impl UI {
    fn new(glade_src: &str) -> Rc<RefCell<Self>> {
        let builder = gtk::Builder::new();
        builder.add_from_string(glade_src);

        let firefox_profiles_path = get_custom_firefox_profiles_path();

        let ui: Rc<RefCell<UI>> = Rc::new(RefCell::new(UI {
            window: builder.get_object("main_window").unwrap(),
            url_entry: builder.get_object("url_input").unwrap(),
            open_btn: builder.get_object("go_url_button").unwrap(),
            add_profile_btn: builder.get_object("add_profile").unwrap(),
            remove_profile_btn: builder.get_object("remove_profile").unwrap(),
            profiles_textview: builder.get_object("profiles_textview").unwrap(),
            creating_profile: CreatingProfileUI {
                window: builder.get_object("creating_profile").unwrap(),
                profile_name_entry: builder.get_object("profile_name_input").unwrap(),
                add_btn: builder.get_object("creating_profile_add_button").unwrap(),
                cancel_btn: builder.get_object("creating_profile_cancel_button").unwrap(),
            },
            statusbar: builder.get_object("status_bar").unwrap(),
            data: DataStore {
                profiles: get_profiles(&firefox_profiles_path),
                //                firefox_profiles_path: firefox_profiles_path,
            }
        }));

        {
            let ui_cloned = ui.clone();
            ui.borrow().add_profile_btn.connect_clicked(move |_| {
                ui_cloned.borrow().creating_profile.window.run();
            });
        }
        {
            let ui_cloned = ui.clone();
            ui.borrow().creating_profile.add_btn.connect_clicked(move |_| {
                let profile_name = ui_cloned.borrow().creating_profile
                    .profile_name_entry.get_text().unwrap();
                create_profile(&profile_name, &firefox_profiles_path);
                ui_cloned.borrow().creating_profile.hide_window();
                //                ui_cloned.borrow().creating_profile.profile_name_entry.set_text("");
                //                ui_cloned.borrow().creating_profile.window.hide();
            });
        }
        {
            let ui_cloned = ui.clone();
            ui.borrow().creating_profile.cancel_btn.connect_clicked(move |_| {
                ui_cloned.borrow().creating_profile.hide_window();
                //                ui_cloned.borrow().creating_profile.profile_name_entry.set_text("");
                //                ui_cloned.borrow().creating_profile.window.hide();
            });
        }
        {
            let ui_cloned = ui.clone();
            ui.borrow().open_btn.connect_clicked(move |_| {
                let url = ui_cloned.borrow().url_entry.get_text().unwrap();
                //                for profile in &profiles {
                //                    profile.open(&url);
                //                }
            });
        }

        ui
    }

}


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src: &str = include_str!("builder_basics.glade");
    let ui = UI::new(glade_src);

    //    let mut text = "".to_owned();
    //    for profile in &profiles {
    //        text.push_str(&format!("{}\n", profile.profile_name));
    //    }
    //    profiles_textview.get_buffer().unwrap().set_text(&text);

    ui.borrow_mut().window.show_all();
    ui.borrow_mut().window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

