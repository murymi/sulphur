
mod node;
mod parser;
mod dom;

use std::str::FromStr;

use dom::Dom;
use node::*;
use parser::Parser;




fn main() {
    let html = r#"<a one="two">
    <one>one in a million</one>
    <one>one in a million</one>
    <one>one in a million</one>
    </a>"#;
    // <world one='two' three="four">
    //     <h1 five=six>
    //     <h1>
    //     <h1>
    //     <p>hello world ghasia</p>
    //     </h1>
    //     <h1></h1>
    //     <h1></h1>
    //     </h1>
    //     </h1>
    //     <footer/>
    // </world>"#;

    //println!("{html}");


    let dom = Dom::from_str(&html).unwrap();

    println!("{:#?}", dom.root().unwrap().borrow().to_string());
}
