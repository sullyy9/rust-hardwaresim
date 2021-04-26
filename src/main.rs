mod command;
mod component;
mod window;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Main
fn main() {
    let mut component_list: Vec<component::Component> = vec![];
    let mut window = window::Window::new();

    let mut commands = command::Input::new();

    window.draw();

    loop {
        let mut user_input = String::new();
        window.read(&mut user_input);

        match commands.parse(&user_input) {
            Ok(_ok) => {}
            Err(_error) => {}
        }

        user_input.clear();
        window.draw();
    }
}
