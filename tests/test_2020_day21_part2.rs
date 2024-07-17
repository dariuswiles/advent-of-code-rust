mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day21_part2").trim(),
        "Ingredients with allergens in order required by challenge: rcqb,cltx,nrl,qjvvcvz,tsqpn,xhnk,tfqsb,zqzmzl"
    );
}
