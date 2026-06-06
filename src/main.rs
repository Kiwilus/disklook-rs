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

    let prozent = if total > 0 {
        (used_gb / total_gb) * 100.0
    } else {
        0.0
    };

    (prozent, used_gb, total_gb)
}

fn main() -> Result<(), io::Error> {
    // get system informations
    let disks = Disks::new_with_refreshed_list();


    // For each disk a loading bar
    for disk in &disks {
        let total = disk.total_space();
        let available = disk.available_space();
        let (prozent, used_gb, total_gb) = disk_usage_info(total, available);

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
                prozent
            )),
        ]);

        // print out the text
        println!("{}", title_line);

        // progress bar out of Charakters
        let balken_breite = 40;
        let belegte_blöcke = ((prozent / 100.0) * balken_breite as f64).round() as usize;
        
        // Determine color
        let farb_code = if prozent > 90.0 {
            "\x1b[31m" // Rot
        } else if prozent > 75.0 {
            "\x1b[33m" // Gelb
        } else {
            "\x1b[32m" // Grün
        };
        let reset_code = "\x1b[0m";

        // Assemble bars
        let gefüllt = "#".repeat(belegte_blöcke);
        let leer = "~".repeat(balken_breite - belegte_blöcke);

        println!("   [{}{}{}]\n", farb_code, gefüllt, reset_code.to_string() + &leer);
    }

    Ok(())
}