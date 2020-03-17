// Solves a quadratic.
// Writes the roots to `roots`, in order from least to greatest.
// Returns true if the roots are real.
// Algorithm derived from "Numerical Recipes In C", chapter 5.5
bool quadratic(float b, float c, out float roots[2]) {

	float disc = b*b - 4.0*c;
	if (disc < 0.0) {
		return false;
	}

	float x1 = -(b + sign(b) * sqrt(disc)) / 2.0;
	float x2 = c / x1;

	roots[0] = min(x1, x2);
	roots[1] = max(x1, x2);

	return true;
}

float tau = 6.283185307179586476925286766559005768394338798750211641949;

// Solves a cubic. Returns the largest root.
// Algorithm derived from "Numerical Recipes In C", chapter 5.5
float cubic(float a1, float a2, float a3) {
	a1 /= 3.0;

	float q = a1*a1 - a2 / 3.0;
	float r = a1*a1*a1 + (a3 - a1*a2) / 2.0;

	if (q*q*q >= r*r) {
		float theta = acos(r * inversesqrt(q*q*q));
		float x1 = -2.0 * sqrt(q) * cos(theta / 3.0) - a1;
		float x2 = -2.0 * sqrt(q) * cos((theta + tau) / 3.0) - a1;
		float x3 = -2.0 * sqrt(q) * cos((theta - tau) / 3.0) - a1;
		return max(x1, max(x2, x3));
	} else {
		float temp = pow(sqrt(r*r - q*q*q) + abs(r), 0.33333333333333333);
		return -sign(r) * (temp + q / temp) - a1;
	}
}

// Solves a quartic.
// Writes the roots to `roots`, in order from least to greatest.
// Returns the number of roots.
int quartic(float a, float b, float c, float d, out float roots[4]) {
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

	float alpha = a / 2.0;

	float tmp1 = b - alpha*alpha;
	float tmp2 = alpha*tmp1 - c;

	float t = sqrt(cubic(2.0*tmp1 - alpha*alpha, tmp1*tmp1 - 2.0*alpha*tmp2 - 4.0*d, -tmp2 * tmp2));

	float p = alpha + t;
	float r = alpha - t;

	float q_plus_s = b - p*r;
	float q_minus_s = (alpha * q_plus_s - c) / t;

	float q = (q_plus_s + q_minus_s) / 2.0;
	float s = (q_plus_s - q_minus_s) / 2.0;


	// So our polynomial is (xx + px + q) (xx + rx + s).

	float roots0[2];
	float roots1[2];

	bool test0 = quadratic(p, q, roots0);
	bool test1 = quadratic(r, s, roots1);

	if (test0) {
		if (test1) {
			roots[0] = min(roots0[0], roots1[0]);
			float x1 = max(roots0[0], roots1[0]);

			float x2 = min(roots0[1], roots1[1]);
			roots[3] = max(roots0[1], roots1[1]);

			roots[1] = min(x1, x2);
			roots[2] = max(x1, x2);

			return 4;
		} else {
			roots[0] = roots0[0];
			roots[1] = roots0[1];
			return 2;
		}
	} else {
		if (test1) {
			roots[0] = roots1[0];
			roots[1] = roots1[1];
			return 2;
		} else {
			return 0;
		}
	}
}
