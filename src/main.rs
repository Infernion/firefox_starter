extern crate gtk;

use gtk::prelude::*;
//use gtk::{Button, Window, WindowType, Box};

use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashSet;

fn create_profile(profile_name: &str, firefox_profiles: &Path) -> FirefoxProfile {
    let profile = FirefoxProfile::new(
        profile_name,
        &format!("{}/{}", firefox_profiles.display(), profile_name)
    );

    let new_profile_opts = &format!("{} {}", profile.profile_name, profile.profile_dir);
    Command::new("firefox")
        .args(&["-CreateProfile", new_profile_opts])
        .output()
        .expect("firefox not found");
    println!("New profile create at: {}", profile.profile_dir);
    profile
}

fn list_profiles(firefox_profiles: &Path) -> HashSet<FirefoxProfile> {
    let mut profiles: HashSet<FirefoxProfile> = HashSet::new();

    println!("List of avaliable profiles: ");
    for path in fs::read_dir(&firefox_profiles).unwrap() {
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

struct FirefoxBrowser;

impl FirefoxBrowser {
    fn new() -> FirefoxBrowser {
        FirefoxBrowser
    }
}

impl FirefoxProfile {
    fn new(profile_name: &str, profile_dir: &str) -> FirefoxProfile {
        FirefoxProfile {
            profile_name: profile_name.to_string(),
            profile_dir: profile_dir.to_string()
        }
    }

    pub fn open(&self, url: &str) {
        Command::new("firefox")
            .args(&[url, "-no-remote", "-P", &self.profile_name])
            .spawn();
    }
}

fn main() {
    //    let email = "sergiykhalimon@gmail.com2";
    let firefox_profiles_path = get_custom_firefox_profiles_path();

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("builder_basics.glade");
    let builder = gtk::Builder::new();
    builder.add_from_string(glade_src);

    let window: gtk::Window = builder.get_object("main_window").unwrap();
    let go_url_button: gtk::Button = builder.get_object("go_url_button").unwrap();
    let add_profile: gtk::Button = builder.get_object("add_profile").unwrap();
    let remove_profile: gtk::Button = builder.get_object("remove_profile").unwrap();
    let url_input: gtk::Entry = builder.get_object("url_input").unwrap();
    let profiles_textview: gtk::TextView = builder.get_object("profiles_textview").unwrap();
    let mut profiles = list_profiles(&firefox_profiles_path);
    let mut text = "".to_owned();
    for profile in &profiles {
        text.push_str(&format!("{}\n", profile.profile_name));
    }
    profiles_textview.get_buffer().unwrap().set_text(&text);

    let status_bar: gtk::Statusbar = builder.get_object("status_bar").unwrap();

    let creating_profile: gtk::MessageDialog = builder.get_object("creating_profile").unwrap();
    let profile_name_input: gtk::Entry = builder.get_object("profile_name_input").unwrap();
    let creating_profile_cancel_button: gtk::Button = builder.get_object("creating_profile_cancel_button").unwrap();
    let creating_profile_add_button: gtk::Button = builder.get_object("creating_profile_add_button").unwrap();

    //    let list_box: gtk::ListBox = builder.get_object("list_box").unwrap();
    //    let list_row = gtk::ListBoxRow::new();
    //    list_box.pack_start()
    let creating_profile_clone = creating_profile.clone();
    add_profile.connect_clicked(move |_| {
        creating_profile_clone.run();
    });

    let creating_profile_clone = creating_profile.clone();
    let profile_name_input_clone = profile_name_input.clone();
    creating_profile_cancel_button.connect_clicked(move |_| {
        profile_name_input_clone.set_text("");
        creating_profile_clone.hide();
    });

    let creating_profile_clone = creating_profile.clone();
    let profile_name_input_clone = profile_name_input.clone();
    creating_profile_add_button.connect_clicked(move |_| {
        let profile_name = profile_name_input_clone.get_text().unwrap();
        create_profile(&profile_name, &firefox_profiles_path);
        profile_name_input_clone.set_text("");
        creating_profile_clone.hide();
    });

    go_url_button.connect_clicked(move |_| {
        let url = url_input.get_text().unwrap();
        for profile in &profiles {
            profile.open(&url);
        }
    });
    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

