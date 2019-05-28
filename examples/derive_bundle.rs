extern crate shred;
#[macro_use]
extern crate shred_derive;

use shred::{Read, Resource, Resources, SystemData, Write};

#[derive(Debug, Default, Clone, Resource)]
struct ResA;

#[derive(Debug, Default, Clone, Resource)]
struct ResB;

#[derive(SystemData)]
struct AutoBundle<'a> {
    a: Read<'a, ResA>,
    b: Write<'a, ResB>,
}

// We can even nest system data
#[derive(SystemData)]
struct Nested<'a> {
    inner: AutoBundle<'a>,
}

fn main() {
    let mut res = Resources::new();
    res.insert(ResA);
    res.insert(ResB);

    {
        let mut bundle = AutoBundle::fetch(&res);

        *bundle.b = ResB;

        println!("{:?}", *bundle.a);
        println!("{:?}", *bundle.b);
    }

    {
        let nested = Nested::fetch(&res);

        println!("a: {:?}", *nested.inner.a);
    }
}
