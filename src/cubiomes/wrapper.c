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


uint32_t c_get_biome_colour(int biomeID)
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

void c_set_global_shift_noise(bool val){
    shiftNoise = val;
}

void c_set_global_main_map(uint32_t* ptr){
    p_main_map = ptr;
}

void c_set_global_noise_maps(uint16_t* _p_t, uint16_t* _p_h, uint16_t* _p_c, uint16_t* _p_e, uint16_t* _p_w){
    p_t = _p_t;
    p_h = _p_h;
    p_c = _p_c;
    p_e = _p_e;
    p_w = _p_w;

}

// Thread No's MUST be between 0 and num_threads - 1 (Optimised with an LLM - TODO: review correctness)
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

    /*
     * Find a grid that is as close to square as possible.
     *
     * For example:
     *   24 threads -> 4 x 6
     *   16 threads -> 4 x 4
     *   12 threads -> 3 x 4
     *    8 threads -> 2 x 4
     */
    int columns = 1;
    int rows = num_threads;

    for (int i = 1; i * i <= num_threads; i++)
    {
        if (num_threads % i == 0)
        {
            rows = i;
            columns = num_threads / i;
        }
    }

    /*
     * thread_no identifies a rectangle in row-major order.
     *
     * Example with 24:
     *
     *  0   1   2   3   4   5
     *  6   7   8   9  10  11
     * 12  13  14  15  16  17
     * 18  19  20  21  22  23
     */

    int row = thread_no / columns;
    int column = thread_no % columns;

    /*
     * Calculate this rectangle's bounds.
     *
     * Using this formula distributes remainder pixels naturally.
     */
    int x0 = start_x + (column * length) / columns;
    int x1 = start_x + ((column + 1) * length) / columns;

    int z0 = start_z + (row * height) / rows;
    int z1 = start_z + ((row + 1) * height) / rows;

    for (int z = z0; z < z1; z++)
    {
        for (int x = x0; x < x1; x++)
        {
            int local_x = x - start_x;
            int local_z = z - start_z;

            uint64_t index =
                (uint64_t)local_z * (uint64_t)length +
                (uint64_t)local_x;

            int biomeID = getBiomeAt(&g, 4, x, 320, z);

            p_main_map[index] = c_get_biome_colour(biomeID);
        }
    }
}