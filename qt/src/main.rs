extern crate panopticon;
extern crate qmlrs;

pub fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.load_local_file("qt/res/Window.qml");
    engine.exec();
}
