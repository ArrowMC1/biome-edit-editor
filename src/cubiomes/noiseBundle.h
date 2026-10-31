#ifndef NOISE_BUNDLE_H
#define NOISE_BUNDLE_H

typedef struct{
    float t;
    float h;
    float c;
    float e;
    float w;

}NoiseBundle;

static inline NoiseBundle newNoiseBundle(float t, float h, float c, float e, float w)
{
    return (NoiseBundle){t, h, c, e, w};
}

#endif