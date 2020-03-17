// Solves a quadratic.
// Writes the roots to `roots`, in order from least to greatest.
// Returns true if the roots are real.
// Algorithm derived from "Numerical Recipes In C", chapter 5.5
fn quadratic(b: f32, c: f32, roots: &mut [f32; 2]) -> bool {
    let disc: f32 = b * b - 4. * c;
    if disc < 0.0 {
        return false;
    }

    let x1: f32 = -(b + b.signum() * disc.sqrt()) / 2.0;
    let x2: f32 = c / x1;

    roots[0] = x1.min(x2);
    roots[1] = x1.max(x2);

    true
}

// Solves a cubic. Returns the largest root.
// Algorithm derived from "Numerical Recipes In C", chapter 5.5
fn cubic(mut a1: f32, a2: f32, a3: f32) -> f32 {
    a1 /= 3.0;

    let q: f32 = a1 * a1 - a2 / 3.0;
    let r: f32 = a1 * a1 * a1 + (a3 - a1 * a2) / 2.0;

    if q * q * q >= r * r {
        let theta = (r / (q * q * q).sqrt()).acos();

        let x1 = -2.0 * q.sqrt() * (theta / 3.0).cos() - a1;
        let x2 = -2.0 * q.sqrt() * ((theta + 2.0 * std::f32::consts::PI) / 3.0).cos() - a1;
        let x3 = -2.0 * q.sqrt() * ((theta - 2.0 * std::f32::consts::PI) / 3.0).cos() - a1;

        x1.max(x2).max(x3)
    } else {
        let temp: f32 = ((r * r - q * q * q).sqrt() + r.abs()).cbrt();
        -r.signum() * (temp + q / temp) - a1
    }
}

// Solves a quartic.
// Writes the roots to `roots`, in order from least to greatest.
// Returns the number of roots.
#[allow(clippy::many_single_char_names, clippy::collapsible_if)]
pub fn quartic(a: f32, b: f32, c: f32, d: f32, roots: &mut [f32; 4]) -> usize {
    // We want the roots of `xxxx + axxx + bxx + cx + d`.
    // Let's say it factors as `(xx + px + q)(xx + rx + s)`.
    // Then:
    //     a = p + r
    //     b = pr + q + s
    //     c = ps + qr
    //     d = qs
    //
    // Let t = (p - r) / 2.
    // Let α = a / 2
    //
    //     αα - tt = pr
    //
    //
    //     2c = 2ps + 2qr
    //       = (p + r)(q + s) - (p - r)(q - s)
    //       = 2α(b - pr) - 2t(q - s)
    //       = 2α(b - pr) - 2t(q - s)
    //
    //     t(q - s) = α(b - pr) - c
    //
    //     tt(q - s)(q - s) = (α(b - pr) - c) (α(b - pr) - c)
    //     tt(q + s)(q + s) = tt(b - pr)(b - pr)
    //
    //     4ttqs = tt(b - pr)(b - pr) - (α(b - pr) - c) (α(b - pr) - c)
    //
    //     4ttd
    //       = tt(b - αα + tt)(b - αα + tt) - (α(b - αα + tt) - c) (α(b - αα + tt) - c)
    //       = tttttt + 2tttt(b - αα) + tt(b - αα)(b - αα) - ttttαα - 2tt(α(b - αα) - c) - (α(b - αα) - c)(α(b - αα) - c)
    //       = (tt)^3 + (2(b - αα) - αα)(tt)^2 + ((b - αα)(b - αα) - 2α(α(b - αα) - c))(tt) - (α(b - αα) - c)(α(b - αα) - c)
    //
    // Let tmp1 = b - αα.
    // Let tmp2 = α tmp1 - c.
    //
    //     0 = (tt)^3 + (2 tmp1 - αα)(tt)^2 + (tmp1 tmp1 - 2α tmp2 - 4d)(tt) + (- tmp2 tmp2)

    let alpha: f32 = a / 2.0;

    let tmp1: f32 = b - alpha * alpha;
    let tmp2: f32 = alpha * tmp1 - c;

    let t: f32 = cubic(
        2.0 * tmp1 - alpha * alpha,
        tmp1 * tmp1 - 2.0 * alpha * tmp2 - 4.0 * d,
        -tmp2 * tmp2,
    )
    .sqrt();

    let p: f32 = alpha + t;
    let r: f32 = alpha - t;

    let q_plus_s: f32 = b - p * r;
    let q_minus_s: f32 = (alpha * q_plus_s - c) / t;

    let q: f32 = (q_plus_s + q_minus_s) / 2.0;
    let s: f32 = (q_plus_s - q_minus_s) / 2.0;

    // So our polynomial is (xx + px + q) (xx + rx + s).

    let mut roots0: [f32; 2] = [99999.; 2];
    let mut roots1: [f32; 2] = [99999.; 2];

    let test0: bool = quadratic(p, q, &mut roots0);
    let test1: bool = quadratic(r, s, &mut roots1);

    if test0 {
        if test1 {
            roots[0] = roots0[0].min(roots1[0]);
            let x1: f32 = roots0[0].max(roots1[0]);

            let x2: f32 = roots0[1].min(roots1[1]);
            roots[3] = roots0[1].max(roots1[1]);

            roots[1] = x1.min(x2);
            roots[2] = x1.max(x2);

            4
        } else {
            roots[0] = roots0[0];
            roots[1] = roots0[1];
            2
        }
    } else {
        if test1 {
            roots[0] = roots1[0];
            roots[1] = roots1[1];
            2
        } else {
            0
        }
    }
}
