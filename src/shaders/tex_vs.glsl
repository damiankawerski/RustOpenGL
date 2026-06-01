#version 330 core

uniform mat4 MVMat;
uniform mat4 ViewMat;
uniform mat4 ProjectionMat;

layout (location=0) in vec3 VertexPosition;
layout (location=1) in vec3 VertexColor;
layout (location=2) in vec3 VertexNormal;
layout (location=3) in vec3 VertexTangent;
layout (location=7) in vec2 VertexUV;

out vec3 v_frag_pos;
out vec3 v_color;
out vec2 uv;
out mat3 v_TBN;

void main()
{
    mat4 MV = ViewMat * MVMat;
    mat3 normalMat = mat3(transpose(inverse(MV)));

    vec3 N = normalize(normalMat * VertexNormal);
    vec3 T = normalize(mat3(MV) * VertexTangent);
    T = normalize(T - dot(T, N) * N); // re-orthogonalise
    vec3 B = cross(N, T);

    v_TBN     = mat3(T, B, N);
    v_frag_pos = vec3(MV * vec4(VertexPosition, 1.0));
    v_color    = VertexColor;
    uv         = VertexUV;

    gl_Position = ProjectionMat * MV * vec4(VertexPosition, 1.0);
}
