use xml_parser::xml::trees::generators::xml_generator::XmlGenerator;
fn main() {
    let data = r#"
        <div id="name" class="style style2">
            hello world
        </div>
    "#;
    let tree = XmlGenerator::gen(data);
    println!("{:?}", tree.text_contents().unwrap());
}
