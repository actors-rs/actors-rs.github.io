extern crate actors;
use actors_rs::*;
use std::time::Duration;

#[derive(Default)]
struct MyActor;

// implement the Actor trait
impl Actor for MyActor {
    type Msg = String;

    fn recv(&mut self, _ctx: &Context<Self::Msg>, msg: Self::Msg, _sender: Sender) {
        println!("Received: {}", msg);
    }
}

// start the system and create an actor
fn main() {
    let sys = ActorSystem::new().unwrap();

    let my_actor = sys.actor_of::<MyActor>("my-actor").unwrap();

    my_actor.tell("Hello my actor!".to_string(), None);

    std::thread::sleep(Duration::from_millis(500));
}
