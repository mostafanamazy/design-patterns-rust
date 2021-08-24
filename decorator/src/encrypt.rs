use crate::data::DataSource;
/// The base decorator class follows the same interface as the
/// other components. The primary purpose of this class is to
/// define the wrapping interface for all concrete decorators.
/// The default implementation of the wrapping code might include
/// a field for storing a wrapped component and the means to
/// initialize it.
/// Concrete decorators must call methods on the wrapped object,
/// but may add something of their own to the result. Decorators
/// can execute the added behavior either before or after the
/// call to a wrapped object.
pub struct EncryptionDecorator<D: DataSource> {
    wrapee: D,
}
impl<D: DataSource> EncryptionDecorator<D> {
    pub fn new(wr: D) -> Self {
        EncryptionDecorator { wrapee: wr }
    }
}
impl<D: DataSource> DataSource for EncryptionDecorator<D> {
    /// 1. Encrypt passed data.
    /// 2. Pass encrypted data to the wrappee's write_data
    /// method.
    fn write_data(&self, w: i8) {
        println!("encrypt {}", w);
        self.wrapee.write_data(w);
    }

    /// 1. Get data from the wrappee's read_data method.
    /// 2. Try to decrypt it if it's encrypted.
    /// 3. Return the result.
    fn read_data(&self) -> i8 {
        println!("read encrypt");
        self.wrapee.read_data()
    }
}


