struct Cipher { map: Vec<(char, char)> }

impl Cipher {
    fn new(map1: &str, map2: &str) -> Cipher { Cipher { map: map1.chars().zip(map2.chars()).collect() } }

    fn encode(&self, string: &str) -> String { string.chars().map(|c| self.map.iter().find(|x| x.0 == c).map_or(c, |y| y.1)).collect() }

    fn decode(&self, string: &str) -> String { string.chars().map(|c| self.map.iter().find(|x| x.1 == c).map_or(c, |y| y.0)).collect() }
}
