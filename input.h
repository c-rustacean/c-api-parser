/**
* Some phony file template
*
* version: -3.0 :)
*/

#define configFOO 1
#define configBAR // no value, just defined

typedef uint8_t smallFlag_t;

typedef struct a_tag {
    uint8_t u8_1;
    uint8_t u8_2;
    uint16_t u16_3;
} a_t;

typedef struct {
    uint8_t variant;
    union {
        uint32_t variant1;
        struct {
            uint16_t a;
            uint32_t b;
        } variant2;
        uint32_t variant3;
    } var;
} poormansTaggedUnion;

/* some comment */
