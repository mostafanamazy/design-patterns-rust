use decorator::data::{DataSource, FileDataSource};
use decorator::encrypt::EncryptionDecorator;
use decorator::compress::CompressionDecorator;
// Option 1. A simple example of a decorator assembly.
fn main() {
    let source = FileDataSource::new("somefile.dat".to_string());
    source.write_data(8);
    // The target file has been written with plain data.

    let source = CompressionDecorator::new(source);
    source.write_data(9);
    // The target file has been written with compressed
    // data.

    let source = EncryptionDecorator::new(source);
    // The source variable now contains this:
    // Encryption > Compression > FileDataSource
    source.write_data(10);
    // The file has been written with compressed and
    // encrypted data.
}
