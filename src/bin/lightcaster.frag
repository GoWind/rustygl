#version 330 core

out vec4 FragColor;

struct Material {
	sampler2D diffuse;
	sampler2D specular;
	float shininess;
};

struct Light {
	vec3 position;
	vec3 direction;
	vec3 ambient;
	vec3 diffuse;
	vec3 specular;
	float constant;
	float linear;
	float quadratic;
	float innerCutoff;
	float outerCutoff;
};

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

uniform vec3 viewPos;
uniform Material material;
uniform Light light;

void main()
{

	vec3 norm = normalize(Normal);

	// lightDir points from Fragment to Light
	vec3 lightDir = normalize(light.position - FragPos);

	//change direction of viewDir and then reflect it off the norm
	vec3 reflectDir = reflect(-lightDir, norm);
	//light.direction points from light to Frag

	// viewDir is from Fragment to View
	vec3 viewDir = normalize(viewPos-FragPos);

	//-light.direction points from Fragment to Light
	float theta = dot(lightDir, normalize(-light.direction));
	float distance = length(light.position - FragPos);

	float epsilon = light.innerCutoff - light.outerCutoff;
	float intensity = clamp ((theta - light.outerCutoff) / epsilon, 0.0, 1.0);
	vec3 ambient = light.ambient * texture(material.diffuse, TexCoords).rgb;

	vec3 result = vec3(0.0, 0.0, 0.0);
	float diff = max(dot(norm, lightDir), 0.0);

	float spec = pow(max(dot(viewDir, reflectDir) , 0.0), material.shininess);
	vec3 specular = light.specular * spec * texture(material.specular, TexCoords).rgb;

	vec3 diffuse = light.diffuse * diff * texture(material.diffuse, TexCoords).rgb;

	diffuse  *= intensity;
	specular *= intensity;
	
	float attenuation = 1.0/ (light.constant + light.linear * distance + light.quadratic * (distance * distance));
	ambient  *= attenuation;
	diffuse  *= attenuation;
	specular *= attenuation;
	result    = ambient + diffuse + specular;
	FragColor = vec4(result, 1.0);
	FragColor = vec4(ambient, 1.0);
}

