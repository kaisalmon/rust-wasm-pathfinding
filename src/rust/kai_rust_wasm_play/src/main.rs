mod lib;

fn main() {
    println!("{}", lib::dijkstra(0, 0, 4, 4, &[
        1, 1, 1, 1,
        1, 1, 0, 1,
        1, 1, 0, 1,
        1, 1, 1, 1
    ]));
}
