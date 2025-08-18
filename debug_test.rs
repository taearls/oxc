use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;

fn main() {
    let allocator = Allocator::default();
    let source_text = r#"
class C {
  @(decorators[0])
  method() {}
}
"#;
    let source_type = SourceType::js();
    let ret = Parser::new(&allocator, source_text, source_type).parse();
    
    if let Ok(ret) = ret {
        println!("{:#?}", ret.program.body[0]);
    }
}
