# Программа курса Rust

Порядок идиоматичный для Rust: владение вводится рано, как в rust-book.cs.brown.edu.
Пометка **<- готов** стоит у модулей, которые уже наполнены теорией и упражнениями.

Ярусы упражнений в каждом модуле: `0x` синтаксис | `1x` базовое | `2x` продвинутое | `3x` хитрое.

## Часть 0 - Инструментарий
- **M00** Тулчейн: rustup, cargo (`run/test/build/clippy/fmt`), doc-tests, miri, чтение ошибок компилятора и borrow-checker. **<- готов**

## Часть I - Основы языка
- **M01** Значения, `let`/`mut`, выражения, блоки, `if`/`loop`/`while`/`for`, тип `!`. **<- готов**
- **M02** Скалярные и составные типы: числа + переполнение, `f64`, `bool`, `char`, кортежи, массивы, срезы. **<- готов**
- **M03** Функции, ссылки `&`/`&mut` (введение), shadowing. **<- готов**
- **M04** struct + методы (`impl`, формы `self`, ассоциированные функции), tuple/newtype/unit-struct. **<- готов**
- **M05** enum + pattern matching: `match`, `if let`, `while let`, guards, связывание, `Option`/`Result` как ADT. **<- готов**

## Часть II - Владение (ядро)
- **M06** Ownership и move-семантика: `Copy`/`Clone`, `Drop`, дерево владения. **<- готов**
- **M07** Заимствование: `&`/`&mut`, shared-XOR-mutable, срезы как заём, freezing. **<- готов**
- **M08** Lifetimes I: элизия, именованные времена жизни в функциях и структурах. **<- готов**
- **M09** Lifetimes II: вариантность, reborrow, NLL, higher-ranked. **<- готов**

## Часть III - Проект
- **M10** Модули, пути, видимость, крейты, cargo-зависимости, workspace. **<- готов**

## Часть IV - Трейты и дженерики
- **M11** Дженерики: обобщённые функции/структуры/impl, мономорфизация. **<- готов**
- **M12** Трейты: определение, bounds, `where`, default-методы, ассоц. константы; orphan rule; замена наследования. **<- готов**
- **M13** Трейты-операторы и конверсии: `From`/`Into`/`TryFrom`, `Add`/`Mul`, `Display`/`Debug`, `PartialEq`/`Ord`, `Index`. **<- готов**
- **M14** Trait objects: `dyn`, динамическая диспетчеризация, dyn compatibility (object safety), `dyn Any` / type erasure. **<- готов**
- **M15** Продвинутые трейты: ассоц. типы, GAT, супертрейты, blanket impls, marker-трейты, `impl Trait`. **<- готов**

## Часть V - Ошибки
- **M16** `Option`/`Result` глубоко, `?`, свои типы ошибок, `From`-конверсии, `panic!`, паника и `Drop`/RAII, `thiserror`/`anyhow`. **<- готов**

## Часть VI - Коллекции, замыкания, итераторы
- **M17** Коллекции: `Vec`, `VecDeque`, `HashMap`, `BTreeMap`, `HashSet`, `String`/`&str`, `Cow`. **<- готов**
- **M18** Замыкания: `Fn`/`FnMut`/`FnOnce`, захваты, `move`. **<- готов**
- **M19** Итераторы: трейт `Iterator`, адаптеры, ленивость, свой итератор, `IntoIterator`. **<- готов**

## Часть VII - Умные указатели
- **M20** `Box`/`Rc`/`Arc`, `Deref`/`Drop`, рекурсивные типы. **<- готов**
- **M21** Interior mutability: `Cell`/`RefCell`, `Rc<RefCell>`, `Weak`, разрыв циклов. **<- готов**

## Часть VIII - Многопоточность
- **M22** Потоки, `Send`/`Sync`, move-замыкания, scoped threads. **<- готов**
- **M23** Разделяемое состояние: `Mutex`/`RwLock`, `Arc<Mutex>`, atomics, `Condvar`, poisoning. **<- готов**
- **M24** Каналы (`mpsc`) и сеть: `TcpListener`/`TcpStream`. **<- готов**
- **M25** async/await: `Future`, `Pin`/`Unpin`, `.await`, executors, async I/O. **<- готов**

## Часть IX - Небезопасность, память, FFI
- **M26** `unsafe`: сырые указатели, `unsafe fn`/блок, инварианты, `MaybeUninit`, `ManuallyDrop`. **<- готов**
- **M27** Представление в памяти: `repr(C)`/`packed`/`transparent`, size/align, байтовый I/O, endianness, `bytemuck`. **<- готов**
- **M28** Свои коллекции на unsafe: `NonNull`, свой `Vec`/`Box`, аллокаторы (`GlobalAlloc`). **<- готов**
- **M29** FFI: `extern "C"`, линковка C, `bindgen`, union, вызов Rust из C.

## Часть X - Метапрограммирование
- **M30** Декларативные макросы: `macro_rules!`, повторения, гигиена, fragment specifiers.
- **M31** Процедурные макросы: derive, атрибутные, функциональные.

## Часть XI - Продвинутые идиомы
- **M32** Type-state, phantom types, sealed traits, builder, DSL.
- **M33** Const generics, `const fn`, трюки с ZST.
