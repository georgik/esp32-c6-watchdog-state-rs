#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println::println;

// Helper function to check RWDT status
fn check_rwdt_status() -> (bool, u32) {
    const LP_WDT_BASE: u32 = 0x600B1C00;
    const WDTCONFIG0_OFFSET: u32 = 0x0;
    
    unsafe {
        let config0_reg = core::ptr::read_volatile((LP_WDT_BASE + WDTCONFIG0_OFFSET) as *const u32);
        let enabled = (config0_reg & (1 << 31)) != 0; // bit 31 is wdt_en
        (enabled, config0_reg)
    }
}

// Helper function to check TIMG0 MWDT status
fn check_timg0_status() -> (bool, u32) {
    const TIMG0_BASE: u32 = 0x60008000;
    const WDTCONFIG0_OFFSET: u32 = 0x48;
    
    unsafe {
        let config0_reg = core::ptr::read_volatile((TIMG0_BASE + WDTCONFIG0_OFFSET) as *const u32);
        let enabled = (config0_reg & (1 << 31)) != 0; // bit 31 is wdt_en
        (enabled, config0_reg)
    }
}

// Helper function to check TIMG1 MWDT status
fn check_timg1_status() -> (bool, u32) {
    const TIMG1_BASE: u32 = 0x60009000;
    const WDTCONFIG0_OFFSET: u32 = 0x48;
    
    unsafe {
        let config0_reg = core::ptr::read_volatile((TIMG1_BASE + WDTCONFIG0_OFFSET) as *const u32);
        let enabled = (config0_reg & (1 << 31)) != 0; // bit 31 is wdt_en
        (enabled, config0_reg)
    }
}

// Helper function to check SWD (Super Watchdog) status
fn check_swd_status() -> (bool, u32) {
    const LP_WDT_BASE: u32 = 0x600B1C00;
    const SWD_CONF_OFFSET: u32 = 0x104;
    
    unsafe {
        let swd_conf_reg = core::ptr::read_volatile((LP_WDT_BASE + SWD_CONF_OFFSET) as *const u32);
        // SWD is enabled when swd_auto_feed_en is 0 (inverted logic)
        let enabled = (swd_conf_reg & (1 << 31)) == 0; // bit 31 is swd_auto_feed_en
        (enabled, swd_conf_reg)
    }
}

// Helper function to print watchdog changes
fn print_wdt_changes(name: &str, before: (bool, u32), after: (bool, u32)) {
    if before.0 != after.0 {
        println!("  {} status changed: {} -> {}", name, before.0, after.0);
    } else {
        println!("  {} status unchanged: {}", name, before.0);
    }
    
    if before.1 != after.1 {
        println!("  {} config0 changed: 0x{:08x} -> 0x{:08x}", name, before.1, after.1);
    } else {
        println!("  {} config0 unchanged: 0x{:08x}", name, before.1);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    println!("ESP32-C6 Watchdog Test - Checking WDT status before and after esp_hal::init()");
    println!("======================================================================\n");
    
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    
    // Check current status of the watchdogs BEFORE init
    println!("Checking watchdog status BEFORE esp_hal::init():");
    
    // Check RWDT status using direct register access
    let rwdt_before = check_rwdt_status();
    
    // Check TIMG0 MWDT status
    let timg0_before = check_timg0_status();
    
    // Check TIMG1 MWDT status
    let timg1_before = check_timg1_status();
    
    println!("RWDT enabled: {}, config0: 0x{:08x}", rwdt_before.0, rwdt_before.1);
    println!("TIMG0 MWDT enabled: {}, config0: 0x{:08x}", timg0_before.0, timg0_before.1);
    println!("TIMG1 MWDT enabled: {}, config0: 0x{:08x}", timg1_before.0, timg1_before.1);
    
    // Call esp_hal::init to configure system
    println!("\n==> Calling esp_hal::init() with default config...");
    let _peripherals = esp_hal::init(config);
    
    // Check new status of the watchdogs AFTER init
    println!("\nChecking watchdog status AFTER esp_hal::init():");
    
    let rwdt_after = check_rwdt_status();
    let timg0_after = check_timg0_status();
    let timg1_after = check_timg1_status();
    
    println!("RWDT enabled: {}, config0: 0x{:08x}", rwdt_after.0, rwdt_after.1);
    println!("TIMG0 MWDT enabled: {}, config0: 0x{:08x}", timg0_after.0, timg0_after.1);
    println!("TIMG1 MWDT enabled: {}, config0: 0x{:08x}", timg1_after.0, timg1_after.1);
    
    // Show changes
    println!("\nChanges after esp_hal::init():");
    print_wdt_changes("RWDT", rwdt_before, rwdt_after);
    print_wdt_changes("TIMG0 MWDT", timg0_before, timg0_after);
    print_wdt_changes("TIMG1 MWDT", timg1_before, timg1_after);
    
    println!("\nWatchdog control registers inspected:");
    println!("- LP_WDT base: 0x600B1C00 (RWDT control)");
    println!("- TIMG0 base: 0x60008000 (MWDT0 control)");
    println!("- TIMG1 base: 0x60009000 (MWDT1 control)");
    println!("- Key register field: wdtconfig0.wdt_en (bit 31)");
    
    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}
