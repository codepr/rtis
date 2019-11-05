mod server;
mod indexer;

fn main() {
    let mut indexer = indexer::Indexer::new();
    indexer.add(String::from("Hello world"));
    let mut server = server::Server::new((String::from("127.0.0.1"), 9766), indexer);
    server.start();
}
