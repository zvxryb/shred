extern crate shred;
#[macro_use]
extern crate shred_derive;

use shred_derive::Resource;
use shred::{DispatcherBuilder, Read, Resource, Resources, System, Write};

#[derive(Debug, Default, Clone, Resource)]
struct ResA;

#[derive(Debug, Default, Clone, Resource)]
struct ResB;

#[derive(SystemData)]
struct Data<'a> {
    a: Read<'a, ResA>,
    b: Write<'a, ResB>,
}

struct EmptySystem;

impl<'a> System<'a> for EmptySystem {
    type SystemData = Data<'a>;

    fn run(&mut self, bundle: Data<'a>) {
        println!("{:?}", &*bundle.a);
        println!("{:?}", &*bundle.b);
    }
}

fn main() {
    let mut resources = Resources::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(EmptySystem, "empty", &[])
        .build();
    dispatcher.setup(&mut resources);

    dispatcher.dispatch_seq(&resources);
}
