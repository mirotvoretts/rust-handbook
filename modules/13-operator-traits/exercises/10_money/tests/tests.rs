use ex_13_10_money::{Currency, Money};

fn rub(kopecks: i64) -> Money {
    Money { kopecks, currency: Currency::Rub }
}
fn usd(kopecks: i64) -> Money {
    Money { kopecks, currency: Currency::Usd }
}

#[test]
fn addition_same_currency() {
    assert_eq!(rub(1050) + rub(200), rub(1250));
}

#[test]
#[should_panic]
fn addition_mixed_currencies_panics() {
    let _ = rub(100) + usd(100);
}

#[test]
fn display_format() {
    assert_eq!(rub(1250).to_string(), "12.50 RUB");
    assert_eq!(usd(5).to_string(), "0.05 USD");
    assert_eq!(rub(100).to_string(), "1.00 RUB");
}

#[test]
fn parse_with_kopecks() {
    assert_eq!(Money::try_from("12.50 RUB"), Ok(rub(1250)));
    assert_eq!(Money::try_from("0.05 USD"), Ok(usd(5)));
}

#[test]
fn parse_whole_amount() {
    assert_eq!(Money::try_from("7 USD"), Ok(usd(700)));
}

#[test]
fn parse_errors() {
    assert!(Money::try_from("12.5 RUB").is_err());   // одна цифра копеек
    assert!(Money::try_from("12.50 EUR").is_err());  // неизвестная валюта
    assert!(Money::try_from("abc RUB").is_err());
    assert!(Money::try_from("12.50").is_err());      // нет валюты
}

#[test]
fn roundtrip() {
    let m = Money::try_from("99.99 RUB").unwrap();
    assert_eq!(m.to_string(), "99.99 RUB");
}
