shader_type canvas_item;
uniform sampler2D tex : hint_screen_texture, filter_linear_mipmap;
uniform float lod: hint_range(0, 5.0) = 0.0;

void fragment(){
	vec4 color = texture(tex, SCREEN_UV, lod);
	COLOR = color;
}