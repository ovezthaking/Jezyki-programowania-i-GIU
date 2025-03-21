dnl  mpn_redc_1 - from x86_64/coreinhm directory for fat binary.
dnl  Generated by configure - DO NOT EDIT.

define(OPERATION_redc_1)
define(__gmpn_redc_1, __gmpn_redc_1_coreinhm)
define(__gmpn_redc_1c,__gmpn_redc_1c_coreinhm)
define(__gmpn_preinv_redc_1,__gmpn_preinv_redc_1_coreinhm)
define(__gmpn_redc_1_cps,__gmpn_redc_1_cps_coreinhm)

dnl  For k6 and k7 gcd_1 calling their corresponding mpn_modexact_1_odd
ifdef(`__gmpn_modexact_1_odd',,
`define(__gmpn_modexact_1_odd,__gmpn_modexact_1_odd_coreinhm)')

define(MUL_TOOM22_THRESHOLD,18)
define(MUL_TOOM33_THRESHOLD,59)
define(SQR_TOOM2_THRESHOLD,28)
define(SQR_TOOM3_THRESHOLD,98)
define(BMOD_1_TO_MOD_1_THRESHOLD,17)

include(../../gmp-src/mpn/x86_64/coreinhm/redc_1.asm)

