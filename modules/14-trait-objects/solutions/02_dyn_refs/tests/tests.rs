use sol_14_02_dyn_refs::{shout_all, Loud, Siren, Speaker};

#[test]
fn individual_shouts() {
    assert_eq!(Siren.shout(), "WEE-OO");
    assert_eq!(Speaker { volume: 2 }.shout(), "BOOMBOOM");
}

#[test]
fn mixed_refs_no_boxes() {
    let siren = Siren;
    let speaker = Speaker { volume: 1 };
    let all: Vec<&dyn Loud> = vec![&siren, &speaker, &siren];
    assert_eq!(shout_all(&all), "WEE-OO BOOM WEE-OO");
}

#[test]
fn empty_is_empty() {
    assert_eq!(shout_all(&[]), "");
}
