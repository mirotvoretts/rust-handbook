use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;

use rh::course::Course;
use rh::progress::{hash_src, Progress, Status};
use rh::runner::WatchEvent;
use rh::{runner, ui};

const PROGRESS_FILE: &str = ".rh-progress.json";

struct App {
    root: PathBuf,
    progress_path: PathBuf,
    course: Course,
    progress: Progress,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {e:#}", "ошибка:".red().bold());
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    ensure_cargo()?;
    let root = find_repo_root()?;
    let course = Course::scan(&root.join("modules"))
        .with_context(|| "не удалось просканировать каталог modules/")?;
    if course.total_exercises() == 0 {
        bail!("в modules/ не найдено ни одного упражнения");
    }
    let progress_path = root.join(PROGRESS_FILE);
    let progress = Progress::load(&progress_path);

    let mut app = App {
        root,
        progress_path,
        course,
        progress,
    };

    runner::install_signal_handler();
    main_loop(&mut app)
}

fn banner(app: &App) {
    println!();
    println!("{}", "  rust-handbook".cyan().bold());
    println!(
        "  {} модулей, {} упражнений",
        app.course.modules.len(),
        app.course.total_exercises()
    );
    if let Some(cur) = &app.progress.current {
        println!("  текущее: {}", cur.yellow());
    }
    println!();
}

fn main_loop(app: &mut App) -> Result<()> {
    let items = [
        "Прогресс курса",
        "Список модулей и упражнений",
        "Текущее упражнение",
        "Проверить текущее",
        "Следующее нерешённое",
        "Выход",
    ];
    loop {
        ui::clear();
        banner(app);
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Меню")
            .items(&items)
            .default(0)
            .interact()?;

        match choice {
            0 => {
                ui::clear();
                ui::print_progress(&app.course, &app.progress);
                pause();
            }
            1 => browse_screen(app)?,
            2 => current_screen(app)?,
            3 => check_current(app)?,
            4 => next_unsolved(app)?,
            _ => break,
        }
    }
    Ok(())
}

fn browse_screen(app: &mut App) -> Result<()> {
    ui::clear();
    let module_items: Vec<String> = app
        .course
        .modules
        .iter()
        .map(|m| {
            let done = m
                .exercises
                .iter()
                .filter(|e| app.progress.is_passed(&e.ex_pkg))
                .count();
            format!("M{:02}  {}  ({}/{})", m.number, m.title, done, m.exercises.len())
        })
        .collect();

    let Some(m_idx) = select_or_back("Модуль", &module_items)? else {
        return Ok(());
    };
    let module = &app.course.modules[m_idx];
    if module.exercises.is_empty() {
        println!("{}", "  в модуле нет упражнений".dimmed());
        pause();
        return Ok(());
    }

    let ex_items: Vec<String> = module
        .exercises
        .iter()
        .map(|e| ui::exercise_line(e, app.progress.status_of(&e.ex_pkg)))
        .collect();

    ui::clear();
    let Some(e_idx) = select_or_back("Упражнение", &ex_items)? else {
        return Ok(());
    };
    let ex_pkg = module.exercises[e_idx].ex_pkg.clone();
    set_current(app, &ex_pkg)?;
    exercise_screen(app, &ex_pkg)
}

fn current_screen(app: &mut App) -> Result<()> {
    let Some(ex_pkg) = app.progress.current.clone() else {
        ui::clear();
        println!("{}", "  текущее упражнение не выбрано (пункт \"Список\")".dimmed());
        pause();
        return Ok(());
    };
    exercise_screen(app, &ex_pkg)
}

fn exercise_screen(app: &mut App, ex_pkg: &str) -> Result<()> {
    loop {
        ui::clear();
        let Some(ex) = app.course.find(ex_pkg).cloned() else {
            println!("{}", "  упражнение не найдено".red());
            pause();
            return Ok(());
        };
        let module = app
            .course
            .modules
            .iter()
            .find(|m| m.number == ex.module_number);

        println!();
        println!(
            "{}  {}",
            ui::status_glyph(app.progress.status_of(ex_pkg)),
            ex.ex_pkg.bold()
        );
        if let Some(m) = module {
            println!("  модуль M{:02} - {}", m.number, m.title.dimmed());
        }
        println!("  {}  {}", "ярус:".dimmed(), ex.tier.label());
        println!("  {}  {}", "путь:".dimmed(), ex.dir.display());
        println!();
        println!("{}", "Условие:".bold());
        ui::print_task_from_src(&ex.src, 8);
        println!();

        let actions = [
            "Проверить один раз",
            "Watch (авто-проверка при сохранении)",
            "Показать теорию модуля",
            "Открыть в редакторе",
            "Назад в меню",
        ];
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Действие")
            .items(&actions)
            .default(0)
            .interact()?;

        match choice {
            0 => {
                ui::clear();
                check_pkg(app, ex_pkg)?;
                pause();
            }
            1 => watch_pkg(app, ex_pkg)?,
            2 => {
                if let Some(m) = module {
                    ui::clear();
                    match std::fs::read_to_string(&m.readme) {
                        Ok(md) => ui::render_markdown(&md),
                        Err(e) => println!("{} {e}", "не удалось прочитать README:".red()),
                    }
                    pause();
                }
            }
            3 => open_in_editor(&ex.src)?,
            _ => return Ok(()),
        }
    }
}

fn check_current(app: &mut App) -> Result<()> {
    let Some(ex_pkg) = app.progress.current.clone() else {
        println!("{}", "  текущее упражнение не выбрано".dimmed());
        pause();
        return Ok(());
    };
    ui::clear();
    check_pkg(app, &ex_pkg)?;
    pause();
    Ok(())
}

fn next_unsolved(app: &mut App) -> Result<()> {
    let next = app
        .course
        .exercises()
        .find(|e| !app.progress.is_passed(&e.ex_pkg))
        .map(|e| e.ex_pkg.clone());

    ui::clear();
    match next {
        Some(ex_pkg) => {
            println!("{} {}", "Следующее:".green().bold(), ex_pkg.yellow());
            set_current(app, &ex_pkg)?;
            exercise_screen(app, &ex_pkg)
        }
        None => {
            println!("{}", "  все упражнения решены!".green().bold());
            pause();
            Ok(())
        }
    }
}

fn check_pkg(app: &mut App, ex_pkg: &str) -> Result<()> {
    let ex = match app.course.find(ex_pkg).cloned() {
        Some(ex) => ex,
        None => return Ok(()),
    };
    let hash = hash_src(&ex.src);

    if let Some(status) = app.progress.cached_status(ex_pkg, &hash) {
        println!("{}", "  (из кеша, файл не менялся)".dimmed());
        let fake = runner::TestOutcome {
            passed: status == Status::Pass,
            output: String::new(),
        };
        ui::print_outcome(&ex, &fake);
        return Ok(());
    }

    let outcome = ui::run_with_spinner(&app.root, ex_pkg)?;
    let status = if outcome.passed { Status::Pass } else { Status::Fail };
    app.progress.record(ex_pkg, status, hash);
    app.progress.save(&app.progress_path)?;
    ui::print_outcome(&ex, &outcome);
    Ok(())
}

fn watch_pkg(app: &mut App, ex_pkg: &str) -> Result<()> {
    let ex = match app.course.find(ex_pkg).cloned() {
        Some(ex) => ex,
        None => return Ok(()),
    };

    let draw = |outcome: &runner::TestOutcome| {
        ui::clear();
        println!("{}  {}", "watch".cyan().bold(), ex.ex_pkg.bold());
        println!("{}", "  сохрани файл, чтобы перепроверить.".dimmed());
        println!("{}", "  Ctrl+C - выход в меню.".dimmed());
        println!();
        ui::print_outcome(&ex, outcome);
    };

    ui::clear();
    println!("{}", "watch включён...".cyan());
    let first = ui::run_with_spinner(&app.root, ex_pkg)?;
    let status = if first.passed { Status::Pass } else { Status::Fail };
    app.progress.record(ex_pkg, status, hash_src(&ex.src));
    app.progress.save(&app.progress_path)?;
    draw(&first);

    let root = app.root.clone();
    let progress_path = app.progress_path.clone();
    let progress = &mut app.progress;

    runner::watch(&root, &ex.ex_pkg, &ex.dir, |event| match event {
        WatchEvent::Rechecked(outcome) => {
            let status = if outcome.passed { Status::Pass } else { Status::Fail };
            progress.record(&ex.ex_pkg, status, hash_src(&ex.src));
            let _ = progress.save(&progress_path);
            draw(&outcome);
        }
        WatchEvent::Stopped => {
            println!();
            println!("{}", "  watch остановлен".dimmed());
        }
    })?;

    pause();
    Ok(())
}

fn set_current(app: &mut App, ex_pkg: &str) -> Result<()> {
    app.progress.set_current(ex_pkg);
    app.progress.save(&app.progress_path)?;
    Ok(())
}

fn open_in_editor(src: &Path) -> Result<()> {
    let editor = match pick_editor()? {
        Some(e) => e,
        None => return Ok(()),
    };
    let mut parts = editor.split_whitespace();
    let bin = parts.next().unwrap_or("nano");
    let status = Command::new(bin).args(parts).arg(src).status();
    match status {
        Ok(s) if s.success() => {}
        Ok(_) => println!("{}", "  редактор завершился с ошибкой".red()),
        Err(e) => println!("{} {e}", format!("  не удалось запустить {bin}:").red()),
    }
    Ok(())
}

fn pick_editor() -> Result<Option<String>> {
    if let Ok(e) = env::var("EDITOR").or_else(|_| env::var("VISUAL")) {
        if !e.trim().is_empty() {
            return Ok(Some(e));
        }
    }

    let candidates = ["micro", "nano", "hx", "code", "vim", "vi"];
    let available: Vec<&str> = candidates
        .into_iter()
        .filter(|name| binary_in_path(name))
        .collect();

    if available.is_empty() {
        println!("{}", "  не найден редактор; поставь nano или micro, либо задай $EDITOR".red());
        pause();
        return Ok(None);
    }
    if available.len() == 1 {
        return Ok(Some(available[0].to_string()));
    }

    ui::clear();
    let mut items: Vec<String> = available.iter().map(|s| s.to_string()).collect();
    items.push("<- Отмена".to_string());
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Чем открыть?")
        .items(&items)
        .default(0)
        .interact()?;
    if choice == available.len() {
        Ok(None)
    } else {
        Ok(Some(available[choice].to_string()))
    }
}

fn binary_in_path(name: &str) -> bool {
    let Some(path) = env::var_os("PATH") else {
        return false;
    };
    env::split_paths(&path).any(|dir| dir.join(name).is_file())
}

fn select_or_back(prompt: &str, items: &[String]) -> Result<Option<usize>> {
    let mut all: Vec<String> = items.to_vec();
    all.push("<- Назад".to_string());
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&all)
        .default(0)
        .interact()?;
    if choice == items.len() {
        Ok(None)
    } else {
        Ok(Some(choice))
    }
}

fn pause() {
    print!("{}", "  Enter для продолжения...".dimmed());
    let _ = io::stdout().flush();
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}

fn ensure_cargo() -> Result<()> {
    Command::new("cargo")
        .arg("--version")
        .output()
        .map_err(|_| {
            anyhow::anyhow!("не найден `cargo` в PATH - установи Rust: https://rustup.rs")
        })?;
    Ok(())
}

fn find_repo_root() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;
    loop {
        if dir.join("modules").is_dir() && dir.join("Cargo.toml").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            bail!("запусти из каталога курса rust-handbook (не найден modules/)");
        }
    }
}
