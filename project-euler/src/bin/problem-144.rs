//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20144
//! Investigating multiple reflections of a laser beam

type Point = (f64, f64);

fn main() {
    let mut start_point = (0.0, 10.1);
    let mut end_point = (1.4, -9.6);

    let mut count = 1; // (1.4, -9.6)の1回分

    for _ in 0..1000 {
        let expr = line_expression(start_point, end_point);
        let next_point = reflection_line_cross_other_point(&expr, end_point);
        // println!("{:?}", next_point);

        if next_point.0.abs() < 0.01 && next_point.1 > 0.0 {
            break;
        }

        count += 1;

        start_point = end_point;
        end_point = next_point;
    }

    println!("{}", count);
}

/// (x1, y1), (x2, y2)を通る直線の式
fn line_expression((x1, y1): Point, (x2, y2): Point) -> impl Fn(f64) -> f64 {
    move |x| (y2 - y1) / (x2 - x1) * (x - x1) + y1
}

/// 楕円Eの(x0, y0)による接線の傾きで(x1, y1)を通る直線の式
fn tangent_expression((x0, y0): Point, (x1, y1): Point) -> impl Fn(f64) -> f64 {
    move |x| -4. * x0 / y0 * (x - x1) + y1
}

/// 楕円Eの(x0, y0)を通る法線の式
fn normal_expression((x0, y0): Point) -> impl Fn(f64) -> f64 {
    move |x| y0 / (4. * x0) * (x - x0) + y0
}

/// 楕円Eの(x0, y0)を通る法線と、(x1, y1)を通り傾きが接線に等しい直線との交点
fn cross_point((x0, y0): Point, (x1, y1): Point) -> Point {
    let m0 = y0 / (4. * x0); // 法線の傾き
    let m1 = -4. * x0 / y0; // 接線の傾き

    let x = (m0 * x0 - y0 - m1 * x1 + y1) / (m0 - m1);
    let y = normal_expression((x0, y0))(x);

    (x, y)
}

/// 「楕円Eの(x0, y0)による接線の傾きで(x1, y1)を通る直線」と「(x1, y1)を通る直線が楕円Eの(x0, y0)で反射した直線」との交点
fn reflection_cross_point((x0, y0): Point, (x1, y1): Point) -> Point {
    let tan_expr = tangent_expression((x0, y0), (x1, y1));
    let (x2, _) = cross_point((x0, y0), (x1, y1));

    let d = (x2 - x1).abs();
    let x = if x2 < 0. { x2 - d } else { x2 + d };

    (x, tan_expr(x))
}

/// 直線lが(x0, y0)で反射したときの直線の式
fn reflection_line_expression(
    l: &impl Fn(f64) -> f64,
    (x0, y0): (f64, f64),
) -> impl Fn(f64) -> f64 {
    let x1 = 0.0;
    let y1 = l(x1);
    let (x2, y2) = reflection_cross_point((x0, y0), (x1, y1));

    line_expression((x0, y0), (x2, y2))
}

/// 直線lが(x0, y0)で反射したときに楕円Eと交差するもうひとつの点
fn reflection_line_cross_other_point(l: &impl Fn(f64) -> f64, (x0, y0): (f64, f64)) -> Point {
    let x1 = 0.0;
    let y1 = l(x1);
    let (x2, y2) = reflection_cross_point((x0, y0), (x1, y1));

    let m = (y2 - y0) / (x2 - x0);
    let a = 4. + m * m;
    let b = -2. * m * (m * x0 - y0);
    let c = m * m * x0 * x0 - 2. * m * x0 * y0 + y0 * y0 - 100.;

    let d = (b * b - 4. * a * c).sqrt();

    let reflec_line = reflection_line_expression(l, (x0, y0));

    let x3 = (-b + d) / (2. * a);
    // near equal
    if (x0 - x3).abs() < 1e-10 {
        let x3 = (-b - d) / (2. * a);
        (x3, reflec_line(x3))
    } else {
        (x3, reflec_line(x3))
    }
}
