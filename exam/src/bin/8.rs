pub trait Hello {
    fn world(&self);
}
struct World;
impl Hello for World {
    fn world(&self) {
        println!("World");
    }
}
struct IWorld;
impl Hello for IWorld {
    fn world(&self) {
        println!("IWorld");
    }
}

fn main() {
    let world = World;
    let iworld = IWorld;
    world.world();
    iworld.world();
}
