#pragma version(1)
#pragma rs_fp_relaxed
#pragma rs java_package_name(com.example.q.renderscriptexample)

#include "rs_debug.rsh"

int32_t histo[256];
float remapArray[256];
int size;

static float bound (float val) {
    float m = fmax(0.0f, val);
    return fmin(1.0f, m);
}

uchar4 __attribute__((kernel)) root(uchar4 in, uint32_t x, uint32_t y) {
    float4 f4 = rsUnpackColor8888(in);

    float Y = 0.299f * f4.r + 0.587f * f4.g + 0.114f * f4.b;
    float U = ((0.492f * (f4.b - Y))+1)/2;
    float V = ((0.877f * (f4.r - Y))+1)/2;

    int32_t val = Y * 255;
    rsAtomicInc(&histo[val]);

    return rsPackColorTo8888(Y, U, V, f4.a);
}

uchar4 __attribute__((kernel)) remaptoRGB(uchar4 in, uint32_t x, uint32_t y) {
    float4 f4 = rsUnpackColor8888(in);

    float Y = f4.r;
    int32_t val = Y * 255;
    Y = remapArray[val];

    float U = (2*f4.g)-1;
    float V = (2*f4.b)-1;

    float red = bound(Y + 1.14f * V);
    float green = bound(Y - 0.395f * U - 0.581f * V);
    float blue = bound(Y + 2.033f * U);

    return rsPackColorTo8888(red, green, blue, f4.a);
}

void init() {
    for (int i = 0; i < 256; i++) {
        histo[i] = 0;
        remapArray[i] = 0.0f;
    }
}

void createRemapArray() {
    float sum = 0;
    for (int i = 0; i < 256; i++) {
        sum += histo[i];
        remapArray[i] = sum / (size);
    }
}
