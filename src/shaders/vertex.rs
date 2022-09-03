pub const VERTEX_SRC: &'static str = r#"
    #version 450
    in vec2 points;
    out vec2 v_text_points;
    void main() {
        v_text_points = (points / 2.0);
        gl_Position = vec4(points, 1.0, 1.0);
    }
"#;
