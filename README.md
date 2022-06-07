# XML-PARSER

## How use

```rust
    let source = r#"<data>DATA</data>"#;
    let xml = XmlTree::from(source);
    assert_eq!(xml.value(),"data");
    assert_eq!(xml.text_content(),"DATA");
    assert_eq!(xml.to_str(),source);
```
