#[macro_export]
macro_rules! matrix_pins {
    ($pins:expr, $row_ids:expr, $col_ids:expr) => {{
        use embassy_rp::gpio::{AnyPin, Input, Output, Pull, Level, Pin};

        let mut all_pins = [
            $pins.PIN_0.degrade(), $pins.PIN_1.degrade(), $pins.PIN_2.degrade(), $pins.PIN_3.degrade(),
            $pins.PIN_4.degrade(), $pins.PIN_5.degrade(), $pins.PIN_6.degrade(), $pins.PIN_7.degrade(),
            $pins.PIN_8.degrade(), $pins.PIN_9.degrade(), $pins.PIN_10.degrade(), $pins.PIN_11.degrade(),
            $pins.PIN_12.degrade(), $pins.PIN_13.degrade(), $pins.PIN_14.degrade(), $pins.PIN_15.degrade(),
            $pins.PIN_16.degrade(), $pins.PIN_17.degrade(), $pins.PIN_18.degrade(), $pins.PIN_19.degrade(),
            $pins.PIN_20.degrade(), $pins.PIN_21.degrade(), $pins.PIN_22.degrade(), $pins.PIN_23.degrade(),
            $pins.PIN_24.degrade(), $pins.PIN_25.degrade(), $pins.PIN_26.degrade(), $pins.PIN_27.degrade(),
            $pins.PIN_28.degrade(), $pins.PIN_29.degrade(),
        ];

        let rows: [Output<'static>; $row_ids.len()] =
            core::array::from_fn(|i| {
                let pin_id = $row_ids[i];
                // ここでは unsafe を使わず、所有権を移動させるために AnyPin を直接生成するのではなく
                // 元の pins から取り出す必要がありますが、マクロの制約上、
                // embassy-rp では AnyPin::from(p) を使います。
                // 実際には Peripherals から直接取るのが一般的ですが、この構成を維持するために
                // 配列から抜き出す形にします。
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
