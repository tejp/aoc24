use std::collections::{BTreeSet, HashMap};

type Set<'a> = BTreeSet<&'a str>;

fn main() {
    let input: Vec<String> = aoc24::input(23)
        .split(|c: char| !c.is_alphabetic())
        .map(str::to_string)
        .collect();

    let mut neighbours: HashMap<&str, Set> = HashMap::new();

    for e in input.chunks_exact(2) {
        neighbours.entry(&e[0]).or_default().insert(e[1].as_str());
        neighbours.entry(&e[1]).or_default().insert(e[0].as_str());
    }

    let mut part1 = 0;

    let mut checked = Set::new();
    for (&e1, v) in &neighbours {
        let mut v = v - &checked;
        checked.insert(e1);
        while let Some(e2) = v.pop_first() {
            let e2_n = neighbours.get(e2).unwrap();
            for e3 in v.iter().filter(|&e3| e2_n.contains(e3)) {
                if e1.starts_with("t") || e2.starts_with("t") || e3.starts_with("t") {
                    part1 += 1;
                }
            }
        }
    }

    println!("{}", part1);

    let max_clique = bron_kerbosch_2(
        &mut Set::new(),
        neighbours.keys().copied().collect(),
        Set::new(),
        &neighbours,
    );

    let part2 = Vec::from_iter(max_clique).join(",");

    println!("{}", part2);
}

fn bron_kerbosch_2<'a>(
    r: &mut Set<'a>,
    mut p: Set<'a>,
    mut x: Set<'a>,
    n: &HashMap<&str, Set<'a>>,
) -> Set<'a> {
    // algorithm BronKerbosch2(R, P, X) is
    // if P and X are both empty then
    //     report R as a maximal clique
    // choose a pivot vertex u in P ⋃ X
    // for each vertex v in P \ N(u) do
    //     BronKerbosch2(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
    //     P := P \ {v}
    //     X := X ⋃ {v}

    let Some(u) = p.union(&x).max_by_key(|&v| n.get(v).unwrap().len()) else {
        return r.clone();
    };

    let mut max_clique = Set::new();

    for v in &p - n.get(u).unwrap() {
        let nv = n.get(v).unwrap();
        r.insert(v);
        let p_ = p.intersection(nv).copied().collect();
        let x_ = x.intersection(nv).copied().collect();
        let clique = bron_kerbosch_2(r, p_, x_, n);
        if clique.len() > max_clique.len() {
            max_clique = clique;
        }
        r.remove(v);
        p.remove(v);
        x.insert(v);
    }
    max_clique
}
