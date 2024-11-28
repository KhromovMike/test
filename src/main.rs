use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    let mut vec_i32: Vec<i32> = Vec::new();
    let mut vec_i32_2: Vec<i32> = Vec::new();

    // 200_000_000 iterations !!!!!!!!1
    // for i in 0..100_000_000 {
    //     let r = i * 2;
    //     vec_i32.push(r);
    // }
    //
    // for i in 0..100_000_000 {
    //     let r = i + 10;
    //     vec_i32_2.push(r);
    // }

    // 100_000_000 iterations !!!!!!!!
    for i in 0..100_000_000 {
        let r = i * 2;

        let o = i + 10;

        vec_i32.push(r);
        vec_i32_2.push(o);
    }
    println!("{:?}", now.elapsed());
}
