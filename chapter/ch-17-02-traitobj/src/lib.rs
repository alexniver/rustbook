pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    comps: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new() -> Self {
        Screen { comps: vec![] }
    }

    pub fn add_comp(&mut self, comp: Box<dyn Draw>) {
        self.comps.push(comp);
    }

    pub fn run(&self) {
        for c in self.comps.iter() {
            c.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u16,
    pub height: u16,
    pub label: String,
}

impl Button {
    pub fn new(width: u16, height: u16, label: String) -> Self {
        Self {
            width,
            height,
            label,
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button, {:?}", self);
    }
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u16,
    pub height: u16,
    pub opts: Vec<String>,
}

impl SelectBox {
    pub fn new(width: u16, height: u16, opts: Vec<String>) -> Self {
        Self {
            width,
            height,
            opts,
        }
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw select box, {:?}", self);
    }
}
