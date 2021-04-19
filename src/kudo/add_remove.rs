use kudo::*;

struct A(f32);
struct B(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut entities = Vec::new();
        for _ in 0..10_000 {
            entities.push(world.spawn((A(0.0),)))
        }

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for entity in &self.1 {
            self.0.add_component(*entity, B(0.0)).unwrap();
        }

        for entity in &self.1 {
            self.0.remove_component::<B>(*entity).unwrap();
        }
    }
}
