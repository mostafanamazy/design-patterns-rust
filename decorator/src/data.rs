/// The component interface defines operations that can be
/// altered by decorators.
pub trait DataSource {
    fn write_data(&self, w: i8);
    fn read_data(&self) -> i8;
}

/// Concrete components provide default implementations for the
/// operations. There might be several variations of these
/// classes in a program.
pub struct FileDataSource {
    name: String,
}
impl FileDataSource {
    pub fn new(n: String) -> Self {
        FileDataSource { name: n }
    }
}
impl DataSource for FileDataSource {
    /// Write data to file.
    fn write_data(&self, w: i8) {
        println!("write {} to FileDataSource {}", w, self.name);
    }

    /// Read data from file
    fn read_data(&self) -> i8 {
        println!("read FileDataSource");
        1
    }
}

