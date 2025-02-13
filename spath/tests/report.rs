use spath::SPath;

fn parse_spath(spath: &str) -> Result<(), spath::ParseError> {
    let registry = spath::json::BuiltinFunctionRegistry::default();
    let _spath = SPath::parse_with_registry(spath, registry)?;
    Ok(())
}

#[test]
fn test_malformed() {
    println!("{}", parse_spath(r#"$.a[?@ >< 2]"#).unwrap_err());
}
