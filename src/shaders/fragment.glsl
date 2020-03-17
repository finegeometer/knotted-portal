void main() {
	int world = eye_world;

	travel(world, eye, v_pos);
	travel(world, v_pos, v_center);

	color = v_colors[world];

	// Not currently needed
	// if color.a < 0.5 {
	// 	discard;
	// }

	color *= v_ambient_factor + v_diffuse_factor * max(dot(v_normal, light_dir), 0.0);

	color = vec4(color.xyz, 1.0);
}
