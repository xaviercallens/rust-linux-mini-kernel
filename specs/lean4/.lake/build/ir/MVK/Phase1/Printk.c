// Lean compiler output
// Module: MVK.Phase1.Printk
// Imports: public import Init public import MVK.Phase2.Common
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
lean_object* lean_nat_to_int(lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_Printk_SUCCESS___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_Printk_SUCCESS___closed__0;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_Printk_SUCCESS;
LEAN_EXPORT uint16_t lp_mvk__specs_MVK_Phase1_Printk_SERIAL__PORT;
LEAN_EXPORT uint16_t lp_mvk__specs_MVK_Phase1_Printk_UART__DATA;
LEAN_EXPORT uint16_t lp_mvk__specs_MVK_Phase1_Printk_UART__IER;
LEAN_EXPORT uint16_t lp_mvk__specs_MVK_Phase1_Printk_UART__IIR;
LEAN_EXPORT uint16_t lp_mvk__specs_MVK_Phase1_Printk_UART__LCR;
LEAN_EXPORT uint16_t lp_mvk__specs_MVK_Phase1_Printk_UART__MCR;
LEAN_EXPORT uint16_t lp_mvk__specs_MVK_Phase1_Printk_UART__LSR;
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_Printk_UART__BAUD__DIVISOR__LO;
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_Printk_UART__BAUD__DIVISOR__HI;
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_Printk_UART__LCR__DLAB;
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_Printk_UART__LCR__8N1;
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_Printk_UART__FCR__ENABLE;
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_Printk_UART__MCR__DTR__RTS;
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_Printk_UART__LSR__TX__READY;
static const lean_string_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "{ "};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__0_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 12, .m_capacity = 12, .m_length = 11, .m_data = "initialized"};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__1_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__1_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__2_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__3_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = " := "};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__4 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__4_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__4_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__5_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__3_value),((lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__6 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__6_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__7;
static const lean_string_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = ","};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__8 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__8_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__9_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__8_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__9 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__9_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__10_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 14, .m_capacity = 14, .m_length = 13, .m_data = "bytes_written"};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__10 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__10_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__10_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__11 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__11_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__12_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__12;
static const lean_string_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__13_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = " }"};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__13 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__13_value;
lean_object* lean_string_length(lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__14_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__14;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__15_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__15;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__16_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__16 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__16_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__17_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__13_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__17 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__17_value;
lean_object* l_Bool_repr___redArg(uint8_t);
lean_object* l_Nat_reprFast(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_Printk_initial__printk__state___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 8, .m_other = 1, .m_tag = 0}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),LEAN_SCALAR_PTR_LITERAL(0, 0, 0, 0, 0, 0, 0, 0)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_Printk_initial__printk__state___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_initial__printk__state___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase1_Printk_initial__printk__state = (const lean_object*)&lp_mvk__specs_MVK_Phase1_Printk_initial__printk__state___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_Printk_printk__exit();
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_Printk_printk__exit___boxed(lean_object*);
static lean_object* _init_lp_mvk__specs_MVK_Phase1_Printk_SUCCESS___closed__0(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_Printk_SUCCESS(void) {
_start:
{
lean_object* x_1; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_Printk_SUCCESS___closed__0, &lp_mvk__specs_MVK_Phase1_Printk_SUCCESS___closed__0_once, _init_lp_mvk__specs_MVK_Phase1_Printk_SUCCESS___closed__0);
return x_1;
}
}
static uint16_t _init_lp_mvk__specs_MVK_Phase1_Printk_SERIAL__PORT(void) {
_start:
{
uint16_t x_1; 
x_1 = 1016;
return x_1;
}
}
static uint16_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__DATA(void) {
_start:
{
uint16_t x_1; 
x_1 = 1016;
return x_1;
}
}
static uint16_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__IER(void) {
_start:
{
uint16_t x_1; 
x_1 = 1017;
return x_1;
}
}
static uint16_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__IIR(void) {
_start:
{
uint16_t x_1; 
x_1 = 1018;
return x_1;
}
}
static uint16_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__LCR(void) {
_start:
{
uint16_t x_1; 
x_1 = 1019;
return x_1;
}
}
static uint16_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__MCR(void) {
_start:
{
uint16_t x_1; 
x_1 = 1020;
return x_1;
}
}
static uint16_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__LSR(void) {
_start:
{
uint16_t x_1; 
x_1 = 1021;
return x_1;
}
}
static uint8_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__BAUD__DIVISOR__LO(void) {
_start:
{
uint8_t x_1; 
x_1 = 12;
return x_1;
}
}
static uint8_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__BAUD__DIVISOR__HI(void) {
_start:
{
uint8_t x_1; 
x_1 = 0;
return x_1;
}
}
static uint8_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__LCR__DLAB(void) {
_start:
{
uint8_t x_1; 
x_1 = 128;
return x_1;
}
}
static uint8_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__LCR__8N1(void) {
_start:
{
uint8_t x_1; 
x_1 = 3;
return x_1;
}
}
static uint8_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__FCR__ENABLE(void) {
_start:
{
uint8_t x_1; 
x_1 = 199;
return x_1;
}
}
static uint8_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__MCR__DTR__RTS(void) {
_start:
{
uint8_t x_1; 
x_1 = 11;
return x_1;
}
}
static uint8_t _init_lp_mvk__specs_MVK_Phase1_Printk_UART__LSR__TX__READY(void) {
_start:
{
uint8_t x_1; 
x_1 = 32;
return x_1;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__7(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(15u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__12(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(17u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__14(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__0));
x_2 = lean_string_length(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__15(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__14, &lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__14_once, _init_lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__14);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg(lean_object* x_1) {
_start:
{
uint8_t x_2; 
x_2 = !lean_is_exclusive(x_1);
if (x_2 == 0)
{
uint8_t x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; uint8_t x_10; lean_object* x_11; lean_object* x_12; lean_object* x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; lean_object* x_22; lean_object* x_23; lean_object* x_24; lean_object* x_25; lean_object* x_26; lean_object* x_27; lean_object* x_28; lean_object* x_29; lean_object* x_30; lean_object* x_31; 
x_3 = lean_ctor_get_uint8(x_1, sizeof(void*)*1);
x_4 = lean_ctor_get(x_1, 0);
x_5 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__5));
x_6 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__6));
x_7 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__7, &lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__7_once, _init_lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__7);
x_8 = l_Bool_repr___redArg(x_3);
x_9 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_9, 0, x_7);
lean_ctor_set(x_9, 1, x_8);
x_10 = 0;
lean_ctor_set_tag(x_1, 6);
lean_ctor_set(x_1, 0, x_9);
lean_ctor_set_uint8(x_1, sizeof(void*)*1, x_10);
x_11 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_11, 0, x_6);
lean_ctor_set(x_11, 1, x_1);
x_12 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__9));
x_13 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_13, 0, x_11);
lean_ctor_set(x_13, 1, x_12);
x_14 = lean_box(1);
x_15 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_15, 0, x_13);
lean_ctor_set(x_15, 1, x_14);
x_16 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__11));
x_17 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_17, 0, x_15);
lean_ctor_set(x_17, 1, x_16);
x_18 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_18, 0, x_17);
lean_ctor_set(x_18, 1, x_5);
x_19 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__12, &lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__12_once, _init_lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__12);
x_20 = l_Nat_reprFast(x_4);
x_21 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_21, 0, x_20);
x_22 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_22, 0, x_19);
lean_ctor_set(x_22, 1, x_21);
x_23 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_23, 0, x_22);
lean_ctor_set_uint8(x_23, sizeof(void*)*1, x_10);
x_24 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_24, 0, x_18);
lean_ctor_set(x_24, 1, x_23);
x_25 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__15, &lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__15_once, _init_lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__15);
x_26 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__16));
x_27 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_27, 0, x_26);
lean_ctor_set(x_27, 1, x_24);
x_28 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__17));
x_29 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_29, 0, x_27);
lean_ctor_set(x_29, 1, x_28);
x_30 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_30, 0, x_25);
lean_ctor_set(x_30, 1, x_29);
x_31 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_31, 0, x_30);
lean_ctor_set_uint8(x_31, sizeof(void*)*1, x_10);
return x_31;
}
else
{
uint8_t x_32; lean_object* x_33; lean_object* x_34; lean_object* x_35; lean_object* x_36; lean_object* x_37; lean_object* x_38; uint8_t x_39; lean_object* x_40; lean_object* x_41; lean_object* x_42; lean_object* x_43; lean_object* x_44; lean_object* x_45; lean_object* x_46; lean_object* x_47; lean_object* x_48; lean_object* x_49; lean_object* x_50; lean_object* x_51; lean_object* x_52; lean_object* x_53; lean_object* x_54; lean_object* x_55; lean_object* x_56; lean_object* x_57; lean_object* x_58; lean_object* x_59; lean_object* x_60; lean_object* x_61; 
x_32 = lean_ctor_get_uint8(x_1, sizeof(void*)*1);
x_33 = lean_ctor_get(x_1, 0);
lean_inc(x_33);
lean_dec(x_1);
x_34 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__5));
x_35 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__6));
x_36 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__7, &lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__7_once, _init_lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__7);
x_37 = l_Bool_repr___redArg(x_32);
x_38 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_38, 0, x_36);
lean_ctor_set(x_38, 1, x_37);
x_39 = 0;
x_40 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_40, 0, x_38);
lean_ctor_set_uint8(x_40, sizeof(void*)*1, x_39);
x_41 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_41, 0, x_35);
lean_ctor_set(x_41, 1, x_40);
x_42 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__9));
x_43 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_43, 0, x_41);
lean_ctor_set(x_43, 1, x_42);
x_44 = lean_box(1);
x_45 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_45, 0, x_43);
lean_ctor_set(x_45, 1, x_44);
x_46 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__11));
x_47 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_47, 0, x_45);
lean_ctor_set(x_47, 1, x_46);
x_48 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_48, 0, x_47);
lean_ctor_set(x_48, 1, x_34);
x_49 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__12, &lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__12_once, _init_lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__12);
x_50 = l_Nat_reprFast(x_33);
x_51 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_51, 0, x_50);
x_52 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_52, 0, x_49);
lean_ctor_set(x_52, 1, x_51);
x_53 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_53, 0, x_52);
lean_ctor_set_uint8(x_53, sizeof(void*)*1, x_39);
x_54 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_54, 0, x_48);
lean_ctor_set(x_54, 1, x_53);
x_55 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__15, &lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__15_once, _init_lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__15);
x_56 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__16));
x_57 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_57, 0, x_56);
lean_ctor_set(x_57, 1, x_54);
x_58 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg___closed__17));
x_59 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_59, 0, x_57);
lean_ctor_set(x_59, 1, x_58);
x_60 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_60, 0, x_55);
lean_ctor_set(x_60, 1, x_59);
x_61 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_61, 0, x_60);
lean_ctor_set_uint8(x_61, sizeof(void*)*1, x_39);
return x_61;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___redArg(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase1_Printk_instReprPrintkState_repr(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_Printk_printk__exit() {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = lean_box(0);
x_3 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_3, 0, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_Printk_printk__exit___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_Printk_printk__exit();
return x_2;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_mvk__specs_MVK_Phase2_Common(uint8_t builtin);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_mvk__specs_MVK_Phase1_Printk(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mvk__specs_MVK_Phase2_Common(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_mvk__specs_MVK_Phase1_Printk_SUCCESS = _init_lp_mvk__specs_MVK_Phase1_Printk_SUCCESS();
lean_mark_persistent(lp_mvk__specs_MVK_Phase1_Printk_SUCCESS);
lp_mvk__specs_MVK_Phase1_Printk_SERIAL__PORT = _init_lp_mvk__specs_MVK_Phase1_Printk_SERIAL__PORT();
lp_mvk__specs_MVK_Phase1_Printk_UART__DATA = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__DATA();
lp_mvk__specs_MVK_Phase1_Printk_UART__IER = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__IER();
lp_mvk__specs_MVK_Phase1_Printk_UART__IIR = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__IIR();
lp_mvk__specs_MVK_Phase1_Printk_UART__LCR = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__LCR();
lp_mvk__specs_MVK_Phase1_Printk_UART__MCR = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__MCR();
lp_mvk__specs_MVK_Phase1_Printk_UART__LSR = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__LSR();
lp_mvk__specs_MVK_Phase1_Printk_UART__BAUD__DIVISOR__LO = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__BAUD__DIVISOR__LO();
lp_mvk__specs_MVK_Phase1_Printk_UART__BAUD__DIVISOR__HI = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__BAUD__DIVISOR__HI();
lp_mvk__specs_MVK_Phase1_Printk_UART__LCR__DLAB = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__LCR__DLAB();
lp_mvk__specs_MVK_Phase1_Printk_UART__LCR__8N1 = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__LCR__8N1();
lp_mvk__specs_MVK_Phase1_Printk_UART__FCR__ENABLE = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__FCR__ENABLE();
lp_mvk__specs_MVK_Phase1_Printk_UART__MCR__DTR__RTS = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__MCR__DTR__RTS();
lp_mvk__specs_MVK_Phase1_Printk_UART__LSR__TX__READY = _init_lp_mvk__specs_MVK_Phase1_Printk_UART__LSR__TX__READY();
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
