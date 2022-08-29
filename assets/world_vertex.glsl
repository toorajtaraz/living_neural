#version 150 core
in ivec3 a_pos;
in ivec2 a_tex_coord;
out vec2 v_TexCoord;
uniform mat4 u_model_view_proj;
void main() {
    v_TexCoord = a_tex_coord;
    gl_Position = u_model_view_proj * vec4(tanh(float(a_pos.x)), (float(a_pos.y)), tanh(float(a_pos.z)), 1.0);
}