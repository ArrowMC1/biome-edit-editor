#include "globalState.h"

uint32_t* p_main_map = NULL;
uint8_t* p_height_map = NULL;
uint16_t* p_t = NULL;
uint16_t* p_h = NULL;
uint16_t* p_c = NULL;
uint16_t* p_e = NULL;
uint16_t* p_w = NULL;
bool shiftNoise = true;
bool writeMode = true;

int imgLength = 2048;
int imgHeight = 2048;