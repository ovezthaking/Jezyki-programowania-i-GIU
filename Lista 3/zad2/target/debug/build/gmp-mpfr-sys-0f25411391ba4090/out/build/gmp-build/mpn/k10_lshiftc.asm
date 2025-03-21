dnl  mpn_lshiftc - from x86_64/k10 directory for fat binary.
dnl  Generated by configure - DO NOT EDIT.

define(OPERATION_lshiftc)
define(__gmpn_lshiftc, __gmpn_lshiftc_k10)
define(__gmpn_lshiftcc,__gmpn_lshiftcc_k10)
define(__gmpn_preinv_lshiftc,__gmpn_preinv_lshiftc_k10)
define(__gmpn_lshiftc_cps,__gmpn_lshiftc_cps_k10)

dnl  For k6 and k7 gcd_1 calling their corresponding mpn_modexact_1_odd
ifdef(`__gmpn_modexact_1_odd',,
`define(__gmpn_modexact_1_odd,__gmpn_modexact_1_odd_k10)')

define(MUL_TOOM22_THRESHOLD,27)
define(MUL_TOOM33_THRESHOLD,81)
define(SQR_TOOM2_THRESHOLD,30)
define(SQR_TOOM3_THRESHOLD,117)
define(BMOD_1_TO_MOD_1_THRESHOLD,15)

include(../../gmp-src/mpn/x86_64/k10/lshiftc.asm)

