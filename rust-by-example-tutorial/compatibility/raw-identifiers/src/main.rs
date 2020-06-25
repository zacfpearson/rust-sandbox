mod foo {

    pub fn r#try() {
        println!("try keyword override!")
    }

}

fn main() {
    foo::r#try();
}