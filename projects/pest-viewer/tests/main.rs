use pest_viewer::SvgPlotter;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_calculator() {
    let plotter = SvgPlotter::default();

    let boot = pest_meta::parse_and_optimize(include_str!("grammar.pest")).unwrap();
    let vm = pest_vm::Vm::new(boot.1);
    let cst = vm.parse("grammar_rules", include_str!("calculator.pest")).unwrap();
    println!("{:#?}", cst);
    let tree = plotter.draw(cst);
    svg::save("tests/calculator.svg", &tree).unwrap();
}

#[test]
fn test_json() {
    let plotter = SvgPlotter::default();

    let boot = pest_meta::parse_and_optimize(include_str!("json.pest")).unwrap();
    let vm = pest_vm::Vm::new(boot.1);
    let cst = vm.parse("json", include_str!("example.json")).unwrap();
    println!("{:#?}", cst);
    let tree = plotter.draw(cst);
    svg::save("tests/json.svg", &tree).unwrap();
}
