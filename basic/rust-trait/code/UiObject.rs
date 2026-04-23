#[derive(Debug)]
enum UiObject {
    Button,
    SelectBox,
}

fn main() {
    let objects = [UiObject::Button, UiObject::SelectBox];

    for o in objects {
        draw(o)
    }
}

fn draw(o: UiObject) {
    println!("{:?}", o);
}
