// Written by Steven Sheffey as an example to help me figure out this trait thing

trait Visit {
    fn visit(&self) -> i32;
}

struct BinOp {
    left: Box<Visit>,
    op: String,
    right: Box<Visit>,
}

#[derive(Clone)]
struct Num {
    value: i32,
}


impl Visit for BinOp {
    fn visit(&self) -> i32 {
        match self.op.as_str() {
            "+" => self.left.visit() + self.right.visit(),
            "-" => self.left.visit() - self.right.visit(),
            _ => unimplemented!(),
        }
    }
}
impl Visit for Num {
    fn visit(&self) -> i32 {
        self.value
    }
}

fn main() {
    let two = Num{value: 2};
    let one = Num{value: 1};
    // Two plus two is four
    let expr_1 = BinOp{left: Box::new(two.clone()), op: "+".into(), right: Box::new(two.clone())};
    // minus one that's three
    let expr_2 = BinOp{left: Box::new(expr_1), op: "-".into(), right: Box::new(one)};
    // Quick maths
    println!("{}", expr_2.visit())
}