use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

type GL = web_sys::WebGl2RenderingContext;

pub struct Renderer {
    canvas: web_sys::HtmlCanvasElement,
    gl: GL,
    program: web_sys::WebGlProgram,

    vao_static: web_sys::WebGlVertexArrayObject,
    vertex_buffer_static: web_sys::WebGlBuffer,

    vao_dynamic: web_sys::WebGlVertexArrayObject,
    vertex_buffer_dynamic: web_sys::WebGlBuffer,

    num_verts_static: usize,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program));
        self.gl.delete_vertex_array(Some(&self.vao_static));
        self.gl.delete_buffer(Some(&self.vertex_buffer_static));
    }
}

impl Renderer {
    pub fn new(
        canvas: &web_sys::HtmlCanvasElement,
        static_geometry: impl IntoIterator<Item = crate::modeling::Triangle>,
    ) -> Self {
        let gl = canvas
            .get_context("webgl2")
            .unwrap_throw()
            .unwrap_throw()
            .dyn_into::<GL>()
            .unwrap_throw();

        gl.enable(GL::DEPTH_TEST);
        gl.enable(GL::CULL_FACE);
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        let vertex_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap_throw();
        gl.shader_source(&vertex_shader, VERTEX_SHADER_SOURCE);
        gl.compile_shader(&vertex_shader);

        let fragment_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap_throw();
        gl.shader_source(&fragment_shader, FRAGMENT_SHADER_SOURCE);
        gl.compile_shader(&fragment_shader);

        web_sys::console::log_1(&gl.get_shader_info_log(&vertex_shader).unwrap_throw().into());
        web_sys::console::log_1(
            &gl.get_shader_info_log(&fragment_shader)
                .unwrap_throw()
                .into(),
        );

        let program = gl.create_program().unwrap_throw();
        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);

        gl.delete_shader(Some(&vertex_shader));
        gl.delete_shader(Some(&fragment_shader));

        let attribute_color0 = gl.get_attrib_location(&program, "color0") as u32;
        let attribute_color1 = gl.get_attrib_location(&program, "color1") as u32;
        let attribute_color2 = gl.get_attrib_location(&program, "color2") as u32;
        let attribute_color3 = gl.get_attrib_location(&program, "color3") as u32;
        let attribute_color4 = gl.get_attrib_location(&program, "color4") as u32;
        let attribute_color5 = gl.get_attrib_location(&program, "color5") as u32;

        let attribute_pos = gl.get_attrib_location(&program, "pos") as u32;
        let attribute_normal = gl.get_attrib_location(&program, "normal") as u32;
        let attribute_center = gl.get_attrib_location(&program, "center") as u32;
        let attribute_ambient = gl.get_attrib_location(&program, "ambient_factor") as u32;
        let attribute_diffuse = gl.get_attrib_location(&program, "diffuse_factor") as u32;

        let vao_static = gl.create_vertex_array().unwrap_throw();
        gl.bind_vertex_array(Some(&vao_static));

        let vertex_buffer_static = gl.create_buffer().unwrap_throw();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer_static));

        gl.enable_vertex_attrib_array(attribute_color0);
        gl.vertex_attrib_pointer_with_i32(attribute_color0, 4, GL::FLOAT, false, 35 * 4, 0);
        gl.enable_vertex_attrib_array(attribute_color1);
        gl.vertex_attrib_pointer_with_i32(attribute_color1, 4, GL::FLOAT, false, 35 * 4, 4 * 4);
        gl.enable_vertex_attrib_array(attribute_color2);
        gl.vertex_attrib_pointer_with_i32(attribute_color2, 4, GL::FLOAT, false, 35 * 4, 8 * 4);
        gl.enable_vertex_attrib_array(attribute_color3);
        gl.vertex_attrib_pointer_with_i32(attribute_color3, 4, GL::FLOAT, false, 35 * 4, 12 * 4);
        gl.enable_vertex_attrib_array(attribute_color4);
        gl.vertex_attrib_pointer_with_i32(attribute_color4, 4, GL::FLOAT, false, 35 * 4, 16 * 4);
        gl.enable_vertex_attrib_array(attribute_color5);
        gl.vertex_attrib_pointer_with_i32(attribute_color5, 4, GL::FLOAT, false, 35 * 4, 20 * 4);

        gl.enable_vertex_attrib_array(attribute_pos);
        gl.vertex_attrib_pointer_with_i32(attribute_pos, 3, GL::FLOAT, false, 35 * 4, 24 * 4);
        gl.enable_vertex_attrib_array(attribute_normal);
        gl.vertex_attrib_pointer_with_i32(attribute_normal, 3, GL::FLOAT, false, 35 * 4, 27 * 4);
        gl.enable_vertex_attrib_array(attribute_center);
        gl.vertex_attrib_pointer_with_i32(attribute_center, 3, GL::FLOAT, false, 35 * 4, 30 * 4);
        gl.enable_vertex_attrib_array(attribute_ambient);
        gl.vertex_attrib_pointer_with_i32(attribute_ambient, 1, GL::FLOAT, false, 35 * 4, 33 * 4);
        gl.enable_vertex_attrib_array(attribute_diffuse);
        gl.vertex_attrib_pointer_with_i32(attribute_diffuse, 1, GL::FLOAT, false, 35 * 4, 34 * 4);

        let data: Vec<f32> = static_geometry
            .into_iter()
            .flat_map(triangle_to_array)
            .collect::<Vec<f32>>();

        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &as_f32_array(&data).into(),
            GL::STATIC_DRAW,
        );

        let vao_dynamic = gl.create_vertex_array().unwrap_throw();
        gl.bind_vertex_array(Some(&vao_dynamic));
        let vertex_buffer_dynamic = gl.create_buffer().unwrap_throw();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer_dynamic));

        gl.enable_vertex_attrib_array(attribute_color0);
        gl.vertex_attrib_pointer_with_i32(attribute_color0, 4, GL::FLOAT, false, 35 * 4, 0);
        gl.enable_vertex_attrib_array(attribute_color1);
        gl.vertex_attrib_pointer_with_i32(attribute_color1, 4, GL::FLOAT, false, 35 * 4, 4 * 4);
        gl.enable_vertex_attrib_array(attribute_color2);
        gl.vertex_attrib_pointer_with_i32(attribute_color2, 4, GL::FLOAT, false, 35 * 4, 8 * 4);
        gl.enable_vertex_attrib_array(attribute_color3);
        gl.vertex_attrib_pointer_with_i32(attribute_color3, 4, GL::FLOAT, false, 35 * 4, 12 * 4);
        gl.enable_vertex_attrib_array(attribute_color4);
        gl.vertex_attrib_pointer_with_i32(attribute_color4, 4, GL::FLOAT, false, 35 * 4, 16 * 4);
        gl.enable_vertex_attrib_array(attribute_color5);
        gl.vertex_attrib_pointer_with_i32(attribute_color5, 4, GL::FLOAT, false, 35 * 4, 20 * 4);

        gl.enable_vertex_attrib_array(attribute_pos);
        gl.vertex_attrib_pointer_with_i32(attribute_pos, 3, GL::FLOAT, false, 35 * 4, 24 * 4);
        gl.enable_vertex_attrib_array(attribute_normal);
        gl.vertex_attrib_pointer_with_i32(attribute_normal, 3, GL::FLOAT, false, 35 * 4, 27 * 4);
        gl.enable_vertex_attrib_array(attribute_center);
        gl.vertex_attrib_pointer_with_i32(attribute_center, 3, GL::FLOAT, false, 35 * 4, 30 * 4);
        gl.enable_vertex_attrib_array(attribute_ambient);
        gl.vertex_attrib_pointer_with_i32(attribute_ambient, 1, GL::FLOAT, false, 35 * 4, 33 * 4);
        gl.enable_vertex_attrib_array(attribute_diffuse);
        gl.vertex_attrib_pointer_with_i32(attribute_diffuse, 1, GL::FLOAT, false, 35 * 4, 34 * 4);

        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &as_f32_array(&[]).into(),
            GL::DYNAMIC_DRAW,
        );

        Self {
            program,

            vao_static,
            vertex_buffer_static,

            vao_dynamic,
            vertex_buffer_dynamic,

            gl,
            canvas: canvas.clone(),

            num_verts_static: data.len() / 35,
        }
    }

    pub fn render(&self, uniforms: Uniforms, mut dynamic_geometry: Vec<crate::modeling::Triangle>) {
        let width = web_sys::window()
            .unwrap_throw()
            .inner_width()
            .unwrap_throw()
            .as_f64()
            .unwrap_throw()
            - 16.;
        let height = web_sys::window()
            .unwrap_throw()
            .inner_height()
            .unwrap_throw()
            .as_f64()
            .unwrap_throw()
            - 16.;

        self.canvas
            .set_attribute("width", &format!("{}", width as i32))
            .unwrap_throw();
        self.canvas
            .set_attribute("height", &format!("{}", height as i32))
            .unwrap_throw();

        self.gl.use_program(Some(&self.program));

        let projection_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new_perspective(
            width as f32 / height as f32,
            std::f32::consts::FRAC_PI_2,
            0.01,
            200.,
        );

        let mat: nalgebra::Matrix4<f32> =
            projection_matrix * uniforms.player_isometry.inverse().to_homogeneous();

        self.gl.uniform_matrix4fv_with_f32_array(
            self.gl.get_uniform_location(&self.program, "mat").as_ref(),
            false,
            &mat.as_slice(),
        );

        self.gl.uniform3f(
            self.gl.get_uniform_location(&self.program, "eye").as_ref(),
            uniforms.player_isometry.translation.vector[0],
            uniforms.player_isometry.translation.vector[1],
            uniforms.player_isometry.translation.vector[2],
        );

        self.gl.uniform3f(
            self.gl.get_uniform_location(&self.program, "eye").as_ref(),
            uniforms.player_isometry.translation.vector[0],
            uniforms.player_isometry.translation.vector[1],
            uniforms.player_isometry.translation.vector[2],
        );

        self.gl.uniform1i(
            self.gl
                .get_uniform_location(&self.program, "eye_world")
                .as_ref(),
            uniforms.player_world,
        );

        self.gl.uniform3f(
            self.gl
                .get_uniform_location(&self.program, "light_dir")
                .as_ref(),
            uniforms.light_dir[0],
            uniforms.light_dir[1],
            uniforms.light_dir[2],
        );

        self.gl.viewport(0, 0, width as i32, height as i32);
        self.gl.clear_color(0., 0., 0., 1.);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.gl.bind_vertex_array(Some(&self.vao_static));
        self.gl
            .draw_arrays(GL::TRIANGLES, 0, self.num_verts_static as i32);

        self.gl.bind_vertex_array(Some(&self.vao_dynamic));
        dynamic_geometry.sort_by_key(|tri| {
            std::cmp::Reverse(
                // farthest first
                (tri.center() - uniforms.player_isometry.translation.vector)
                    .norm_squared()
                    .to_bits(), // to_bits is monotonic on positive floats, so this is an easy way to ignore NaN.
            )
        });
        let data: Vec<f32> = dynamic_geometry
            .into_iter()
            .flat_map(triangle_to_array)
            .collect::<Vec<f32>>();
        self.gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &as_f32_array(&data).into(),
            GL::DYNAMIC_DRAW,
        );
        self.gl
            .draw_arrays(GL::TRIANGLES, 0, (data.len() / 35) as i32);
    }
}

pub struct Uniforms {
    pub player_isometry: nalgebra::Isometry3<f32>, // Player space -> World Space
    pub player_world: i32,
    pub light_dir: nalgebra::Vector3<f32>,
}

fn triangle_to_array(tri: crate::modeling::Triangle) -> impl IntoIterator<Item = f32> {
    let [v1, v2, v3] = tri.vertices;

    let normal: nalgebra::Vector3<f32> = (v2 - v1).cross(&(v3 - v1)).normalize();
    let center: nalgebra::Vector3<f32> = tri.center();

    let mut out = Vec::with_capacity(3 * 35);
    for &pos in &tri.vertices {
        for &color in &tri.colors {
            out.extend_from_slice(&color);
        }
        out.extend_from_slice(pos.as_slice());
        out.extend_from_slice(normal.as_slice());
        out.extend_from_slice(center.as_slice());
        out.push(tri.ambient_factor);
        out.push(tri.diffuse_factor);
    }

    out
}

const VERTEX_SHADER_SOURCE: &str = include_str!("shaders/vertex.glsl");
const FRAGMENT_SHADER_SOURCE: &str = concat!(
    include_str!("shaders/fragment_prelude.glsl"),
    include_str!("shaders/quartic.glsl"),
    include_str!("shaders/portal.glsl"),
    include_str!("shaders/fragment.glsl"),
);

fn as_f32_array(v: &[f32]) -> js_sys::Float32Array {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()
        .unwrap_throw()
        .buffer();

    let location = v.as_ptr() as u32 / 4;

    js_sys::Float32Array::new(&memory_buffer).subarray(location, location + v.len() as u32)
}
