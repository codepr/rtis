mod indexer;

fn main() {
    let mut indexer = indexer::Indexer::new();
    indexer.add(String::from("Hello world"));
    indexer.search("world");
}
