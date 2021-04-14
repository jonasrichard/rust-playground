use cucumber;
use async_trait::async_trait;
use std::convert::Infallible;

pub struct State {
    conn: String
}

impl State {
    async fn test_fn(&mut self) {
        self.conn = "test".to_string();
    }
}

#[async_trait(?Send)]
impl cucumber::World for State {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(State {
            conn: "test".to_string()
        })
    }
}

mod connect {
    use cucumber::Steps;

    pub fn steps() -> Steps<super::State> {
        let mut builder: Steps<super::State> = Steps::new();

        builder
            .given_async(
                "A connectionion",
                cucumber::t!(|mut world, _step| {
                    world.test_fn().await;
                    println!("The conn is {}", world.conn);
                    world
                })
            )
            .when("it connects", |world, _step| {
                world
            })
            .then("I am ok", |world, _| {
                world
            });

        builder
    }
}

fn main() {
    let runner = cucumber::Cucumber::<State>::new()
        .features(&["./features"])
        .steps(connect::steps());

    futures::executor::block_on(runner.run());
}
