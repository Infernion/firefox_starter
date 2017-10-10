#![windows_subsystem = "windows"]
extern crate gtk;

use gtk::prelude::*;
//use gtk::{Button, Window, WindowType, Box};

use std::env;
use std::process::Command;
use std::fs;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn create_profile(profile_name: &str, path: &str) -> FirefoxProfile {
    let profile = FirefoxProfile::new(
        profile_name,
        &format!("{}/{}", path, profile_name)
    );

    let new_profile_opts = &format!("{} {}", profile.profile_name, profile.profile_dir);
    Command::new("firefox-developer")
        .args(&["-CreateProfile", new_profile_opts])
        .output()
        .expect("firefox not found");
    println!("New profile create at: {}", profile.profile_dir);
    profile
}

fn get_profiles(path: &str) -> HashSet<FirefoxProfile> {
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

fn get_custom_firefox_profiles_path() -> String {
    let mut profiles_path = env::current_dir().unwrap();
    profiles_path.push("firefox_profiles");
    profiles_path.into_os_string().into_string().unwrap()
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct FirefoxProfile {
    pub profile_name: String,
    profile_dir: String,
}

impl FirefoxProfile {
    fn new(profile_name: &str, profile_dir: &str) -> FirefoxProfile {
        FirefoxProfile {
            profile_name: profile_name.to_string(),
            profile_dir: profile_dir.to_string()
        }
    }
    fn get_profile_name_as_label(&self) -> gtk::Label {
        gtk::Label::new(&*self.profile_name)
    }

    pub fn open(&self, url: &str) {
        Command::new("firefox").args(&[url, "-no-remote", "-P", &self.profile_name]).spawn();
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
    fn hide_window(&self) {
        &self.window.hide();
        &self.profile_name_entry.set_text("");
    }
}

#[derive(Debug, Clone)]
struct ProfileManager {
    profiles: Option<HashSet<FirefoxProfile>>,
    firefox_profiles_path: String
}

impl ProfileManager {
    fn new() -> Self {
        ProfileManager {
            profiles: None,
            firefox_profiles_path: get_custom_firefox_profiles_path(),
        }
    }

    fn profiles(&self) -> HashSet<FirefoxProfile> {
//        let profiles = get_profiles(&self.firefox_profiles_path);
        let profiles = get_profiles(&self.firefox_profiles_path);
        profiles
////        let profiles = &self.profiles;
//        match self.profiles {
//            Some(profiles) => self.profiles,
//            None => {
//                let profiles = get_profiles(&self.firefox_profiles_path);
//                self.profiles = Some(profiles);
//                self.profiles
//            }
//        }
    }
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
    profiles_listbox: gtk::ListBox,
    // Creating profile
    creating_profile: CreatingProfileUI,

    // Statusbar
    statusbar: gtk::Statusbar,
}

impl UI {
    fn new(glade_src: &str) -> Rc<RefCell<Self>> {
        let builder = gtk::Builder::new();
        builder.add_from_string(glade_src);

        let profiles_manager = ProfileManager::new();
        let firefox_profiles_path = get_custom_firefox_profiles_path();
        let firefox_profiles_path2 = get_custom_firefox_profiles_path();

        let ui: Rc<RefCell<UI>> = Rc::new(RefCell::new(UI {
            window: builder.get_object("main_window").unwrap(),
            url_entry: builder.get_object("url_input").unwrap(),
            open_btn: builder.get_object("go_url_button").unwrap(),
            add_profile_btn: builder.get_object("add_profile").unwrap(),
            remove_profile_btn: builder.get_object("remove_profile").unwrap(),
            profiles_listbox: builder.get_object("profiles_listbox").unwrap(),
            creating_profile: CreatingProfileUI {
                window: builder.get_object("creating_profile").unwrap(),
                profile_name_entry: builder.get_object("profile_name_input").unwrap(),
                add_btn: builder.get_object("creating_profile_add_button").unwrap(),
                cancel_btn: builder.get_object("creating_profile_cancel_button").unwrap(),
            },
            statusbar: builder.get_object("status_bar").unwrap(),
        }));

        {
            ui.borrow().add_profile_btn.connect_clicked(clone!(ui => move | _ | {
                ui.borrow().creating_profile.window.run();
            }));
        }
        {
            ui.borrow().creating_profile.add_btn.connect_clicked(clone!(ui => move | _| {
                let profile_name = ui.borrow().creating_profile.profile_name_entry.get_text().unwrap();
                create_profile(&profile_name, &firefox_profiles_path);
                ui.borrow().creating_profile.hide_window();
            }));
        }
        {
            ui.borrow().creating_profile.cancel_btn.connect_clicked(clone!(ui => move | _| {
                ui.borrow().creating_profile.hide_window();
            }));
        }
        {
            let profiles_manager_cloned = profiles_manager.clone();
            ui.borrow().open_btn.connect_clicked(clone!(ui => move | _ | {
                let url = ui.borrow().url_entry.get_text().unwrap();
                for profile in &profiles_manager_cloned.profiles() {
                    profile.open(&url);
                }
            }));
        }
        {

            let ff_dir_description = gtk::Label::new(&*format!(
                "Firefox profiles dir path:\n {:?}", &firefox_profiles_path2));
            let header_row = gtk::ListBoxRow::new();
            header_row.add(&ff_dir_description);
            header_row.set_selectable(false);
            ui.borrow().profiles_listbox.insert(&header_row, -1);

            for profile in &profiles_manager.profiles() {
                let row = gtk::ListBoxRow::new();
                let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 2);
                hbox.add(&profile.get_profile_name_as_label());
                hbox.show_all();
                row.add(&hbox);
                ui.borrow().profiles_listbox.insert(&row, -1);
            }

        }

        ui
    }
}


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

//    let FF: PathBuf = get_custom_firefox_profiles_path();
    let glade_src: &str = include_str!("builder_basics.glade");
    let ui = UI::new(glade_src);


    ui.borrow().window.show_all();
    ui.borrow().window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}


#[cfg(test)]
mod tests {
    //    use super::client;

    #[test]
    fn it_works() {
        //        client::connect();
    }
}
