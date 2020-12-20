// pub const SHADER: &str = r#"
//     uniform float time_color_red;
//     uniform float time_color_blue;
//     uniform float time_color_green;

//     attribute vec4 aPosition;
//     attribute float aY;
//     attribute vec3 aVertexNormal;

//     uniform mat4 uNormalsRotation;
//     uniform mat4 uProjection;
//     varying lowp vec4 vColor;

//     void main() {
//         gl_Position = uProjection * vec4(aPosition.x, aY, aPosition.z, 1.0);

//         time_color_blue;
//         time_color_red;
//         time_color_green;

//         vec3 ambientLight = vec3(0.0, 0.0, 0.0);
//         vec3 directionalLightColor = vec3(1, 1, 1);
//         vec3 directionalVector = normalize(vec3(-0.85, 0.8, 0.75));

//         vec4 transformedNormal = uNormalsRotation * vec4(aVertexNormal, 1.0);
//         float directional = max(dot(transformedNormal.xyz, directionalVector), 0.0);
//         vec3 vLighting = ambientLight + (directionalLightColor * directional);
//         vec3 baseColor = vec3(time_color_red, time_color_blue, time_color_green);

//         vColor = vec4(baseColor * vLighting, 1.0);
//     }
// "#;
pub const SHADER: &str = r#"#version 300 es
uniform float time_color_red;
uniform float time_color_blue;
uniform float time_color_green;


in vec4 aPosition;
in float aY;
in vec3 aVertexNormal;

uniform mat4 uNormalsRotation;
uniform mat4 uProjection;
out vec4 vColor;
// out vec4 gl_Position;


void main() {
    gl_Position = uProjection * vec4(aPosition.x, aY, aPosition.z, 1.0);

    time_color_blue;
    time_color_red;
    time_color_green;

    vec3 ambientLight = vec3(0.0, 0.0, 0.0);
    vec3 directionalLightColor = vec3(1, 1, 1);
    vec3 directionalVector = normalize(vec3(-0.85, 0.8, 0.75));

    vec4 transformedNormal = uNormalsRotation * vec4(aVertexNormal, 1.0);
    float directional = max(dot(transformedNormal.xyz, directionalVector), 0.0);
    vec3 vLighting = ambientLight + (directionalLightColor * directional);
    vec3 baseColor = vec3(time_color_red, time_color_blue, time_color_green);

    vColor = vec4(baseColor * vLighting, 1.0);
}
"#;
