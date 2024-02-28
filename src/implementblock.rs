
pub fn starting_point(){
   
         let mut f = Foo::new();
         println!("{}", f.borrow());
         *f.borrow_mut() = 10;
         let g = f.consume();
         println!("{}", g.borrow());
       
}
struct Foo(i32);
impl Foo {
fn consume(self) -> Self {
Self(self.0 + 1)
}

fn borrow(&self) -> &i32 {
&self.0
}

fn borrow_mut(&mut self) -> &mut i32 {
 &mut self.0
 }

fn new() -> Self {
Self(0)
}
}