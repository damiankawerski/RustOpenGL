#version 330 core

uniform mat4 MVMat;
uniform mat4 ViewMat;
uniform mat4 ProjectionMat;

layout (location=0) in vec3 VertexPosition;
layout (location=2) in vec3 VertexNormal;
layout (location=7) in vec2 VertexUV;

out vec3 v_normal;
out vec3 v_frag_pos;
out vec2 v_uv;

void main()
{
    mat4 MV = ViewMat * MVMat;
    gl_Position = ProjectionMat * MV * vec4(VertexPosition, 1.0);
    v_frag_pos  = vec3(MV * vec4(VertexPosition, 1.0));
    v_normal    = mat3(transpose(inverse(MV))) * VertexNormal;
    v_uv        = VertexUV;
}
