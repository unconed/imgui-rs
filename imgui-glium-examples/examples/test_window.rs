mod support;

const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

fn main() {
    support::run("test_window.rs".to_owned(), CLEAR_COLOR, |ui, _, _| {
        let mut open = true;
        ui.show_demo_window(&mut open);
        open
    });
}
