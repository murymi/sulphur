
mod node;
mod parser;
mod dom;

use std::{ cell::RefCell, ops::Deref, rc::Rc, str::FromStr};

use dom::Dom;
use node::*;
use parser::Parser;




fn main() {
    let html = r#"<a one="two">
    <one pos="two">one in a trillion</one>
    <one pos="two">one in a million</one>
    <one pos=three>one in a million</one>
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

    let root = dom.root().unwrap();

    //let root  = root.deref().borrow();
    //let childs = root.children();
    //childs.borrow_mut().push(Rc::new(RefCell::new(Node::new("hello".into()))));

    Node::append_element(&root, Node::new("hello".into())).unwrap();

    println!("{:#?}", root.borrow().to_string());
}
