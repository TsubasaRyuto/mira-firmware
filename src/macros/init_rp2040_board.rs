#[macro_export]
macro_rules! init_rp2040_board {
    // ----------------------------------------------------
    // Normal Keyboard
    // ----------------------------------------------------
    ($p:expr, $row_pins:expr, $col_pins:expr, $irqs:expr) => {{
        let (rows, cols) = $crate::matrix_pins!($p, $row_pins, $col_pins);
        let usb_driver = embassy_rp::usb::Driver::new($p.USB, $irqs);
        
        (rows, cols, usb_driver)
    }};

    // ----------------------------------------------------
    // Split Keyboard
    // ----------------------------------------------------
    ($p:expr, $row_pins:expr, $col_pins:expr, $uart_peripheral:expr, 
     ($left_tx:expr, $left_rx:expr), 
     ($right_tx:expr, $right_rx:expr), 
     $irqs:expr) => {{
        use embassy_rp::uart::{self, BufferedUart};

        let (rows, cols) = $crate::matrix_pins!($p, $row_pins, $col_pins);
        let usb_driver = embassy_rp::usb::Driver::new($p.USB, $irqs);

        static TX_BUF: static_cell::StaticCell<[u8; 16]> = static_cell::StaticCell::new();
        static RX_BUF: static_cell::StaticCell<[u8; 16]> = static_cell::StaticCell::new();
        let uart_config = uart::Config::default();

        #[cfg(feature = "left")]
        let (uart_tx, uart_rx) = {
            let uart = BufferedUart::new(
                $uart_peripheral,
                $left_tx,
                $left_rx,
                $irqs,
                TX_BUF.init([0; 16]),
                RX_BUF.init([0; 16]),
                uart_config,
            );
            uart.split()
        };

        #[cfg(feature = "right")]
        let (uart_tx, uart_rx) = {
            let uart = BufferedUart::new(
                $uart_peripheral,
                $right_tx,
                $right_rx,
                $irqs,
                TX_BUF.init([0; 16]),
                RX_BUF.init([0; 16]),
                uart_config,
            );
            uart.split()
        };

        (rows, cols, usb_driver, uart_tx, uart_rx)
    }};
}
