extern crate shred;
#[macro_use]
extern crate shred_derive;

use shred::{Read, ResourceId, Resources, Resource, SystemData, Write};

#[derive(Debug, Default, Clone, Resource)]
struct ResA;

#[derive(Debug, Default, Clone, Resource)]
struct ResB;

struct ExampleBundle<'a> {
    a: Read<'a, ResA>,
    b: Write<'a, ResB>,
}

impl<'a> SystemData<'a> for ExampleBundle<'a> {
    fn setup(res: &mut Resources) {
        res.entry().or_insert(ResA);
        res.entry().or_insert(ResB);
    }

    fn fetch(res: &'a Resources) -> Self {
        ExampleBundle {
            a: SystemData::fetch(res),
            b: SystemData::fetch(res),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<ResA>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<ResB>()]
    }
}

fn main() {
    let mut res = Resources::new();
    res.insert(ResA);
    res.insert(ResB);

    let mut bundle = ExampleBundle::fetch(&res);
    *bundle.b = ResB;

    println!("{:?}", *bundle.a);
    println!("{:?}", *bundle.b);
}
