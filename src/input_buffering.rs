use libc::*;
use nix::sys::signal::{self, SaFlags, SigAction, SigHandler, SigSet, Signal};
use std::mem::MaybeUninit;
use std::os::unix::io::RawFd;
use std::sync::Once;

static mut ORIGINAL_TIO: MaybeUninit<termios> = MaybeUninit::uninit();
static INIT: Once = Once::new();

extern "C" fn handle_interrupt(_sig: c_int) {
    println!("\nSIGINT received. Restoring terminal settings...");
    restore_input_buffering();
    std::process::exit(0);
}
pub fn setup() {
    // Set up SIGINT handler (Ctrl+C)
    unsafe {
        let sig_action = SigAction::new(
            SigHandler::Handler(handle_interrupt),
            SaFlags::empty(),
            SigSet::empty(),
        );

        signal::sigaction(Signal::SIGINT, &sig_action).expect("Failed to register SIGINT handler");
    }

    disable_input_buffering();
}

pub fn disable_input_buffering() {
    unsafe {
        let fd: RawFd = STDIN_FILENO;
        INIT.call_once(|| {
            tcgetattr(fd, ORIGINAL_TIO.as_mut_ptr());
        });

        let mut new_tio = ORIGINAL_TIO.assume_init();
        new_tio.c_lflag &= !(ICANON | ECHO);
        tcsetattr(fd, TCSANOW, &new_tio);
    }
}

pub fn restore_input_buffering() {
    unsafe {
        let fd: RawFd = STDIN_FILENO;
        tcsetattr(fd, TCSANOW, &ORIGINAL_TIO.assume_init());
    }
}

pub fn check_key() -> bool {
    unsafe {
        let fd: RawFd = STDIN_FILENO;
        let mut readfds = std::mem::zeroed::<fd_set>();
        FD_ZERO(&mut readfds);
        FD_SET(fd, &mut readfds);

        let mut timeout = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };

        select(
            fd + 1,
            &mut readfds,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut timeout,
        ) > 0
    }
}
