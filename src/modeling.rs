pub struct Triangle {
    pub vertices: [nalgebra::Vector3<f32>; 3],
    pub colors: [Option<[f32; 3]>; 6],

    pub ambient_factor: f32,
    pub diffuse_factor: f32,
}

mod trefoil {
    fn trefoil(t: f32) -> nalgebra::Vector3<f32> {
        nalgebra::Vector3::new(
            t.sin() + 2. * (2. * t).sin(),
            t.cos() - 2. * (2. * t).cos(),
            (3. * t).sin(),
        )
    }

    fn trefoil_derivative(t: f32) -> nalgebra::Vector3<f32> {
        nalgebra::Vector3::new(
            t.cos() + 4. * (2. * t).cos(),
            -t.sin() + 4. * (2. * t).sin(),
            3. * (3. * t).cos(),
        )
    }

    // Warning: theta = 0 is on the seam between worlds.
    pub fn trefoil_tube(t: f32, theta: f32) -> nalgebra::Vector3<f32> {
        let [dx, dy, _]: [f32; 3] = trefoil_derivative(t).into();

        let (s, c) = theta.sin_cos();
        trefoil(t)
            + 0.2
                * (nalgebra::Vector3::new(dy, -dx, 0.).normalize() * s - nalgebra::Vector3::z() * c)
    }
}

pub fn trefoil() -> impl Iterator<Item = Triangle> {
    const TAU: f32 = 2. * std::f32::consts::PI;

    let ambient_factor = 0.2;
    let diffuse_factor = 0.8;

    let f = |a: usize, b: usize| {
        let t = a as f32 * TAU / 96.;
        let u = (4 * b + 1) as f32 * TAU / 48.;
        trefoil::trefoil_tube(t, 4. * t + u)
    };

    (0..96).flat_map(move |a| {
        const R: Option<[f32; 3]> = Some([1.0, 0.0, 0.0]);
        const G: Option<[f32; 3]> = Some([0.0, 1.0, 0.0]);
        const B: Option<[f32; 3]> = Some([0.0, 0.0, 1.0]);

        let colors = match a {
            28..=59 => [B, G, G, B, R, R], // Arc C
            60..=91 => [R, R, B, G, G, B], // Arc A
            _ => [G, B, R, R, B, G],       // Arc B
        };

        (0..12).flat_map(move |b| {
            let v0 = f(a, b);
            let v1 = f(a + 1, b);
            let v2 = f(a, b + 1);
            let v3 = f(a + 1, b + 1);

            let t0 = Triangle {
                vertices: [v0, v1, v2],
                colors,
                ambient_factor,
                diffuse_factor,
            };
            let t1 = Triangle {
                vertices: [v3, v2, v1],
                colors,
                ambient_factor,
                diffuse_factor,
            };

            std::iter::once(t0).chain(std::iter::once(t1))
        })
    })
}

pub fn skybox() -> impl IntoIterator<Item = Triangle> {
    let colors = [
        Some([0.2, 0.7, 1.0]),
        Some([0.2, 1.0, 0.7]),
        Some([0.7, 1.0, 0.2]),
        Some([0.7, 0.2, 1.0]),
        Some([1.0, 0.2, 0.7]),
        Some([1.0, 0.7, 0.2]),
    ];

    let ambient_factor = 1.0;
    let diffuse_factor = 0.0;

    let v0 = nalgebra::Vector3::new(-100., -100., 100.);
    let v1 = nalgebra::Vector3::new(-100., 100., -100.);
    let v2 = nalgebra::Vector3::new(100., -100., -100.);
    let v3 = nalgebra::Vector3::new(100., 100., 100.);
    vec![
        Triangle {
            vertices: [v2, v1, v0],
            colors,
            ambient_factor,
            diffuse_factor,
        },
        Triangle {
            vertices: [v0, v1, v3],
            colors,
            ambient_factor,
            diffuse_factor,
        },
        Triangle {
            vertices: [v3, v2, v0],
            colors,
            ambient_factor,
            diffuse_factor,
        },
        Triangle {
            vertices: [v1, v2, v3],
            colors,
            ambient_factor,
            diffuse_factor,
        },
    ]
}

pub fn ground() -> impl IntoIterator<Item = Triangle> {
    const GRAY: Option<[f32; 3]> = Some([0.5, 0.5, 0.5]);
    let colors = [GRAY; 6];

    let ambient_factor = 0.2;
    let diffuse_factor = 0.8;

    let v0 = nalgebra::Vector3::new(-100., -100., -2.);
    let v1 = nalgebra::Vector3::new(100., -100., -2.);
    let v2 = nalgebra::Vector3::new(100., 100., -2.);
    let v3 = nalgebra::Vector3::new(-100., 100., -2.);
    vec![
        Triangle {
            vertices: [v0, v1, v2],
            colors,
            ambient_factor,
            diffuse_factor,
        },
        Triangle {
            vertices: [v2, v3, v0],
            colors,
            ambient_factor,
            diffuse_factor,
        },
    ]
}
