#[macro_export]
macro_rules! matrix_pins {
    ($pins:expr, $row_ids:expr, $col_ids:expr) => {{
        use embassy_rp::gpio::{AnyPin, Input, Output, Pull, Level};

        let rows: [Output<'static>; $row_ids.len()] =
            core::array::from_fn(|i| {
                let pin_id = $row_ids[i];
                let pin = unsafe { AnyPin::steal(pin_id as u8) };
                Output::new(pin, Level::High)
            });

        let cols: [Input<'static>; $col_ids.len()] =
            core::array::from_fn(|i| {
                let pin_id = $col_ids[i];
                let pin = unsafe { AnyPin::steal(pin_id as u8) };
                Input::new(pin, Pull::Up)
            });

        (rows, cols)
    }};
}
