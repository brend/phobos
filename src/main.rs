use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator4);

pub mod ast;

#[test]
fn calculator4() {
    let expr = calculator4::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}

fn main() {}
