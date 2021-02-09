use actix::{Actor, Addr, Context, Handler, Message};

#[derive(Debug)]
struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<Msg> for MyActor {
    type Result = ();

    fn handle(&mut self, _message: Msg, _ctx: &mut Self::Context) {
        println!("Message handled");
    }
}

// Server has some clients connected to it
#[derive(Debug)]
struct Server {
    clients: Vec<Addr<MyActor>>,
    called: bool,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
struct Msg(pub String);

impl Server {
    fn new() -> Self {
        Server {
            clients: vec![],
            called: false,
        }
    }

    // call sends message to connected clients and change inner variable
    fn call(&mut self) {
        for (i, c) in self.clients.iter().enumerate() {
            c.do_send(Msg(format!("hello, client {}", i)));
        }

        self.called = true;
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    use actix::dev::channel;

    // Test server is server with two channels as clients
    fn test_server() -> Server {
        // First addres
        let channel: (
            channel::AddressSender<MyActor>,
            channel::AddressReceiver<MyActor>,
        ) = channel::channel(1024);
        let addr0 = Addr::new(channel.0);

        // Second address
        let channel: (
            channel::AddressSender<MyActor>,
            channel::AddressReceiver<MyActor>,
        ) = channel::channel(1024);
        let addr1 = Addr::new(channel.0);

        let mut server = Server::new();

        server.clients.push(addr0);
        server.clients.push(addr1);

        server
    }

    #[test]
    fn test() {
        let mut server = test_server();

        assert_eq!(server.called, false);
        server.call();
        assert_eq!(server.called, true);
    }
}
