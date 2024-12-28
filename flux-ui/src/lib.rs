#[macro_use]
extern crate glib;
use flux_core::Flux;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Entry, Label, Orientation};

pub fn init() {
    let app = Application::new(
        Some("com.example.navigateur_ui"),
        Default::default(),
    );

    app.connect_activate(move |app| {
        build_ui(app);
    });

    app.run();
}

fn build_ui(app: &Application) {
    // Création de la fenêtre principale
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Fluxa")
        .default_width(800)
        .default_height(600)
        .build();

    // Créer une boîte verticale pour organiser les widgets
    let vbox = Box::new(Orientation::Vertical, 5);
    vbox.set_margin_top(10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);
    window.set_child(Some(&vbox));

    // Création de la barre supérieure avec le titre, l'URL et le bouton Go
    let hbox_top = Box::new(Orientation::Horizontal, 5);

    // Titre à gauche
    let title = Label::new(Some("Fluxa"));
    title.set_xalign(0.0); // Alignement à gauche
    hbox_top.append(&title);

    // Champ de saisie pour l'URL
    let url_entry = Entry::new();
    url_entry.set_placeholder_text(Some("Entrez l'URL ici"));
    url_entry.set_hexpand(true); // Prendre tout l'espace horizontal disponible
    hbox_top.append(&url_entry);

    // Bouton Go
    let go_button = Button::with_label("Go");
    hbox_top.append(&go_button);

    vbox.append(&hbox_top);

    // Conteneur principal pour afficher le DOM (placeholder)
    let main_container = Label::new(Some("Contenu du DOM affiché ici"));
    main_container.set_hexpand(true);
    main_container.set_vexpand(true);
    vbox.append(&main_container);

    let load_url = {
        let url_entry = url_entry.clone();
        let main_container = main_container.clone();
        move || {
            let url = url_entry.text().to_string();
            main_container.set_text(&format!("Chargement de : {}", url));
        }
    };

    {
        let load_url = load_url.clone();
        go_button.connect_clicked(move |_| {
            load_url();
        });
    }
    {
        let load_url = load_url.clone();
        url_entry.connect_activate(move |_| {
            load_url();
        });
    }

    window.show();
}
