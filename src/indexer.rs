use std::collections::HashMap;

pub struct Indexer {
    documents: Vec<(String, HashMap<String, u32>)>
}

impl Indexer {

    pub fn new() -> Indexer { Indexer { documents: Vec::new() } }

    pub fn add(&mut self, document: String) {
        let freq = term_frequency(&document);
        self.documents.push((document, freq));
    }

    pub fn search(&self, text: String) -> Option<Vec<(f64, &String)>> {
        let frequency: HashMap<String, u32> = term_frequency(&text);
        let mut results = Vec::new();
        for (word, freq) in self.documents.iter() {
            let relation = relation(&frequency, &freq);
            if relation != 0.0 {
                results.push((relation, word));
            }
        }
        if results.is_empty() { None } else { Some(results) }
    }
}

fn term_frequency(text: &String) -> HashMap<String, u32> {
    let words: Vec<&str> = text.split(" ").collect();
    let mut freq: HashMap<String, u32> = HashMap::new();
    for word in &words {
        *freq.entry(String::from(*word)).or_insert(0) += 1
    }
    return freq;
}

fn magnitude(freq: &HashMap<String, u32>) -> f64 {
        let total: u32 = freq.values().map(|x| x.pow(2)).sum();
	return (total as f64).sqrt()
}

fn relation(freq1: &HashMap<String, u32>, freq2: &HashMap<String, u32>) -> f64 {
    let mut topval: f64 = 0.0;
    for (word, count) in freq1 {
        if freq2.contains_key(word) {
            topval += (count * freq2[word]) as f64;
        }
    }
    let relevance = magnitude(freq1) * magnitude(freq2);
    if relevance != 0.0 {
        return topval / relevance
    }
    return 0.0
}
