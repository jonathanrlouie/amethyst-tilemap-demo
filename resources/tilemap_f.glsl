#version 150 core

uniform sampler2D TilesheetTexture;

const int TILEMAP_BUF_LENGTH = 4096;

layout (std140) uniform TileMapBuffer {
    vec4 u_Data[TILEMAP_BUF_LENGTH];
};

layout (std140) uniform FragmentArgs {
    vec4 u_WorldSize;
    vec4 u_TilesheetSize;
};

in VertexData {
    vec4 position;
    vec3 normal;
    vec3 tangent;
    vec2 tex_coord;
} vertex;

out vec4 Color;

void main() {

    vec2 texCoord = vertex.tex_coord;

    // base coordinates for the charmap tile of the "nearest" (left/down) vertex.
    vec2 bufTileCoords = floor(texCoord);

    // "raw" offset, expressed as 0.0..1.0, for the offset position of the current
    // fragment
    // need to invert y-coordinates
    vec2 rawUvOffsets = vec2(texCoord.x - bufTileCoords.x, 1.0 - (texCoord.y - bufTileCoords.y));

    vec4 texData;

    if (bufTileCoords.x >= 0.0 && bufTileCoords.x < u_WorldSize.x && bufTileCoords.y >= 0.0 && bufTileCoords.y < u_WorldSize.y) {
        int bufIdx = int((bufTileCoords.y * u_WorldSize.x) + bufTileCoords.x);

        vec4 entry = u_Data[bufIdx];
        vec2 uvCoords = (entry.xy + rawUvOffsets) / u_TilesheetSize.xy;
        texData = texture(TilesheetTexture, uvCoords);
    } else {
        // if we're here it means the buftilecoords are outside the buffer, so let's just show black
        texData = vec4(0.0,0.0,0.0,1.0);
    }

    Color = texData;
}