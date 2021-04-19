use cgmath::*;
use kudo::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        for _ in 0..10_000 {
            world.spawn((
                Transform(Matrix4::from_scale(1.0)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            ));
        }

        Self(world)
    }

    pub fn run(&mut self) {
        let mut query = self
            .0
            .query::<(&Velocity, &mut Position, &Rotation)>()
            .unwrap();
        for (velocity, position, _rotation) in query.iter_mut() {
            position.0 += velocity.0;
        }
    }
}
