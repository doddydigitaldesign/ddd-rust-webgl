// pub const SHADER: &str = r#"
//     precision mediump float;

//     uniform float uOpacity;

//     varying lowp vec4 vColor;

//     void main() {
//         gl_FragColor = vec4(vColor.r, vColor.g, vColor.b, vColor.a * uOpacity);
//     }
// "#;
pub const SHADER: &str = r#"#version 300 es

    precision mediump float;

    in highp vec4 vColor;

    layout (location = 0) out vec4 fragColor;

    void main()
    {
        fragColor = vec4(vColor.r, vColor.g, vColor.b, vColor.a);
    }
"#;
