mod cipher;
mod init;

use init::init;
use cipher::cipher;

fn main() {
    init();
    cipher();
}
