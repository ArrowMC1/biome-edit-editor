#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include "biomes.h"
#include "generator.h"
#include "globalState.h"

/*typedef union {
    struct {
        uint8_t r;
        uint8_t g;
        uint8_t b;
        uint8_t a;
    };
    uint32_t all;
} Color;*/


uint32_t c_get_biome_colour_cubitect(int biomeID)
{
    switch (biomeID)
    {
        case badlands:                          return 0xc9782fff;
        case badlands_plateau:                  return 0x9b6841ff;
        case bamboo_jungle:                     return 0x3c5e08ff;
        case bamboo_jungle_hills:               return 0x315400ff;
        case beach:                             return 0xb2b3a2ff;
        case birch_forest:                      return 0x3f4a27ff;
        case birch_forest_hills:                return 0x374222ff;
        case cherry_grove:                      return 0x917f6fff;
        case cold_ocean:                        return 0x2d5989ff;
        case dappled_forest:                    return 0x9c5522ff;
        case dark_forest:                       return 0x394922ff;
        case dark_forest_hills:                 return 0x34411aff;
        case deep_cold_ocean:                   return 0x2b4c88ff;
        case deep_dark:                         return 0x092229ff;
        case deep_frozen_ocean:                 return 0x4b78baff;
        case deep_lukewarm_ocean:               return 0x2d7d9cff;
        case deep_ocean:                        return 0x2d5b8eff;
        case deep_warm_ocean:                   return 0x2d8992ff;
        case desert:                            return 0xd3c99bff;
        case desert_hills:                      return 0xbfb588ff;
        case desert_lakes:                      return 0xe7deafff;
        case dripstone_caves:                   return 0x766666ff;
        case eroded_badlands:                   return 0xb36541ff;
        case flower_forest:                     return 0x3d5620ff;
        case forest:                            return 0x36561eff;
        case frozen_ocean:                      return 0x6084cfff;
        case frozen_peaks:                      return 0xb3c5e1ff;
        case frozen_river:                      return 0x6b9dd7ff;
        case giant_spruce_taiga_hills:          return 0x3a3616ff;
        case giant_tree_taiga_hills:            return 0x353517ff;
        case grove:                             return 0x888d83ff;
        case ice_spikes:                        return 0x7ab2f3ff;
        case jagged_peaks:                      return 0xbbbfc2ff;
        case jungle:                            return 0x175100ff;
        case jungle_hills:                      return 0x134900ff;
        case lukewarm_ocean:                    return 0x408498ff;
        case lush_caves:                        return 0x5c7552ff;
        case mangrove_swamp:                    return 0x3c4618ff;
        case meadow:                            return 0x426d2dff;
        case modified_badlands_plateau:         return 0xa86c4dff;
        case modified_gravelly_mountains:       return 0x8f978dff;
        case modified_jungle:                   return 0x1e590bff;
        case modified_jungle_edge:              return 0x326e1aff;
        case modified_wooded_badlands_plateau:  return 0x916244ff;
        case mountain_edge:                     return 0x656859ff;
        case mushroom_field_shore:              return 0x856369ff;
        case mushroom_fields:                   return 0x92665dff;
        case ocean:                             return 0x315f8fff;
        case old_growth_birch_forest:           return 0x3a5025ff;
        case old_growth_pine_taiga:             return 0x3c3c1cff;
        case old_growth_spruce_taiga:           return 0x403c1cff;
        case pale_garden:                       return 0x555751ff;
        case plains:                            return 0x446b21ff;
        case rainforest:                        return 0x144f00ff;
        case river:                             return 0x2d608cff;
        case savanna:                           return 0x66612dff;
        case savanna_plateau:                   return 0x716c37ff;
        case seasonal_forest:                   return 0x245600ff;
        case shattered_savanna_plateau:         return 0x776c4cff;
        case shrubland:                         return 0x465a1eff;
        case snowy_beach:                       return 0xd5dde2ff;
        case snowy_mountains:                   return 0xadb6b6ff;
        case snowy_plains:                      return 0xc1cacaff;
        case snowy_slopes:                      return 0xf2f7f7ff;
        case snowy_taiga:                       return 0x506d50ff;
        case snowy_taiga_hills:                 return 0x446244ff;
        case snowy_taiga_mountains:             return 0x446244ff;
        case sparse_jungle:                     return 0x276400ff;
        case stony_peaks:                       return 0x919190ff;
        case stony_shore:                       return 0x626785ff;
        case sulfur_caves:                      return 0x000000ff;
        case sunflower_plains:                  return 0x4b7033ff;
        case swamp:                             return 0x35492aff;
        case swamp_hills:                       return 0x344122ff;
        case taiga:                             return 0x263e1cff;
        case taiga_hills:                       return 0x213700ff;
        case taiga_mountains:                   return 0x193100ff;
        case tall_birch_hills:                  return 0x3d4724ff;
        case warm_ocean:                        return 0x489097ff;
        case windswept_forest:                  return 0x3b5a2dff;
        case windswept_gravelly_hills:          return 0x81897fff;
        case windswept_hills:                   return 0x5e7665ff;
        case windswept_savanna:                 return 0x6c6242ff;
        case wooded_badlands:                   return 0xa35739ff;
        case wooded_hills:                      return 0x2a4d15ff;

        case end_barrens:                       return 0xff00ffff;

        default:
            return 0x000000ff;
    }
}

uint32_t c_get_biome_colour_amidst(int biomeID)
{
    switch (biomeID)
    {
        case ocean:                             return 0x000070ff;
        case plains:                            return 0x8db360ff;
        case desert:                            return 0xfa9418ff;
        case windswept_hills:                   return 0x606060ff;
        case forest:                            return 0x056621ff;
        case taiga:                             return 0x0b6a5fff;
        case swamp:                             return 0x07f9b2ff;
        case river:                             return 0x0000ffff;
        case nether_wastes:                     return 0x572526ff;
        case the_end:                           return 0x8080ffff;
        case frozen_ocean:                      return 0x7070d6ff;
        case frozen_river:                      return 0xa0a0ffff;
        case snowy_plains:                      return 0xffffffff;
        case snowy_mountains:                   return 0xa0a0a0ff;
        case mushroom_fields:                   return 0xff00ffff;
        case mushroom_field_shore:              return 0xa000ffff;
        case beach:                             return 0xfade55ff;
        case desert_hills:                      return 0xd25f12ff;
        case wooded_hills:                      return 0x22551cff;
        case taiga_hills:                       return 0x163933ff;
        case mountain_edge:                     return 0x72789aff;
        case jungle:                            return 0x507b0aff;
        case jungle_hills:                      return 0x2c4205ff;
        case sparse_jungle:                     return 0x60930fff;
        case deep_ocean:                        return 0x000030ff;
        case stony_shore:                       return 0xa2a284ff;
        case snowy_beach:                       return 0xfaf0c0ff;
        case birch_forest:                      return 0x307444ff;
        case birch_forest_hills:                return 0x1f5f32ff;
        case dark_forest:                       return 0x40511aff;
        case snowy_taiga:                       return 0x31554aff;
        case snowy_taiga_hills:                 return 0x243f36ff;
        case old_growth_pine_taiga:             return 0x596651ff;
        case giant_tree_taiga_hills:            return 0x454f3eff;
        case windswept_forest:                  return 0x5b7352ff;
        case savanna:                           return 0xbdb25fff;
        case savanna_plateau:                   return 0xa79d64ff;
        case badlands:                          return 0xd94515ff;
        case wooded_badlands:                   return 0xb09765ff;
        case badlands_plateau:                  return 0xca8c65ff;
        case small_end_islands:                 return 0x4b4babff;
        case end_midlands:                      return 0xc9c959ff;
        case end_highlands:                     return 0xb5b536ff;
        case end_barrens:                       return 0x7070ccff;
        case warm_ocean:                        return 0x0000acff;
        case lukewarm_ocean:                    return 0x000090ff;
        case cold_ocean:                        return 0x202070ff;
        case deep_warm_ocean:                   return 0x000050ff;
        case deep_lukewarm_ocean:               return 0x000040ff;
        case deep_cold_ocean:                   return 0x202038ff;
        case deep_frozen_ocean:                 return 0x404090ff;
        case seasonal_forest:                   return 0x2f560fff;
        case rainforest:                        return 0x47840eff;
        case shrubland:                         return 0x789e31ff;
        case the_void:                          return 0x000000ff;
        case sunflower_plains:                  return 0xb5db88ff;
        case desert_lakes:                      return 0xffbc40ff;
        case windswept_gravelly_hills:          return 0x888888ff;
        case flower_forest:                     return 0x2d8e49ff;
        case taiga_mountains:                   return 0x339287ff;
        case swamp_hills:                       return 0x2fffdaff;
        case ice_spikes:                        return 0xb4dcdcff;
        case modified_jungle:                   return 0x78a332ff;
        case modified_jungle_edge:              return 0x88bb37ff;
        case old_growth_birch_forest:           return 0x589c6cff;
        case tall_birch_hills:                  return 0x47875aff;
        case dark_forest_hills:                 return 0x687942ff;
        case snowy_taiga_mountains:             return 0x597d72ff;
        case old_growth_spruce_taiga:           return 0x818e79ff;
        case giant_spruce_taiga_hills:          return 0x6d7766ff;
        case modified_gravelly_mountains:       return 0x839b7aff;
        case windswept_savanna:                 return 0xe5da87ff;
        case shattered_savanna_plateau:         return 0xcfc58cff;
        case eroded_badlands:                   return 0xff6d3dff;
        case modified_wooded_badlands_plateau:  return 0xd8bf8dff;
        case modified_badlands_plateau:         return 0xf2b48dff;
        case bamboo_jungle:                     return 0x849500ff;
        case bamboo_jungle_hills:               return 0x5c6c04ff;
        case soul_sand_valley:                  return 0x4d3a2eff;
        case crimson_forest:                    return 0x981a11ff;
        case warped_forest:                     return 0x49907bff;
        case basalt_deltas:                     return 0x645f63ff;
        case dripstone_caves:                   return 0x4e3012ff;
        case lush_caves:                        return 0x283c00ff;
        case meadow:                            return 0x60a445ff;
        case grove:                             return 0x477268ff;
        case snowy_slopes:                      return 0xc4c4c4ff;
        case jagged_peaks:                      return 0xdcdcc8ff;
        case frozen_peaks:                      return 0xb0b3ceff;
        case stony_peaks:                       return 0x7b8f74ff;
        case deep_dark:                         return 0x031f29ff;
        case mangrove_swamp:                    return 0x2ccc8eff;
        case cherry_grove:                      return 0xff91c8ff;
        case pale_garden:                       return 0x696d95ff;
        case sulfur_caves:                      return 0xc8d232ff;
        case dappled_forest:                    return 0xeb9234ff;

        default:
            return 0x000000ff;
    }
}

void c_set_global_shift_noise(bool val){
    shiftNoise = val;
}

void c_set_global_write_mode(bool val){
    writeMode = val;
}

void c_set_global_main_map(uint32_t* ptr){
    p_main_map = ptr;
}


void c_set_global_height_map(uint8_t* ptr){
    p_height_map = ptr;
}

void c_set_global_noise_maps(uint16_t* _p_t, uint16_t* _p_h, uint16_t* _p_c, uint16_t* _p_e, uint16_t* _p_w){
    p_t = _p_t;
    p_h = _p_h;
    p_c = _p_c;
    p_e = _p_e;
    p_w = _p_w;

}

void c_get_biome_colour_square_parallel(
    int64_t seed,
    int start_x,
    int start_z,
    int length,
    int height,
    int num_threads,
    int thread_no)
{
    if (num_threads <= 0 ||
        thread_no < 0 ||
        thread_no >= num_threads)
    {
        fprintf(stderr, "Invalid thread configuration\n");
        abort();
    }

    Generator g;
    setupGenerator(&g, MC_26_3, 0);
    applySeed(&g, DIM_OVERWORLD, seed);


    SurfaceNoise sn;
    initSurfaceNoise(&sn, DIM_OVERWORLD, seed);

    // Assign this thread its horizontal strip.
    int z0 = start_z + (thread_no * height) / num_threads;
    int z1 = start_z + ((thread_no + 1) * height) / num_threads;

    int thread_height = z1 - z0;

    if (thread_height == 0 || length <= 0)
        return;

    // Generate the entire rectangle in one call.
    Range r = {
        4,
        start_x,
        z0,
        length,
        thread_height,
        320,
        1
    };

    int *ids = allocCache(&g, r);

    if (!ids)
    {
        fprintf(stderr, "Failed to allocate biome cache\n");
        abort();
    }

    int result = genBiomes(&g, ids, r);

    if (result == 0)
    {
        for (int z = 0; z < thread_height; z++)
        {
            for (int x = 0; x < length; x++)
            {
                int index = z * length + x;

                int world_x = start_x + x;
                int world_z = z0 + z;

                p_main_map[(uint64_t)z * length + (uint64_t)x
                        + (uint64_t)(z0 - start_z) * length] =
                    c_get_biome_colour_amidst(ids[index]);


                float height;
                int result = mapApproxHeight(
                    &height,
                    NULL,
                    &g,
                    &sn,
                    world_x, world_z, // X, Z
                    1, 1  // Width, height
                );

                if (result != 0)
                {
                    height = 0.0f;
                }

                height = fmaxf(0.0f, fminf(height, 255.0f));

                uint8_t grayscale = (uint8_t)height;

                p_height_map[(uint64_t)z * length + (uint64_t)x
                        + (uint64_t)(z0 - start_z) * length] =
                    grayscale;

                // Your per-pixel operation here:
                // world_x, world_z
            }
        }
    }

    free(ids);
}
