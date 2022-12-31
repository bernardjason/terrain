
pub const SKYBOX_VS:&str = "#version 300 es
precision lowp float;
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;
out vec3 Pos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
	gl_Position = projection * view * model * vec4(aPos, 1.0f);
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
	Pos = vec3(aPos);
}
";

pub const SKYBOX_FS:&str = "#version 300 es
precision lowp float;
out vec4 FragColor;

in vec2 TexCoord;
in vec3 Pos;

// texture samplers
uniform sampler2D texture0;

void main()
{
	FragColor = texture(texture0, TexCoord);

}
";
