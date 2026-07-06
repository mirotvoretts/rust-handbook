# M13 — Трейты-операторы и конверсии

> Часть IV «Трейты и дженерики». Предыдущий модуль: [M12 Трейты](../12-traits/README.md).
> Дальше: [M14 Trait objects](../14-trait-objects/README.md).

## Цель модуля

Увидеть, что **каждый оператор Rust — сахар для вызова трейтового метода**: `a + b` —
это `Add::add(a, b)`, `a == b` — `PartialEq::eq(&a, &b)`, `xs[i]` —
`*Index::index(&xs, i)`. Реализовав нужный трейт, ваш тип участвует в операторах на
равных с `i32`. Здесь же — конверсии `From`/`Into`/`TryFrom` и печать
`Display`/`Debug`: фундамент любого публичного API.

После модуля вы умеете:

- [ ] перегружать арифметику: `Add`/`Sub`/`Mul`/`Neg` и `*Assign`-семейство;
- [ ] объяснить роль `Output` (ассоциированный тип) и `Rhs` (параметр с default'ом);
- [ ] реализовать `PartialEq` руками и знать, почему `f64` не `Eq`;
- [ ] реализовать `Ord` через `Ordering` и сортировать свои типы;
- [ ] писать `Display` (для людей) и выводить `Debug` (для отладки);
- [ ] проектировать конверсии: `From` — и получать `Into` бесплатно; `TryFrom` — для
      конверсий с отказом;
- [ ] перегружать `[]` через `Index`/`IndexMut`, включая экзотические индексы.

---

## 1. Оператор = трейт: карта соответствия

| Выражение | Разворачивается в | Трейт (`std::ops`/`std::cmp`) |
|---|---|---|
| `a + b` | `Add::add(a, b)` | `Add` (аналогично `Sub`, `Mul`, `Div`, `Rem`) |
| `a += b` | `AddAssign::add_assign(&mut a, b)` | `AddAssign` и семейство |
| `-a` | `Neg::neg(a)` | `Neg` (унарный; `Not` для `!`) |
| `a == b` | `PartialEq::eq(&a, &b)` | `PartialEq` (`!=` — `ne`) |
| `a < b` | `PartialOrd::partial_cmp(...)` | `PartialOrd`/`Ord` |
| `xs[i]` | `*Index::index(&xs, i)` | `Index`/`IndexMut` |
| `*p` | `*Deref::deref(&p)` | `Deref`/`DerefMut` (M20) |
| `println!("{a}")` | `Display::fmt(&a, f)` | `Display` (`{:?}` — `Debug`) |

В C++ то же называется перегрузкой операторов (`operator+`). Отличия Rust: перегрузка
возможна **только** через трейты (нет свободных `operator`-функций), набор операторов
фиксирован, и требования выражены в системе типов — обобщённая функция честно пишет
`T: Add<Output = T>`.

---

## 2. Арифметика: `Add` и его анатомия

```rust
pub trait Add<Rhs = Self> {          // Rhs — параметр типа с default'ом Self
    type Output;                     // ассоциированный тип: тип результата
    fn add(self, rhs: Rhs) -> Self::Output;   // берёт операнды ПО ЗНАЧЕНИЮ
}
```

Три вещи, которые здесь важны:

- **`Rhs = Self`** — по умолчанию складываем два одинаковых типа, но можно и разные:
  `impl Add<f64> for Money`, `impl Mul<f64> for Vec2` (вектор × скаляр).
- **`type Output`** — тип результата не обязан совпадать с операндами
  (`&str`-конкатенации нет, а `Instant - Instant = Duration` — есть).
- **`self` по значению** — для `Copy`-типов незаметно, для владеющих — оператор
  поглощает операнды (поэтому для больших типов реализуют и `impl Add for &BigInt`).

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 { pub x: f64, pub y: f64 }

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Mul<f64> for Vec2 {   // Rhs = f64: вектор × скаляр
    type Output = Vec2;
    fn mul(self, k: f64) -> Vec2 { Vec2 { x: self.x * k, y: self.y * k } }
}
```

`AddAssign` (`+=`) — отдельный трейт с `&mut self`; компилятор **не** выводит его из
`Add` автоматически, реализуйте оба, если тип используется с `+=`.

---

## 3. Равенство: `PartialEq` и `Eq`

```rust
pub trait PartialEq<Rhs = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }   // default
}
pub trait Eq: PartialEq<Self> {}     // маркер: «отношение полное»
```

Почему «partial»? `Eq` обещает **полную эквивалентность** (рефлексивность в том
числе: `a == a` всегда). `f64` нарушает её: `NAN != NAN` — поэтому `f64: PartialEq`,
но не `Eq`. Практическое следствие: `f64` нельзя класть ключом в `HashMap` (тому
нужен `Eq + Hash`).

Ручной impl нужен, когда равенство хитрее почленного:

```rust
struct Username(String);

impl PartialEq for Username {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)   // регистронезависимо
    }
}
```

`Rhs` позволяет сравнивать **разные** типы: `impl PartialEq<str> for Username` — и
`user == *"alice"` работает. Стандартная библиотека так сравнивает `String` с `&str`.

---

## 4. Порядок: `PartialOrd`, `Ord`, `Ordering`

```rust
pub enum Ordering { Less, Equal, Greater }

pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;  // None — «несравнимы»
}
pub trait Ord: Eq + PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;                 // всегда есть ответ
}
```

`f64::NAN` ни больше, ни меньше, ни равен чему-либо — `partial_cmp` возвращает
`None`, поэтому `Vec<f64>::sort()` не существует (требует `Ord`), а
`sort_by(|a, b| a.partial_cmp(b).unwrap())` — пожалуйста (паника на NaN — ваша
ответственность). Отдельный `<=>` как в C++20 не нужен: `cmp` и есть three-way.

Идиома ручной реализации — определить `Ord`, остальное делегировать:

```rust
#[derive(PartialEq, Eq)]
struct Version { major: u32, minor: u32, patch: u32 }

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
        // кортежи сравниваются лексикографически — бесплатный «сначала major, потом minor...»
    }
}
impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
```

`derive(PartialOrd, Ord)` сравнивает поля в порядке объявления — если порядок полей
совпадает со смыслом сравнения, derive достаточно.

---

## 5. Печать: `Display` и `Debug`

```rust
use std::fmt;

struct Celsius(f64);

impl fmt::Display for Celsius {                       // {} — для пользователя
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}
```

- **`Display`** (`{}`) — представление для людей; derive **нет** — формат для
  пользователя всегда осмысленное решение. Бонус: `impl Display` бесплатно даёт
  `.to_string()` (через blanket impl `ToString`).
- **`Debug`** (`{:?}`) — представление для программиста; почти всегда
  `#[derive(Debug)]`. `{:#?}` печатает многострочно с отступами.
- Внутри `fmt` пишите в `f` макросом `write!`; `fmt::Result` — это
  `Result<(), fmt::Error>` (`?` работает).

Конвенция: публичный тип реализует `Debug` практически всегда, `Display` — если у
него есть каноничное текстовое представление.

---

## 6. Конверсии: `From`, `Into`, `TryFrom`

```rust
pub trait From<T>  { fn from(value: T) -> Self; }
pub trait Into<T>  { fn into(self) -> T; }

impl<T, U: From<T>> Into<U> for T { /* blanket impl в std */ }
```

Правило: **реализуйте `From` — `Into` получите бесплатно** (blanket impl из std;
обратное неверно). `From` обязан быть **без потерь и без отказов**:

```rust
struct Meters(f64);
struct Millimeters(f64);

impl From<Meters> for Millimeters {
    fn from(m: Meters) -> Self { Millimeters(m.0 * 1000.0) }
}

let mm: Millimeters = Meters(1.5).into();      // .into() выводит целевой тип
let mm2 = Millimeters::from(Meters(2.0));      // эквивалент
```

Конверсия, которая может **не получиться**, — это `TryFrom` с `Result`:

```rust
struct Age(u8);

impl TryFrom<i64> for Age {
    type Error = String;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        if (0..=130).contains(&v) {
            Ok(Age(v as u8))
        } else {
            Err(format!("возраст вне диапазона: {v}"))
        }
    }
}
```

Зачем это всё? Обобщённые API: `fn set_timeout(t: impl Into<Duration>)` принимает и
`Duration`, и секунды-`u64`, если есть `From`. А `?` в M16 будет автоматически
конвертировать ошибки через `From` — тот же механизм.

---

## 7. Индексация: `Index` и `IndexMut`

```rust
pub trait Index<Idx> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;   // возвращает ССЫЛКУ
}
```

`xs[i]` — сахар для `*xs.index(i)`. Индекс — любой тип, и разные `Idx` дают разные
`Output`: у срезов `xs[2]` — `&T`, а `xs[2..4]` — `&[T]` (два разных impl!). Свой
пример — матрица с индексом-кортежем:

```rust
impl std::ops::Index<(usize, usize)> for Grid {
    type Output = f64;
    fn index(&self, (r, c): (usize, usize)) -> &f64 {
        &self.cells[r * self.width + c]
    }
}
// grid[(1, 2)] — читаем; с IndexMut: grid[(1, 2)] = 5.0
```

По контракту `Index` паникует на неверном индексе (как `Vec`); «мягкий» доступ — это
отдельный метод `get(...) -> Option<&T>`.

---

## 8. Тизер к M14

Всюду в этом модуле компилятор знал конкретный тип на этапе компиляции —
мономорфизация из M11. Следующий модуль — **trait objects**: `Box<dyn Shape>`,
гетерогенные коллекции, vtable и динамическая диспетчеризация — стирание типов по
запросу, когда набор типов неизвестен до рантайма. И почему не каждый трейт можно
превратить в `dyn` (dyn compatibility).

---

## Упражнения

Решайте по порядку, ярусами. В каждом крейте: условие и `todo!()` в `src/lib.rs`,
тесты в `tests/tests.rs` (менять нельзя). Запуск: `cargo test -p ex-13-NN-...`.

**0x — синтаксис**
- `01_add_vec2` — `Add`/`Sub` для 2D-вектора.
- `02_display` — `Display` для температуры; `to_string` бесплатно.
- `03_from_into` — `From` между единицами измерения; `.into()`.

**1x — базовое**
- `04_partial_eq` — регистронезависимый `Username`: ручной `PartialEq` + `PartialEq<str>`.
- `05_version_ord` — `Ord` для семантической версии через кортежи; сортировка.
- `06_try_from` — `TryFrom<i64>` с валидацией диапазона.

**2x — продвинутое**
- `07_scalar_mul` — `Mul<f64>` для вектора и `Neg`; Rhs ≠ Self.
- `08_assign_ops` — `AddAssign`/`SubAssign` для счётчика ресурсов.
- `09_index_grid` — `Index<(usize, usize)>`/`IndexMut` для сетки.

**3x — хитрое**
- `10_money` — капстоун: `Money` с `Add`, `Display`, `TryFrom<&str>` ("12.50"), `Ord`; валюты не смешиваются (паника при разных валютах).
