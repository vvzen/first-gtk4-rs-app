use std::cell::Cell;
use std::rc::Rc;

use glib::clone;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Label, Orientation};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

// Following along here: https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_memory_management.html
// Widget Gallery: https://docs.gtk.org/gtk4/visual_index.html
// Button docs: https://docs.gtk.org/gtk4/class.Button.html#implements

fn build_ui(app: &Application) {
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = Rc::new(Cell::new(0));

    let label = Label::builder().build();
    label.set_label(&format!("Number: {}", number.get()));

    button_increase.connect_clicked(clone!(
        #[strong]
        number,
        #[weak]
        label,
        move |_| {
            let n = number.get() + 1;
            number.set(n);
            label.set_label(&format!("Number: {n}"));
        }
    ));

    button_decrease.connect_clicked(clone!(
        #[strong]
        number,
        #[weak]
        label,
        move |_| {
            let n = number.get() - 1;
            number.set(n);
            label.set_label(&format!("Number: {n}"));
        }
    ));

    // VBox
    let vbox = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    vbox.append(&button_increase);
    vbox.append(&label);
    vbox.append(&button_decrease);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&vbox)
        .build();

    window.present()
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
