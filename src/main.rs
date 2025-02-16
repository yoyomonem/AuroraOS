#![no_std]
#![no_main]
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  let vga_buffer = 0xb8000 as *mut u8;
  static IT_DOESNT_WORK: &[u8] = b"┌───────────────────────────────────────┐\n│ Error                             [X] │\n└───────────────────────────────────────┘\n│                                       │\n│  An error occurred and AuroraOS       │\n│  crashed. Sorry! :(                   │\n│                                       │\n└───────────────────────────────────────┘";
  for (i, &byte) in IT_DOESNT_WORK.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte;
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
  }
  loop {}
}
static DOES_IT_WORK: &[u8] = b"┌───────────────────────────────────────┐\n│ AuroraOS                          [X] │\n└───────────────────────────────────────┘\n│                                       │\n│  If you see this, AuroraOS works!     │\n│  Congratulations!                     │\n│                                       │\n└───────────────────────────────────────┘";
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
  let vga_buffer = 0xb8000 as *mut u8;
  for (i, &byte) in DOES_IT_WORK.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte;
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
  }
  loop {}
}
