#version 150 core

uniform sampler2D albedo;

const int TILEMAP_BUF_LENGTH = 4096;

layout (std140) uniform b_TileMap {
    vec4 u_Data[TILEMAP_BUF_LENGTH];
};

layout (std140) uniform FragmentArgs {
    vec4 u_WorldSize;
    vec4 u_TilesheetSize;
    vec2 u_TileOffsets;
};

in VertexData {
    vec4 position;
    vec3 normal;
    vec3 tangent;
    vec2 tex_coord;
} vertex;

out vec4 color;

void main() {

    // apply offset to v_BufPos
    vec2 offset_bufpos = vertex.tex_coord + (u_TileOffsets / u_WorldSize.zz);

    // base coordinates for the charmap tile of the "nearest" (left/down) vertex.
    vec2 bufTileCoords = floor(offset_bufpos);

    // "raw" offset, expressed as 0.0..1.0, for the offset position of the current
    // fragment
    vec2 rawUvOffsets = vec2(offset_bufpos.x - bufTileCoords.x, offset_bufpos.y - bufTileCoords.y);

    vec4 texData;

    if (bufTileCoords.x >= 0.0 && bufTileCoords.x < u_WorldSize.x && bufTileCoords.y >= 0.0 && bufTileCoords.y < u_WorldSize.y) {
        int bufIdx = int((bufTileCoords.y * u_WorldSize.x) + bufTileCoords.x);


        if (bufIdx > -1 && bufIdx < 4096) {
            vec4 entry = u_Data[bufIdx];
            vec2 uvCoords = (entry.xy + rawUvOffsets) / u_TilesheetSize.xy;
            texData = texture(albedo, uvCoords);
        } else {
            texData = vec4(0.0,1.0,0.0,1.0);
        }
    } else {
        // if we're here it means the buftilecoords are outside the buffer, so let's just show red
        texData = vec4(1.0,0.0,0.0,1.0);
    }

    color = texData;
}