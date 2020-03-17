#version 300 es

precision mediump float;

in vec3 v_pos;
in vec3 v_normal;
in vec3 v_center;
in vec4[6] v_colors;
in float v_ambient_factor;
in float v_diffuse_factor;

out vec4 color;

uniform vec3 eye;
uniform int eye_world;
uniform vec3 light_dir;
