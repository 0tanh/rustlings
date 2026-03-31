#[derive(Debug)]
enum Idea {
    Number(i32)
}


impl Idea {
    fn answer(&mut self){
        match self {
            Number => *Number += 42,
        }
    }
}

struct Plane {
    Engine: String,
    Wings: String,
    Fuselage: String,
    Passengers: u8,
}

//
impl Plane {
    fn fly(self){
        println!("I'm flying!")
    }
}


fn main(){
    let mut x:Idea = Idea::Number(5); 
    x.answer();
    dbg!(&x);
    println!("{:?}",&x);
    
    let myPlane: Plane = Plane{
                Engine:String::from("big one"), 
                Wings:String::from("cool Biplane ones"), 
                Fuselage:String::from("Gaping"),
                Passengers: 5};
    
    myPlane.fly();
}



