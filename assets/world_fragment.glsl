#version 150 core
precision mediump float;
uniform sampler2D u_image;
in vec2 texCoord;
uniform vec2 onePixel;
uniform int doStep;
uniform vec4 colorMask;
uniform mat3 kernel;
out vec4 color_out;
vec2 getCoords(vec2 coord, vec2 offset){
    return mod(coord + onePixel * offset, 1.0);
}

float activation(float x) {
    return x;

}
void main(void){
    if(doStep == 1){
        float cur = texture2D(u_image, getCoords(texCoord, vec2(0.0, 0.0))).a;
        if (cur != 0.) {
            color_out = vec4(cur, cur, cur, cur);
            return;
        }        
        // kernel indexes
        //    0       1       2
        //    3       4       5
        //    6       7       8
        // corresponding pixel coordinates (c, r)
        // ( 1,-1) ( 0,-1) (-1,-1)
        // ( 1, 0) ( 0, 0) (-1, 0)
        // ( 1, 1) ( 0, 1) (-1, 1)
        //                                          pixel( c,  r)   kernel weight[i]
        float sum = 
                texture2D(u_image, getCoords(texCoord, vec2( 1.,-1.))).a * kernel[0][0] 
            + texture2D(u_image, getCoords(texCoord, vec2( 0.,-1.))).a * kernel[0][1]
            + texture2D(u_image, getCoords(texCoord, vec2(-1.,-1.))).a * kernel[0][2]
            + texture2D(u_image, getCoords(texCoord, vec2( 1., 0.))).a * kernel[1][0]
            + texture2D(u_image, getCoords(texCoord, vec2( 0., 0.))).a * kernel[1][1]
            + texture2D(u_image, getCoords(texCoord, vec2(-1., 0.))).a * kernel[1][2]
            + texture2D(u_image, getCoords(texCoord, vec2( 1., 1.))).a * kernel[2][0]
            + texture2D(u_image, getCoords(texCoord, vec2( 0., 1.))).a * kernel[2][1]
            + texture2D(u_image, getCoords(texCoord, vec2(-1., 1.))).a * kernel[2][2];
        
        // Note on reversed implementation:
        // According to https://en.wikipedia.org/wiki/Kernel_(image_processing)#Convolution if the kernel
        // is not symmetric, it should be reversed before computing. This is how it is implemented in 
        // a number of python libraries, and thus how I implemented it here. I find it more intuitive.
        float x = activation(sum);
        
        color_out = vec4(x, x, x, x);
    } else {
        // color masking
        float x = texture2D(u_image, texCoord).a;
        color_out = vec4(x, x, x, x) * colorMask;
        
    }
}
// in vec2 v_TexCoord;
// out vec4 o_Color;
// uniform sampler2D t_color;
// void main() {
//     vec4 tex = texture(t_color, v_TexCoord);
//     float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
//     o_Color = mix(tex, vec4(0.0,0.0,0.0,0.0), sin(blend*1.0));
// }