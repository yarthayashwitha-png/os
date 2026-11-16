//! In-Kernel Page Replacement Visualizer
//! Demonstrates Tsai & Lei (ACM SAC'17) Log-Based Algorithm on Bare Metal

use crate::println;
use crate::print;
use crate::vga::{WRITER, Color};

pub fn run_kernel_demo() {
    println!("\n=== AETHER-OS BARE-METAL PAGE REPLACEMENT DEMO (ACM SAC'17) ===");
    println!("Paper: Frequency Derived From Reference History (Tsai & Lei)");
    println!("Physical Frames: 4 | Virtual Pages: 0..7 | Log Window Size: 8\n");

    // Reference trace with hot pages 1 and 2, cold pages 3, 4, 5, 6
    let trace: [u8; 16] = [1, 2, 1, 3, 1, 2, 4, 1, 5, 2, 1, 6, 2, 1, 7, 2];
    let mut frames: [i8; 4] = [-1, -1, -1, -1];
    let mut log_window: [i8; 8] = [-1; 8];
    let mut log_head = 0;
    let mut log_size = 0;
    let mut ref_counts: [u8; 8] = [0; 8];

    let mut hits = 0;
    let mut faults = 0;

    println!("Step | Ref Page | Frames [F0, F1, F2, F3] | Log Window Ref Counts | Event");
    println!("-----+----------+-------------------------+-----------------------+--------");

    for (step, &page) in trace.iter().enumerate() {
        // 1. Maintain sliding window log
        if log_size == 8 {
            let tail_idx = (log_head + 8 - log_size) % 8;
            let old_page = log_window[tail_idx];
            if old_page >= 0 && ref_counts[old_page as usize] > 0 {
                ref_counts[old_page as usize] -= 1;
            }
        } else {
            log_size += 1;
        }

        log_window[log_head] = page as i8;
        log_head = (log_head + 1) % 8;
        ref_counts[page as usize] += 1;

        // 2. Check frame hit
        let mut hit = false;
        for &f in frames.iter() {
            if f == page as i8 {
                hit = true;
                break;
            }
        }

        let mut event_str = "FAULT";
        if hit {
            hits += 1;
            event_str = "HIT  ";
        } else {
            faults += 1;
            // Find empty frame or evict frame with lowest reference rate in log
            let mut empty_idx = None;
            for (idx, &f) in frames.iter().enumerate() {
                if f == -1 {
                    empty_idx = Some(idx);
                    break;
                }
            }

            if let Some(idx) = empty_idx {
                frames[idx] = page as i8;
            } else {
                // Algorithm 1: Evict frame with lowest RC in current log window
                let mut victim_idx = 0;
                let mut min_rc = 255;
                for (idx, &f) in frames.iter().enumerate() {
                    let rc = ref_counts[f as usize];
                    if rc < min_rc {
                        min_rc = rc;
                        victim_idx = idx;
                    }
                }
                frames[victim_idx] = page as i8;
                event_str = "EVICT";
            }
        }

        print!(" {:2}  |    P{}     | [", step + 1, page);
        for (i, &f) in frames.iter().enumerate() {
            if f == -1 {
                print!(" - ");
            } else {
                print!(" P{} ", f);
            }
            if i < 3 { print!(","); }
        }
        print!("] | P1:{} P2:{} P3:{} P4:{}    | ",
               ref_counts[1], ref_counts[2], ref_counts[3], ref_counts[4]);

        if event_str == "HIT  " {
            WRITER.lock().set_color(Color::LightGreen, Color::Black);
            print!("HIT\n");
        } else if event_str == "EVICT" {
            WRITER.lock().set_color(Color::Yellow, Color::Black);
            print!("FAULT (REPLACED)\n");
        } else {
            WRITER.lock().set_color(Color::LightRed, Color::Black);
            print!("PAGE FAULT\n");
        }
        WRITER.lock().set_color(Color::LightCyan, Color::Black);
    }

    let hit_ratio = (hits * 100) / trace.len();
    println!("-------------------------------------------------------------------------------");
    println!("Summary: Total References: {} | Hits: {} | Faults: {} | Hit Ratio: {}%\n",
             trace.len(), hits, faults, hit_ratio);
}
