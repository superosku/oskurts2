
// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    // Have the indice index (number of the group of 3 vertex available)
//    @location(2) index: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    // Output the triangle number
//    @location(1) index: u32,
}

@vertex
fn vs_main(
    model: VertexInput,
//    @builtin(vertex_index) in_vertex_index: u32,

) -> VertexOutput {
//    var out: VertexOutput;
//    let x = f32(1 - i32(in_vertex_index)) * 0.5;
//    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
//    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
//    out.vert_pos = out.clip_position.xyz;
//    return out;

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
//    out.index = model.index;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//    return vec4<f32>(in.color, 1.0);
    var sampled = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // Check if it is black and return white if yes
//    if (sampled.r == 0.0 && sampled.g == 0.0 && sampled.b == 0.0) {
////        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
//        // return average of sampled and white
//        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
//    }

    // Sample color should be green if the triangle is the first one and red if the second one

//    return vec4<f32>((sampled.r + 1.0) / 2.0, (sampled.g + 1.0) / 2.0, (sampled.b + 1.0) / 2.0, 1.0);
    return sampled;


//    return vec4<f32>(
//
//        in.vert_pos.x + 0.5,
//        in.vert_pos.y + 0.5,
//        in.vert_pos.z + 0.5,
////        in.clip_position.x / 400.0,
////        in.clip_position.x / 400.0,
////        in.clip_position.x / 400.0,
////        0.3,
////        0.2,
////        0.1,
//        1.0,
//    );
}

