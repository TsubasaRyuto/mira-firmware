use core::sync::atomic::Ordering;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio_programs::uart::{PioUartRx, PioUartTx};
use embassy_time::{Duration, Timer};
use embedded_io_async::{Read, Write};
use portable_atomic::AtomicU32;

// スレーブから受信したキーボードマスク（20ビット分）を保持するグローバル変数
pub static SLAVE_KEYS: AtomicU32 = AtomicU32::new(0);

const SYNC_BYTE: u8 = 0xAA;

/// スレーブ側の送信タスク
#[embassy_executor::task]
pub async fn serial_tx_task(mut tx: PioUartTx<'static, PIO0, 0>) -> ! {
    defmt::info!("Serial TX task started");
    let mut packet = [0u8; 5];
    packet[0] = SYNC_BYTE;

    loop {
        // スレーブ自身の最新 of キー状態を取得する
        let mask = MY_SLAVE_KEYS.load(Ordering::Relaxed);

        // 定常送信 (2ms 周期)
        packet[1] = (mask & 0xFF) as u8;
        packet[2] = ((mask >> 8) & 0xFF) as u8;
        packet[3] = ((mask >> 16) & 0xFF) as u8;
        packet[4] = packet[1] ^ packet[2] ^ packet[3]; // 簡易チェックサム

        match tx.write_all(&packet).await {
            Ok(()) => {}
            Err(e) => {
                defmt::warn!("Serial write error: {:?}", e);
            }
        }

        Timer::after(Duration::from_millis(2)).await;
    }
}

/// マスタ側の受信タスク
#[embassy_executor::task]
pub async fn serial_rx_task(mut rx: PioUartRx<'static, PIO0, 0>) -> ! {
    defmt::info!("Serial RX task started");
    let mut buf = [0u8; 1];
    let mut packet = [0u8; 5];
    let mut idx = 0;

    loop {
        match rx.read_exact(&mut buf).await {
            Ok(()) => {
                let b = buf[0];
                if idx == 0 {
                    if b == SYNC_BYTE {
                        packet[0] = b;
                        idx = 1;
                    }
                } else {
                    packet[idx] = b;
                    idx += 1;
                    if idx == 5 {
                        // パケット検証
                        let checksum = packet[1] ^ packet[2] ^ packet[3];
                        if packet[4] == checksum {
                            let mask = (packet[1] as u32)
                                | ((packet[2] as u32) << 8)
                                | ((packet[3] as u32) << 16);
                            SLAVE_KEYS.store(mask, Ordering::Relaxed);
                        } else {
                            defmt::warn!("Serial packet checksum mismatch");
                        }
                        idx = 0; // 次の同期へ
                    }
                }
            }
            Err(e) => {
                defmt::warn!("Serial read error: {:?}", e);
                idx = 0; // エラー時は同期リセット
                Timer::after(Duration::from_millis(10)).await;
            }
        }
    }
}

// スレーブ自身がスキャンして得たキー状態をTXタスクに渡すためのグローバル変数
pub static MY_SLAVE_KEYS: AtomicU32 = AtomicU32::new(0);
