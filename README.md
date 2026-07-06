# Rust Handbook

[![verify](https://github.com/mirotvoretts/rust-handbook/actions/workflows/verify.yml/badge.svg)](https://github.com/mirotvoretts/rust-handbook/actions/workflows/verify.yml)

*[Русская версия ниже](#справочник-по-rust)*

A Rust course focused on the memory model and systems programming. Theory
explains exactly what the language guarantees and what it compiles down to;
exercises reinforce every topic with code that must pass a fixed test suite.

## Table of contents

- [How it's organized](#how-its-organized)
- [How to work through it](#how-to-work-through-it)
- [Curriculum](#curriculum)
- [Requirements](#requirements)
- [License](#license)
- [Русская версия](#справочник-по-rust)

## How it's organized

- **Theory** - in `modules/NN-topic/README.md`. This is the core of the
  course, meant to be read in full.
- **Exercises** - crates under `modules/NN-topic/exercises/`. Each one has a
  `src/lib.rs` with the problem statement and `todo!()` stubs, and a
  `tests/tests.rs` with the fixed test suite (do not edit the tests).
- **Solutions** - reference implementations in `modules/NN-topic/solutions/`.

The exercise number encodes its difficulty tier: `0x` syntax, `1x` basic,
`2x` advanced, `3x` tricky.

## How to work through it

```sh
# one exercise:
cargo test -p ex-06-01-move-basics
```

`cargo test --workspace` is **not** the right command here: exercise crates
(`ex-*`) are intentionally broken or failing until you solve them - that's
the point, not a bug. Use `cargo test -p ex-...` for the exercise you're on,
and `./scripts/verify.sh` to check all reference solutions are green.

Workflow: open `modules/NN-topic/exercises/NN_slug/src/lib.rs`, read the
problem, replace `todo!()` with your implementation, run
`cargo test -p ex-...` until it's green. Only look in `solutions/` after a
genuine attempt.

## Curriculum

The full module list is in [CURRICULUM.md](CURRICULUM.md): about 33 modules
across 11 parts. The course is filled in incrementally; finished modules are
marked in the curriculum. 19 modules are available right now, from the
toolchain through collections.

## Requirements

Rust stable (pinned in `rust-toolchain.toml`). Nothing beyond `cargo` is
needed to work through the course - some reference solutions pull small
crates from crates.io (currently `anyhow`, `thiserror`) automatically via
their own `Cargo.toml`.

## License

MIT. See [LICENSE](LICENSE).

---

# Справочник по Rust 

Курс по Rust с упором на модель памяти и системное программирование.
Теория разбирает, что именно язык гарантирует и во что это компилируется; упражнения
закрепляют каждую тему кодом, который проходит готовые тесты.

## Оглавление

- [Как устроено](#как-устроено)
- [Как проходить](#как-проходить)
- [Программа курса](#программа-курса)
- [Требования](#требования)
- [Лицензия](#лицензия)
- [English version](#rust-course)

## Как устроено

- **Теория** - в `modules/NN-topic/README.md`. Это ядро курса, читать целиком.
- **Упражнения** - крейты в `modules/NN-topic/exercises/`. В каждом `src/lib.rs` лежит условие
  задачи и заглушки `todo!()`, а `tests/tests.rs` содержит готовые тесты (менять их нельзя).
- **Решения** - эталоны в `modules/NN-topic/solutions/`.

Номер упражнения кодирует ярус сложности: `0x` - синтаксис, `1x` - базовое, `2x` - продвинутое,
`3x` - хитрое.

## Как проходить

```sh
# одно упражнение:
cargo test -p ex-06-01-move-basics
```

`cargo test --workspace` тут не подходит: крейты упражнений (`ex-*`) намеренно не
компилируются или не проходят тесты, пока их не решишь, - это норма, а не баг.
Используйте `cargo test -p ex-...` для конкретного упражнения и `./scripts/verify.sh`,
чтобы проверить все эталонные решения разом.

Порядок работы: открыть `modules/NN-topic/exercises/NN_slug/src/lib.rs`, прочитать условие,
заменить `todo!()` на реализацию, гонять `cargo test -p ex-...` до зелёного. В `solutions/`
заглядывать после честной попытки.

## Программа курса

Полный список модулей - в [CURRICULUM.md](CURRICULUM.md): около 33 модулей в 11 частях.
Курс наполняется постепенно; готовые модули отмечены в программе. Сейчас доступны 19
модулей, от тулчейна до коллекций.

## Требования

Rust stable (версия зафиксирована в `rust-toolchain.toml`). Кроме `cargo` ничего не нужно:
некоторые эталонные решения тянут небольшие крейты с crates.io (сейчас это `anyhow`,
`thiserror`) автоматически через свой `Cargo.toml`.

## Лицензия

MIT. См. [LICENSE](LICENSE).
