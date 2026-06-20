struct Interface<'text, 'manager> {
    manager: &'manager mut Manager<'text>,
}

impl<'text, 'manager> Interface<'text, 'manager> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'text> {
    text: &'text str,
}

struct List<'text> {
    manager: Manager<'text>,
}

impl<'text> List<'text> {
    pub fn get_interface<'manager>(&'manager mut self) -> Interface<'text, 'manager>
    where
        'text: 'manager,
    {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // this fails because inmutable/mutable borrow
    // but Interface should be already dropped here and the borrow released
    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
