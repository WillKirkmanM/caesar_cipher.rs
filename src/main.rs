pub mod cipher;
pub mod decipher;
pub mod tests;

mod init;
use init::init;

fn main() {
    init();
}
