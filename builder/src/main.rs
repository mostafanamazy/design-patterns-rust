use builder::persistence::{PersistenceAttribute, PersistenceType};
use builder::reader::{Reader, NUM_ENTRIES};
use builder::unix::UnixBuilder;
use builder::vms::VmsBuilder;

fn main() {
    let input = [
        PersistenceAttribute {
            mtype: PersistenceType::File,
            value: "state.dat".into(),
        },
        PersistenceAttribute {
            mtype: PersistenceType::File,
            value: "config.sys".into(),
        },
        PersistenceAttribute {
            mtype: PersistenceType::Queue,
            value: "compute".into(),
        },
        PersistenceAttribute {
            mtype: PersistenceType::Queue,
            value: "log".into(),
        },
        PersistenceAttribute {
            mtype: PersistenceType::Pathway,
            value: "authentication".into(),
        },
        PersistenceAttribute {
            mtype: PersistenceType::Pathway,
            value: "error processing".into(),
        },
    ];
    let unix_builder = UnixBuilder::new();
    let vms_builder = VmsBuilder::new();
    let mut reader = Reader::new(Box::new(unix_builder));

    //reader.set_builder(Box::new(unix_builder));
    reader.construct(&input, NUM_ENTRIES);
    println!("{}", reader.get_state());

    reader.set_builder(Box::new(vms_builder));
    reader.construct(&input, NUM_ENTRIES);
    println!("{}", reader.get_state());
}
