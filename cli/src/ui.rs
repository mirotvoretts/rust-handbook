use std::path::Path;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;

use crate::course::{Course, Exercise, Tier};
use crate::progress::{Progress, Status};
use crate::runner::{self, TestOutcome};

pub fn status_glyph(status: Status) -> String {
    match status {
        Status::Pass => "[+]".green().bold().to_string(),
        Status::Fail => "[!]".red().bold().to_string(),
        Status::Unknown => "[ ]".dimmed().to_string(),
    }
}

pub fn exercise_line(ex: &Exercise, status: Status) -> String {
    format!(
        "{} {:02}  {}  {}",
        status_glyph(status),
        ex.number,
        ex.slug,
        format!("({})", ex.tier.label()).dimmed()
    )
}

pub fn render_markdown(md: &str) {
    let skin = termimad::MadSkin::default();
    skin.print_text(md);
}

pub fn print_task_from_src(src: &Path) {
    let text = match std::fs::read_to_string(src) {
        Ok(t) => t,
        Err(_) => {
            println!("{}", "  (не удалось прочитать src/lib.rs)".dimmed());
            return;
        }
    };
    let mut any = false;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("//!") {
            println!("  {}", rest.trim_start().dimmed());
            any = true;
        } else if any {
            break;
        }
    }
}

pub fn run_with_spinner(root: &Path, ex_pkg: &str) -> anyhow::Result<TestOutcome> {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(90));
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["-", "\\", "|", "/", "*"]),
    );
    spinner.set_message(format!("cargo test -p {ex_pkg}"));

    let outcome = runner::run_test(root, ex_pkg)?;
    spinner.finish_and_clear();
    Ok(outcome)
}

pub fn print_outcome(ex: &Exercise, outcome: &TestOutcome) {
    if outcome.passed {
        println!("{}  {}", "[+] решено".green().bold(), ex.ex_pkg.green());
    } else {
        println!("{}  {}", "[!] не пройдено".red().bold(), ex.ex_pkg.red());
        println!();
        println!("{}", runner::tail(&outcome.output, 20).dimmed());
    }
}

pub fn print_progress(course: &Course, progress: &Progress) {
    let total = course.total_exercises();
    let done = course
        .exercises()
        .filter(|e| progress.is_passed(&e.ex_pkg))
        .count();

    println!();
    println!("{}", "Прогресс курса".bold().underline());
    println!();
    print_bar(done, total);
    println!();

    println!("{}", "По ярусам:".bold());
    for tier in [Tier::Syntax, Tier::Basic, Tier::Advanced, Tier::Tricky] {
        let (t_done, t_total) = tier_counts(course, progress, tier);
        if t_total > 0 {
            println!("  {:<14} {}/{}", tier.label(), t_done, t_total);
        }
    }
    println!();

    println!("{}", "По модулям:".bold());
    for m in &course.modules {
        if m.exercises.is_empty() {
            continue;
        }
        let m_done = m
            .exercises
            .iter()
            .filter(|e| progress.is_passed(&e.ex_pkg))
            .count();
        let marker = if m_done == m.exercises.len() {
            "[+]".green().to_string()
        } else {
            "[ ]".dimmed().to_string()
        };
        println!(
            "  {} M{:02}  {:<40} {}/{}",
            marker,
            m.number,
            truncate(&m.title, 40),
            m_done,
            m.exercises.len()
        );
    }
    println!();
}

fn tier_counts(course: &Course, progress: &Progress, tier: Tier) -> (usize, usize) {
    let mut done = 0;
    let mut total = 0;
    for e in course.exercises() {
        if e.tier == tier {
            total += 1;
            if progress.is_passed(&e.ex_pkg) {
                done += 1;
            }
        }
    }
    (done, total)
}

fn print_bar(done: usize, total: usize) {
    let width = 30usize;
    let filled = (done * width).checked_div(total).unwrap_or(0);
    let bar: String = "#".repeat(filled) + &"-".repeat(width - filled);
    let pct = (done * 100).checked_div(total).unwrap_or(0);
    println!("  [{}] {}/{} ({}%)", bar.green(), done, total, pct);
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{cut}...")
    }
}
