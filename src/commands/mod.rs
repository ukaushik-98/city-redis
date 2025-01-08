#[derive(Debug)]
pub enum Commands {
    PING(Ping),
    GET(Get),
    SET(Set),
    PUBLISH(Publish),
    SUBSCRIBE(Subscribe),
}

#[derive(Debug)]
struct Ping {
    message: String,
}

#[derive(Debug)]
struct Set {}

#[derive(Debug)]
struct Get {}

#[derive(Debug)]
struct Publish {}

#[derive(Debug)]
struct Subscribe {}
