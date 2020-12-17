//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20150
//! Searching a triangular array for a sub-triangle having minimum-sum

fn main() {
    let bound = 1000;
    let modulo = 2_i128.pow(20);
    let d = modulo / 2;

    let ns: Vec<_> = (0..bound * (bound + 1) / 2)
        .scan(0, |t, _| {
            *t = (615949 * *t + 797807) % modulo;
            Some(*t - d)
        })
        .collect();

    let sum = ns.iter().sum::<i128>();
    let mut min = sum;

    for top in 0..50 {
        let del_top = (0..top)
            .map(|i| (i..bound).map(|j| ns[j * (j + 1) / 2 + i]).sum::<i128>())
            .sum::<i128>();

        for bottom in 850..bound {
            let del_bottom = (bottom..bound)
                .map(|i| {
                    (i * (i + 1) / 2 + top..(i + 1) * (i + 2) / 2)
                        .map(|j| ns[j])
                        .sum::<i128>()
                })
                .sum::<i128>();

            for right in 1..50 {
                let del_right = (1..right)
                    .map(|i| {
                        (top + i..bottom + 1)
                            .map(|j| ns[j * (j + 1) / 2 - i])
                            .sum::<i128>()
                    })
                    .sum::<i128>();

                min = min.min(sum - del_top - del_bottom - del_right);
            }
        }
    }

    println!("{}", min);
}
