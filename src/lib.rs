use elfo::prelude::*;

pub fn new() -> Schema {
    ActorGroup::new().exec(|_| async {
        for _ in Y::default().iter() {
            async {}.await
        }
    })
}

#[derive(Default)]
pub struct Y {
    s: std::collections::HashMap<(), ()>,
}

impl Y {
    pub fn iter(&mut self) -> impl Iterator<Item = u64> + '_ {
        let x: u32 = todo!();

        std::iter::from_fn(move || {
            &self.s;
            None
        })
    }
}

