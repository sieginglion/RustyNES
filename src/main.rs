fn main() {
    let timer = std::time::Instant::now();
    for i in 0..1000 {
        let a = [0, 1, 2];
        let b = [3, 4, 5];
        let c = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        // let mut c = 0;
        // for (x, y) in a.iter().zip(b.iter()) {
        //     c += x * y
        // }
    }
    println!("{}", timer.elapsed().as_nanos())
}
