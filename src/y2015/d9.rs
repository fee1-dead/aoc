use crate::*;

fn distance_permutations(s: String) -> Result<impl Iterator<Item = usize>> {
    let mut all_cities = HashSet::default();
    let mut map = HashMap::default();
    for l in s.lines().map(str::trim) {
        let a: String;
        let b: String;
        let distance: usize;

        scanfmt!(l, "{} to {} = {}", a, b, distance);

        map.insert([a.clone(), b.clone()], distance);
        map.insert([b.clone(), a.clone()], distance);

        all_cities.insert(a);
        all_cities.insert(b);
    }

    let n_cities = all_cities.len();

    all_cities
        .into_iter()
        .permutations(n_cities)
        .map(move |cities| cities.windows(2).map(|v| map[v]).sum())
        .pipe(Ok)
}

pub fn part1(s: String) -> Result<()> {
    let min_distance = distance_permutations(s)?.min();

    dbg!(min_distance);

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let max_distance = distance_permutations(s)?.max();

    dbg!(max_distance);

    Ok(())
}
