use crate::data::DataSource;
// You can wrap objects in several layers of decorators.
pub struct CompressionDecorator<D: DataSource> {
    wrapee: D,
}
impl<D: DataSource> CompressionDecorator<D> {
    pub fn new(wr: D) -> Self {
        CompressionDecorator { wrapee: wr }
    }
}
impl<D: DataSource> DataSource for CompressionDecorator<D> {
    fn write_data(&self, w: i8) {
        println!("compress {}", w);
        self.wrapee.write_data(w);
    }

    // 1. Encrypt passed data.
    // 2. Pass encrypted data to the wrappee's write_data
    // method.

    fn read_data(&self) -> i8 {
        println!("read compress");
        self.wrapee.read_data()
    }
}


