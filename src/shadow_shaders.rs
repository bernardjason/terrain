
pub const SHADOW_MAPPING_VS:&str = "#version 300 es
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoords;

out vec2 TexCoords;

out vec3 vs_out_FragPos;
out vec3 vs_out_Normal;
out vec2 vs_out_TexCoords;
out vec4 vs_out_FragPosLightSpace;
out vec3 v_fogDepth;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform mat4 lightSpaceMatrix;

void main()
{
    vs_out_FragPos = vec3(model * vec4(aPos, 1.0));
    //vs_out.Normal = transpose(inverse(mat3(model))) * aNormal;
    vs_out_Normal = transpose(inverse(mat3(model))) * vec3(1.0,1.0,0.0);
    vs_out_TexCoords = aTexCoords;
    vs_out_FragPosLightSpace = lightSpaceMatrix * vec4(vs_out_FragPos, 1.0);
    gl_Position = projection * view * model * vec4(aPos, 1.0);

    float fogAmount = smoothstep(0.001,0.7,  7.0/gl_Position.z);
    v_fogDepth = vec3(fogAmount) ;

}
";
pub const SHADOW_MAPPING_FS:&str = "#version 300 es
precision mediump float;
out vec4 FragColor;

in vec3 v_fogDepth;
in vec3 vs_out_FragPos;
in vec3 vs_out_Normal;
in vec2 vs_out_TexCoords;
in vec4 vs_out_FragPosLightSpace;

uniform sampler2D diffuseTexture;
uniform sampler2D shadowMap;

uniform vec3 lightPos;
uniform vec3 viewPos;

float ShadowCalculation(vec4 fragPosLightSpace)
{
    // perform perspective divide
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    // transform to [0,1] range
    projCoords = projCoords * 0.5 + 0.5;
    // get closest depth value from light's perspective (using [0,1] range fragPosLight as coords)
    float closestDepth = texture(shadowMap, projCoords.xy).r;
    // get depth of current fragment from light's perspective
    float currentDepth = projCoords.z;
    // check whether current frag pos is in shadow
    //float shadow = currentDepth > closestDepth  ? 1.0 : 0.0;
    float bias = 0.005;
    float shadow = currentDepth - bias > closestDepth  ? 1.0 : 0.0;

    return shadow;
}

void main()
{
    vec3 color = texture(diffuseTexture, vs_out_TexCoords).rgb;
    vec3 normal = normalize(vs_out_Normal);
    vec3 lightColor = vec3(0.6);
    // ambient
    vec3 ambient = 0.9 * color;
    // diffuse
    vec3 lightDir = normalize(lightPos - vs_out_FragPos);
    float diff = max(dot(lightDir, normal), 0.0);
    vec3 diffuse = diff * lightColor;
    // specular
    vec3 viewDir = normalize(viewPos - vs_out_FragPos);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = 0.0;
    vec3 halfwayDir = normalize(lightDir + viewDir);
    spec = pow(max(dot(normal, halfwayDir), 0.0), 64.0);
    vec3 specular = spec * lightColor;
    // calculate shadow
    float shadow = ShadowCalculation(vs_out_FragPosLightSpace);

    vec3 lighting = (ambient + (1.0 - shadow) * (diffuse + specular)) * color * v_fogDepth ;

    FragColor = vec4(lighting, 1.0);
}

";
pub const SHADOW_MAPPING_DEPTH_VS:&str = "#version 300 es
layout (location = 0) in vec3 aPos;

uniform mat4 lightSpaceMatrix;
uniform mat4 model;

void main()
{
    gl_Position = lightSpaceMatrix * model * vec4(aPos, 1.0);
}
";
pub const SHADOW_MAPPING_DEPTH_FS:&str = "#version 300 es

void main()
{
    // gl_FragDepth = gl_FragCoord.z;
}
";

