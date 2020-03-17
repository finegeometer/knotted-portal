mod quartic;

/*
 ┏━━┓  ┏━━┓
┏┛  ┗┓┏┛  ┗┓
┃    ┗┓    ┃
┃   ┏┛┗┓   ┃
┗┓ ┏┛  ┗┓ ┏┛
 ┗━┃━━━━━━┛
   ┃    ┃
   ┗┓  ┏┛
    ┗━━┛

The boundary of the trefoil portal is parameterized by (sin(t) + 2sin(2t), cos(t)-2cos(2t), sin(3t)).

The trefoil's projection onto the xy-plane is the solution set of this quartic equation:
    4rrrr - 12rry + 16yyy - 27rr + 27 = 0
(rr == xx + yy)

The trefoil lies on this (topological) torus:
    zz = 1 - (rr - 5)^2 / 16

The xy plane is divided into twelve important regions by these inequalities:
    1.) x > 0
    2.) x < y √3
    3.) x < -y √3
    4.) r > 1.5

For a point (x,y,z) on the trefoil, z is positive whenever an even number of these inequalities hold.

The inequalities also allow you to deduce which arc in the knot diagram contains a given point.

                       ┃  (4) holds  │ (4) doesn't ┃
━━━━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━┿━━━━━━━━━━━━━┫
(2) holds, (1) doesn't ┃    Arc A    │    Arc C    ┃
───────────────────────╂─────────────┼─────────────┨
(1) holds, (3) doesn't ┃    Arc B    │    Arc A    ┃
───────────────────────╂─────────────┼─────────────┨
(3) holds, (2) doesn't ┃    Arc C    │    Arc B    ┃
━━━━━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━┷━━━━━━━━━━━━━┛

(A = top left, B = right, C = bottom)


Passing under an arc causes you to switch worlds.

      ╔═══╗       ╔═══╗
      ║ 1 ║───C───║ 2 ║
      ╚═══╝       ╚═══╝
      ╱   ╲       ╱   ╲
     A     B     A     B
    ╱       ╲   ╱       ╲
╔═══╗        ╲ ╱        ╔═══╗
║ 0 ║───C─────╳─────C───║ 3 ║
╚═══╝        ╱ ╲        ╚═══╝
    ╲       ╱   ╲       ╱
     B     A     B     A
      ╲   ╱       ╲   ╱
      ╔═══╗       ╔═══╗
      ║ 5 ║───C───║ 4 ║
      ╚═══╝       ╚═══╝

*/

const SQRT_3: f32 = 1.732_050_8;

// If you travel in a straight line from `start` to `end`, in which world do you end up?
#[rustfmt::skip]
pub fn travel(world: &mut i32, start: nalgebra::Vector3<f32>, end: nalgebra::Vector3<f32>) {

    // We define `x(t)`, `y(t)` to be linear polynomials parameterizing the line of travel.
    // Then we calculate `trefoil_projection_quartic(x(t), y(t))`, which is a quartic polynomial in t.
    // If t is a root of that quartic, then (x(t), y(t)) lies on the projection of the trefoil.

    // Linear Polynomials
    let mut x: [f32; 2] = [7777.; 2];
    let mut y: [f32; 2] = [7777.; 2];

    let mut v = (end - start).xy();
    let t_max = v.norm();
    v /= t_max;

    x[0] = start.x;
    y[0] = start.y;

    x[1] = v.x;
    y[1] = v.y;


    // Quadratic Polynomial
    let mut rr: [f32; 3] = [7777.; 3];
    rr[0] =       x[0] * x[0] +       y[0] * y[0];
    rr[1] = 2.0 * x[0] * x[1] + 2.0 * y[0] * y[1];
    rr[2] =       x[1] * x[1] +       y[1] * y[1];


    // Quartic Polynomial
    let mut poly: [f32; 5] = [7777.; 5];
    poly[0] = 4.0 * (      rr[0] * rr[0]                ) - 12.0 * (rr[0] * y[0]               ) + (16.0 * y[0] * y[0] * y[0]) - 27.0 * rr[0] + 27.0;
    poly[1] = 4.0 * (2.0 * rr[0] * rr[1]                ) - 12.0 * (rr[1] * y[0] + rr[0] * y[1]) + (48.0 * y[0] * y[0] * y[1]) - 27.0 * rr[1];
    poly[2] = 4.0 * (2.0 * rr[0] * rr[2] + rr[1] * rr[1]) - 12.0 * (rr[2] * y[0] + rr[1] * y[1]) + (48.0 * y[0] * y[1] * y[1]) - 27.0 * rr[2];
    poly[3] = 4.0 * (2.0 * rr[1] * rr[2]                ) - 12.0 * (               rr[2] * y[1]) + (16.0 * y[1] * y[1] * y[1]);
    poly[4] = 4.0 * (      rr[2] * rr[2]                );



    let mut roots: [f32; 4] = [6666.; 4];
	let num_roots: usize = quartic::quartic(
		poly[3] / poly[4],
		poly[2] / poly[4],
		poly[1] / poly[4],
		poly[0] / poly[4],
		&mut roots
	);

    for &root in roots.iter().take(num_roots) {
        if 0.0 < root && root < t_max {


            let pos = start.lerp(&end, root / t_max);

            let rr: f32 = pos.x*pos.x + pos.y*pos.y;

            let test1: bool = pos.x > 0.0;
            let test2: bool = pos.x < pos.y * SQRT_3;
            let test3: bool = pos.x < pos.y * -SQRT_3;
            let test4: bool = rr > 2.25;

            let trefoil_z: f32 =
                (1.0 - ((rr - 5.0) * (rr - 5.0) / 16.0)).sqrt() *
                (if test1 ^ test2 ^ test3 ^ test4 {-1.0} else {1.0});

            if pos.z < trefoil_z {
                // Arc A = 1, B = 5, C = 3
                eprintln!("{:?}", [test1,test2,test3,test4]);

                #[allow(clippy::suspicious_else_formatting, clippy::collapsible_if)]
                let mut arc: i32 = if test1
                    {if test3 {3} else {5}} else
                    {if test2 {1} else {3}};
                arc += if test4 {0} else {2};

                *world = arc - *world;
            }
        }
    }

    *world = world.rem_euclid(6);
}
