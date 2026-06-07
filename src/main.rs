use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use std::io;
use sysinfo::Disks;

// function to calculate GB from bytes
fn disk_usage_info(total: u64, available: u64) -> (f64, f64, f64) {
    let total_gb = total as f64 / 1_073_741_824.0;
    let available_gb = available as f64 / 1_073_741_824.0;
    let used_gb = total_gb - available_gb;

    let precent = if total > 0 {
        (used_gb / total_gb) * 100.0
    } else {
        0.0
    };

    (precent, used_gb, total_gb)
}

fn main() -> Result<(), io::Error> {
    // get system informations
    let disks = Disks::new_with_refreshed_list();


    // For each disk a loading bar
    for disk in &disks {
        let total = disk.total_space();
        let available = disk.available_space();
        let (precent, used_gb, total_gb) = disk_usage_info(total, available);

        // Prepare text for the current hard drive
        let title_line = Line::from(vec![
            Span::styled("* ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{}", disk.mount_point().display()),
                Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
            ),
            Span::raw(format!(
                " [{:?}] -> {:.1} GB / {:.1} GB ({:.1}%)",
                disk.kind(),
                used_gb,
                total_gb,
                precent
            )),
        ]);

        // print out the text
        println!("{}", title_line);

        // progress bar out of Charakters
        let bar_width = 40;
        let occupied_blocks = ((precent / 100.0) * bar_width as f64).round() as usize;
        
        // Determine color
        let color_code = if precent > 90.0 {
            "\x1b[31m" // red
        } else if precent > 75.0 {
            "\x1b[33m" // yellow
        } else {
            "\x1b[32m" // green
        };
        let reset_code = "\x1b[0m";

        // Assemble bars
        let filled = "#".repeat(occupied_blocks);
        let empty = "~".repeat(bar_width - occupied_blocks);

        println!("   [{}{}{}]\n", color_code, filled, reset_code.to_string() + &empty);
    }

    Ok(())
}