use kudo::*;

macro_rules! create_entities {
    ($world:ident; $( $variants:ident ),*) => {
        $(
            struct $variants(f32);
            for _ in 0..20 {
                $world.spawn(($variants(0.0), Data(1.0)));
            }
        )*
    };
}

struct Data(f32);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(world)
    }

    pub fn run(&mut self) {
        let mut query = self.0.query::<(&mut Data,)>().unwrap();
        for mut data in &mut query {
            data.0 *= 2.0;
        }
    }
}
