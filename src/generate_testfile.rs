use rand::Rng;
pub fn make_testfile() {
    let how_many = 1;
    let mut rng = rand::thread_rng();
    let _d = 1;
    for _ in 0..how_many {
        let a = rng.gen_range(i32::MIN..=i32::MAX);
        print!("{} ", a);
    }
}
