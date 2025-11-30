#[macro_export]
macro_rules! matrix_pins {
    ($pins:expr, $row_ids:expr, $col_ids:expr) => {{
        use rp2040_hal::gpio::{DynPinId, FunctionSio, Pin, PullDown, PullUp, SioInput, SioOutput};

        let mut p0 = Some($pins.gpio0);
        let mut p1 = Some($pins.gpio1);
        let mut p2 = Some($pins.gpio2);
        let mut p3 = Some($pins.gpio3);
        let mut p4 = Some($pins.gpio4);
        let mut p5 = Some($pins.gpio5);
        let mut p6 = Some($pins.gpio6);
        let mut p7 = Some($pins.gpio7);
        let mut p8 = Some($pins.gpio8);
        let mut p9 = Some($pins.gpio9);
        let mut p10 = Some($pins.gpio10);
        let mut p11 = Some($pins.gpio11);
        let mut p12 = Some($pins.gpio12);
        let mut p13 = Some($pins.gpio13);
        let mut p14 = Some($pins.gpio14);
        let mut p15 = Some($pins.gpio15);
        let mut p16 = Some($pins.gpio16);
        let mut p17 = Some($pins.gpio17);
        let mut p18 = Some($pins.gpio18);
        let mut p19 = Some($pins.gpio19);
        let mut p20 = Some($pins.gpio20);
        let mut p21 = Some($pins.gpio21);
        let mut p22 = Some($pins.gpio22);
        let mut p23 = Some($pins.gpio23);
        let mut p24 = Some($pins.gpio24);
        let mut p25 = Some($pins.gpio25);
        let mut p26 = Some($pins.gpio26);
        let mut p27 = Some($pins.gpio27);
        let mut p28 = Some($pins.gpio28);
        let mut p29 = Some($pins.gpio29);

        let rows: [Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; $row_ids.len()] =
            core::array::from_fn(|i| match $row_ids[i] {
                0 => p0.take().unwrap().into_push_pull_output().into_dyn_pin(),
                1 => p1.take().unwrap().into_push_pull_output().into_dyn_pin(),
                2 => p2.take().unwrap().into_push_pull_output().into_dyn_pin(),
                3 => p3.take().unwrap().into_push_pull_output().into_dyn_pin(),
                4 => p4.take().unwrap().into_push_pull_output().into_dyn_pin(),
                5 => p5.take().unwrap().into_push_pull_output().into_dyn_pin(),
                6 => p6.take().unwrap().into_push_pull_output().into_dyn_pin(),
                7 => p7.take().unwrap().into_push_pull_output().into_dyn_pin(),
                8 => p8.take().unwrap().into_push_pull_output().into_dyn_pin(),
                9 => p9.take().unwrap().into_push_pull_output().into_dyn_pin(),
                10 => p10.take().unwrap().into_push_pull_output().into_dyn_pin(),
                11 => p11.take().unwrap().into_push_pull_output().into_dyn_pin(),
                12 => p12.take().unwrap().into_push_pull_output().into_dyn_pin(),
                13 => p13.take().unwrap().into_push_pull_output().into_dyn_pin(),
                14 => p14.take().unwrap().into_push_pull_output().into_dyn_pin(),
                15 => p15.take().unwrap().into_push_pull_output().into_dyn_pin(),
                16 => p16.take().unwrap().into_push_pull_output().into_dyn_pin(),
                17 => p17.take().unwrap().into_push_pull_output().into_dyn_pin(),
                18 => p18.take().unwrap().into_push_pull_output().into_dyn_pin(),
                19 => p19.take().unwrap().into_push_pull_output().into_dyn_pin(),
                20 => p20.take().unwrap().into_push_pull_output().into_dyn_pin(),
                21 => p21.take().unwrap().into_push_pull_output().into_dyn_pin(),
                22 => p22.take().unwrap().into_push_pull_output().into_dyn_pin(),
                23 => p23.take().unwrap().into_push_pull_output().into_dyn_pin(),
                24 => p24.take().unwrap().into_push_pull_output().into_dyn_pin(),
                25 => p25.take().unwrap().into_push_pull_output().into_dyn_pin(),
                26 => p26.take().unwrap().into_push_pull_output().into_dyn_pin(),
                27 => p27.take().unwrap().into_push_pull_output().into_dyn_pin(),
                28 => p28.take().unwrap().into_push_pull_output().into_dyn_pin(),
                29 => p29.take().unwrap().into_push_pull_output().into_dyn_pin(),
                _ => panic!("Invalid Row Pin ID"),
            });

        let cols: [Pin<DynPinId, FunctionSio<SioInput>, PullUp>; $col_ids.len()] =
            core::array::from_fn(|i| match $col_ids[i] {
                0 => p0.take().unwrap().into_pull_up_input().into_dyn_pin(),
                1 => p1.take().unwrap().into_pull_up_input().into_dyn_pin(),
                2 => p2.take().unwrap().into_pull_up_input().into_dyn_pin(),
                3 => p3.take().unwrap().into_pull_up_input().into_dyn_pin(),
                4 => p4.take().unwrap().into_pull_up_input().into_dyn_pin(),
                5 => p5.take().unwrap().into_pull_up_input().into_dyn_pin(),
                6 => p6.take().unwrap().into_pull_up_input().into_dyn_pin(),
                7 => p7.take().unwrap().into_pull_up_input().into_dyn_pin(),
                8 => p8.take().unwrap().into_pull_up_input().into_dyn_pin(),
                9 => p9.take().unwrap().into_pull_up_input().into_dyn_pin(),
                10 => p10.take().unwrap().into_pull_up_input().into_dyn_pin(),
                11 => p11.take().unwrap().into_pull_up_input().into_dyn_pin(),
                12 => p12.take().unwrap().into_pull_up_input().into_dyn_pin(),
                13 => p13.take().unwrap().into_pull_up_input().into_dyn_pin(),
                14 => p14.take().unwrap().into_pull_up_input().into_dyn_pin(),
                15 => p15.take().unwrap().into_pull_up_input().into_dyn_pin(),
                16 => p16.take().unwrap().into_pull_up_input().into_dyn_pin(),
                17 => p17.take().unwrap().into_pull_up_input().into_dyn_pin(),
                18 => p18.take().unwrap().into_pull_up_input().into_dyn_pin(),
                19 => p19.take().unwrap().into_pull_up_input().into_dyn_pin(),
                20 => p20.take().unwrap().into_pull_up_input().into_dyn_pin(),
                21 => p21.take().unwrap().into_pull_up_input().into_dyn_pin(),
                22 => p22.take().unwrap().into_pull_up_input().into_dyn_pin(),
                23 => p23.take().unwrap().into_pull_up_input().into_dyn_pin(),
                24 => p24.take().unwrap().into_pull_up_input().into_dyn_pin(),
                25 => p25.take().unwrap().into_pull_up_input().into_dyn_pin(),
                26 => p26.take().unwrap().into_pull_up_input().into_dyn_pin(),
                27 => p27.take().unwrap().into_pull_up_input().into_dyn_pin(),
                28 => p28.take().unwrap().into_pull_up_input().into_dyn_pin(),
                29 => p29.take().unwrap().into_pull_up_input().into_dyn_pin(),
                _ => panic!("Invalid Col Pin ID"),
            });

        (rows, cols)
    }};
}
