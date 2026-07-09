//! 02 (0x) - move-замыкание. Эталонное решение.

use std::thread;

/// Перевести строку в верхний регистр в отдельном потоке, владеющем ею.
pub fn shout_in_thread(msg: String) -> String {
    let handle = thread::spawn(move || msg.to_uppercase());
    handle.join().unwrap()
}
