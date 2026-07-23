
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    Syntax,
    Basic,
    Advanced,
    Tricky,
}

impl Tier {
    pub fn from_number(number: u32) -> Tier {
        match number / 10 {
            0 => Tier::Syntax,
            1 => Tier::Basic,
            2 => Tier::Advanced,
            _ => Tier::Tricky,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Tier::Syntax => "синтаксис",
            Tier::Basic => "базовое",
            Tier::Advanced => "продвинутое",
            Tier::Tricky => "хитрое",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPkg {
    pub kind: PkgKind,
    pub module_number: u32,
    pub number: u32,
    pub slug: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PkgKind {
    Exercise,
    Solution,
}

pub fn parse_pkg_name(name: &str) -> Option<ParsedPkg> {
    let (kind, rest) = if let Some(rest) = name.strip_prefix("ex-") {
        (PkgKind::Exercise, rest)
    } else if let Some(rest) = name.strip_prefix("sol-") {
        (PkgKind::Solution, rest)
    } else {
        return None;
    };

    let mut parts = rest.splitn(3, '-');
    let module_number = parts.next()?.parse().ok()?;
    let number = parts.next()?.parse().ok()?;
    let slug = parts.next()?.to_string();
    if slug.is_empty() {
        return None;
    }

    Some(ParsedPkg {
        kind,
        module_number,
        number,
        slug,
    })
}

#[derive(Debug, Clone)]
pub struct Exercise {
    pub ex_pkg: String,
    #[allow(dead_code)]
    pub sol_pkg: Option<String>,
    pub module_number: u32,
    pub number: u32,
    pub tier: Tier,
    pub slug: String,
    pub dir: PathBuf,
    pub src: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub number: u32,
    pub title: String,
    pub readme: PathBuf,
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Clone)]
pub struct Course {
    pub modules: Vec<Module>,
}

impl Course {
    pub fn scan(modules_dir: &Path) -> Result<Course> {
        let mut modules = Vec::new();

        let entries = fs::read_dir(modules_dir)
            .with_context(|| format!("не читается каталог модулей {}", modules_dir.display()))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if let Some(module) = scan_module(&path)? {
                modules.push(module);
            }
        }

        modules.sort_by_key(|m| m.number);
        Ok(Course { modules })
    }

    pub fn exercises(&self) -> impl Iterator<Item = &Exercise> {
        self.modules.iter().flat_map(|m| m.exercises.iter())
    }

    pub fn total_exercises(&self) -> usize {
        self.exercises().count()
    }

    pub fn find(&self, ex_pkg: &str) -> Option<&Exercise> {
        self.exercises().find(|e| e.ex_pkg == ex_pkg)
    }
}

fn scan_module(dir: &Path) -> Result<Option<Module>> {
    let dir_name = match dir.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return Ok(None),
    };
    let number: u32 = match dir_name.split('-').next().and_then(|n| n.parse().ok()) {
        Some(n) => n,
        None => return Ok(None),
    };

    let readme = dir.join("README.md");
    if !readme.exists() {
        return Ok(None);
    }
    let title = read_title(&readme).unwrap_or_else(|| format!("Модуль {number:02}"));

    let mut exercises = Vec::new();
    let exercises_dir = dir.join("exercises");
    if exercises_dir.is_dir() {
        for entry in fs::read_dir(&exercises_dir)? {
            let ex_dir = entry?.path();
            if !ex_dir.is_dir() {
                continue;
            }
            if let Some(ex) = scan_exercise(&ex_dir)? {
                exercises.push(ex);
            }
        }
    }
    exercises.sort_by_key(|e| e.number);

    Ok(Some(Module {
        number,
        title,
        readme,
        exercises,
    }))
}

fn scan_exercise(dir: &Path) -> Result<Option<Exercise>> {
    let cargo_toml = dir.join("Cargo.toml");
    if !cargo_toml.exists() {
        return Ok(None);
    }
    let name = match read_package_name(&cargo_toml)? {
        Some(name) => name,
        None => return Ok(None),
    };
    let parsed = match parse_pkg_name(&name) {
        Some(p) if p.kind == PkgKind::Exercise => p,
        _ => return Ok(None),
    };

    let sol_pkg = format!(
        "sol-{:02}-{:02}-{}",
        parsed.module_number, parsed.number, parsed.slug
    );

    Ok(Some(Exercise {
        ex_pkg: name,
        sol_pkg: Some(sol_pkg),
        module_number: parsed.module_number,
        number: parsed.number,
        tier: Tier::from_number(parsed.number),
        slug: parsed.slug,
        src: dir.join("src").join("lib.rs"),
        dir: dir.to_path_buf(),
    }))
}

fn read_package_name(cargo_toml: &Path) -> Result<Option<String>> {
    let text = fs::read_to_string(cargo_toml)?;
    Ok(parse_package_name(&text))
}

pub fn parse_package_name(cargo_toml: &str) -> Option<String> {
    let mut in_package = false;
    for line in cargo_toml.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }
        if in_package {
            if let Some(rest) = line.strip_prefix("name") {
                let rest = rest.trim_start().strip_prefix('=')?.trim();
                let name = rest.trim_matches(|c| c == '"' || c == '\'');
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

fn read_title(readme: &Path) -> Option<String> {
    let text = fs::read_to_string(readme).ok()?;
    let first = text.lines().next()?;
    let title = first.trim_start_matches('#').trim();
    if title.is_empty() {
        None
    } else {
        Some(title.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_exercise_name() {
        let p = parse_pkg_name("ex-33-01-array-sum").unwrap();
        assert_eq!(p.kind, PkgKind::Exercise);
        assert_eq!(p.module_number, 33);
        assert_eq!(p.number, 1);
        assert_eq!(p.slug, "array-sum");
    }

    #[test]
    fn parses_solution_name_with_multiword_slug() {
        let p = parse_pkg_name("sol-06-21-rc-ref-cell").unwrap();
        assert_eq!(p.kind, PkgKind::Solution);
        assert_eq!(p.module_number, 6);
        assert_eq!(p.number, 21);
        assert_eq!(p.slug, "rc-ref-cell");
    }

    #[test]
    fn rejects_bad_names() {
        assert!(parse_pkg_name("array-sum").is_none());
        assert!(parse_pkg_name("ex-33-array").is_none());
        assert!(parse_pkg_name("ex-33-01-").is_none());
        assert!(parse_pkg_name("ex-xx-01-slug").is_none());
    }

    #[test]
    fn tier_from_number() {
        assert_eq!(Tier::from_number(1), Tier::Syntax);
        assert_eq!(Tier::from_number(11), Tier::Basic);
        assert_eq!(Tier::from_number(21), Tier::Advanced);
        assert_eq!(Tier::from_number(31), Tier::Tricky);
    }

    #[test]
    fn extracts_package_name() {
        let toml = "[package]\nname = \"ex-33-01-array-sum\"\nversion = \"0.1.0\"\n";
        assert_eq!(
            parse_package_name(toml).as_deref(),
            Some("ex-33-01-array-sum")
        );
    }

    #[test]
    fn package_name_only_from_package_section() {
        let toml = "[dependencies]\nname = \"wrong\"\n\n[package]\nname = \"right\"\n";
        assert_eq!(parse_package_name(toml).as_deref(), Some("right"));
    }
}
