// Steve Operating System
// Stephen Marz
// 21 Sep 2019
#![no_std]
#![feature(panic_info_message,asm)]

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
		use core::fmt::Write;
		let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
	});
}
#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

// ///////////////////////////////////
// / LANGUAGE STRUCTURES / FUNCTIONS
// ///////////////////////////////////
#[no_mangle]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	print!("Aborting: ");
	if let Some(p) = info.location() {
		println!(
			"line {}, file {}: {}",
			p.line(),
			p.file(),
			info.message().unwrap()
		);
	}
	else {
		println!("no information available.");
	}
	abort();
}
#[no_mangle]
extern "C"
fn abort() -> ! {
	loop {
		unsafe {
			asm!("wfi"::::"volatile");
		}
	}
}

// ///////////////////////////////////
// / CONSTANTS
// ///////////////////////////////////

// ///////////////////////////////////
// / ENTRY POINT
// ///////////////////////////////////
#[no_mangle]
extern "C"
fn kmain() {
	// Main should initialize all sub-systems and get
	// ready to start scheduling. The last thing this
	// should do is start the timer.
	let mut my_uart = uart::Uart::new(0x1000_0000);
	my_uart.init();

	println!("Praxis PX-1 booting. Please stand by.");
	println!("Emergency terminal emulation prompt. Type to echo to the terminal.");

	// Now see if we can read stuff:
	// Usually we can use #[test] modules in Rust, but it would convolute the
	// task at hand. So, we'll just add testing snippets.
	loop {
		if let Some(c) = my_uart.get() {
			match c {
				8 => {
					// This is a backspace, so we essentially have
					// to write a space and backup again:
					print!("{}{}{}", 8 as char, 32 as char, 8 as char);
				},
				10 | 13 => {
					// Newline or carriage-return
					println!();
				},
				0x1b => {
					// Those familiar with ANSI escape sequences
					// knows that this is one of them. The next
					// thing we should get is the left bracket [
					// These are multi-byte sequences, so we can take
					// a chance and get from UART ourselves.
					// Later, we'll button this up.
					if let Some(c) = my_uart.get() {
						if c == 91 {
							// This is a right bracket! We're on our way!
							if let Some(b) = my_uart.get() {
								match b as char {
									'A' => {
										println!("That's the up arrow!");
									},
									'B' => {
										println!("That's the down arrow!");
									},
									'C' => {
										println!("That's the right arrow!");
									},
									'D' => {
										println!("That's the left arrow!");
									},
									_ => {
										println!("That's something else.....");
									}
								}
							}
						}
					}
				},
				_ => {
					print!("{}", c as char);
				}
			}
		}
	}	
}

// ///////////////////////////////////
// / RUST MODULES
// ///////////////////////////////////
pub mod uart;