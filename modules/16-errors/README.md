# M16 - Обработка ошибок

> Часть V "Ошибки". Предыдущий модуль: [M15 Продвинутые трейты](../15-advanced-traits/README.md).

## Цель модуля

`Option` и `Result` вы используете с M05. Пора собрать всю систему: **какие ошибки
паникуют, а какие возвращаются**, что происходит при панике (разматывание стека и
RAII), как `?` пробрасывает и **конвертирует** ошибки через `From` (M13 выстрелил),
как проектировать собственные типы ошибок и когда брать `thiserror`/`anyhow`.

После модуля вы умеете:

- [ ] отличать ошибку программиста (паника) от ошибки окружения (`Result`) и
      обосновать выбор для конкретной функции;
- [ ] объяснить stack unwinding, поведение `Drop` при панике и почему паника в
      деструкторе фатальна;
- [ ] свободно пользоваться комбинаторами `Option`/`Result` (`map`, `and_then`,
      `ok_or`, `unwrap_or_else`, ...);
- [ ] написать десахаризацию `?` и объяснить роль `From::from` в ней;
- [ ] спроектировать свой тип ошибки: enum + `Display` + `Error` + `From`;
- [ ] применять `let else` для ранних выходов;
- [ ] выбирать между `thiserror` (библиотека) и `anyhow` (приложение).

---

## 1. Две категории ошибок

| | Ошибка программиста (баг) | Ошибка окружения |
|---|---|---|
| примеры | выход за границу, деление на 0, нарушенный инвариант | файла нет, сеть упала, ввод кривой |
| можно ли предвидеть | нет - это дефект кода | да - это **ожидаемый** исход |
| механизм | `panic!` | `Result<T, E>` / `Option<T>` |
| кто обрабатывает | никто (чинится в коде) | вызывающий - обязан решить |

Правило: паника - для состояний, которые **не должны были случиться**; `Result` -
для исходов, которые случаются в исправной программе. Сигнатура функции - договор:
`fn parse(s: &str) -> Result<Config, ParseError>` честно сообщает "могу не суметь",
и компилятор не даст забыть про `Err` (в отличие от исключений C++/Java, невидимых
в сигнатуре; `throws` в Java - ближайший родственник `Result`).

Статистика в тему (исследование OSDI'14): 92% катастрофических сбоев распределённых
систем - следствие **неверной обработки явно сигнализируемых нефатальных ошибок**.
Rust заставляет эти ошибки видеть.

---

## 2. Паника: разматывание стека и RAII

`panic!` не завершает процесс мгновенно (по умолчанию). Идёт **unwinding**: стек
разматывается кадр за кадром, и у всех живых значений вызывается `Drop` - в обратном
порядке, как обычно:

```rust
struct Guard(&'static str);
impl Drop for Guard {
    fn drop(&mut self) { println!("drop {}", self.0); }
}

fn bar() { let _g = Guard("bar"); panic!("boom"); }
fn foo() { let _g = Guard("foo"); bar(); }
// паника в bar -> печать: "drop bar", затем "drop foo"
```

Отсюда сила RAII из M06: файлы, блокировки, транзакции корректно освобождаются
**одним и тем же путём** и при успехе, и при панике - "try-finally, который нельзя
забыть написать". Альтернативная стратегия - `panic = "abort"` в Cargo.toml:
процесс умирает сразу, без деструкторов (меньше бинарник, embedded).

Ловить панику можно, но только на **границах**:

```rust
let handle = std::thread::spawn(|| might_panic());
let result = handle.join();                        // Err, если поток запаниковал (M22)

let result = std::panic::catch_unwind(|| might_panic());   // явная граница
```

Легитимные границы: поток в пуле, запрос в сервере, плагин, FFI. Использовать
`catch_unwind` как try/catch для логики - антипаттерн.

> [!warning] Паника во время паники = abort
> Если `Drop` паникует, пока уже идёт разматывание, процесс немедленно завершается.
> Деструктор, которому "нужно" паниковать (недосброшенный буфер), обязан проверить
> `std::thread::panicking()` и промолчать, если паника уже идёт.

---

## 3. `Option`/`Result`: рабочий набор комбинаторов

Комбинаторы избавляют от лестниц `match`. Ядро, которое стоит знать наизусть:

```rust
// достать значение:
r.unwrap()                  // T или паника (только в тестах/прототипах)
r.expect("контекст")        // то же + сообщение - всегда лучше unwrap
r.unwrap_or(default)        // T или готовое значение
r.unwrap_or_else(|e| ...)   // T или вычислить из ошибки (лениво)
r.unwrap_or_default()       // T или Default::default()

// преобразовать внутри:
r.map(|t| ...)              // Result<T, E> -> Result<U, E>
r.map_err(|e| ...)          // Result<T, E> -> Result<T, F>
r.and_then(|t| ...)         // цепочка: f возвращает Result (flatMap)

// перейти между мирами:
r.ok()                      // Result<T, E> -> Option<T>
opt.ok_or(err)              // Option<T> -> Result<T, E>
opt.as_ref()                // &Option<T> -> Option<&T>  (предпочитайте правый)
opt.take()                  // забрать из &mut, оставив None (M06 п.8)
```

Стиль: если ветвление по ошибке нетривиально - `match`; если это линейная цепочка
"преобразуй или пробрось" - комбинаторы читаются лучше.

---

## 4. Оператор `?`

`expr?` - сахар, который стоит один раз увидеть развёрнутым:

```rust
match expr {
    Ok(value) => value,
    Err(e) => return Err(From::from(e)),   // <- не просто проброс: ещё и From!
}
```

Свойства:

- работает и с `Option` (`None` пробрасывается);
- смешивать нельзя: в функции `-> Result` оператор `?` на `Option` требует `ok_or`;
- `main` тоже может вернуть `Result` - ошибка напечатается через `Debug`, код выхода
  будет ненулевым;
- **`From::from` в десахаризации** - ключ к следующему разделу: `?` автоматически
  конвертирует тип ошибки в тип из сигнатуры, если есть `From`-реализация.

```rust
use std::fs;

fn read_int(path: &str) -> Result<i32, MyError> {
    let s = fs::read_to_string(path)?;      // io::Error    --From--> MyError
    let n = s.trim().parse()?;              // ParseIntError --From--> MyError
    Ok(n)
}
```

Две строки, два разных типа ошибок, ноль ручных конверсий.

---

## 5. Свой тип ошибки

Полный канон - enum по вариантам отказа + три impl:

```rust
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Validation(String),
}

impl fmt::Display for MyError {                    // (1) человекочитаемый текст
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "ошибка ввода-вывода: {e}"),
            MyError::Parse(e) => write!(f, "не число: {e}"),
            MyError::Validation(msg) => write!(f, "неверные данные: {msg}"),
        }
    }
}

impl std::error::Error for MyError {               // (2) стандартный трейт
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),             // цепочка причин для отчётов
            MyError::Parse(e) => Some(e),
            MyError::Validation(_) => None,
        }
    }
}

impl From<std::io::Error> for MyError {            // (3) From - топливо для ?
    fn from(e: std::io::Error) -> Self { MyError::Io(e) }
}
impl From<std::num::ParseIntError> for MyError {
    fn from(e: std::num::ParseIntError) -> Self { MyError::Parse(e) }
}
```

`std::error::Error` требует `Debug + Display` и даёт `source()` - цепочку причин.
Он же делает ошибку совместимой с `Box<dyn Error>` - "любая ошибка" для случаев,
когда точный тип не важен:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = read_int("data.txt")?;    // любая Error-ошибка сконвертируется сама
    println!("{n}");
    Ok(())
}
```

---

## 6. `let else`: ранний выход без лесенки

Когда нужно "распакуй или выйди", а `?` не подходит (нестандартный выход/дефолт):

```rust
fn describe(input: &str) -> String {
    let Ok(n) = input.parse::<i32>() else {
        return String::from("не число");
    };
    let Some(half) = n.checked_div(2) else {   // паттерн слева, else-ветка обязана
        unreachable!();                        //   расходиться (return/panic/continue)
    };
    format!("половина: {half}")
}
```

Стабильно с Rust 1.65. Отличие от `if let`: успешные привязки остаются в **текущей**
области - код не уезжает вправо.

---

## 7. `thiserror` и `anyhow`

Бойлерплейт из п.5 в экосистеме давно автоматизирован:

**`thiserror`** - derive для ваших типов ошибок. Тот же enum, три impl бесплатно:

```rust
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("ошибка ввода-вывода: {0}")]
    Io(#[from] std::io::Error),               // #[from] генерирует и From, и source
    #[error("не число: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("неверные данные: {0}")]
    Validation(String),
}
```

**`anyhow`** - один универсальный тип `anyhow::Error` ("умный `Box<dyn Error>`") плюс
метод `.context()`:

```rust
use anyhow::{Context, Result};

fn load(path: &str) -> Result<Config> {       // Result<T> = Result<T, anyhow::Error>
    let s = std::fs::read_to_string(path)
        .with_context(|| format!("не смог прочитать {path}"))?;
    parse(&s).context("конфиг не разбирается")
}
```

Правило выбора: **библиотека** - свой enum (`thiserror`): вызывающий матчится по
вариантам. **Приложение** - `anyhow`: точный тип не нужен, нужен понятный отчёт с
контекстом. Оба крейта - тонкие обёртки над механикой этого модуля, никакой магии.

---

## 8. Тизер к M17

Ошибки укрощены - можно строить программы с данными. Часть VI открывают
**коллекции**: `Vec` изнутри (ptr/len/cap и амортизация), `HashMap`/`BTreeMap` и
требования к ключам (`Hash + Eq` против `Ord` - M13 снова в деле), `String`/`&str`
и правда про UTF-8, `Cow<str>` - владей-по-необходимости.

---

## Упражнения

Решайте по порядку, ярусами. В каждом крейте: условие и `todo!()` в `src/lib.rs`,
тесты в `tests/tests.rs` (менять нельзя). Запуск: `cargo test -p ex-16-NN-...`.

**0x - синтаксис**
- `01_option_combinators` - `map`/`and_then`/`unwrap_or`/`ok_or` вместо match.
- `02_result_combinators` - `map`/`map_err`/`ok`/`unwrap_or_else`.
- `03_question_mark` - перепиши лесенку match на `?`.

**1x - базовое**
- `04_option_question` - `?` на Option; смешение с Result через `ok_or`.
- `05_custom_error` - enum-ошибка + `Display`; ручной проброс.
- `06_let_else` - ранние выходы `let else`.

**2x - продвинутое**
- `07_from_conversions` - `From` для двух низкоуровневых ошибок; `?` конвертирует сам.
- `08_error_trait` - `std::error::Error` + `source()`; функция под `Box<dyn Error>`.
- `09_drop_panic_guard` - деструктор с проверкой `thread::panicking()`.

**3x - хитрое**
- `10_thiserror` - тот же тип ошибки через derive (крейт `thiserror`).
- `11_anyhow_app` - "приложение": `anyhow::Result` и `.context()` (крейт `anyhow`).
