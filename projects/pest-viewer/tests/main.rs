use pest_viewer::{create_parser, SvgPlotter};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_calculator() {
    let plotter = SvgPlotter::default();
    let parser = create_parser(include_str!("grammar.pest")).unwrap();
    let tree = parser.parse("grammar_rules", include_str!("calculator.pest")).unwrap();
    let svg = plotter.draw(tree);
    svg::save("tests/calculator.svg", &svg).unwrap();
}

#[test]
fn test_json() {
    let plotter = SvgPlotter::default();
    let parser = create_parser(include_str!("json.pest")).unwrap();
    let tree = parser.parse("json", include_str!("example.json")).unwrap();
    let svg = plotter.draw(tree);
    svg::save("tests/json.svg", &svg).unwrap();
}
