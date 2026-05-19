#version 330 core

uniform vec3  LightPos;
uniform vec3  LightColor;
uniform vec3  MaterialAmbient;
uniform vec3  MaterialDiffuse;
uniform vec3  MaterialSpecular;

const float MaterialShininess = 200.0;

in vec3 v_normal;
in vec3 v_frag_pos;
in vec3 v_color;

out vec4 FragColor;

void main()
{
    vec3 normal    = normalize(v_normal);
    vec3 light_dir = normalize(LightPos - v_frag_pos);
    vec3 view_dir  = normalize(-v_frag_pos); 

    vec3 ambient  = MaterialAmbient * LightColor;

    float diff    = max(0.0, dot(normal, light_dir));
    vec3 diffuse  = diff * MaterialDiffuse * LightColor * v_color;

    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec    = pow(max(0.0, dot(view_dir, reflect_dir)), MaterialShininess);
    vec3 specular = spec * MaterialSpecular * LightColor;

    FragColor = vec4(ambient + diffuse + specular, 1.0);
}
