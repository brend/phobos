use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator1);

#[test]
fn calculator1test() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

fn main() {}
