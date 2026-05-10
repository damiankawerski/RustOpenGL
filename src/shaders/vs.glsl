#version 330 core

uniform vec3 Color;
uniform mat4 MVMat;
uniform mat4 ViewMat;

layout (location=0) in vec3 VertexPosition;
layout (location=1) in vec3 VertexColor;

out vec4 v_color;

void main()
{
    gl_Position = ViewMat * MVMat * vec4(VertexPosition, 1.0);
    v_color = vec4(VertexColor, 1.0);
}
