#version 450

in vec2 vTexCoord;

uniform sampler2D tex;

out vec4 fragColor;

void main()
{
    fragColor = texture(tex, vTexCoord);
}