use std::collections::HashMap;

fn main() {
    {
        let mut score_map = HashMap::new();
        let t_blue = String::from("blue");
        let t_red = String::from("red");
        score_map.insert(t_blue.clone(), 10);
        score_map.insert(t_red.clone(), 20);

        let blue_score = score_map.get(&t_blue).copied().unwrap_or(0);
        println!("blue score : {}", blue_score);

        for (k, v) in &score_map {
            println!("k: {}, v: {}", k, v);
        }

        let k = String::from("k");
        let v = String::from("v");
        let mut map2 = HashMap::new();
        map2.insert(k, v);
    }
    // println!("k: {}, v: {}", k, v);
    {
        let t_blue = String::from("blue");
        let mut map = HashMap::new();
        map.insert(t_blue.clone(), 10);
        map.insert(t_blue.clone(), 30);
        println!("blue score : {:?}", map.get(&t_blue).copied().unwrap_or(0));
    }

    {
        let t_blue = String::from("blue");
        let mut map = HashMap::new();
        map.entry(t_blue.clone()).or_insert(20);
        map.entry(t_blue.clone()).or_insert(50);

        println!("blue score : {:?}", map.get(&t_blue).copied().unwrap_or(0));
    }

    {
        let text = "aa bb cc dd aa bb aa aa";
        let mut map = HashMap::new();

        for word in text.split(' ') {
            let v = map.entry(word).or_insert(0);
            *v += 1;
        }

        println!("{:?}", map);
    }
}
