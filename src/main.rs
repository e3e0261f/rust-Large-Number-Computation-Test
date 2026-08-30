use rug::Integer;
use rug::ops::Pow;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    // 2^1000000 计算
    let base = Integer::from(2);
    let result = base.pow(100000000);  // 直接 pow，支持任意大指数

    // 如果想打印完整结果（30 万位），取消下面注释
    // println!("{}", result);

    let duration = start.elapsed();
    println!("计算完成，耗时: {:.3?}", duration);

    // 可选：打印位数验证
    println!("结果位数: {} 位", result.to_string().len());
}
