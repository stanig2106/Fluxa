use glib::MainContext;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Entry, Label, Orientation};
use std::sync::Arc;

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
    let vbox = GtkBox::new(Orientation::Vertical, 5);
    vbox.set_margin_top(10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);
    window.set_child(Some(&vbox));

    // Création de la barre supérieure avec le titre, l'URL et le bouton Go
    let hbox_top = GtkBox::new(Orientation::Horizontal, 5);

    // Titre à gauche
    let title = Label::new(Some("Fluxa"));
    title.set_xalign(0.0); // Alignement à gauche
    hbox_top.append(&title);

    // Champ de saisie pour l'URL
    let url_entry = Entry::new();
    url_entry.set_placeholder_text(Some("Entrez l'URL ici"));
    url_entry.set_text("fluxa://hello");
    url_entry.set_hexpand(true); // Prendre tout l'espace horizontal disponible
    hbox_top.append(&url_entry);

    // Bouton Go
    let go_button = Button::with_label("Go");
    hbox_top.append(&go_button);

    vbox.append(&hbox_top);

    // Conteneur principal pour afficher le DOM (placeholder)
    let main_container = GtkBox::new(Orientation::Vertical, 0);
    main_container.set_hexpand(true);
    main_container.set_vexpand(true);
    vbox.append(&main_container);

    let load_url = Arc::new({
        let url_entry = url_entry.clone();
        let main_container = main_container.clone();
        move || {
            let url_entry = url_entry.clone();
            let main_container = main_container.clone();
            // Spawn the async task onto the main context
            MainContext::default().spawn_local(async move {
                let url = url_entry.text().to_string();
                let response = match flux_network::fetch(&url).await {
                    Ok(response) => response,
                    Err(e) => {
                        eprintln!("Error fetching URL: {:?}", e);
                        return;
                    }
                };
                // Convert body from Vec<u8> to String
                let dom = match flux_parser::parse_document(
                    &String::from_utf8_lossy(&response.body).to_string(),
                    "text/html",
                ) {
                    Ok(flux_parser::ParsedDocument::Html(dom)) => dom,
                    Err(e) => {
                        eprintln!("Error parsing document: {:?}", e);
                        return;
                    }
                };

                flux_dom::draw_dom(main_container, dom);
            });
        }
    });

    // Clone the Arc for go_button
    let load_url_clone = Arc::clone(&load_url);
    go_button.connect_clicked(move |_| {
        load_url_clone();
    });

    // Clone the Arc for url_entry
    let load_url_clone = Arc::clone(&load_url);
    url_entry.connect_activate(move |_| {
        load_url_clone();
    });

    window.show();
    Arc::clone(&load_url)();
}
