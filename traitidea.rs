
struct Foo<T>{
    bar: T,
    lig: u8
}

trait Baz<T>{
    fn baz_doer(&self) {
        println!("I'm baz!")
    }
    
    fn go_getter(&self)->T;

}

trait Kaz<T>{
    fn go_getterz(&self){
        println!("idk")
    }
}

impl <f32: Baz, Kaz> Foo<f32>{
    fn go_getter(&self)->f32{
        self.bar
    }
}

impl Baz<u8> for Foo<u8>{
    fn go_getter(&self)->u8{
        self.bar
    }
}

fn main(){
    //Type u8
    let goo = Foo{bar:5, lig:9};
    goo.baz_doer();
    let zep: u8 = goo.bar;
    goo.go_getter();

    //Type float
    let fop = Foo{bar:6.6, lig:7};
    fop.go_getter();
}