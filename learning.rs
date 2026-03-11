#[derive(Debug)]
enum Idea {
    Number(i32)
}

impl Idea {
    fn answer(&mut self){
        match self {
            Number => Number += 42,
        }
    }
}

fn main(){
    let mut x:Idea = Idea::Number(5); 
    x.answer();
    dbg!(&x);
    println!("{:?}",&x)
}