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


float sqrt_3 = sqrt(3.0);

// If you travel in a straight line from `start` to `end`, in which world do you end up?
void travel(inout int world, vec3 start, vec3 end) {

	// We define `x(t)`, `y(t)` to be linear polynomials parameterizing the line of travel.
	// Then we calculate `trefoil_projection_quartic(x(t), y(t))`, which is a quartic polynomial in t.
	// If t is a root of that quartic, then (x(t), y(t)) lies on the projection of the trefoil.

	// Linear Polynomials
	float x[2];
	float y[2];

	vec2 v = end.xy - start.xy;
	float t_max = length(v);
	v /= t_max;

	x[0] = start.x;
	y[0] = start.y;

	x[1] = v.x;
	y[1] = v.y;


	// Quadratic Polynomial
	float rr[3];
	rr[0] =       x[0] * x[0] +       y[0] * y[0];
	rr[1] = 2.0 * x[0] * x[1] + 2.0 * y[0] * y[1];
	rr[2] =       x[1] * x[1] +       y[1] * y[1];


	// Quartic Polynomial
	float poly[5];
	poly[0] = 4.0 * (      rr[0] * rr[0]                ) - 12.0 * (rr[0] * y[0]               ) + (16.0 * y[0] * y[0] * y[0]) - 27.0 * rr[0] + 27.0;
	poly[1] = 4.0 * (2.0 * rr[0] * rr[1]                ) - 12.0 * (rr[1] * y[0] + rr[0] * y[1]) + (48.0 * y[0] * y[0] * y[1]) - 27.0 * rr[1];
	poly[2] = 4.0 * (2.0 * rr[0] * rr[2] + rr[1] * rr[1]) - 12.0 * (rr[2] * y[0] + rr[1] * y[1]) + (48.0 * y[0] * y[1] * y[1]) - 27.0 * rr[2];
	poly[3] = 4.0 * (2.0 * rr[1] * rr[2]                ) - 12.0 * (               rr[2] * y[1]) + (16.0 * y[1] * y[1] * y[1]);
	poly[4] = 4.0 * (      rr[2] * rr[2]                );



	float roots[4];
	int num_roots = quartic(
		poly[3] / poly[4],
		poly[2] / poly[4],
		poly[1] / poly[4],
		poly[0] / poly[4],
		roots
	);

	for(int i = 0; i < num_roots; i++) {

		if (0.0 < roots[i] && roots[i] < t_max) {

			vec3 pos = mix(start, end, roots[i] / t_max);

			float rr = pos.x*pos.x + pos.y*pos.y;

			bool test1 = pos.x > 0.0;
			bool test2 = pos.x < pos.y * sqrt_3;
			bool test3 = pos.x < pos.y * -sqrt_3;
			bool test4 = rr > 2.25;

			float trefoil_z =
				sqrt(1.0 - ((rr - 5.0) * (rr - 5.0) / 16.0)) *
				((test1 ^^ test2 ^^ test3 ^^ test4) ? -1.0 : 1.0);

			if (pos.z < trefoil_z) {

				// Arc A = 1, B = 5, C = 3

				int arc = test1
					? (test3 ? 3 : 5)
					: (test2 ? 1 : 3);
				arc += test4 ? 0 : 2;

				world = arc - world;
			}
		}
	}

	// Workaround for the fact that % behaves incorrectly for negative numbers.
	// (I need euclidean division, glsl provides truncated division.)
	world = (world + 18) % 6;
}
