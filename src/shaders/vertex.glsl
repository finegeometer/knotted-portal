#version 300 es

in vec4 color0;
in vec4 color1;
in vec4 color2;
in vec4 color3;
in vec4 color4;
in vec4 color5;
in vec3 pos;
in vec3 normal;
in vec3 center;
in float ambient_factor;
in float diffuse_factor;

uniform mat4 mat;

out vec3 v_pos;
out vec3 v_normal;
out vec3 v_center;
out vec4[6] v_colors;
out float v_ambient_factor;
out float v_diffuse_factor;

void main() {
	v_colors[0] = color0;
	v_colors[1] = color1;
	v_colors[2] = color2;
	v_colors[3] = color3;
	v_colors[4] = color4;
	v_colors[5] = color5;

	v_pos = pos;
	v_normal = normal;
	v_center = center;
	v_ambient_factor = ambient_factor;
	v_diffuse_factor = diffuse_factor;

	gl_Position = mat * vec4(pos, 1.0);
}
