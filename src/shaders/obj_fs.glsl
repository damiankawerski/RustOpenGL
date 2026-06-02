#version 330 core

uniform sampler2D DiffuseSampler;
uniform vec3 LightPos;
uniform vec3 LightColor;
uniform vec3 MaterialAmbient;
uniform vec3 MaterialSpecular;

const float MaterialShininess = 64.0;

in vec3 v_normal;
in vec3 v_frag_pos;
in vec2 v_uv;

out vec4 FragColor;

void main()
{
    vec3 tex_color   = texture(DiffuseSampler, v_uv).rgb;
    vec3 normal      = normalize(v_normal);
    vec3 light_dir   = normalize(LightPos - v_frag_pos);
    vec3 view_dir    = normalize(-v_frag_pos);
    vec3 reflect_dir = reflect(-light_dir, normal);

    vec3 ambient  = MaterialAmbient * LightColor * tex_color;
    float diff    = max(0.0, dot(normal, light_dir));
    vec3 diffuse  = diff * LightColor * tex_color;
    float spec    = pow(max(0.0, dot(view_dir, reflect_dir)), MaterialShininess);
    vec3 specular = spec * MaterialSpecular * LightColor;

    FragColor = vec4(ambient + diffuse + specular, 1.0);
}
