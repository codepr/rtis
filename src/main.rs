mod server;
mod indexer;

fn main() {
    let indexer = indexer::Indexer::new();
    let mut server = server::Server::new((String::from("127.0.0.1"), 9766), indexer);
    println!("Listening on {}:{}", "127.0.0.1", 9766);
    server.serve();
}
