use flux_parser::html::parser::HtmlNode;
use flux_parser::html::HtmlDocument;
use gtk::prelude::*;
use gtk::{Box, Button, Image, Label, Orientation};


pub fn draw_dom(main_container: Box, dom: HtmlDocument) {
    while let Some(c) = main_container.last_child() {
        main_container.remove(&c);
    }

    dom.root_nodes.iter().for_each(|node| {
        draw_node(node, main_container.clone());
    });
}
fn draw_node(node: &HtmlNode, parent: Box) {
    match node {
        HtmlNode::Element(element) => {
            if element.tag_name == "head" ||
                element.tag_name == "style" ||
                element.tag_name == "script" {
                return; // TODO: Gérer les éléments head
            }
            if handle_special_element(&element.tag_name, parent.clone(), element) {
                return;
            }


            let container = Box::new(Orientation::Vertical, 0);
            parent.append(&container);

            element.children.iter().for_each(|child| {
                draw_node(child, container.clone());
            });
        }
        HtmlNode::Text(text) => {
            // Supprime tous les \n et \t du texte
            let text = text.replace("\n", "").replace("\t", "");
            if text.is_empty() {
                return;
            }
            let label = Label::new(Some(&text));
            parent.append(&label);
        }
        HtmlNode::Comment(_) => {}
    }
}

/// Gère les éléments spéciaux comme button, img, etc.
/// Retourne `true` si l'élément a été traité comme un type spécial.
fn handle_special_element(tag_name: &str, parent: Box, element: &flux_parser::html::parser::HtmlElement) -> bool {
    match tag_name {
        "button" => {
            let button = Button::with_label(
                element
                    .get_attribute("label")
                    .unwrap_or(&"Button".to_string()),
            );
            parent.append(&button);
            true
        }
        "img" => {
            if let Some(src) = element.get_attribute("src") {
                let image = Image::from_file(src);
                parent.append(&image);
            }
            true
        }
        _ => false, // Ce n'est pas un type spécial
    }
}
