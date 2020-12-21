#version 100
precision highp float;

uniform vec2 u_position;
uniform float u_zoom;
uniform vec2 u_resolution;
uniform vec4 u_back_color;
uniform vec4 u_line_color;

float grid_intensity = 0.7;

// Thick lines 
float grid(vec2 fragCoord, float space, float gridWidth)
{
    vec2 p  = fragCoord - vec2(.5);
    vec2 size = vec2(gridWidth - .5);
    
    vec2 a1 = mod(p - size, space);
    vec2 a2 = mod(p + size, space);
    vec2 a = a2 - a1;
       
    float g = min(a.x, a.y);
    return clamp(g, 0., 1.0);
}

void main() {
    vec2 fragCoord = gl_FragCoord.xy;

    fragCoord = fragCoord + u_position;

    // // Pixel color
    vec3 col = vec3(grid_intensity,grid_intensity,grid_intensity);

    // // Gradient across screen
    vec2 p = fragCoord;     // Point
	vec2 c = u_resolution / 2.0;   // Center
    col *= (1.0 - length(c - p) / u_resolution.x * 0.5);

    col *= clamp(
        grid(fragCoord, 10., 1.) * grid(fragCoord, 50., 1.5), 
        grid_intensity, 
        1.0
    );

    gl_FragColor = vec4(col,1.0);
}