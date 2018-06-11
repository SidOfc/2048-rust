use super::masks::COL_MASK;
use std::time::Duration;
use std::thread::sleep;

// board helpers
pub struct Helpers;

#[allow(dead_code)]
impl Helpers {
    pub fn sleep(t: u64) -> u64 {
        sleep(Duration::from_millis(t));
        t
    }

    pub fn transpose(board: u64) -> u64 {
        let a1 = board & 0xF0F0_0F0F_F0F0_0F0F_u64;
        let a2 = board & 0x0000_F0F0_0000_F0F0_u64;
        let a3 = board & 0x0F0F_0000_0F0F_0000_u64;

        let a  = a1 | (a2 << 12) | (a3 >> 12);

        let b1 = a & 0xFF00_FF00_00FF_00FF_u64;
        let b2 = a & 0x00FF_00FF_0000_0000_u64;
        let b3 = a & 0x0000_0000_FF00_FF00_u64;

        b1 | (b2 >> 24) | (b3 << 24)
    }

    pub fn column_from(row: u64) -> u64 {
        (row | (row << 12) | (row << 24) | (row << 36)) & COL_MASK
    }

    pub fn print(board: u64) {
        let spacer: String  = " ".repeat(0);

        // map 4 bits to one digit, 64 bits / 16 cells / 4 bits per cell.
        let cells: Vec<u64> = (0..16).rev().map(|n| 1_u64 << (board >> (n << 2) & 0xF))
                                           .map(|r| if r > 1 { r } else { 0 }).collect();

        // print top area.
        println!("{}*-------------------------------------------*", spacer);
        println!("{}|   _____________________________________   |", spacer);
        println!("{}|   |        |        |        |        |   |", spacer);

        // print middle area.
        println!("{}|   |{:^8}|{:^8}|{:^8}|{:^8}|   |",             spacer, cells[0], cells[1], cells[2], cells[3]);
        println!("{}|   |--------|--------|--------|--------|   |", spacer);
        println!("{}|   |{:^8}|{:^8}|{:^8}|{:^8}|   |",             spacer, cells[4], cells[5], cells[6], cells[7]);
        println!("{}|   |--------|--------|--------|--------|   |", spacer);
        println!("{}|   |{:^8}|{:^8}|{:^8}|{:^8}|   |",             spacer, cells[8], cells[9], cells[10], cells[11]);
        println!("{}|   |--------|--------|--------|--------|   |", spacer);
        println!("{}|   |{:^8}|{:^8}|{:^8}|{:^8}|   |",             spacer, cells[12], cells[13], cells[14], cells[15]);

        // print bottom area.
        println!("{}|   |________|________|________|________|   |", spacer);
        println!("{}|                                           |", spacer);
        println!("{}*-------------------------------------------*", spacer);
    }
}
