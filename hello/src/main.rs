use hello::RealSum;
use hello::Summable;
use hello::sum_vec;

fn main() {
    println!("Hello rust");
    let a = 10;
    let b = 5;

    let rs = RealSum {};

    println!("sum: {} + {} = {}", a, b, rs.sum(a, b));

    println!("acc: {}", sum_vec(vec![1, 2, 3], &rs));
}
