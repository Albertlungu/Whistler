use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::Window;
use cacao::input::TextField;
use cacao::view::View;
use cacao::geometry::Rect;
use cacao::layout::Layout;

struct BasicApp {
    window: Window,
    content: View,
    text_field: TextField,
}

impl Default for BasicApp {
    fn default() -> Self {
        BasicApp {
            window: Window::default(),
            content: View::new(),
            text_field: TextField::new(),
        }
    }
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        self.text_field.set_frame(Rect {
            top: 350.0,  // this parameter is measured from the bottom (lower number = lower down in the woindow)
            left: 20.0,
            width: 360.0,
            height: 30.0,
        });

        self.content.add_subview(&self.text_field);

        self.window.set_minimum_content_size(400, 400);
        self.window.set_title("Whistler");
        self.window.set_content_view(&self.content);

        self.window.make_key_and_order_front();
        self.window.show();

        App::activate();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

fn main() {
    App::new("com.whistler.app", BasicApp::default()).run();
}