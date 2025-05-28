mod dancing_links;

use dancing_links::DancingLinks;

fn main() {
    let mut dl = DancingLinks::from_vecs(vec![vec![1, 0, 0], vec![0, 1, 1], vec![1, 1, 0]]);
    let solutions = dl.solve();
    println!("Solutions: {:?}", solutions);
}
