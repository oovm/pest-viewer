use pest_viewer::SvgPlotter;


#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_classes() {
    let plotter = SvgPlotter::default();
    let text = r##"
class ClassStatement {
    DecoratorCall* ModifierCall* ^KW_CLASS (name:Identifier)
}
class ClassBlock {
    '{' '|'? Expression '}'
}
token {
    OP_REMARK: '^'
}
"##;
    let cst = pest_vm::Vm::parse(text, BootstrapRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let tree = plotter.draw(cst);
    svg::save("tests/bootstrap.svg", &tree).unwrap();
}

// fn main() {
//     let root = tree();
//     let layout = layout_position(&Tree, &root);
//     println!("{:?}", layout)
// }
