use core_graphics::display::CGDisplay;

pub fn set_temp(temp: u32) {
    use std::os::raw::c_int;

    let ratio: f64 = (temp % 500) as f64 / 500f64;
    let gamma = crate::avg(temp, ratio);

    let main_display = CGDisplay::main();

    extern "C" {
        fn CGSetDisplayTransferByTable(
            display: u32,
            table_size: u32,
            red_table: *const f32,
            green_table: *const f32,
            blue_table: *const f32,
        ) -> c_int;
        fn CGDisplayGammaTableCapacity(display: u32) -> u32;
    }

    let table_size = unsafe { CGDisplayGammaTableCapacity(main_display.id) } as usize;

    let mut red_table = vec![0.0; table_size];
    let mut green_table = vec![0.0; table_size];
    let mut blue_table = vec![0.0; table_size];

    for i in 0..table_size {
        let value = (i as f32) / (table_size as f32 - 1.0);
        red_table[i] = (value * gamma.red as f32).min(1.0);
        green_table[i] = (value * gamma.green as f32).min(1.0);
        blue_table[i] = (value * gamma.blue as f32).min(1.0);
    }

    unsafe {
        CGSetDisplayTransferByTable(
            main_display.id,
            table_size as u32,
            red_table.as_ptr(),
            green_table.as_ptr(),
            blue_table.as_ptr(),
        );
    }
}
