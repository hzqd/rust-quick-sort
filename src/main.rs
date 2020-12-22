use rayon::prelude::*;
use my_ext::kt_ext::*;
use rand::{thread_rng, Rng};

fn gen_rand_vec(n: u32) -> Vec<i128> {
    let mut vec = vec![];
    (0..n).for_each(|_| vec.push(thread_rng().gen_range(i128::MIN, i128::MAX)));
    vec
}

fn main() {
    // Vec:
    let v = gen_rand_vec(u32::MAX / 100);
    
    // Std自带排序
    let mut t = v.clone();
    let now = std::time::Instant::now();
        t.sort();
    now.elapsed().echo();

    // 快速排序
    let t = v.clone();
    let now = std::time::Instant::now();
        t.quick_sort();
    now.elapsed().echo();

    // 并行快排
    let t = v.clone();
    let now = std::time::Instant::now();
        t.qsort_p();
    now.elapsed().echo();

    // Rayon自带并行
    let mut t = v.clone();
    let now = std::time::Instant::now();
        t.par_sort();
    now.elapsed().echo();
}
