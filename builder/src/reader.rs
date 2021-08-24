use crate::builder::Builder;
use crate::persistence::{PersistenceAttribute, PersistenceType};

pub const NUM_ENTRIES: usize = 6;
pub struct Reader {
    builder: Box<dyn Builder>,
}
impl Reader {
    pub fn new(b: Box<dyn Builder>) -> Self {
        Reader { builder: b }
    }
    pub fn set_builder(&mut self, b: Box<dyn Builder>) {
        self.builder = b;
    }

    pub fn construct(&mut self, list: &[PersistenceAttribute; NUM_ENTRIES], num: usize) {
        for i in 0..num {
            if list[i].mtype == PersistenceType::File {
                self.builder.configure_file(&list[i].value);
            } else if list[i].mtype == PersistenceType::Queue {
                self.builder.configure_queue(&list[i].value);
            } else if list[i].mtype == PersistenceType::Pathway {
                self.builder.configure_pathway(&list[i].value);
            }
        }
    }
    pub fn get_state(&self) -> &String {
        self.builder.get_result().get_state()
    }
}
