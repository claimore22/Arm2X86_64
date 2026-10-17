# AI_PROGRESS — worker heartbeat + ledger

<!-- Supervisor parses: IN_PROGRESS, COMPLETED, HEARTBEAT, CHECKPOINT, NEED_CONTINUE -->
<!-- Worker: update HEARTBEAT at least every 5 min while active. Atomic write. -->

IN_PROGRESS:

## Ledger (append-only)

COMPLETED: ADDG | tests/addg.rs | 2026-10-04T13:45:00Z
COMPLETED: ADDPT | tests/addpt.rs | 2026-10-04T14:05:00Z
COMPLETED: BLRAA, BLRAAZ, BLRAB, BLRABZ | semantic_ir/tests/blraa.rs | 2026-10-05T05:09:03Z
COMPLETED: BRAA, BRAAZ, BRAB, BRABZ | semantic_ir/tests/braa.rs | 2026-10-05T05:09:49Z
COMPLETED: BRK | semantic_ir/tests/brk.rs | 2026-10-05T05:13:42Z
COMPLETED: BTI | semantic_ir/tests/bti.rs | 2026-10-05T05:28:15Z
COMPLETED: CAS, CASA, CASAL, CASL | semantic_ir/tests/cas.rs | 2026-10-05T05:31:30Z
COMPLETED: CASB, CASAB, CASALB, CASLB | semantic_ir/tests/casb.rs | 2026-10-05T05:32:35Z
COMPLETED: CASH, CASAH, CASALH, CASLH | semantic_ir/tests/cash.rs | 2026-10-05T05:33:20Z
COMPLETED: CASP, CASPA, CASPAL, CASPL | semantic_ir/tests/casp.rs | 2026-10-05T05:34:25Z
COMPLETED: CASPT, CASPAT, CASPALT, CASPLT | semantic_ir/tests/caspt.rs | 2026-10-05T05:35:15Z
COMPLETED: CAST, CASAT, CASALT, CASLT | semantic_ir/tests/cast.rs | 2026-10-05T05:35:55Z
COMPLETED: CB<cc> (immediate) | semantic_ir/tests/cbcc_imm.rs | 2026-10-05T05:36:50Z
COMPLETED: CB<cc> (register) | semantic_ir/tests/cbcc_regs.rs | 2026-10-05T05:37:35Z
COMPLETED: CBB<cc>, CBBLE, CBBLO, CBBLS, CBBLT | semantic_ir/tests/cbbcc.rs | 2026-10-05T05:38:10Z
COMPLETED: CBGE (immediate), CBHS (immediate), CBLE (immediate), CBLS (immediate) | semantic_ir/tests/cbcc_imm.rs | 2026-10-05T05:38:10Z
COMPLETED: CBLE (register), CBLO (register), CBLS (register), CBLT (register) | semantic_ir/tests/cbcc_regs.rs | 2026-10-05T05:38:10Z
COMPLETED: CBH<cc>, CBHLE, CBHLO, CBHLS, CBHLT | semantic_ir/tests/cbhcc.rs | 2026-10-05T05:40:00Z
COMPLETED: CBNZ | semantic_ir/tests/cbnz.rs | 2026-10-05T06:47:11Z
COMPLETED: CBZ | semantic_ir/tests/cbz.rs | 2026-10-05T06:47:11Z
COMPLETED: CFINV | semantic_ir/tests/cfinv.rs | 2026-10-05T06:49:37Z
COMPLETED: CFP | semantic_ir/tests/cfp.rs | 2026-10-05T06:52:37Z
COMPLETED: CHKFEAT | semantic_ir/tests/chkfeat.rs | 2026-10-05T06:56:49Z
COMPLETED: CINC | semantic_ir/tests/cinc.rs | 2026-10-05T03:14:20Z
COMPLETED: CINV | semantic_ir/tests/cinv.rs | 2026-10-05T03:17:14Z
COMPLETED: CLRBHB | semantic_ir/tests/clrbhb.rs | 2026-10-05T03:20:12Z
COMPLETED: CLREX | semantic_ir/tests/clrex.rs | 2026-10-05T03:21:26Z
COMPLETED: CLS | semantic_ir/tests/cls.rs | 2026-10-05T03:22:37Z
COMPLETED: CLZ | semantic_ir/tests/clz.rs | 2026-10-05T03:23:15Z
COMPLETED: CMN (extended register) | semantic_ir/tests/cmn_ext_reg.rs | 2026-10-05T03:25:09Z
COMPLETED: CMN (immediate) | semantic_ir/tests/cmn_imm.rs | 2026-10-05T03:29:16Z
COMPLETED: CMN (shifted register) | semantic_ir/tests/cmn_shifted.rs | 2026-10-05T03:32:57Z
COMPLETED: CMP (extended register) | semantic_ir/tests/cmp_ext_reg.rs | 2026-10-05T03:40:00Z
COMPLETED: CMP (immediate) | semantic_ir/tests/cmp_imm.rs | 2026-10-05T03:45:00Z
COMPLETED: CMP (shifted register) | semantic_ir/tests/cmp_shifted.rs | 2026-10-05T03:50:00Z
COMPLETED: SUBPS | semantic_ir/tests/subps.rs | 2026-10-05T03:55:00Z
COMPLETED: CMPP | semantic_ir/tests/cmpp.rs | 2026-10-05T03:55:00Z
COMPLETED: CNEG | semantic_ir/tests/cneg.rs | 2026-10-05T04:00:00Z
COMPLETED: CNT | semantic_ir/tests/cnt.rs | 2026-10-05T04:05:00Z
COMPLETED: COSP | semantic_ir/tests/cosp.rs | 2026-10-05T04:10:00Z
COMPLETED: CPP | semantic_ir/tests/cpp.rs | 2026-10-05T04:15:00Z
COMPLETED: CPYFP, CPYFM, CPYFE | semantic_ir/tests/cpyfp.rs | 2026-10-05T04:20:00Z
COMPLETED: CPYFPN, CPYFMN, CPYFEN | semantic_ir/tests/cpyfpn.rs | 2026-10-05T04:25:00Z
COMPLETED: CPYFPRN, CPYFMRN, CPYFERN | semantic_ir/tests/cpyfprn.rs | 2026-10-05T04:30:00Z
COMPLETED: CPYFPRT, CPYFMRT, CPYFERT | semantic_ir/tests/cpyfprt.rs | 2026-10-05T04:35:00Z
COMPLETED: CPYFPRTN, CPYFMRTN, CPYFERTN | semantic_ir/tests/cpyfprtn.rs | 2026-10-05T04:40:00Z
COMPLETED: CPYFPRTRN, CPYFMRTRN, CPYFERTRN | semantic_ir/tests/cpyfprtrn.rs | 2026-10-05T04:45:00Z
COMPLETED: CPYFPRTWN, CPYFMRTWN, CPYFERTWN | semantic_ir/tests/cpyfprtwn.rs | 2026-10-05T04:50:00Z
COMPLETED: CPYFPT, CPYFMT, CPYFET | semantic_ir/tests/cpyfpt.rs | 2026-10-05T04:55:00Z
COMPLETED: CPYFPTN, CPYFMTN, CPYFETN | semantic_ir/tests/cpyfptn.rs | 2026-10-05T05:00:00Z
COMPLETED: CPYFPTRN, CPYFMTRN, CPYFETRN | semantic_ir/tests/cpyfptrn.rs | 2026-10-05T05:05:00Z
COMPLETED: CPYFPTWN, CPYFMTWN, CPYFETWN | semantic_ir/tests/cpyfptwn.rs | 2026-10-05T05:10:00Z
COMPLETED: CPYFPWN, CPYFMWN, CPYFEWN | semantic_ir/tests/cpyfpwn.rs | 2026-10-05T05:15:00Z
COMPLETED: CPYFPWT, CPYFMWT, CPYFEWT | semantic_ir/tests/cpyfpwt.rs | 2026-10-05T05:20:00Z
COMPLETED: CPYFPWTN, CPYFMWTN, CPYFEWTN | semantic_ir/tests/cpyfpwtn.rs | 2026-10-05T05:25:00Z
COMPLETED: CPYFPWTRN, CPYFMWTRN, CPYFEWTRN | semantic_ir/tests/cpyfpwtrn.rs | 2026-10-05T05:35:00Z
COMPLETED: CPYFPWTWN, CPYFMWTWN, CPYFEWTWN | semantic_ir/tests/cpyfpwtwn.rs | 2026-10-05T05:35:00Z
COMPLETED: CPYP, CPYM, CPYE | semantic_ir/tests/cpyp.rs | 2026-10-05T05:40:00Z
COMPLETED: CPYPN, CPYMN, CPYEN | semantic_ir/tests/cpypn.rs | 2026-10-05T05:45:00Z
COMPLETED: CPYPRN, CPYMRN, CPYERN | semantic_ir/tests/cpyprn.rs | 2026-10-05T05:50:00Z
COMPLETED: CPYPRT, CPYMRT, CPYERT | semantic_ir/tests/cpyprt.rs | 2026-10-05T07:40:00Z
COMPLETED: CPYPRTN, CPYMRTN, CPYERTN | semantic_ir/tests/cpyprtn.rs | 2026-10-05T07:50:00Z
COMPLETED: CPYPRTRN, CPYMRTRN, CPYERTRN | semantic_ir/tests/cpyprtrn.rs | 2026-10-05T08:00:00Z
COMPLETED: CPYPRTWN, CPYMRTWN, CPYERTWN | semantic_ir/tests/cpyprtwn.rs | 2026-10-05T08:05:00Z
COMPLETED: CPYPT, CPYMT, CPYET | semantic_ir/tests/cpypt.rs | 2026-10-05T08:15:00Z
COMPLETED: CPYPTN, CPYMTN, CPYETN | semantic_ir/tests/cpyptn.rs | 2026-10-05T08:20:00Z
COMPLETED: CPYPTRN, CPYMTRN, CPYETRN | semantic_ir/tests/cpyptrn.rs | 2026-10-05T08:20:00Z
COMPLETED: CPYPTWN, CPYMTWN, CPYETWN | semantic_ir/tests/cpyptwn.rs | 2026-10-05T08:20:00Z
COMPLETED: CPYPWN, CPYMWN, CPYEWN | semantic_ir/tests/cpypwn.rs | 2026-10-05T08:25:00Z
COMPLETED: CPYPWT, CPYMWT, CPYEWT | semantic_ir/tests/cpypwt.rs | 2026-10-05T08:25:00Z
COMPLETED: CPYPWTN, CPYMWTN, CPYEWTN | semantic_ir/tests/cpypwtn.rs | 2026-10-05T08:25:00Z
COMPLETED: CPYPWTRN, CPYMWTRN, CPYEWTRN | semantic_ir/tests/cpypwtrn.rs | 2026-10-05T08:25:00Z
COMPLETED: CPYPWTWN, CPYMWTWN, CPYEWTWN | semantic_ir/tests/cpypwtwn.rs | 2026-10-05T08:25:00Z
COMPLETED: CRC32B, CRC32H, CRC32W, CRC32X | semantic_ir/tests/crc32.rs | 2026-10-05T08:40:00Z
COMPLETED: CRC32CB, CRC32CH, CRC32CW, CRC32CX | semantic_ir/tests/crc32c.rs | 2026-10-05T08:45:00Z
COMPLETED: CSDB | semantic_ir/tests/csdb.rs | 2026-10-05T09:00:00Z
COMPLETED: CSET | semantic_ir/tests/cset.rs | 2026-10-05T09:00:00Z
COMPLETED: CSETM | semantic_ir/tests/csetm.rs | 2026-10-05T09:05:00Z
COMPLETED: CTZ | semantic_ir/tests/ctz.rs | 2026-10-05T09:05:00Z
COMPLETED: DC | semantic_ir/tests/dc.rs | 2026-10-05T09:10:00Z
COMPLETED: DCPS1 | semantic_ir/tests/dcps1.rs | 2026-10-05T09:10:00Z
COMPLETED: DCPS2 | semantic_ir/tests/dcps2.rs | 2026-10-05T09:10:00Z
COMPLETED: DCPS3 | semantic_ir/tests/dcps3.rs | 2026-10-05T09:10:00Z
COMPLETED: DGH | semantic_ir/tests/dgh.rs | 2026-10-05T09:15:00Z
COMPLETED: DMB | semantic_ir/tests/dmb.rs | 2026-10-05T09:15:00Z
COMPLETED: DRPS | semantic_ir/tests/drps.rs | 2026-10-05T09:15:00Z
COMPLETED: DSB | semantic_ir/tests/dsb.rs | 2026-10-05T09:15:00Z
COMPLETED: DVP | semantic_ir/tests/dvp.rs | 2026-10-05T09:20:00Z
COMPLETED: EON (shifted register) | semantic_ir/tests/eon_shifted.rs | 2026-10-05T09:20:00Z
COMPLETED: ERETAA, ERETAB | semantic_ir/tests/eretaa.rs | 2026-10-05T09:25:00Z
COMPLETED: ESB | semantic_ir/tests/esb.rs | 2026-10-05T09:35:00Z
COMPLETED: GCSB | semantic_ir/tests/gcsb.rs | 2026-10-05T09:50:00Z
COMPLETED: GCSPOPCX | semantic_ir/tests/gcspopcx.rs | 2026-10-05T09:50:00Z
COMPLETED: GCSPOPM | semantic_ir/tests/gcspopm.rs | 2026-10-05T09:50:00Z
COMPLETED: GCSPOPX | semantic_ir/tests/gcspopx.rs | 2026-10-05T09:50:00Z
COMPLETED: GCSPUSHM | semantic_ir/tests/gcspushm.rs | 2026-10-05T09:50:00Z
COMPLETED: GCSPUSHX | semantic_ir/tests/gcspushx.rs | 2026-10-05T09:50:00Z
COMPLETED: GCSSS1 | semantic_ir/tests/gcsss1.rs | 2026-10-05T09:50:00Z
COMPLETED: GCSSS2 | semantic_ir/tests/gcsss2.rs | 2026-10-05T09:50:00Z
COMPLETED: GCSSTR | semantic_ir/tests/gcsstr.rs | 2026-10-05T09:55:00Z
COMPLETED: GCSSTTR | semantic_ir/tests/gcssttr.rs | 2026-10-05T09:55:00Z
COMPLETED: GMI | semantic_ir/tests/gmi.rs | 2026-10-05T10:10:00Z
COMPLETED: HINT | semantic_ir/tests/hint.rs | 2026-10-05T10:10:00Z
COMPLETED: HLT | semantic_ir/tests/hlt.rs | 2026-10-05T10:10:00Z
COMPLETED: HVC | semantic_ir/tests/hvc.rs | 2026-10-05T10:10:00Z
COMPLETED: IC | semantic_ir/tests/ic.rs | 2026-10-05T10:15:00Z
COMPLETED: IRG | semantic_ir/tests/irg.rs | 2026-10-05T10:15:00Z
COMPLETED: ISB | semantic_ir/tests/isb.rs | 2026-10-05T10:10:00Z
COMPLETED: LD64B | semantic_ir/tests/ld64b.rs | 2026-10-05T10:30:00Z
COMPLETED: LDADD, LDADDA, LDADDAL, LDADDL | semantic_ir/tests/ldadd.rs | 2026-10-05T10:30:00Z
COMPLETED: LDADDB, LDADDAB, LDADDALB, LDADDLB | semantic_ir/tests/ldaddb.rs | 2026-10-05T10:35:00Z
COMPLETED: LDADDH, LDADDAH, LDADDALH, LDADDLH | semantic_ir/tests/ldaddh.rs | 2026-10-05T10:35:00Z
COMPLETED: LDAPR | semantic_ir/tests/ldapr.rs | 2026-10-05T10:50:00Z
COMPLETED: LDAPRB | semantic_ir/tests/ldapr.rs | 2026-10-05T10:50:00Z
COMPLETED: LDAPRH | semantic_ir/tests/ldapr.rs | 2026-10-05T10:50:00Z
COMPLETED: LDAPUR | semantic_ir/tests/ldapur.rs | 2026-10-05T10:55:00Z
COMPLETED: LDAPURB | semantic_ir/tests/ldapur.rs | 2026-10-05T10:55:00Z
COMPLETED: LDAPURH | semantic_ir/tests/ldapur.rs | 2026-10-05T10:55:00Z
COMPLETED: LDAPURSB | semantic_ir/tests/ldapur.rs | 2026-10-05T10:55:00Z
COMPLETED: LDAPURSH | semantic_ir/tests/ldapur.rs | 2026-10-05T10:55:00Z
COMPLETED: LDAPURSW | semantic_ir/tests/ldapur.rs | 2026-10-05T10:55:00Z
COMPLETED: LDAR | semantic_ir/tests/ldar.rs | 2026-10-05T10:55:00Z
COMPLETED: LDARB | semantic_ir/tests/ldar.rs | 2026-10-05T10:55:00Z
COMPLETED: LDARH | semantic_ir/tests/ldar.rs | 2026-10-05T10:55:00Z
COMPLETED: LDATXR | semantic_ir/tests/ldatxr.rs | 2026-10-05T11:10:00Z
COMPLETED: LDAXP | semantic_ir/tests/ldaxp.rs | 2026-10-05T11:10:00Z
COMPLETED: LDAXR | semantic_ir/tests/ldaxr.rs | 2026-10-05T11:10:00Z
COMPLETED: LDAXRB | semantic_ir/tests/ldaxr.rs | 2026-10-05T11:10:00Z
COMPLETED: LDAXRH | semantic_ir/tests/ldaxr.rs | 2026-10-05T11:10:00Z
COMPLETED: LDCLR, LDCLRA, LDCLRAL, LDCLRL | semantic_ir/tests/ldclr.rs | 2026-10-05T11:15:00Z
COMPLETED: LDCLRB, LDCLRAB, LDCLRALB, LDCLRLB | semantic_ir/tests/ldclrb.rs | 2026-10-05T11:15:00Z
COMPLETED: LDCLRH, LDCLRAH, LDCLRALH, LDCLRLH | semantic_ir/tests/ldclrh.rs | 2026-10-05T11:15:00Z
COMPLETED: LDCLRP, LDCLRPA, LDCLRPAL, LDCLRPL | semantic_ir/tests/ldclrp.rs | 2026-10-05T11:20:00Z
COMPLETED: LDEOR, LDEORA, LDEORAL, LDEORL | semantic_ir/tests/ldeor.rs | 2026-10-05T11:15:00Z
COMPLETED: LDEORB, LDEORAB, LDEORALB, LDEORLB | semantic_ir/tests/ldeorb.rs | 2026-10-05T11:15:00Z
COMPLETED: LDEORH, LDEORAH, LDEORALH, LDEORLH | semantic_ir/tests/ldeorh.rs | 2026-10-05T11:15:00Z
COMPLETED: LDG | semantic_ir/tests/ldg.rs | 2026-10-05T11:25:00Z
COMPLETED: LDGM | semantic_ir/tests/ldg.rs | 2026-10-05T11:25:00Z
COMPLETED: LDSET, LDSETA, LDSETAL, LDSETL | semantic_ir/tests/ldset.rs | 2026-10-05T11:30:00Z
COMPLETED: LDSETB, LDSETAB, LDSETALB, LDSETLB | semantic_ir/tests/ldsetb.rs | 2026-10-05T11:30:00Z
COMPLETED: LDSETH, LDSETAH, LDSETALH, LDSETLH | semantic_ir/tests/ldseth.rs | 2026-10-05T11:30:00Z
COMPLETED: LDSMAX, LDSMAXA, LDSMAXAL, LDSMAXL | semantic_ir/tests/ldsmax.rs | 2026-10-05T11:30:00Z
COMPLETED: LDSMAXB, LDSMAXAB, LDSMAXALB, LDSMAXLB | semantic_ir/tests/ldsmaxb.rs | 2026-10-05T11:30:00Z
COMPLETED: LDSMAXH, LDSMAXAH, LDSMAXALH, LDSMAXLH | semantic_ir/tests/ldsmaxh.rs | 2026-10-05T11:30:00Z
COMPLETED: LDSMIN, LDSMINA, LDSMINAL, LDSMINL | semantic_ir/tests/ldsmin.rs | 2026-10-05T11:30:00Z
COMPLETED: LDSMINB, LDSMINAB, LDSMINALB, LDSMINLB | semantic_ir/tests/ldsminb.rs | 2026-10-05T11:30:00Z
COMPLETED: LDSMINH, LDSMINAH, LDSMINALH, LDSMINLH | semantic_ir/tests/ldsminh.rs | 2026-10-05T11:30:00Z
COMPLETED: LDUMAX, LDUMAXA, LDUMAXAL, LDUMAXL | semantic_ir/tests/ldumax.rs | 2026-10-05T11:30:00Z
COMPLETED: LDUMAXB, LDUMAXAB, LDUMAXALB, LDUMAXLB | semantic_ir/tests/ldumaxb.rs | 2026-10-05T11:30:00Z
COMPLETED: LDUMAXH, LDUMAXAH, LDUMAXALH, LDUMAXLH | semantic_ir/tests/ldumaxh.rs | 2026-10-05T11:30:00Z
COMPLETED: LDUMIN, LDUMINA, LDUMINAL, LDUMINL | semantic_ir/tests/ldumin.rs | 2026-10-05T11:30:00Z
COMPLETED: LDUMINB, LDUMINAB, LDUMINALB, LDUMINLB | semantic_ir/tests/lduminb.rs | 2026-10-05T11:30:00Z
COMPLETED: LDUMINH, LDUMINAH, LDUMINALH, LDUMINLH | semantic_ir/tests/lduminh.rs | 2026-10-05T11:30:00Z
COMPLETED: LDLAR | semantic_ir/tests/ldlar.rs | 2026-10-05T11:45:00Z
COMPLETED: LDLARB | semantic_ir/tests/ldlar.rs | 2026-10-05T11:45:00Z
COMPLETED: LDLARH | semantic_ir/tests/ldlar.rs | 2026-10-05T11:45:00Z
COMPLETED: LDNP | semantic_ir/tests/ldnp.rs | 2026-10-05T11:50:00Z
COMPLETED: LDP | semantic_ir/tests/ldp.rs | 2026-10-05T11:50:00Z
COMPLETED: LDPSW | semantic_ir/tests/ldpsw.rs | 2026-10-05T11:50:00Z
COMPLETED: LDR (register) | semantic_ir/tests/ldr_reg.rs | 2026-10-05T11:55:00Z
COMPLETED: LDRAA, LDRAB | semantic_ir/tests/ldraa.rs | 2026-10-05T12:00:00Z
COMPLETED: LDRB (immediate) | semantic_ir/tests/ldrb_imm.rs | 2026-10-05T11:55:00Z
COMPLETED: LDRB (register) | semantic_ir/tests/ldrb_reg.rs | 2026-10-05T11:55:00Z
COMPLETED: LDRH (immediate) | semantic_ir/tests/ldrh_imm.rs | 2026-10-05T11:55:00Z
COMPLETED: LDRH (register) | semantic_ir/tests/ldrh_reg.rs | 2026-10-05T11:55:00Z
COMPLETED: LDRSB (immediate) | semantic_ir/tests/ldrsb_imm.rs | 2026-10-05T11:55:00Z
COMPLETED: LDRSB (register) | semantic_ir/tests/ldrsb_reg.rs | 2026-10-05T11:55:00Z
COMPLETED: LDRSH (immediate) | semantic_ir/tests/ldrsh_imm.rs | 2026-10-05T11:55:00Z
COMPLETED: LDRSH (register) | semantic_ir/tests/ldrsh_reg.rs | 2026-10-05T11:55:00Z
COMPLETED: LDRSW (immediate) | semantic_ir/tests/ldrsw_imm.rs | 2026-10-05T11:55:00Z
COMPLETED: LDRSW (literal) | semantic_ir/tests/ldrsw_lit.rs | 2026-10-05T11:55:00Z
COMPLETED: LDRSW (register) | semantic_ir/tests/ldrsw_reg.rs | 2026-10-05T11:55:00Z
COMPLETED: LDSETP, LDSETPA, LDSETPAL, LDSETPL | semantic_ir/tests/ldsetp.rs | 2026-10-05T12:00:00Z
COMPLETED: LDTADD, LDTADDA, LDTADDAL, LDTADDL | semantic_ir/tests/ldtadd.rs | 2026-10-05T12:05:00Z
COMPLETED: LDTCLR, LDTCLRA, LDTCLRAL, LDTCLRL | semantic_ir/tests/ldtclr.rs | 2026-10-05T12:05:00Z
COMPLETED: LDTNP | semantic_ir/tests/ldtp.rs | 2026-10-05T12:05:00Z
COMPLETED: LDTP | semantic_ir/tests/ldtp.rs | 2026-10-05T12:05:00Z
COMPLETED: LDTR | semantic_ir/tests/ldtr.rs | 2026-10-05T12:05:00Z
COMPLETED: LDTRB | semantic_ir/tests/ldtrb.rs | 2026-10-05T12:05:00Z
COMPLETED: LDTRH | semantic_ir/tests/ldtrh.rs | 2026-10-05T12:05:00Z
COMPLETED: LDTRSB | semantic_ir/tests/ldtrsb.rs | 2026-10-05T12:05:00Z
COMPLETED: LDTRSH | semantic_ir/tests/ldtrsh.rs | 2026-10-05T12:05:00Z
COMPLETED: LDTRSW | semantic_ir/tests/ldtrsw.rs | 2026-10-05T12:05:00Z
COMPLETED: LDTSET, LDTSETA, LDTSETAL, LDTSETL | semantic_ir/tests/ldtset.rs | 2026-10-05T12:05:00Z
COMPLETED: LDTXR | semantic_ir/tests/ldtxr.rs | 2026-10-05T12:15:00Z
COMPLETED: LDUR | semantic_ir/tests/ldur.rs | 2026-10-05T12:15:00Z
COMPLETED: LDURB | semantic_ir/tests/ldurb.rs | 2026-10-05T12:15:00Z
COMPLETED: LDURH | semantic_ir/tests/ldurh.rs | 2026-10-05T12:15:00Z
COMPLETED: LDURSB | semantic_ir/tests/ldursb.rs | 2026-10-05T12:15:00Z
COMPLETED: LDURSH | semantic_ir/tests/ldursh.rs | 2026-10-05T12:15:00Z
COMPLETED: LDURSW | semantic_ir/tests/ldursw.rs | 2026-10-05T12:15:00Z
COMPLETED: LDXP | semantic_ir/tests/ldxp.rs | 2026-10-05T12:20:00Z
COMPLETED: LDXR | semantic_ir/tests/ldxr.rs | 2026-10-05T12:20:00Z
COMPLETED: LDXRB | semantic_ir/tests/ldxr.rs | 2026-10-05T12:20:00Z
COMPLETED: LDXRH | semantic_ir/tests/ldxr.rs | 2026-10-05T12:20:00Z
COMPLETED: LSL (immediate) | semantic_ir/tests/lsl_imm.rs | 2026-10-05T12:30:00Z
COMPLETED: LSL (register) | semantic_ir/tests/lsl_reg.rs | 2026-10-05T12:30:00Z
COMPLETED: LSR (immediate) | semantic_ir/tests/lsr_imm.rs | 2026-10-05T12:30:00Z
COMPLETED: LSR (register) | semantic_ir/tests/lsr_reg.rs | 2026-10-05T12:30:00Z
COMPLETED: MADDPT | semantic_ir/tests/maddpt.rs | 2026-10-05T12:40:00Z
COMPLETED: MNEG | semantic_ir/tests/mneg.rs | 2026-10-05T12:35:00Z
COMPLETED: MOV (bitmask immediate) | semantic_ir/tests/mov_imm.rs | 2026-10-05T12:30:00Z
COMPLETED: MOV (inverted wide immediate) | semantic_ir/tests/mov_movn.rs | 2026-10-05T12:30:00Z
COMPLETED: MOV (register) | semantic_ir/tests/mov_reg.rs | 2026-10-05T12:30:00Z
COMPLETED: MOV (to/from SP) | semantic_ir/tests/mov_sp.rs | 2026-10-05T12:30:00Z
COMPLETED: MOV (wide immediate) | semantic_ir/tests/mov_movz.rs | 2026-10-05T12:30:00Z
COMPLETED: LDIAPP | semantic_ir/tests/ldiapp.rs | 2026-10-05T14:00:00Z
COMPLETED: MRRS | semantic_ir/tests/mrrs.rs | 2026-10-05T12:45:00Z
COMPLETED: MRS | semantic_ir/tests/mrs.rs | 2026-10-05T12:45:00Z
COMPLETED: MSR (immediate) | semantic_ir/tests/msr_imm.rs | 2026-10-05T12:45:00Z
COMPLETED: MSR (register) | semantic_ir/tests/msr_reg.rs | 2026-10-05T12:45:00Z
COMPLETED: MSRR | semantic_ir/tests/msrr.rs | 2026-10-05T12:45:00Z
COMPLETED: MSUBPT | semantic_ir/tests/msubpt.rs | 2026-10-05T12:40:00Z
COMPLETED: MUL | semantic_ir/tests/mul.rs | 2026-10-05T12:35:00Z
COMPLETED: MVN | semantic_ir/tests/mvn.rs | 2026-10-05T12:35:00Z
COMPLETED: NEG (shifted register) | semantic_ir/tests/neg.rs | 2026-10-05T12:35:00Z
COMPLETED: NEGS | semantic_ir/tests/negs.rs | 2026-10-05T12:35:00Z
COMPLETED: NGC | semantic_ir/tests/ngc.rs | 2026-10-05T12:35:00Z
COMPLETED: NGCS | semantic_ir/tests/ngcs.rs | 2026-10-05T12:35:00Z
COMPLETED: NOP | semantic_ir/tests/nop.rs | 2026-10-05T12:35:00Z
COMPLETED: PACDA, PACDZA | semantic_ir/tests/pacda.rs | 2026-10-05T12:55:00Z
COMPLETED: PACDB, PACDZB | semantic_ir/tests/pacdb.rs | 2026-10-05T12:55:00Z
COMPLETED: PACGA | semantic_ir/tests/pacga.rs | 2026-10-05T12:55:00Z
COMPLETED: PACIA, PACIA1716, PACIASP, PACIAZ, PACIZA | semantic_ir/tests/pacia.rs | 2026-10-05T12:55:00Z
COMPLETED: PACIA171615 | semantic_ir/tests/pacia171615.rs | 2026-10-05T13:00:00Z
COMPLETED: PACIASPPC | semantic_ir/tests/paciasppc.rs | 2026-10-05T13:00:00Z
COMPLETED: PACIB, PACIB1716, PACIBSP, PACIBZ, PACIZB | semantic_ir/tests/pacib.rs | 2026-10-05T12:55:00Z
COMPLETED: PACIB171615 | semantic_ir/tests/pacib171615.rs | 2026-10-05T13:00:00Z
COMPLETED: PACIBSPPC | semantic_ir/tests/pacibsppc.rs | 2026-10-05T13:00:00Z
COMPLETED: PACM | semantic_ir/tests/pacm.rs | 2026-10-05T13:00:00Z
COMPLETED: PACNBIASPPC | semantic_ir/tests/pacnbiasppc.rs | 2026-10-05T13:00:00Z
COMPLETED: PACNBIBSPPC | semantic_ir/tests/pacnbibsppc.rs | 2026-10-05T13:00:00Z
COMPLETED: PRFM (immediate) | semantic_ir/tests/prfm_imm.rs | 2026-10-05T13:05:00Z
COMPLETED: PRFM (literal) | semantic_ir/tests/prfm_lit.rs | 2026-10-05T13:05:00Z
COMPLETED: PRFM (register) | semantic_ir/tests/prfm_reg.rs | 2026-10-05T13:05:00Z
COMPLETED: PRFUM | semantic_ir/tests/prfum.rs | 2026-10-05T13:05:00Z
COMPLETED: PSB CSYNC | semantic_ir/tests/psb.rs | 2026-10-05T13:05:00Z
COMPLETED: PSSBB | semantic_ir/tests/pssbb.rs | 2026-10-05T13:05:00Z
COMPLETED: RCWCAS, RCWCASA, RCWCASAL, RCWCASL | semantic_ir/tests/rcwcas.rs | 2026-10-05T12:20:00Z
COMPLETED: RCWCASP, RCWCASPA, RCWCASPAL, RCWCASPL | semantic_ir/tests/rcwcasp.rs | 2026-10-05T12:20:00Z
COMPLETED: RCWCLR, RCWCLRA, RCWCLRAL, RCWCLRL | semantic_ir/tests/rcwclr.rs | 2026-10-05T12:20:00Z
COMPLETED: RCWCLRP, RCWCLRPA, RCWCLRPAL, RCWCLRPL | semantic_ir/tests/rcwclrp.rs | 2026-10-05T12:20:00Z
COMPLETED: RCWSCAS, RCWSCASA, RCWSCASAL, RCWSCASL | semantic_ir/tests/rcwscas.rs | 2026-10-05T12:20:00Z
COMPLETED: RCWSCASP, RCWSCASPA, RCWSCASPAL, RCWSCASPL | semantic_ir/tests/rcwscasp.rs | 2026-10-05T12:25:00Z
COMPLETED: RCWSCLR, RCWSCLRA, RCWSCLRAL, RCWSCLRL | semantic_ir/tests/rcwsclr.rs | 2026-10-05T12:20:00Z
COMPLETED: RCWSCLRP, RCWSCLRPA, RCWSCLRPAL, RCWSCLRPL | semantic_ir/tests/rcwsclrp.rs | 2026-10-05T12:25:00Z
COMPLETED: RCWSET, RCWSETA, RCWSETAL, RCWSETL | semantic_ir/tests/rcwset.rs | 2026-10-05T12:20:00Z
COMPLETED: RCWSETP, RCWSETPA, RCWSETPAL, RCWSETPL | semantic_ir/tests/rcwsetp.rs | 2026-10-05T12:20:00Z
COMPLETED: RCWSSET, RCWSSETA, RCWSSETAL, RCWSSETL | semantic_ir/tests/rcwsset.rs | 2026-10-05T12:20:00Z
COMPLETED: RCWSSETP, RCWSSETPA, RCWSSETPAL, RCWSSETPL | semantic_ir/tests/rcwssetp.rs | 2026-10-05T12:25:00Z
COMPLETED: RCWSSWP, RCWSSWPA, RCWSSWPAL, RCWSSWPL | semantic_ir/tests/rcwsswp.rs | 2026-10-05T12:20:00Z
COMPLETED: RCWSSWPP, RCWSSWPPA, RCWSSWPPAL, RCWSSWPPL | semantic_ir/tests/rcwsswpp.rs | 2026-10-05T12:25:00Z
COMPLETED: RCWSWP, RCWSWPA, RCWSWPAL, RCWSWPL | semantic_ir/tests/rcwswp.rs | 2026-10-05T12:20:00Z
COMPLETED: RCWSWPP, RCWSWPPA, RCWSWPPAL, RCWSWPPL | semantic_ir/tests/rcwswpp.rs | 2026-10-05T12:25:00Z
COMPLETED: RETAA, RETAB | semantic_ir/tests/retaa.rs | 2026-10-05T12:35:00Z
COMPLETED: RETAASPPC, RETABSPPC | semantic_ir/tests/retaasppc.rs | 2026-10-05T12:35:00Z
COMPLETED: RETAASPPCR, RETABSPPCR | semantic_ir/tests/retaasppc.rs | 2026-10-05T12:35:00Z
COMPLETED: REV | semantic_ir/tests/rev.rs | 2026-10-05T12:40:00Z
COMPLETED: REV32 | semantic_ir/tests/rev32.rs | 2026-10-05T12:40:00Z
COMPLETED: REV64 | semantic_ir/tests/rev64.rs | 2026-10-05T12:40:00Z
COMPLETED: RMIF | semantic_ir/tests/rmif.rs | 2026-10-05T12:40:00Z
COMPLETED: ROR (immediate) | semantic_ir/tests/ror_imm.rs | 2026-10-05T12:40:00Z
COMPLETED: ROR (register) | semantic_ir/tests/ror_reg.rs | 2026-10-05T12:40:00Z
COMPLETED: RPRFM | semantic_ir/tests/rprfm_reg.rs | 2026-10-05T12:40:00Z
COMPLETED: SB | semantic_ir/tests/sb.rs | 2026-10-05T12:50:00Z
COMPLETED: SBC | semantic_ir/tests/sbc.rs | 2026-10-05T12:50:00Z
COMPLETED: SBCS | semantic_ir/tests/sbcs.rs | 2026-10-05T12:50:00Z
COMPLETED: SBFIZ | semantic_ir/tests/sbfiz.rs | 2026-10-05T12:50:00Z
COMPLETED: SBFX | semantic_ir/tests/sbfx.rs | 2026-10-05T12:50:00Z
COMPLETED: SDIV (quotient) | semantic_ir/tests/sdiv.rs | 2026-10-05T12:50:00Z
COMPLETED: SETF8, SETF16 | semantic_ir/tests/setf.rs | 2026-10-05T12:45:00Z
COMPLETED: SETP, SETM, SETE | semantic_ir/tests/setp.rs | 2026-10-05T12:45:00Z
COMPLETED: SETPN, SETMN, SETEN | semantic_ir/tests/setpn.rs | 2026-10-05T12:45:00Z
COMPLETED: SETPT, SETMT, SETET | semantic_ir/tests/setpt.rs | 2026-10-05T12:45:00Z
COMPLETED: SETPTN, SETMTN, SETETN | semantic_ir/tests/setptn.rs | 2026-10-05T12:45:00Z
COMPLETED: SETGP, SETGM, SETGE | semantic_ir/tests/setgp.rs | 2026-10-05T12:45:00Z
COMPLETED: SETGPN, SETGMN, SETGEN | semantic_ir/tests/setgpn.rs | 2026-10-05T12:45:00Z
COMPLETED: SETGPT, SETGMT, SETGET | semantic_ir/tests/setgpt.rs | 2026-10-05T12:45:00Z
COMPLETED: SETGPTN, SETGMTN, SETGETN | semantic_ir/tests/setgptn.rs | 2026-10-05T12:45:00Z
COMPLETED: SEV | semantic_ir/tests/sev.rs | 2026-10-05T12:50:00Z
COMPLETED: SEVL | semantic_ir/tests/sevl.rs | 2026-10-05T12:50:00Z
COMPLETED: SMAX (immediate) | semantic_ir/tests/smax_imm.rs | 2026-10-05T12:55:00Z
COMPLETED: SMIN (immediate) | semantic_ir/tests/smin_imm.rs | 2026-10-05T12:55:00Z
COMPLETED: SMC | semantic_ir/tests/smc.rs | 2026-10-05T12:50:00Z
COMPLETED: SMNEGL | semantic_ir/tests/smnegl.rs | 2026-10-05T12:55:00Z
COMPLETED: SMSTART | semantic_ir/tests/smstart.rs | 2026-10-05T12:55:00Z
COMPLETED: SMSTOP | semantic_ir/tests/smstop.rs | 2026-10-05T12:55:00Z
COMPLETED: SMSUBL | semantic_ir/tests/smsubl.rs | 2026-10-05T12:55:00Z
COMPLETED: SMULH | semantic_ir/tests/smulh.rs | 2026-10-05T12:55:00Z
COMPLETED: SMULL (32-bit) | semantic_ir/tests/smull.rs | 2026-10-05T12:55:00Z
COMPLETED: SSBB | semantic_ir/tests/ssbb.rs | 2026-10-05T12:55:00Z
COMPLETED: ST2G | semantic_ir/tests/st2g.rs | 2026-10-05T13:00:00Z
COMPLETED: ST64B | semantic_ir/tests/st64b.rs | 2026-10-05T13:00:00Z
COMPLETED: ST64BV | semantic_ir/tests/st64bv.rs | 2026-10-05T13:00:00Z
COMPLETED: ST64BV0 | semantic_ir/tests/st64bv0.rs | 2026-10-05T13:00:00Z
COMPLETED: STADD, STADDL | semantic_ir/tests/stadd.rs | 2026-10-05T13:05:00Z
COMPLETED: STADDB, STADDLB | semantic_ir/tests/staddb.rs | 2026-10-05T13:05:00Z
COMPLETED: STADDH, STADDLH | semantic_ir/tests/staddh.rs | 2026-10-05T13:05:00Z
COMPLETED: STCLR, STCLRL | semantic_ir/tests/stclr.rs | 2026-10-05T13:05:00Z
COMPLETED: STCLRB, STCLRLB | semantic_ir/tests/stclrb.rs | 2026-10-05T13:05:00Z
COMPLETED: STCLRH, STCLRLH | semantic_ir/tests/stclrh.rs | 2026-10-05T13:05:00Z
COMPLETED: STEOR, STEORL | semantic_ir/tests/steor.rs | 2026-10-05T13:05:00Z
COMPLETED: STEORB, STEORLB | semantic_ir/tests/steorb.rs | 2026-10-05T13:05:00Z
COMPLETED: STEORH, STEORLH | semantic_ir/tests/steorh.rs | 2026-10-05T13:05:00Z
COMPLETED: STG | semantic_ir/tests/stg.rs | 2026-10-05T13:10:00Z
COMPLETED: STGM | semantic_ir/tests/stgm.rs | 2026-10-05T13:10:00Z
COMPLETED: STGP | semantic_ir/tests/stgp.rs | 2026-10-05T13:10:00Z
COMPLETED: STILP | semantic_ir/tests/stilp.rs | 2026-10-05T13:10:00Z
COMPLETED: STLLR | semantic_ir/tests/stllr.rs | 2026-10-05T13:10:00Z
COMPLETED: STLLRB | semantic_ir/tests/stllrb.rs | 2026-10-05T13:10:00Z
COMPLETED: STLLRH | semantic_ir/tests/stllrh.rs | 2026-10-05T13:10:00Z
COMPLETED: STLR | semantic_ir/tests/stlr.rs | 2026-10-05T13:10:00Z
COMPLETED: STLRB | semantic_ir/tests/stlrb.rs | 2026-10-05T13:10:00Z
COMPLETED: STLRH | semantic_ir/tests/stlrh.rs | 2026-10-05T13:10:00Z
COMPLETED: STLTXR | semantic_ir/tests/stltxr.rs | 2026-10-05T13:10:00Z
COMPLETED: STLUR | semantic_ir/tests/stlur.rs | 2026-10-05T13:10:00Z
COMPLETED: STLURB | semantic_ir/tests/stlurb.rs | 2026-10-05T13:10:00Z
COMPLETED: STLURH | semantic_ir/tests/stlurh.rs | 2026-10-05T13:10:00Z
COMPLETED: STLXP | semantic_ir/tests/stlxp.rs | 2026-10-05T13:10:00Z
COMPLETED: STLXR | semantic_ir/tests/stlxr.rs | 2026-10-05T13:10:00Z
COMPLETED: STLXRB | semantic_ir/tests/stlxr.rs | 2026-10-05T13:10:00Z
COMPLETED: STLXRH | semantic_ir/tests/stlxr.rs | 2026-10-05T13:10:00Z
COMPLETED: STNP | semantic_ir/tests/stnp.rs | 2026-10-05T13:10:00Z
COMPLETED: STP | semantic_ir/tests/stp.rs | 2026-10-05T13:10:00Z
COMPLETED: STR (register) | semantic_ir/tests/str_reg.rs | 2026-10-05T13:15:00Z
COMPLETED: STRB (immediate) | semantic_ir/tests/strb_imm.rs | 2026-10-05T13:15:00Z
COMPLETED: STRB (register) | semantic_ir/tests/strb_reg.rs | 2026-10-05T13:15:00Z
COMPLETED: STRH (immediate) | semantic_ir/tests/strh_imm.rs | 2026-10-05T13:15:00Z
COMPLETED: STRH (register) | semantic_ir/tests/strh_reg.rs | 2026-10-05T13:15:00Z
COMPLETED: STSET, STSETL | semantic_ir/tests/stset.rs | 2026-10-05T13:20:00Z
COMPLETED: STSETB, STSETLB | semantic_ir/tests/stsetb.rs | 2026-10-05T13:20:00Z
COMPLETED: STSETH, STSETLH | semantic_ir/tests/stseth.rs | 2026-10-05T13:20:00Z
COMPLETED: STSHH | semantic_ir/tests/stshh.rs | 2026-10-05T13:20:00Z
COMPLETED: STSMAX, STSMAXL | semantic_ir/tests/stsmax.rs | 2026-10-05T13:20:00Z
COMPLETED: STSMAXB, STSMAXLB | semantic_ir/tests/stsmaxb.rs | 2026-10-05T13:20:00Z
COMPLETED: STSMAXH, STSMAXLH | semantic_ir/tests/stsmaxh.rs | 2026-10-05T13:20:00Z
COMPLETED: STSMIN, STSMINL | semantic_ir/tests/stsmin.rs | 2026-10-05T13:20:00Z
COMPLETED: STSMINB, STSMINLB | semantic_ir/tests/stsminb.rs | 2026-10-05T13:20:00Z
COMPLETED: STSMINH, STSMINLH | semantic_ir/tests/stsminh.rs | 2026-10-05T13:20:00Z
COMPLETED: STTADD, STTADDL | semantic_ir/tests/sttadd.rs | 2026-10-05T13:20:00Z
COMPLETED: STTCLR, STTCLRL | semantic_ir/tests/sttclr.rs | 2026-10-05T13:20:00Z
COMPLETED: STTNP | semantic_ir/tests/sttnp.rs | 2026-10-05T13:20:00Z
COMPLETED: STTP | semantic_ir/tests/sttp.rs | 2026-10-05T13:20:00Z
COMPLETED: STTR | semantic_ir/tests/sttr.rs | 2026-10-05T13:15:00Z
COMPLETED: STTRB | semantic_ir/tests/sttrb.rs | 2026-10-05T13:15:00Z
COMPLETED: STTRH | semantic_ir/tests/sttrh.rs | 2026-10-05T13:15:00Z
COMPLETED: STTSET, STTSETL | semantic_ir/tests/sttset.rs | 2026-10-05T13:20:00Z
COMPLETED: STTXR | semantic_ir/tests/sttxr.rs | 2026-10-05T13:20:00Z
COMPLETED: STUMAX, STUMAXL | semantic_ir/tests/stumax.rs | 2026-10-05T13:20:00Z
COMPLETED: STUMAXB, STUMAXLB | semantic_ir/tests/stumaxb.rs | 2026-10-05T13:20:00Z
COMPLETED: STUMAXH, STUMAXLH | semantic_ir/tests/stumaxh.rs | 2026-10-05T13:20:00Z
COMPLETED: STUMIN, STUMINL | semantic_ir/tests/stumin.rs | 2026-10-05T13:20:00Z
COMPLETED: STUMINB, STUMINLB | semantic_ir/tests/stuminb.rs | 2026-10-05T13:20:00Z
COMPLETED: STUMINH, STUMINLH | semantic_ir/tests/stuminh.rs | 2026-10-05T13:20:00Z
COMPLETED: STUR | semantic_ir/tests/stur.rs | 2026-10-05T13:25:00Z
COMPLETED: STURB | semantic_ir/tests/sturb.rs | 2026-10-05T13:25:00Z
COMPLETED: STURH | semantic_ir/tests/sturh.rs | 2026-10-05T13:25:00Z
COMPLETED: STXP | semantic_ir/tests/stxp.rs | 2026-10-05T13:25:00Z
COMPLETED: STXR | semantic_ir/tests/stxr.rs | 2026-10-05T13:25:00Z
COMPLETED: STXRB | semantic_ir/tests/stxr.rs | 2026-10-05T13:25:00Z
COMPLETED: STXRH | semantic_ir/tests/stxr.rs | 2026-10-05T13:25:00Z
COMPLETED: STZ2G | semantic_ir/tests/stz2g.rs | 2026-10-05T13:25:00Z
COMPLETED: STZG | semantic_ir/tests/stzg.rs | 2026-10-05T13:25:00Z
COMPLETED: STZGM | semantic_ir/tests/stzgm.rs | 2026-10-05T13:25:00Z
COMPLETED: SUBG | semantic_ir/tests/subg.rs | 2026-10-05T14:00:00Z
COMPLETED: SUBP | semantic_ir/tests/subp.rs | 2026-10-05T13:30:00Z
COMPLETED: SUBPT | semantic_ir/tests/subpt.rs | 2026-10-05T13:30:00Z
COMPLETED: SVC | semantic_ir/tests/svc.rs | 2026-10-05T13:30:00Z
COMPLETED: SWP, SWPA, SWPAL, SWPL | semantic_ir/tests/swp.rs | 2026-10-05T13:30:00Z
COMPLETED: SWPB, SWPAB, SWPALB, SWPLB | semantic_ir/tests/swpb.rs | 2026-10-05T13:30:00Z
COMPLETED: SWPH, SWPAH, SWPALH, SWPLH | semantic_ir/tests/swph.rs | 2026-10-05T13:30:00Z
COMPLETED: SWPP, SWPPA, SWPPAL, SWPPL | semantic_ir/tests/swpp.rs | 2026-10-05T13:30:00Z
COMPLETED: SWPT, SWPTA, SWPTAL, SWPTL | semantic_ir/tests/swpt.rs | 2026-10-05T13:30:00Z
COMPLETED: SXTB | semantic_ir/tests/sxtb.rs | 2026-10-05T13:45:00Z
COMPLETED: SXTH | semantic_ir/tests/sxth.rs | 2026-10-05T13:45:00Z
COMPLETED: SXTW | semantic_ir/tests/sxtw.rs | 2026-10-05T13:45:00Z
COMPLETED: SYS | semantic_ir/tests/sys.rs | 2026-10-05T13:45:00Z
COMPLETED: SYSL | semantic_ir/tests/sysl.rs | 2026-10-05T13:45:00Z
COMPLETED: SYSP | semantic_ir/tests/sysp.rs | 2026-10-05T13:45:00Z
COMPLETED: TLBI | semantic_ir/tests/tlbi.rs | 2026-10-05T13:45:00Z
COMPLETED: TLBIP | semantic_ir/tests/tlbip.rs | 2026-10-05T13:45:00Z
COMPLETED: TRCIT | semantic_ir/tests/trcit.rs | 2026-10-05T13:45:00Z
COMPLETED: TSB | semantic_ir/tests/tsb.rs | 2026-10-05T13:45:00Z
COMPLETED: TST (immediate) | semantic_ir/tests/tst_imm.rs | 2026-10-05T13:45:00Z
COMPLETED: TST (shifted register) | semantic_ir/tests/tst_reg.rs | 2026-10-05T13:45:00Z
COMPLETED: UBFIZ | semantic_ir/tests/ubfiz.rs | 2026-10-05T13:45:00Z
COMPLETED: UBFX | semantic_ir/tests/ubfx.rs | 2026-10-05T13:45:00Z
COMPLETED: UDF | semantic_ir/tests/udf.rs | 2026-10-05T13:45:00Z
COMPLETED: UDIV (quotient) | semantic_ir/tests/udiv.rs | 2026-10-05T13:45:00Z
COMPLETED: UMAX (immediate) | semantic_ir/tests/umax_imm.rs | 2026-10-05T13:45:00Z
COMPLETED: UMIN (immediate) | semantic_ir/tests/umin_imm.rs | 2026-10-05T13:45:00Z
COMPLETED: UMNEGL | semantic_ir/tests/umnegl.rs | 2026-10-05T13:45:00Z
COMPLETED: UMSUBL | semantic_ir/tests/umsubl.rs | 2026-10-05T13:45:00Z
COMPLETED: UMULH | semantic_ir/tests/umulh.rs | 2026-10-05T13:45:00Z
COMPLETED: UMULL (32-bit) | semantic_ir/tests/umull.rs | 2026-10-05T13:45:00Z
COMPLETED: UXTB | semantic_ir/tests/uxtb.rs | 2026-10-05T13:45:00Z
COMPLETED: UXTH | semantic_ir/tests/uxth.rs | 2026-10-05T13:45:00Z
COMPLETED: WFE | semantic_ir/tests/wfe.rs | 2026-10-05T13:45:00Z
COMPLETED: WFET | semantic_ir/tests/wfet.rs | 2026-10-05T13:45:00Z
COMPLETED: WFI | semantic_ir/tests/wfi.rs | 2026-10-05T13:45:00Z
COMPLETED: WFIT | semantic_ir/tests/wfit.rs | 2026-10-05T13:45:00Z
COMPLETED: XAFLAG | semantic_ir/tests/xaflag.rs | 2026-10-05T13:45:00Z
COMPLETED: XPACD, XPACI, XPACLRI | semantic_ir/tests/xpacd.rs | 2026-10-05T13:45:00Z
COMPLETED: YIELD | semantic_ir/tests/yield.rs | 2026-10-05T13:50:00Z
COMPLETED: ADDSUB | semantic_ir/tests/add_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR (register) | semantic_ir/tests/asr_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: EOR (shifted register) | semantic_ir/tests/eor_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ORR (shifted register) | semantic_ir/tests/orr_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: SUB (shifted register) | semantic_ir/tests/sub_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: SUBS (shifted register) | semantic_ir/tests/subs_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: NEG (shifted register) | semantic_ir/tests/neg_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: ROR (shifted register) | semantic_ir/tests/ror_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: LSL (shifted register) | semantic_ir/tests/lsl_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: LSR (shifted register) | semantic_ir/tests/lsr_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: ADC (shifted register) | semantic_ir/tests/adc_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: ADCS (shifted register) | semantic_ir/tests/adcs_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: ADD (immediate) | semantic_ir/tests/add_imm.rs | 2026-10-16T16:00:00Z|COMPLETED: SUBS (shifted register) | semantic_ir/tests/subs_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: NEG (shifted register) | semantic_ir/tests/neg_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: ROR (shifted register) | semantic_ir/tests/ror_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: LSL (shifted register) | semantic_ir/tests/lsl_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: LSR (shifted register) | semantic_ir/tests/lsr_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: ADC (shifted register) | semantic_ir/tests/adc_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: ADCS (shifted register) | semantic_ir/tests/adcs_shifted.rs | 2026-10-16T16:00:00Z`nCOMPLETED: ADD (immediate) | semantic_ir/tests/add_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: NEG (shifted register) | semantic_ir/tests/neg_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ROR (shifted register) | semantic_ir/tests/ror_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: LSL (shifted register) | semantic_ir/tests/lsl_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: LSR (shifted register) | semantic_ir/tests/lsr_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ABS | semantic_ir/tests/abs.rs | 2026-10-16T16:00:00Z
COMPLETED: ADC | semantic_ir/tests/adc.rs | 2026-10-16T16:00:00Z
COMPLETED: ADCS | semantic_ir/tests/adcs.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD (shifted register) | semantic_ir/tests/add_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD (immediate) | semantic_ir/tests/add_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ADR | semantic_ir/tests/adr.rs | 2026-10-16T16:00:00Z
COMPLETED: ADRP | semantic_ir/tests/adrp.rs | 2026-10-16T16:00:00Z
COMPLETED: AND | semantic_ir/tests/and.rs | 2026-10-16T16:00:00Z
COMPLETED: AND (shifted register) | semantic_ir/tests/and_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: AND (immediate) | semantic_ir/tests/and_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDS (shifted register) | semantic_ir/tests/ands_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDS (immediate) | semantic_ir/tests/ands_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR (shifted register) | semantic_ir/tests/asr_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR (immediate) | semantic_ir/tests/asr_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: BIC (shifted register) | semantic_ir/tests/bic_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: BICS (shifted register) | semantic_ir/tests/bics_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ADC (shifted register) | semantic_ir/tests/adc_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ADCS (shifted register) | semantic_ir/tests/adcs_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ADC (immediate) | semantic_ir/tests/adc_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ADCS (immediate) | semantic_ir/tests/adcs_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (register) | semantic_ir/tests/mov_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (wide immediate) | semantic_ir/tests/mov_movz.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (inverted wide immediate) | semantic_ir/tests/mov_movn.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (bitmask immediate) | semantic_ir/tests/mov_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (to/from SP) | semantic_ir/tests/mov_sp.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVK | semantic_ir/tests/movk.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVN | semantic_ir/tests/movn.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVZ | semantic_ir/tests/movz.rs | 2026-10-16T16:00:00Z
COMPLETED: ORR (immediate) | semantic_ir/tests/orr_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ORN (shifted register) | semantic_ir/tests/orn_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ORN (immediate) | semantic_ir/tests/orn_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: SUB (immediate) | semantic_ir/tests/sub_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: SUBS (immediate) | semantic_ir/tests/subs_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDG | semantic_ir/tests/addg.rs | 2026-10-16T16:00:00Z
COMPLETED: ORN (shifted register) | semantic_ir/tests/orr_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ORN (immediate) | semantic_ir/tests/orr_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: MUL | semantic_ir/tests/mul.rs | 2026-10-16T16:00:00Z
COMPLETED: MADD | semantic_ir/tests/madd.rs | 2026-10-16T16:00:00Z
COMPLETED: MSUB | semantic_ir/tests/msub.rs | 2026-10-16T16:00:00Z
COMPLETED: SMULL | semantic_ir/tests/smull.rs | 2026-10-16T16:00:00Z
COMPLETED: UMULL | semantic_ir/tests/umull.rs | 2026-10-16T16:00:00Z
COMPLETED: SDIV | semantic_ir/tests/sdiv.rs | 2026-10-16T16:00:00Z
COMPLETED: UDIV | semantic_ir/tests/udiv.rs | 2026-10-16T16:00:00Z
COMPLETED: LDR (register offset) | semantic_ir/tests/ldr_reg_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: STR (register offset) | semantic_ir/tests/str_reg_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDR (immediate offset) | semantic_ir/tests/ldr_imm_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: STR (immediate offset) | semantic_ir/tests/str_imm_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: B | semantic_ir/tests/b.rs | 2026-10-16T16:00:00Z
COMPLETED: BL | semantic_ir/tests/bl.rs | 2026-10-16T16:00:00Z
COMPLETED: BR | semantic_ir/tests/br.rs | 2026-10-16T16:00:00Z
COMPLETED: CBZ | semantic_ir/tests/cbz.rs | 2026-10-16T16:00:00Z
COMPLETED: CBNZ | semantic_ir/tests/cbnz.rs | 2026-10-16T16:00:00Z
COMPLETED: TBZ | semantic_ir/tests/tbz.rs | 2026-10-16T16:00:00Z
COMPLETED: TBNZ | semantic_ir/tests/tbnz.rs | 2026-10-16T16:00:00Z
COMPLETED: CCMP (immediate) | semantic_ir/tests/ccmp_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: CCMP (register) | semantic_ir/tests/ccmp_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: CCMN (immediate) | semantic_ir/tests/ccmn_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: CCMN (register) | semantic_ir/tests/ccmn_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: CSET | semantic_ir/tests/cset.rs | 2026-10-16T16:00:00Z
COMPLETED: CSINC | semantic_ir/tests/csinc.rs | 2026-10-16T16:00:00Z
COMPLETED: CSINV | semantic_ir/tests/csinv.rs | 2026-10-16T16:00:00Z
COMPLETED: CSNEG | semantic_ir/tests/csneg.rs | 2026-10-16T16:00:00Z
COMPRETED: LDP (offset) | semantic_ir/tests/ldp_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDP (pre-index) | semantic_ir/tests/ldp_pre_idx.rs | 2026-10-16T16:00:00Z
COMPLETED: LDP (post-index) | semantic_ir/tests/ldp_post_idx.rs | 2026-10-16T16:00:00Z
COMPLETED: STP (offset) | semantic_ir/tests/stp_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: STP (pre-index) | semantic_ir/tests/stp_pre_idx.rs | 2026-10-16T16:00:00Z
COMPLETED: STP (post-index) | semantic_ir/tests/stp_post_idx.rs | 2026-10-16T16:00:00Z
COMPLETED: LDRB (unsigned offset) | semantic_ir/tests/ldrb_unsigned.rs | 2026-10-16T16:00:00Z
COMPLETED: LDRSB | semantic_ir/tests/ldrsb.rs | 2026-10-16T16:00:00Z
COMPLETED: STRB | semantic_ir/tests/strb.rs | 2026-10-16T16:00:00Z
COMPLETED: LDRH (unsigned offset) | semantic_ir/tests/ldrh.rs | 2026-10-16T16:00:00Z
COMPLETED: LDRSH | semantic_ir/tests/ldrsh.rs | 2026-10-16T16:00:00Z
COMPLETED: STRH | semantic_ir/tests/strh.rs | 2026-10-16T16:00:00Z
COMPLETED: LDRSW | semantic_ir/tests/ldrsw.rs | 2026-10-16T16:00:00Z
COMPLETED: LDUR (register offset) | semantic_ir/tests/ldur_reg_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: STUR | semantic_ir/tests/stur.rs | 2026-10-16T16:00:00Z
COMPLETED: LDURB | semantic_ir/tests/ldurb.rs | 2026-10-16T16:00:00Z
COMPLETED: STURB | semantic_ir/tests/sturb.rs | 2026-10-16T16:00:00Z
COMPLETED: LDURH | semantic_ir/tests/ldurh.rs | 2026-10-16T16:00:00Z
COMPLETED: STURH | semantic_ir/tests/sturh.rs | 2026-10-16T16:00:00Z
COMPLETED: LDURSB | semantic_ir/tests/ldursb.rs | 2026-10-16T16:00:00Z
COMPLETED: LDURSH | semantic_ir/tests/ldursh.rs | 2026-10-16T16:00:00Z
COMPLETED: LDURSW | semantic_ir/tests/ldursw.rs | 2026-10-16T16:00:00Z
COMPLETED: PRFM (literal) | semantic_ir/tests/prfm_literal.rs | 2026-10-16T16:00:00Z
COMPLETED: PRFM (register) | semantic_ir/tests/prfm_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: PLD | semantic_ir/tests/pld.rs | 2026-10-16T16:00:00Z
COMPLETED: PLI | semantic_ir/tests/pli.rs | 2026-10-16T16:00:00Z
COMPLETED: PMEM | semantic_ir/tests/pmem.rs | 2026-10-16T16:00:00Z
COMPLETED: CLZ | semantic_ir/tests/clz.rs | 2026-10-16T16:00:00Z
COMPLETED: CLS | semantic_ir/tests/cls.rs | 2026-10-16T16:00:00Z
COMPLETED: RBIT | semantic_ir/tests/rbit.rs | 2026-10-16T16:00:00Z
COMPLETED: REV | semantic_ir/tests/rev.rs | 2026-10-16T16:00:00Z
COMPLETED: REV16 | semantic_ir/tests/rev16.rs | 2026-10-16T16:00:00Z
COMPLETED: REV32 | semantic_ir/tests/rev32.rs | 2026-10-16T16:00:00Z
COMPLETED: SXTB | semantic_ir/tests/sxtb.rs | 2026-10-16T16:00:00Z
COMPLETED: SXTH | semantic_ir/tests/sxth.rs | 2026-10-16T16:00:00Z
COMPLETED: SXTW | semantic_ir/tests/sxtw.rs | 2026-10-16T16:00:00Z
COMPLETED: UXTB | semantic_ir/tests/uxtb.rs | 2026-10-16T16:00:00Z
COMPLETED: UXTH | semantic_ir/tests/uxth.rs | 2026-10-16T16:00:00Z
COMPLETED: UXTW | semantic_ir/tests/uxtw.rs | 2026-10-16T16:00:00Z
COMPLETED: WFE | semantic_ir/tests/wfe.rs | 2026-10-16T16:00:00Z
COMPLETED: WFI | semantic_ir/tests/wfi.rs | 2026-10-16T16:00:00Z
COMPLETED: YIELD | semantic_ir/tests/yield.rs | 2026-10-16T16:00:00Z
COMPLETED: NOP | semantic_ir/tests/nop.rs | 2026-10-16T16:00:00Z
COMPLETED: HLT | semantic_ir/tests/hlt.rs | 2026-10-16T16:00:00Z
COMPLETED: BRK | semantic_ir/tests/brk.rs | 2026-10-16T16:00:00Z
COMPLETED: SVC | semantic_ir/tests/svc.rs | 2026-10-16T16:00:00Z
COMPLETED: HVC | semantic_ir/tests/hvc.rs | 2026-10-16T16:00:00Z
COMPLETED: SMC | semantic_ir/tests/smc.rs | 2026-10-16T16:00:00Z
COMPLETED: DCPS1 | semantic_ir/tests/dcps1.rs | 2026-10-16T16:00:00Z
COMPLETED: DCPS2 | semantic_ir/tests/dcps2.rs | 2026-10-16T16:00:00Z
COMPLETED: DCPS3 | semantic_ir/tests/dcps3.rs | 2026-10-16T16:00:00Z
COMPLETED: ERSB | semantic_ir/tests/ersb.rs | 2026-10-16T16:00:00Z
COMPLETED: SB | semantic_ir/tests/sb.rs | 2026-10-16T16:00:00Z
COMPLETED: CSDB | semantic_ir/tests/csdb.rs | 2026-10-16T16:00:00Z
COMPLETED: PSSBB | semantic_ir/tests/pssbb.rs | 2026-10-16T16:00:00Z
COMPLETED: TSB CSYNC | semantic_ir/tests/tsb_csync.rs | 2026-10-16T16:00:00Z
COMPLETED: DMB | semantic_ir/tests/dmb.rs | 2026-10-16T16:00:00Z
COMPLETED: DSB | semantic_ir/tests/dsb.rs | 2026-10-16T16:00:00Z
COMPLETED: ISB | semantic_ir/tests/isb.rs | 2026-10-16T16:00:00Z
COMPLETED: DMB (sy) | semantic_ir/tests/dmb_sy.rs | 2026-10-16T16:00:00Z
COMPLETED: DSB (sy) | semantic_ir/tests/dsb_sy.rs | 2026-10-16T16:00:00Z
COMPLETED: ISB (sy) | semantic_ir/tests/isb_sy.rs | 2026-10-16T16:00:00Z
COMPLETED: CFP | semantic_ir/tests/cfp.rs | 2026-10-16T16:00:00Z
COMPLETED: SIGP | semantic_ir/tests/sigp.rs | 2026-10-16T16:00:00Z
COMPLETED: AT S1E0R | semantic_ir/tests/at_s1e0r.rs | 2026-10-16T16:00:00Z
COMPLETED: AT S1E0W | semantic_ir/tests/at_s1e0w.rs | 2026-10-16T16:00:00Z
COMPLETED: AT S1E1R | semantic_ir/tests/at_s1e1r.rs | 2026-10-16T16:00:00Z
COMPLETED: AT S1E1W | semantic_ir/tests/at_s1e1w.rs | 2026-10-16T16:00:00Z
COMPLETED: AT S12E1R | semantic_ir/tests/at_s12e1r.rs | 2026-10-16T16:00:00Z
COMPLETED: AT S12E1W | semantic_ir/tests/at_s12e1w.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI VMALLE1IS | semantic_ir/tests/tlbi_vmalle1is.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI VAALE1IS | semantic_ir/tests/tlbi_vaale1is.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI VMALLE1 | semantic_ir/tests/tlbi_vmalle1.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI VAALE1 | semantic_ir/tests/tlbi_vaale1.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI ASIDE1IS | semantic_ir/tests/tlbi_aside1is.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI IPAS2E1IS | semantic_ir/tests/tlbi_ipas2e1is.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI IPAS2LE1IS | semantic_ir/tests/tlbi_ipas2le1is.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI VMALLS12E1IS | semantic_ir/tests/tlbi_vmalls12e1is.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI RVAE1IS | semantic_ir/tests/tlbi_rvae1is.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI RVAE1 | semantic_ir/tests/tlbi_rvae1.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI VAAE1IS | semantic_ir/tests/tlbi_vaae1is.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI VAAE1 | semantic_ir/tests/tlbi_vaae1.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI VMALLS1E1IS | semantic_ir/tests/tlbi_vmalls1e1is.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI IPAS2E1 | semantic_ir/tests/tlbi_ipas2e1.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI IPAS2LE1 | semantic_ir/tests/tlbi_ipas2le1.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI VMALLS12E1 | semantic_ir/tests/tlbi_vmalls12e1.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI RVAE2IS | semantic_ir/tests/tlbi_rvae2is.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI RVAE2 | semantic_ir/tests/tlbi_rvae2.rs | 2026-10-16T16:00:00Z
COMPLETED: TLBI VMALLS1E1 | semantic_ir/tests/tlbi_vmalls1e1.rs | 2026-10-16T16:00:00Z
COMPLETED: IC IALLUIS | semantic_ir/tests/ic_ialluis.rs | 2026-10-16T16:00:00Z
COMPLETED: IC IALLU | semantic_ir/tests/ic_iallu.rs | 2026-10-16T16:00:00Z
COMPLETED: IC IVAU | semantic_ir/tests/ic_ivau.rs | 2026-10-16T16:00:00Z
COMPLETED: DC CIVAC | semantic_ir/tests/dc_civac.rs | 2026-10-16T16:00:00Z
COMPLETED: DC CSW | semantic_ir/tests/dc_csw.rs | 2026-10-16T16:00:00Z
COMPLETED: DC CVAC | semantic_ir/tests/dc_cvac.rs | 2026-10-16T16:00:00Z
COMPLETED: DC CVAU | semantic_ir/tests/dc_cvau.rs | 2026-10-16T16:00:00Z
COMPLETED: DC IGVAC | semantic_ir/tests/dc_igvac.rs | 2026-10-16T16:00:00Z
COMPLETED: DC ISW | semantic_ir/tests/dc_isw.rs | 2026-10-16T16:00:00Z
COMPLETED: DC CVAP | semantic_ir/tests/dc_cvap.rs | 2026-10-16T16:00:00Z
COMPLETED: DC CVADP | semantic_ir/tests/dc_cvadp.rs | 2026-10-16T16:00:00Z
COMPLETED: DC CGVAC | semantic_ir/tests/dc_cgvac.rs | 2026-10-16T16:00:00Z
COMPLETED: DC CGVADP | semantic_ir/tests/dc_cgvadp.rs | 2026-10-16T16:00:00Z
COMPLETED: DC IGVADP | semantic_ir/tests/dc_igvadp.rs | 2026-10-16T16:00:00Z
COMPLETED: DC IGSW | semantic_ir/tests/dc_igsw.rs | 2026-10-16T16:00:00Z
COMPLETED: NEG | semantic_ir/tests/neg.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDLP | semantic_ir/tests/saddlp.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDLP | semantic_ir/tests/uaddlp.rs | 2026-10-16T16:00:00Z
COMPLETED: SQADD | semantic_ir/tests/sqadd.rs | 2026-10-16T16:00:00Z
COMPLETED: UQADD | semantic_ir/tests/uqadd.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSUB | semantic_ir/tests/sqsub.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSUB | semantic_ir/tests/uqsub.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHL | semantic_ir/tests/sqshl.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHL | semantic_ir/tests/uqshl.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHLU | semantic_ir/tests/sqshlu.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHRN | semantic_ir/tests/sqshrn.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHRN | semantic_ir/tests/uqshrn.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTN | semantic_ir/tests/sqxtn.rs | 2026-10-16T16:00:00Z
COMPLETED: UQXTN | semantic_ir/tests/uqxtn.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTN2 | semantic_ir/tests/sqxtn2.rs | 2026-10-16T16:00:00Z
COMPLETED: UQXTN2 | semantic_ir/tests/uqxtn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTUN | semantic_ir/tests/sqxtun.rs | 2026-10-16T16:00:00Z
COMPLETED: SSHR | semantic_ir/tests/sshr.rs | 2026-10-16T16:00:00Z
COMPLETED: USHR | semantic_ir/tests/ushr.rs | 2026-10-16T16:00:00Z
COMPLETED: SRSHR | semantic_ir/tests/srshr.rs | 2026-10-16T16:00:00Z
COMPLETED: URSHR | semantic_ir/tests/urshr.rs | 2026-10-16T16:00:00Z
COMPLETED: SSRA | semantic_ir/tests/ssra.rs | 2026-10-16T16:00:00Z
COMPLETED: USRA | semantic_ir/tests/usra.rs | 2026-10-16T16:00:00Z
COMPLETED: SMAX | semantic_ir/tests/smax.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAX | semantic_ir/tests/umax.rs | 2026-10-16T16:00:00Z
COMPLETED: SMIN | semantic_ir/tests/smin.rs | 2026-10-16T16:00:00Z
COMPLETED: UMIN | semantic_ir/tests/umin.rs | 2026-10-16T16:00:00Z
COMPLETED: SMAXV | semantic_ir/tests/smaxv.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAXV | semantic_ir/tests/umaxv.rs | 2026-10-16T16:00:00Z
COMPLETED: SMINV | semantic_ir/tests/sminv.rs | 2026-10-16T16:00:00Z
COMPLETED: UMINV | semantic_ir/tests/uminv.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDP (scalar) | semantic_ir/tests/addp_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDP (vector) | semantic_ir/tests/addp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDLV | semantic_ir/tests/uaddlv.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDV | semantic_ir/tests/saddv.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAX | semantic_ir/tests/fmax.rs | 2026-10-16T16:00:00Z
COMPLETED: FMIN | semantic_ir/tests/fmin.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXV | semantic_ir/tests/fmaxv.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINV | semantic_ir/tests/fminv.rs | 2026-10-16T16:00:00Z
COMPLETED: FADDP (scalar) | semantic_ir/tests/faddp_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FADDP (vector) | semantic_ir/tests/faddp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAD | semantic_ir/tests/fmad.rs | 2026-10-16T16:00:00Z
COMPLETED: FMSUB | semantic_ir/tests/fmsub.rs | 2026-10-16T16:00:00Z
COMPLETED: FMULX | semantic_ir/tests/fmulx.rs | 2026-10-16T16:00:0Z
COMPLETED: FRECPE | semantic_ir/tests/frecpe.rs | 2026-10-16T16:00:00Z
COMPLETED: FRECPS | semantic_ir/tests/frecps.rs | 2026-10-16T16:00:00Z
COMPLETED: FRSQRTE | semantic_ir/tests/frsqrte.rs | 2026-10-16T16:00:00Z
COMPLETED: FRSQRTS | semantic_ir/tests/frsqrts.rs | 2026-10-16T16:00:00Z
COMPLETED: FSQRT | semantic_ir/tests/fsqrt.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVT | semantic_ir/tests/fcvt.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZS | semantic_ir/tests/fcvtzs.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU | semantic_ir/tests/fcvtzu.rs | 2026-10-16T16:00:00Z
COMPLETED: FJCVTZS | semantic_ir/tests/fjcvtzs.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF | semantic_ir/tests/scvf.rs | 2026-10-16T16:00:00Z
COMPLETED: UCVTF | semantic_ir/tests/ucvf.rs | 2026-10-16T16:00:00Z
COMPLETED: FADD (vector) | semantic_ir/tests/fadd_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FSUB (vector) | semantic_ir/tests/fsub_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMUL (vector) | semantic_ir/tests/fmul_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FDIV (vector) | semantic_ir/tests/fdiv_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FADD (scalar) | semantic_ir/tests/fadd_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FSUB (scalar) | semantic_ir/tests/fsub_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMUL (scalar) | semantic_ir/tests/fmul_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FDIV (scalar) | semantic_ir/tests/fdiv_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FNEG | semantic_ir/tests/fneg.rs | 2026-10-16T16:00:00Z
COMPLETED: FABS | semantic_ir/tests/fabs.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMP | semantic_ir/tests/fcmp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMPE | semantic_ir/tests/fcmpe.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMP (vector) | semantic_ir/tests/fcmp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMPE (vector) | semantic_ir/tests/fcmpe_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTMS | semantic_ir/tests/fcvtms.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (scalar) | semantic_ir/tests/fcvtzu_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTMS (vector) | semantic_ir/tests/fcvtms_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZS (scalar) | semantic_ir/tests/fcvtzs_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (scalar) | semantic_ir/tests/scvf_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: UCVTF (scalar) | semantic_ir/tests/ucvf_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (vector) | semantic_ir/tests/scvf_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UCVTF (vector) | semantic_ir/tests/ucvf_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FDIV | semantic_ir/tests/fdiv.rs | 2026-10-16T16:00:00Z
COMPLETED: FMUL | semantic_ir/tests/fmul.rs | 2026-10-16T16:00:00Z
COMPLETED: FNEG (vector) | semantic_ir/tests/fneg_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FABS (vector) | semantic_ir/tests/fabs_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAX (vector) | semantic_ir/tests/fmax_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMIN (vector) | semantic_ir/tests/fmin_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FADDV | semantic_ir/tests/faddv.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLA | semantic_ir/tests/fmla.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLS | semantic_ir/tests/fmls.rs | 2026-10-16T16:00:00Z
COMPLETED: FMMLA | semantic_ir/tests/fmmla.rs | 2026-10-16T16:00:00Z
COMPLETED: FADD (scalar, half) | semantic_ir/tests/fadd_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FSUB (scalar, half) | semantic_ir/tests/fsub_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FMUL (scalar, half) | semantic_ir/tests/fmul_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FDIV (scalar, half) | semantic_ir/tests/fdiv_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZS (half) | semantic_ir/tests/fcvtzs_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (half) | semantic_ir/tests/fcvtzu_half.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (half) | semantic_ir/tests/scvf_half.rs | 2026-10-16T16:00:00Z
COMPLETED: UCVTF (half) | semantic_ir/tests/ucvf_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOV (register) | semantic_ir/tests/fmov_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOV (immediate) | semantic_ir/tests/fmov_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: FNEG (scalar) | semantic_ir/tests/fneg_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FABS (scalar) | semantic_ir/tests/fabs_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMP (scalar) | semantic_ir/tests/fcmp_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMPE (scalar) | semantic_ir/tests/fcmpe_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLA (vector) | semantic_ir/tests/fmla_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLS (vector) | semantic_ir/tests/fmls_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FADD (vector, half) | semantic_ir/tests/fadd_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FSUB (vector, half) | semantic_ir/tests/fsub_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FMUL (vector, half) | semantic_ir/tests/fmul_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FDIV (vector, half) | semantic_ir/tests/fdiv_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLA (scalar) | semantic_ir/tests/fmla_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLS (scalar) | semantic_ir/tests/fmls_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLA (vector, half) | semantic_ir/tests/fmla_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLS (vector, half) | semantic_ir/tests/fmls_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLA (scalar, half) | semantic_ir/tests/fmla_scalar_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLS (scalar, half) | semantic_ir/tests/fmls_scalar_half.rs | 2026-10-16T16:00:00Z
COMPLETED: SQADD (vector) | semantic_ir/tests/sqadd_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQADD (vector) | semantic_ir/tests/uqadd_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSUB (vector) | semantic_ir/tests/sqsub_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSUB (vector) | semantic_ir/tests/uqsub_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHL (vector) | semantic_ir/tests/sqshl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHL (vector) | semantic_ir/tests/uqshl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHL (scalar) | semantic_ir/tests/sqshl_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHL (scalar) | semantic_ir/tests/uqshl_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHR | semantic_ir/tests/sqshr.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHR | semantic_ir/tests/uqshr.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHRN (vector) | semantic_ir/tests/sqshrn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHRN (vector) | semantic_ir/tests/uqshrn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTN (vector) | semantic_ir/tests/sqxtn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQXTN (vector) | semantic_ir/tests/uqxtn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTUN | semantic_ir/tests/sqxtun_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTUN2 | semantic_ir/tests/sqxtun2.rs | 2026-10-16T16:00:00Z
COMPLETED: SSHR (vector) | semantic_ir/tests/sshr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USHR (vector) | semantic_ir/tests/ushr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SSRA (vector) | semantic_ir/tests/ssra_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USRA (vector) | semantic_ir/tests/usra_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SRSHR (vector) | semantic_ir/tests/srshr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: URSHR (vector) | semantic_ir/tests/urshr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMAX (vector) | semantic_ir/tests/smax_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAX (vector) | semantic_ir/tests/umax_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMIN (vector) | semantic_ir/tests/smin_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMIN (vector) | semantic_ir/tests/umin_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDLV | semantic_ir/tests/uaddlv_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDV | semantic_ir/tests/saddv_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SHLL | semantic_ir/tests/shll.rs | 2026-10-16T16:00:00Z
COMPLETED: UHADD | semantic_ir/tests/uhadd.rs | 2026-10-16T16:00:00Z
COMPLETED: UHSUB | semantic_ir/tests/uhsub.rs | 2026-10-16T16:00:00Z
COMPLETED: SHADD | semantic_ir/tests/shadd.rs | 2026-10-16T16:00:00Z
COMPLETED: SHSUB | semantic_ir/tests/shsub.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDLP2 | semantic_ir/tests/saddlp2.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDLP2 | semantic_ir/tests/uaddlp2.rs | 2026-10-16T16:00:00Z
COMPLETED: UQADDLP | semantic_ir/tests/uqaddlp.rs | 2026-10-16T16:00:00Z
COMPLETED: SQADDLP | semantic_ir/tests/sqaddlp.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSUBLP | semantic_ir/tests/uqsublp.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSUBLP | semantic_ir/tests/sqsublp.rs | 2026-10-16T16:00:00Z
COMPLETED: SABAL | semantic_ir/tests/sabal.rs | 2026-10-16T16:00:00Z
COMPLETED: SABDL | semantic_ir/tests/sabdl.rs | 2026-10-16T16:00:00Z
COMPLETED: UABAL | semantic_ir/tests/uabal.rs | 2026-10-16T16:00:00Z
COMPLETED: UABDL | semantic_ir/tests/uabdl.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDLP | semantic_ir/tests/saddlp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDLP | semantic_ir/tests/uaddlp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDV | semantic_ir/tests/saddv_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXP | semantic_ir/tests/fmaxp.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINP | semantic_ir/tests/fminp.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXP (vector) | semantic_ir/tests/fmaxp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINP (vector) | semantic_ir/tests/fminp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDLV | semantic_ir/tests/uaddlv_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SHLL2 | semantic_ir/tests/shll2.rs | 2026-10-16T16:00:00Z
COMPLETED: UHADD | semantic_ir/tests/uhadd_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UHSUB | semantic_ir/tests/uhsub_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SHADD | semantic_ir/tests/shadd_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SHSUB | semantic_ir/tests/shsub_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SABDL2 | semantic_ir/tests/sabdl2.rs | 2026-10-16T16:00:00Z
COMPLETED: UABDL2 | semantic_ir/tests/uabdl2.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDLP2 | semantic_ir/tests/saddlp2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDLP2 | semantic_ir/tests/uaddlp2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SABAL | semantic_ir/tests/sabal_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UABAL | semantic_ir/tests/uabal_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQADDLP | semantic_ir/tests/sqaddlp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQADDLP | semantic_ir/tests/uqaddlp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSUBLP | semantic_ir/tests/sqsublp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSUBLP | semantic_ir/tests/uqsublp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQXTN | semantic_ir/tests/uqxtn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTN | semantic_ir/tests/sqxtn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTUN2 | semantic_ir/tests/sqxtun2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQXTN2 | semantic_ir/tests/uqxtn2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTN2 | semantic_ir/tests/sqxtn2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SSHR | semantic_ir/tests/sshr_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: USHR | semantic_ir/tests/ushr_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SSRA | semantic_ir/tests/ssra_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: USRA | semantic_ir/tests/usra_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SRSHR | semantic_ir/tests/srshr_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: URSHR | semantic_ir/tests/urshr_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SSHL | semantic_ir/tests/sshll.rs | 2026-10-16T16:00:00Z
COMPLETED: USHL | semantic_ir/tests/ushll.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHL | semantic_ir/tests/sqshl_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHL | semantic_ir/tests/uqshl_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SMAX | semantic_ir/tests/smax_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAX | semantic_ir/tests/umax_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SMIN | semantic_ir/tests/smin_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: UMIN | semantic_ir/tests/umin_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHRN | semantic_ir/tests/sqshrn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHRN | semantic_ir/tests/uqshrn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHRN2 | semantic_ir/tests/sqshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHRN2 | semantic_ir/tests/uqshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHRN | semantic_ir/tests/sqshrn_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHRN | semantic_ir/tests/uqshrn_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLA | semantic_ir/tests/fmla_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLS | semantic_ir/tests/fmls_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMMLA | semantic_ir/tests/fmmla_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMULX | semantic_ir/tests/fmulx_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRECPE | semantic_ir/tests/frecpe_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRECPS | semantic_ir/tests/frecps_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRSQRTE | semantic_ir/tests/frsqrte_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRSQRTS | semantic_ir/tests/frsqrts_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FSQRT | semantic_ir/tests/fsqrt_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRECPE (scalar) | semantic_ir/tests/frecpe_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRECPS (scalar) | semantic_ir/tests/frecps_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRSQRTE (scalar) | semantic_ir/tests/frsqrte_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRSQRTS (scalar) | semantic_ir/tests/frsqrts_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FSQRT (scalar) | semantic_ir/tests/fsqrt_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVT | semantic_ir/tests/fcvt_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVT (scalar) | semantic_ir/tests/fcvt_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZS | semantic_ir/tests/fcvtzs_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU | semantic_ir/tests/fcvtzu_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF | semantic_ir/tests/scvf_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UCVTF | semantic_ir/tests/ucvf_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FNEG | semantic_ir/tests/fneg_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FABS | semantic_ir/tests/fabs_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMP | semantic_ir/tests/fcmp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMPE | semantic_ir/tests/fcmpe_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FADD | semantic_ir/tests/fadd_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FSUB | semantic_ir/tests/fsub_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMUL | semantic_ir/tests/fmul_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FDIV | semantic_ir/tests/fdiv_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMADD | semantic_ir/tests/fmadd.rs | 2026-10-16T16:00:00Z
COMPLETED: FMADD (scalar) | semantic_ir/tests/fmadd_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMSUB (scalar) | semantic_ir/tests/fmsub_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAX | semantic_ir/tests/fmax_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAX (scalar) | semantic_ir/tests/fmax_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMIN | semantic_ir/tests/fmin_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMIN (scalar) | semantic_ir/tests/fmin_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOV (vector) | semantic_ir/tests/fmov_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOV (scalar) | semantic_ir/tests/fmov_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOV (register, scalar) | semantic_ir/tests/fmov_reg_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOV (immediate, scalar) | semantic_ir/tests/fmov_imm_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMULX (scalar) | semantic_ir/tests/fmulx_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FADD (scalar, half) | semantic_ir/tests/fadd_scalar_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FSUB (scalar, half) | semantic_ir/tests/fsub_scalar_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FMUL (scalar, half) | semantic_ir/tests/fmul_scalar_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FDIV (scalar, half) | semantic_ir/tests/fdiv_scalar_half.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVT | semantic_ir/tests/fcvt_half.rs | 2026-10-16T16:00:00Z
COMPLETED: SQADD | semantic_ir/tests/sqadd_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQADD | semantic_ir/tests/uqadd_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSUB | semantic_ir/tests/sqsub_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSUB | semantic_ir/tests/uqsub_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHL | semantic_ir/tests/sqshl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHL | semantic_ir/tests/uqshl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHR | semantic_ir/tests/sqshr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHR | semantic_ir/tests/uqshr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHRN2 | semantic_ir/tests/sqshrn2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHRN2 | semantic_ir/tests/uqshrn2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTN | semantic_ir/tests/sqxtn_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: UQXTN | semantic_ir/tests/uqxtn_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTUN | semantic_ir/tests/sqxtun_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SSHR | semantic_ir/tests/sshr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USHR | semantic_ir/tests/ushr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SSRA | semantic_ir/tests/ssra_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USRA | semantic_ir/tests/usra_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SRSHR | semantic_ir/tests/srshr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: URSHR | semantic_ir/tests/urshr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMAX | semantic_ir/tests/smax_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAX | semantic_ir/tests/umax_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMIN | semantic_ir/tests/smin_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMIN | semantic_ir/tests/umin_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SHLL | semantic_ir/tests/shll_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SABDL | semantic_ir/tests/sabdl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UABDL | semantic_ir/tests/uabdl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SABDL2 | semantic_ir/tests/sabdl2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UABDL2 | semantic_ir/tests/uabdl2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SSHL | semantic_ir/tests/sshll_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USHL | semantic_ir/tests/ushll_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHRN2 | semantic_ir/tests/sqshrn2_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHRN2 | semantic_ir/tests/uqshrn2_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHR | semantic_ir/tests/sqshr_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHR | semantic_ir/tests/uqshr_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: LDUR | semantic_ir/tests/ldur_reg_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: FMULX | semantic_ir/tests/fmulx.rs | 2026-10-16T16:00:00Z
COMPLETED: VRSHL | semantic_ir/tests/vrshl.rs | 2026-10-16T16:00:00Z
COMPLETED: VRSQRTE | semantic_ir/tests/vrsqrte.rs | 2026-10-16T16:00:00Z
COMPLETED: VRSRA | semantic_ir/tests/vrsra.rs | 2026-10-16T16:00:00Z
COMPLETED: VSHLL | semantic_ir/tests/vshll_vshll2.rs | 2026-10-16T16:00:00Z
COMPLETED: VSHRN | semantic_ir/tests/vshrn_vshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: VSQRT | semantic_ir/tests/vsqrt_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VSUB | semantic_ir/tests/vsub_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VTRN1 | semantic_ir/tests/vtrn1.rs | 2026-10-16T16:00:00Z
COMPLETED: VTRN2 | semantic_ir/tests/vtrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: VUZP1 | semantic_ir/tests/vuzp1.rs | 2026-10-16T16:00:00Z
COMPLETED: VUZP2 | semantic_ir/tests/vuzp2.rs | 2026-10-16T16:00:00Z
COMPLETED: XAFLAG | semantic_ir/tests/xaflag.rs | 2026-10-16T16:00:00Z
COMPLETED: XPACD | semantic_ir/tests/xpacd.rs | 2026-10-16T16:00:00Z
COMPLETED: XPACI | semantic_ir/tests/xpaci.rs | 2026-10-16T16:00:00Z
COMPLETED: XPACLRI | semantic_ir/tests/xpaclri.rs | 2026-10-16T16:00:00Z
COMPLETED: XAR | semantic_ir/tests/xar.rs | 2026-10-16T16:00:00Z
COMPLETED: WFET | semantic_ir/tests/wfet.rs | 2026-10-16T16:00:00Z
COMPLETED: WFIT | semantic_ir/tests/wfit.rs | 2026-10-16T16:00:00Z
COMPLETED: VADD | semantic_ir/tests/vadd_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VSUB | semantic_ir/tests/vsub_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VMUL | semantic_ir/tests/vmul_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VDIV | semantic_ir/tests/vdiv_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VMAX | semantic_ir/tests/vmax_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VMIN | semantic_ir/tests/vmin_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VNEG | semantic_ir/tests/vneg_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VADD | semantic_ir/tests/vadd_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VMUL | semantic_ir/tests/vmul_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VDIV | semantic_ir/tests/vdiv_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VMAX | semantic_ir/tests/vmax_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VMIN | semantic_ir/tests/vmin_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VABS | semantic_ir/tests/vabs_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VABS | semantic_ir/tests/vabs_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VNEG | semantic_ir/tests/vneg_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VADD (vector, half) | semantic_ir/tests/vadd_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: VSUB (vector, half) | semantic_ir/tests/vsub_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: VMUL (vector, half) | semantic_ir/tests/vmul_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: VDIV (vector, half) | semantic_ir/tests/vdiv_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: VMAX (vector, half) | semantic_ir/tests/vmax_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: VMIN (vector, half) | semantic_ir/tests/vmin_vector_half.rs | 2026-10-16T16:00:00Z
COMPLETED: VMUL (scalar, half) | semantic_ir/tests/vmul_scalar_half.rs | 2026-10-16T16:00:00Z
COMPLETED: VADD (scalar, half) | semantic_ir/tests/vadd_scalar_half.rs | 2026-10-16T16:00:00Z
COMPLETED: VSUB (scalar, half) | semantic_ir/tests/vsub_scalar_half.rs | 2026-10-16T16:00:00Z
COMPLETED: VMAX (scalar, half) | semantic_ir/tests/vmax_scalar_half.rs | 2026-10-16T16:00:00Z
COMPLETED: VMIN (scalar, half) | semantic_ir/tests/vmin_scalar_half.rs | 2026-10-16T16:00:00Z
COMPLETED: VAND | semantic_ir/tests/vand_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VORR | semantic_ir/tests/vorr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VEOR | semantic_ir/tests/veor_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VBIC | semantic_ir/tests/vbic_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VMVN | semantic_ir/tests/vmvn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VAND (scalar) | semantic_ir/tests/vand_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VORR (scalar) | semantic_ir/tests/vorr_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VEOR (scalar) | semantic_ir/tests/veor_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VBIC (scalar) | semantic_ir/tests/vbic_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VMVN (scalar) | semantic_ir/tests/vmvn_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VBSL | semantic_ir/tests/vbsl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VBSL (scalar) | semantic_ir/tests/vbsl_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VORN | semantic_ir/tests/vorn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VORN (scalar) | semantic_ir/tests/vorn_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VBIF | semantic_ir/tests/vbif_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VBIF (scalar) | semantic_ir/tests/vbif_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VBIT | semantic_ir/tests/vbit_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VBIT (scalar) | semantic_ir/tests/vbit_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VSLI | semantic_ir/tests/vsli_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VSLI (scalar) | semantic_ir/tests/vsli_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VSHL | semantic_ir/tests/vshl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VSHL (scalar) | semantic_ir/tests/vshl_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VSHLL | semantic_ir/tests/vshll_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VSHLL (scalar) | semantic_ir/tests/vshll_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VSHRN | semantic_ir/tests/vshrn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VSHRN (scalar) | semantic_ir/tests/vshrn_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VSQRT (scalar) | semantic_ir/tests/vsqrt_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VSQRT (sve) | semantic_ir/tests/vsqrt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VRSHL | semantic_ir/tests/vrshl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VRSHL (scalar) | semantic_ir/tests/vrshl_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VRSQRTE | semantic_ir/tests/vrsqrte_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VRSQRTE (scalar) | semantic_ir/tests/vrsqrte_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VRSQRTE (sve) | semantic_ir/tests/vrsqrte_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VRSRA | semantic_ir/tests/vrsra_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VRSRA (scalar) | semantic_ir/tests/vrsra_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VTRN1 | semantic_ir/tests/vtrn1_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VTRN2 | semantic_ir/tests/vtrn2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VUZP1 | semantic_ir/tests/vuzp1_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VUZP2 | semantic_ir/tests/vuzp2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VTRN1 (sve) | semantic_ir/tests/vtrn1_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VTRN2 (sve) | semantic_ir/tests/vtrn2_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VUZP1 (sve) | semantic_ir/tests/vuzp1_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VUZP2 (sve) | semantic_ir/tests/vuzp2_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VZIP1 | semantic_ir/tests/vzip1_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VZIP2 | semantic_ir/tests/vzip2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VZIP1 (sve) | semantic_ir/tests/vzip1_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VZIP2 (sve) | semantic_ir/tests/vzip2_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: XAR (sve) | semantic_ir/tests/xar_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LDP | semantic_ir/tests/ldp_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: STP | semantic_ir/tests/stp_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDR | semantic_ir/tests/ldr_reg_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: STR | semantic_ir/tests/str_reg_offset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDRB | semantic_ir/tests/ldrb_unsigned.rs | 2026-10-16T16:00:00Z
COMPLETED: LDRH | semantic_ir/tests/ldrh.rs | 2026-10-16T16:00:00Z
COMPLETED: PRFM | semantic_ir/tests/prfm_literal.rs | 2026-10-16T16:00:00Z
COMPLETED: MADDPT | semantic_ir/tests/maddpt.rs | 2026-10-16T16:00:00Z
COMPLETED: FADDP | semantic_ir/tests/faddp.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOV | semantic_ir/tests/fmov_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ABS (merging) | semantic_ir/tests/abs_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADCLB | semantic_ir/tests/adclb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADCLT | semantic_ir/tests/adclt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_imm_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_ext_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD (to array, array and multiple vectors) | semantic_ir/tests/sme_add_to_array_array_and_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD (to array, multiple and single vector) | semantic_ir/tests/sme_add_to_array_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD (to array, multiple vectors) | semantic_ir/tests/sme_add_to_array_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD (to vector, multiple vectors) | semantic_ir/tests/sme_add_to_vector_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDHN | semantic_ir/tests/addhn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDHNB | semantic_ir/tests/addhnb_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDHNT | semantic_ir/tests/addhnt_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDP | semantic_ir/tests/addp_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDP | semantic_ir/tests/addp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDP | semantic_ir/tests/addp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDP | semantic_ir/tests/addp_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDPL | semantic_ir/tests/addpl_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDQV | semantic_ir/tests/addqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDS | semantic_ir/tests/adds_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDS | semantic_ir/tests/adds_ext_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDS | semantic_ir/tests/adds_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDV | semantic_ir/tests/addv_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDVL | semantic_ir/tests/addvl_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: AESD | semantic_ir/tests/aesd_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: AESD | semantic_ir/tests/aesd_sve_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: AESD | semantic_ir/tests/aesd_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: AESDIMC | semantic_ir/tests/aesdimc_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: AESE | semantic_ir/tests/aese_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: AESE | semantic_ir/tests/aese_sve_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: AESE | semantic_ir/tests/aese_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: AESEMC | semantic_ir/tests/aesemc_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: AESIMC | semantic_ir/tests/aesimc_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: AESIMC | semantic_ir/tests/aesimc_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: AESMC | semantic_ir/tests/aesmc_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: AESMC | semantic_ir/tests/aesmc_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDQV | semantic_ir/tests/andqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDS | semantic_ir/tests/ands_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDS | semantic_ir/tests/ands_sve_predicates.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDS | semantic_ir/tests/ands_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDV | semantic_ir/tests/andv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: APAS | semantic_ir/tests/apas.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_vectors_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_sve_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_vectors_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_wide_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_wide_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ABS (merging) | semantic_ir/tests/abs_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADCLB | semantic_ir/tests/adclb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADCLT | semantic_ir/tests/adclt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_imm_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_ext_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD | semantic_ir/tests/add_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD (to array, array and multiple vectors) | semantic_ir/tests/sme_add_to_array_array_and_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD (to array, multiple and single vector) | semantic_ir/tests/sme_add_to_array_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD (to array, multiple vectors) | semantic_ir/tests/sme_add_to_array_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: ADD (to vector, multiple vectors) | semantic_ir/tests/sme_add_to_vector_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDHN | semantic_ir/tests/addhn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDHNB | semantic_ir/tests/addhnb_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDHNT | semantic_ir/tests/addhnt_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDP | semantic_ir/tests/addp_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDP | semantic_ir/tests/addp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDP | semantic_ir/tests/addp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDP | semantic_ir/tests/addp_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDPL | semantic_ir/tests/addpl_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDQV | semantic_ir/tests/addqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDS | semantic_ir/tests/adds_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDS | semantic_ir/tests/adds_ext_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDS | semantic_ir/tests/adds_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDV | semantic_ir/tests/addv_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDVL | semantic_ir/tests/addvl_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: AESD | semantic_ir/tests/aesd_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: AESD | semantic_ir/tests/aesd_sve_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: AESD | semantic_ir/tests/aesd_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: AESDIMC | semantic_ir/tests/aesdimc_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: AESE | semantic_ir/tests/aese_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: AESE | semantic_ir/tests/aese_sve_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: AESE | semantic_ir/tests/aese_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: AESEMC | semantic_ir/tests/aesemc_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: AESIMC | semantic_ir/tests/aesimc_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: AESIMC | semantic_ir/tests/aesimc_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: AESMC | semantic_ir/tests/aesmc_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: AESMC | semantic_ir/tests/aesmc_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDQV | semantic_ir/tests/andqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDS | semantic_ir/tests/ands_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDS | semantic_ir/tests/ands_sve_predicates.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDS | semantic_ir/tests/ands_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ANDV | semantic_ir/tests/andv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: APAS | semantic_ir/tests/apas.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_vectors_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_sve_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_vectors_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_wide_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_wide_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASR | semantic_ir/tests/asr_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASRD | semantic_ir/tests/asrd_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASRD | semantic_ir/tests/asrd_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASRR | semantic_ir/tests/asrr_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASRR | semantic_ir/tests/asrr_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ASRV | semantic_ir/tests/asrv.rs | 2026-10-16T16:00:00Z
COMPLETED: AT | semantic_ir/tests/at.rs | 2026-10-16T16:00:00Z
COMPLETED: AUTDA | semantic_ir/tests/autda.rs | 2026-10-16T16:00:00Z
COMPLETED: AUTDB | semantic_ir/tests/autdb.rs | 2026-10-16T16:00:00Z
COMPLETED: AUTIA | semantic_ir/tests/autia.rs | 2026-10-16T16:00:00Z
COMPLETED: AUTIA171615 | semantic_ir/tests/autia171615.rs | 2026-10-16T16:00:00Z
COMPLETED: AUTIASPPC | semantic_ir/tests/autiasppc.rs | 2026-10-16T16:00:00Z
COMPLETED: AUTIASPPCR | semantic_ir/tests/autiasppcr.rs | 2026-10-16T16:00:00Z
COMPLETED: AUTIB | semantic_ir/tests/autib.rs | 2026-10-16T16:00:00Z
COMPLETED: AUTIB171615 | semantic_ir/tests/autib171615.rs | 2026-10-16T16:00:00Z
COMPLETED: AUTIBSPPC | semantic_ir/tests/autibsppc.rs | 2026-10-16T16:00:00Z
COMPLETED: AUTIBSPPCR | semantic_ir/tests/autibsppcr.rs | 2026-10-16T16:00:00Z
COMPLETED: AXFLAG | semantic_ir/tests/axflag.rs | 2026-10-16T16:00:00Z
COMPLETED: B.cond | semantic_ir/tests/b_cond.rs | 2026-10-16T16:00:00Z
COMPLETED: BC.cond | semantic_ir/tests/bc_cond.rs | 2026-10-16T16:00:00Z
COMPLETED: BCAX | semantic_ir/tests/bcax_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: BCAX | semantic_ir/tests/bcax_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BDEP | semantic_ir/tests/bdep_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BEXT | semantic_ir/tests/bext_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BF1CVT | semantic_ir/tests/bf1cvt_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BF1CVTL | semantic_ir/tests/bf1cvtl_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: BF1CVTL, BF2CVTL | semantic_ir/tests/bf1cvtl_bf2cvtl.rs | 2026-10-16T16:00:00Z
COMPLETED: BF1CVTLT | semantic_ir/tests/bf1cvlt_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BF1CVTLT | semantic_ir/tests/bf1cvtlt_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: BFADD | semantic_ir/tests/bfadd_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFADD | semantic_ir/tests/bfadd_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFCLAMP | semantic_ir/tests/bfclamp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: BFCVT | semantic_ir/tests/bfcvt.rs | 2026-10-16T16:00:00Z
COMPLETED: BFCVT | semantic_ir/tests/bfcvt_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFCVT | semantic_ir/tests/bfcvt_float.rs | 2026-10-16T16:00:00Z
COMPLETED: BFCVT (BFloat16 to 8-bit floating-point) | semantic_ir/tests/sme_bfcvt_bfloat16_to_8_bit_floating_point.rs | 2026-10-16T16:00:00Z
COMPLETED: BFCVT (scalar) | semantic_ir/tests/bfcvt_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: BFCVT (single-precision to BFloat16) | semantic_ir/tests/sme_bfcvt_single_precision_to_bfloat16.rs | 2026-10-16T16:00:00Z
COMPLETED: BFCVTN | semantic_ir/tests/bfcvtn_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: BFCVTN | semantic_ir/tests/bfcvtn_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFCVTN, BFCVTN2 | semantic_ir/tests/bfcvtn_bfcvtn2.rs | 2026-10-16T16:00:00Z
COMPLETED: BFDOT | semantic_ir/tests/bfdot_advsimd_elt.rs | 2026-10-16T16:00:00Z
COMPLETED: BFDOT | semantic_ir/tests/bfdot_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: BFDOT | semantic_ir/tests/bfdot_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFDOT (multiple and indexed vector) | semantic_ir/tests/sme_bfdot_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFDOT (multiple and single vector) | semantic_ir/tests/sme_bfdot_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFDOT (multiple vectors) | semantic_ir/tests/sme_bfdot_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: BFM | semantic_ir/tests/bfm.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMAX | semantic_ir/tests/bfmax_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMAX (multiple and single vector) | semantic_ir/tests/sme_bfmax_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMAX (multiple vectors) | semantic_ir/tests/sme_bfmax_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMAXNM | semantic_ir/tests/bfmaxnm_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMAXNM (multiple and single vector) | semantic_ir/tests/sme_bfmaxnm_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMAXNM (multiple vectors) | semantic_ir/tests/sme_bfmaxnm_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMIN | semantic_ir/tests/bfmin_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMIN (multiple and single vector) | semantic_ir/tests/sme_bfmin_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMIN (multiple vectors) | semantic_ir/tests/sme_bfmin_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMINNM | semantic_ir/tests/bfminnm_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMINNM (multiple and single vector) | semantic_ir/tests/sme_bfminnm_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMINNM (multiple vectors) | semantic_ir/tests/sme_bfminnm_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLA | semantic_ir/tests/bfmla_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLA (multiple and indexed vector) | semantic_ir/tests/sme_bfmla_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLA (multiple and single vector) | semantic_ir/tests/sme_bfmla_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLA (multiple vectors) | semantic_ir/tests/sme_bfmla_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLAL (multiple and indexed vector) | semantic_ir/tests/sme_bfmlal_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLAL (multiple and single vector) | semantic_ir/tests/sme_bfmlal_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLAL (multiple vectors) | semantic_ir/tests/sme_bfmlal_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLALB | semantic_ir/tests/bfmlal_advsimd_elt.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLALB | semantic_ir/tests/bfmlal_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLALB, BFMLALT (vector) | semantic_ir/tests/bfmlalb_bfmlalt_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLS (multiple and indexed vector) | semantic_ir/tests/sme_bfmls_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLS (multiple and single vector) | semantic_ir/tests/sme_bfmls_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLS (multiple vectors) | semantic_ir/tests/sme_bfmls_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLSL (multiple and indexed vector) | semantic_ir/tests/sme_bfmlsl_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLSL (multiple and single vector) | semantic_ir/tests/sme_bfmlsl_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMLSL (multiple vectors) | semantic_ir/tests/sme_bfmlsl_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMMLA | semantic_ir/tests/bfmmla_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMUL | semantic_ir/tests/bfmul_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMUL | semantic_ir/tests/bfmul_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMUL (indexed) | semantic_ir/tests/bfmul_sve_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMUL (multiple and single vector) | semantic_ir/tests/sme_bfmul_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFMUL (multiple vectors) | semantic_ir/tests/sme_bfmul_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: BFSCALE | semantic_ir/tests/bfscale_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: BFSCALE (multiple and single vector) | semantic_ir/tests/sme_bfscale_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: BFSCALE (multiple vectors) | semantic_ir/tests/sme_bfscale_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: BFSUB | semantic_ir/tests/bfsub_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BFSUB | semantic_ir/tests/bfsub_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BIC | semantic_ir/tests/bic_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: BIC | semantic_ir/tests/bic_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BIC | semantic_ir/tests/bic_advsimd_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: BIC | semantic_ir/tests/bic_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: BIC | semantic_ir/tests/bic_advsimd_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: BICS | semantic_ir/tests/bics_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: BIF | semantic_ir/tests/bif_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: BIT | semantic_ir/tests/bit_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: BLR | semantic_ir/tests/blr.rs | 2026-10-16T16:00:00Z
COMPLETED: BLRAA | semantic_ir/tests/blraa.rs | 2026-10-16T16:00:00Z
COMPLETED: BRAA | semantic_ir/tests/braa.rs | 2026-10-16T16:00:00Z
COMPLETED: BRB | semantic_ir/tests/brb.rs | 2026-10-16T16:00:00Z
COMPLETED: BSL | semantic_ir/tests/bsl_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: CADD | semantic_ir/tests/cadd_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: CCMN | semantic_ir/tests/ccmn_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: CCMN | semantic_ir/tests/ccmn_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: CCMP | semantic_ir/tests/ccmp_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: CCMP | semantic_ir/tests/ccmp_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: CDOT (vectors) | semantic_ir/tests/cdot_vectors_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: CMEQ | semantic_ir/tests/cmeq_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: CMEQ (zero) | semantic_ir/tests/cmeq_zero.rs | 2026-10-16T16:00:00Z
COMPLETED: CMGE | semantic_ir/tests/cmge_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: CMGE (zero) | semantic_ir/tests/cmge_zero.rs | 2026-10-16T16:00:00Z
COMPLETED: CMGT | semantic_ir/tests/cmgt_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: CMGT (zero) | semantic_ir/tests/cmgt_zero.rs | 2026-10-16T16:00:00Z
COMPLETED: CMHI | semantic_ir/tests/cmhi_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: CMHS | semantic_ir/tests/cmhs_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: CMLE | semantic_ir/tests/cmle_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: CMLE (zero) | semantic_ir/tests/cmle_zero.rs | 2026-10-16T16:00:00Z
COMPLETED: CMLT | semantic_ir/tests/cmlt_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: CMLT (zero) | semantic_ir/tests/cmlt_zero.rs | 2026-10-16T16:00:00Z
COMPLETED: CMN | semantic_ir/tests/cmn_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: CMN | semantic_ir/tests/cmn_ext_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: CMN | semantic_ir/tests/cmn_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: CMP | semantic_ir/tests/cmp_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: CMP | semantic_ir/tests/cmp_ext_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: CMP | semantic_ir/tests/cmp_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: CMP<cc> (vectors) | semantic_ir/tests/cmp_sve_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: CMTST | semantic_ir/tests/cmtst_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: CNOT | semantic_ir/tests/cnot_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: CNOT | semantic_ir/tests/cnot_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: CNTP | semantic_ir/tests/cntp_counter_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: COMPACT | semantic_ir/tests/compact_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: CPY | semantic_ir/tests/cpy_immediate_zeroing_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: CPY_SCALAR | semantic_ir/tests/cpy_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFP | semantic_ir/tests/cpyfp.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPN | semantic_ir/tests/cpyfpn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPRN | semantic_ir/tests/cpyfprn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPRT | semantic_ir/tests/cpyfprt.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPRTN | semantic_ir/tests/cpyfprtn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPRTRN | semantic_ir/tests/cpyfprtrn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPRTWN | semantic_ir/tests/cpyfprtwn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPT | semantic_ir/tests/cpyfpt.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPTN | semantic_ir/tests/cpyfptn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPTRN | semantic_ir/tests/cpyfptrn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPTWN | semantic_ir/tests/cpyfptwn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPWN | semantic_ir/tests/cpyfpwn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPWT | semantic_ir/tests/cpyfpwt.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPWTN | semantic_ir/tests/cpyfpwtn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPWTRN | semantic_ir/tests/cpyfpwtrn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYFPWTWN | semantic_ir/tests/cpyfpwtwn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYP | semantic_ir/tests/cpyp.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPN | semantic_ir/tests/cpypn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPRN | semantic_ir/tests/cpyprn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPRT | semantic_ir/tests/cpyprt.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPRTN | semantic_ir/tests/cpyprtn.rs | 2026-10-16T16:00:00Z
COMPLETED: cpyprtrn | semantic_ir/tests/cpyprtrn.rs | 2026-10-16T16:00:00Z
COMPLETED: cpyprtwn | semantic_ir/tests/cpyprtwn.rs | 2026-10-16T16:00:00Z
COMPLETED: cpypt | semantic_ir/tests/cpypt.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPTN | semantic_ir/tests/cpyptn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPTRN | semantic_ir/tests/cpyptrn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPTWN | semantic_ir/tests/cpyptwn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPWN | semantic_ir/tests/cpypwn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPWT | semantic_ir/tests/cpypwt.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPWTN | semantic_ir/tests/cpypwtn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPWTRN | semantic_ir/tests/cpypwtrn.rs | 2026-10-16T16:00:00Z
COMPLETED: CPYPWTWN | semantic_ir/tests/cpypwtwn.rs | 2026-10-16T16:00:00Z
COMPLETED: CSEL | semantic_ir/tests/csel.rs | 2026-10-16T16:00:00Z
COMPLETED: DECP | semantic_ir/tests/decp_scalar_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: DUP | semantic_ir/tests/dup_advsimd_gen.rs | 2026-10-16T16:00:00Z
COMPLETED: DUP | semantic_ir/tests/dup_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: DUP | semantic_ir/tests/dup_advsimd_elt.rs | 2026-10-16T16:00:00Z
COMPLETED: DUP (element) | semantic_ir/tests/dup_element.rs | 2026-10-16T16:00:00Z
COMPLETED: DUP (general) | semantic_ir/tests/dup_general.rs | 2026-10-16T16:00:00Z
COMPLETED: EON | semantic_ir/tests/eon_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: EON | semantic_ir/tests/eon_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: EOR | semantic_ir/tests/eor_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: EOR | semantic_ir/tests/eor_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: EOR | semantic_ir/tests/eor_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: EOR | semantic_ir/tests/eor_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: EOR | semantic_ir/tests/eor_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: EOR | semantic_ir/tests/eor_sve_predicates.rs | 2026-10-16T16:00:00Z
COMPLETED: EOR3 | semantic_ir/tests/eor3_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: EORQV | semantic_ir/tests/eorqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: EORV | semantic_ir/tests/eorv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ERET | semantic_ir/tests/eret.rs | 2026-10-16T16:00:00Z
COMPLETED: ERETAA | semantic_ir/tests/eretaa.rs | 2026-10-16T16:00:00Z
COMPLETED: EXPAND | semantic_ir/tests/expand_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: EXT | semantic_ir/tests/ext_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: EXT | semantic_ir/tests/ext_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: EXTQ | semantic_ir/tests/extq_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: EXTR | semantic_ir/tests/extr.rs | 2026-10-16T16:00:00Z
COMPLETED: F1CVT | semantic_ir/tests/f1cvt_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: F1CVTL | semantic_ir/tests/f12cvtl_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: F1CVTLT | semantic_ir/tests/f1cvlt_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FABD | semantic_ir/tests/fabd_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FABD | semantic_ir/tests/fabd_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: FACGE | semantic_ir/tests/fac_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FACGE | semantic_ir/tests/facge_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: FACGT | semantic_ir/tests/facgt_advsimd.rs | 2026-10-16T16:00:00Z
COMPLETED: FADDA | semantic_ir/tests/fadda_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FADDQV | semantic_ir/tests/faddqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FAMAX | semantic_ir/tests/famax.rs | 2026-10-16T16:00:00Z
COMPLETED: FAMAX | semantic_ir/tests/famax_famin_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FAMIN | semantic_ir/tests/famin.rs | 2026-10-16T16:00:00Z
COMPLETED: FCADD | semantic_ir/tests/fcadd_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FCADD | semantic_ir/tests/fcadd.rs | 2026-10-16T16:00:00Z
COMPLETED: FCCMP | semantic_ir/tests/fccmp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCCMPE | semantic_ir/tests/fccmpe.rs | 2026-10-16T16:00:00Z
COMPLETED: FCLAMP | semantic_ir/tests/fclamp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMEQ | semantic_ir/tests/fcmeq_sve_predicates.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMEQ (register) | semantic_ir/tests/fcmeq_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMEQ (register) | semantic_ir/tests/fcmeq_register.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMEQ (zero) | semantic_ir/tests/fcmeq_zero.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMGE | semantic_ir/tests/fcm_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMGE (register) | semantic_ir/tests/fcmge_register.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMGE (register) | semantic_ir/tests/fcmge_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMGE (zero) | semantic_ir/tests/fcmge_zero.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMGT (register) | semantic_ir/tests/fcmgt_register.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMGT (register) | semantic_ir/tests/fcmgt_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMGT (zero) | semantic_ir/tests/fcmgt_zero.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMLA | semantic_ir/tests/simd_fp_fcmla.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMLA | semantic_ir/tests/fcmla.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMLA (by element) | semantic_ir/tests/fcmla_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMLA (by element) | semantic_ir/tests/simd_fp_fcmla_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMLA (indexed) | semantic_ir/tests/fcmla_sve_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMLA (vectors) | semantic_ir/tests/fcmla_sve_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMLE (zero) | semantic_ir/tests/fcmle_zero.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMLT (zero) | semantic_ir/tests/fcmlt_zero.rs | 2026-10-16T16:00:00Z
COMPLETED: FCPY | semantic_ir/tests/fcpy_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FCSEL | semantic_ir/tests/fcsel.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVT (narrowing, FP16 to FP8) | semantic_ir/tests/sme_fcvt_narrowing_fp16_to_fp8.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVT (narrowing, FP32 to FP16) | semantic_ir/tests/sme_fcvt_narrowing_fp32_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVT (narrowing, FP32 to FP8) | semantic_ir/tests/sme_fcvt_narrowing_fp32_to_fp8.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTAS (scalar SIMD&FP) | semantic_ir/tests/fcvtas_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTAS (scalar) | semantic_ir/tests/fcvtas_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTAS (vector) | semantic_ir/tests/fcvtas_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTAU (scalar SIMD&FP) | semantic_ir/tests/fcvtau_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTAU (scalar) | semantic_ir/tests/fcvtau_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTAU (vector) | semantic_ir/tests/fcvtau_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTL, FCVTL2 | semantic_ir/tests/fcvtl_fcvtl2.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTLT | semantic_ir/tests/fcvtlt_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTMS (scalar SIMD&FP) | semantic_ir/tests/fcvtms_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTMS (scalar) | semantic_ir/tests/fcvtms_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTMU (scalar SIMD&FP) | semantic_ir/tests/fcvtmu_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTMU (scalar) | semantic_ir/tests/fcvtmu_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTMU (vector) | semantic_ir/tests/fcvtmu_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTN (FP32 to FP16) | semantic_ir/tests/sme_fcvtn_fp32_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTN (FP32 to FP8) | semantic_ir/tests/sme_fcvtn_fp32_to_fp8.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTN, FCVTN2 (single-precision to 8_bit floating-point) | semantic_ir/tests/fcvtn_fcvtn2_single-precision_to_8-bit_floating-point.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTN, FCVTN2 (single-precision to 8-bit floating-point) | semantic_ir/tests/fcvtn_fcvtn2_single_precision_to_8_bit_floating_point.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTN, FCVTN2 (single-precision to 8-bit floating-point) | semantic_ir/tests/simd_fp_fcvtn_fcvtn2_single_precision_to_8_bit_floating_point.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTNB | semantic_ir/tests/fcvtnb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTNS (scalar SIMD&FP) | semantic_ir/tests/fcvtns_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTNS (scalar) | semantic_ir/tests/fcvtns_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTNS (vector) | semantic_ir/tests/fcvtns_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTNT (merging) | semantic_ir/tests/fcvtnt_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTNU (scalar SIMD&FP) | semantic_ir/tests/fcvtnu_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTNU (scalar) | semantic_ir/tests/fcvtnu_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTNU (vector) | semantic_ir/tests/fcvtnu_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTPS (scalar SIMD&FP) | semantic_ir/tests/fcvtps_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTPS (scalar) | semantic_ir/tests/fcvtps_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTPS (vector) | semantic_ir/tests/fcvtps_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTPU (scalar SIMD&FP) | semantic_ir/tests/fcvtpu_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTPU (scalar) | semantic_ir/tests/fcvtpu_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTPU (vector) | semantic_ir/tests/fcvtpu_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTX (merging) | semantic_ir/tests/fcvtx_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTXN, FCVTXN2 | semantic_ir/tests/fcvtxn_fcvtxn2.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTXNT (merging) | semantic_ir/tests/fcvtxnt_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZS (scalar SIMD&FP) | semantic_ir/tests/fcvtzs_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZS (scalar, fixed-point) | semantic_ir/tests/fcvtzs_scalar_fixed-point.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZS (scalar, integer) | semantic_ir/tests/fcvtzs_scalar_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZS (vector, fixed-point) | semantic_ir/tests/fcvtzs_vector_fixed-point.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZS (vector, integer) | semantic_ir/tests/fcvtzs_vector_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (scalar SIMD&FP) | semantic_ir/tests/fcvtzu_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (scalar SIMD&FP) | semantic_ir/tests/simd_fp_fcvtzu_scalar_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (scalar, fixed-point) | semantic_ir/tests/simd_fp_fcvtzu_scalar_fixed_point.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (scalar, fixed-point) | semantic_ir/tests/fcvtzu_scalar_fixed-point.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (scalar, integer) | semantic_ir/tests/simd_fp_fcvtzu_scalar_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (scalar, integer) | semantic_ir/tests/fcvtzu_scalar_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (vector, fixed-point) | semantic_ir/tests/fcvtzu_vector_fixed-point.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (vector, fixed-point) | semantic_ir/tests/simd_fp_fcvtzu_vector_fixed_point.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (vector, integer) | semantic_ir/tests/fcvtzu_vector_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: FCVTZU (vector, integer) | semantic_ir/tests/simd_fp_fcvtzu_vector_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: FDIVR | semantic_ir/tests/fdivr_sve_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FDIVR | semantic_ir/tests/fdivr_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT | semantic_ir/tests/fdot_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (2-way, multiple and indexed vector, FP16 to FP32) | semantic_ir/tests/sme_fdot_2_way_multiple_and_indexed_vector_fp16_to_fp32.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (2-way, multiple and indexed vector, FP8 to FP16) | semantic_ir/tests/sme_fdot_2_way_multiple_and_indexed_vector_fp8_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (2-way, multiple and single vector, FP16 to FP32) | semantic_ir/tests/sme_fdot_2_way_multiple_and_single_vector_fp16_to_fp32.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (2-way, multiple and single vector, FP8 to FP16) | semantic_ir/tests/sme_fdot_2_way_multiple_and_single_vector_fp8_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (2-way, multiple vectors, FP16 to FP32) | semantic_ir/tests/sme_fdot_2_way_multiple_vectors_fp16_to_fp32.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (2-way, multiple vectors, FP8 to FP16) | semantic_ir/tests/sme_fdot_2_way_multiple_vectors_fp8_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (4-way, multiple and indexed vector) | semantic_ir/tests/sme_fdot_4_way_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (4-way, multiple and single vector) | semantic_ir/tests/sme_fdot_4_way_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (4-way, multiple vectors) | semantic_ir/tests/sme_fdot_4_way_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (8_bit floating-point to half-precision, by element) | semantic_ir/tests/fdot_8-bit_floating-point_to_half-precision_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (8_bit floating-point to half-precision, vector) | semantic_ir/tests/fdot_8-bit_floating-point_to_half-precision_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (8_bit floating-point to single-precision, by element) | semantic_ir/tests/fdot_8-bit_floating-point_to_single-precision_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (8_bit floating-point to single-precision, vector) | semantic_ir/tests/fdot_8-bit_floating-point_to_single-precision_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (8-bit floating-point to half-precision, by element) | semantic_ir/tests/fdot_8_bit_floating_point_to_half_precision_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (8-bit floating-point to half-precision, vector) | semantic_ir/tests/fdot_8_bit_floating_point_to_half_precision_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (8-bit floating-point to single-precision, by element) | semantic_ir/tests/fdot_8_bit_floating_point_to_single_precision_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FDOT (8-bit floating-point to single-precision, vector) | semantic_ir/tests/fdot_8_bit_floating_point_to_single_precision_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FDUP | semantic_ir/tests/fdup_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FEXPA | semantic_ir/tests/fexpa_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FIRSTP | semantic_ir/tests/firstp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FLOGB | semantic_ir/tests/flogb_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAX (multiple and single vector) | semantic_ir/tests/sme_fmax_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAX (multiple vectors) | semantic_ir/tests/sme_fmax_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNM | semantic_ir/tests/fmaxnm_fminnm_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNM | semantic_ir/tests/fmaxnm_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNM | semantic_ir/tests/fmaxnm_sve_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNM (multiple and single vector) | semantic_ir/tests/sme_fmaxnm_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNM (multiple vectors) | semantic_ir/tests/sme_fmaxnm_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNM (scalar) | semantic_ir/tests/fmaxnm_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNM (vector) | semantic_ir/tests/fmaxnm_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNMP | semantic_ir/tests/fmaxnmp_fminnmp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNMP | semantic_ir/tests/fmaxnmp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNMP (scalar) | semantic_ir/tests/fmaxnmp_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNMP (vector) | semantic_ir/tests/fmaxnmp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNMQV | semantic_ir/tests/fmaxnmqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNMV | semantic_ir/tests/fmaxnmv.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXNMV | semantic_ir/tests/fmaxnmv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXP (scalar) | semantic_ir/tests/fmaxp_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMAXQV | semantic_ir/tests/fmaxqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FMIN (multiple and single vector) | semantic_ir/tests/sme_fmin_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMIN (multiple vectors) | semantic_ir/tests/sme_fmin_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNM | semantic_ir/tests/fminnm_sve_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNM | semantic_ir/tests/fminnm_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNM (multiple and single vector) | semantic_ir/tests/sme_fminnm_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNM (multiple vectors) | semantic_ir/tests/sme_fminnm_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNM (scalar) | semantic_ir/tests/fminnm_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNM (vector) | semantic_ir/tests/fminnm_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNMP | semantic_ir/tests/fminnmp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNMP (scalar) | semantic_ir/tests/fminnmp_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNMP (vector) | semantic_ir/tests/fminnmp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNMQV | semantic_ir/tests/fminnmqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNMV | semantic_ir/tests/fminnmv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINNMV | semantic_ir/tests/fminnmv.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINP (scalar) | semantic_ir/tests/fminp_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FMINQV | semantic_ir/tests/fminqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLA (by element) | semantic_ir/tests/fmla_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLA (multiple and indexed vector) | semantic_ir/tests/sme_fmla_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLA (multiple and single vector) | semantic_ir/tests/sme_fmla_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLA (multiple vectors) | semantic_ir/tests/sme_fmla_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLAL (multiple and indexed vector, FP16 to FP32) | semantic_ir/tests/sme_fmlal_multiple_and_indexed_vector_fp16_to_fp32.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLAL (multiple and indexed vector, FP8 to FP16) | semantic_ir/tests/sme_fmlal_multiple_and_indexed_vector_fp8_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLAL (multiple and single vector, FP16 to FP32) | semantic_ir/tests/sme_fmlal_multiple_and_single_vector_fp16_to_fp32.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLAL (multiple and single vector, FP8 to FP16) | semantic_ir/tests/sme_fmlal_multiple_and_single_vector_fp8_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLAL (multiple vectors, FP16 to FP32) | semantic_ir/tests/sme_fmlal_multiple_vectors_fp16_to_fp32.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLAL (multiple vectors, FP8 to FP16) | semantic_ir/tests/sme_fmlal_multiple_vectors_fp8_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLAL, FMLAL2 (by element) | semantic_ir/tests/fmlal_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLAL, FMLAL2 (vector) | semantic_ir/tests/fmlal_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLALB, FMLALT (by element) | semantic_ir/tests/simd_fp_fmlalb_fmlalt_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLALB, FMLALT (vector) | semantic_ir/tests/simd_fp_fmlalb_fmlalt_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLALL (multiple and indexed vector) | semantic_ir/tests/sme_fmlall_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLALL (multiple and single vector) | semantic_ir/tests/sme_fmlall_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLALL (multiple vectors) | semantic_ir/tests/sme_fmlall_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLALLBB, FMLALLBT, FMLALLTB, FMLALLTT (by element) | semantic_ir/tests/simd_fp_fmlallbb_fmlallbt_fmlalltb_fmlalltt_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLALLBB, FMLALLBT, FMLALLTB, FMLALLTT (vector) | semantic_ir/tests/simd_fp_fmlallbb_fmlallbt_fmlalltb_fmlalltt_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLALT (indexed, FP16 to FP32) | semantic_ir/tests/fmlalt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLS (by element) | semantic_ir/tests/fmls_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLS (multiple and indexed vector) | semantic_ir/tests/sme_fmls_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLS (multiple and single vector) | semantic_ir/tests/sme_fmls_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLS (multiple vectors) | semantic_ir/tests/sme_fmls_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLSL (multiple and indexed vector) | semantic_ir/tests/sme_fmlsl_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLSL (multiple and single vector) | semantic_ir/tests/sme_fmlsl_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLSL (multiple vectors) | semantic_ir/tests/sme_fmlsl_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLSL, FMLSL2 (by element) | semantic_ir/tests/fmlsl_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLSL, FMLSL2 (vector) | semantic_ir/tests/fmlsl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMMLA (widening, 8_bit floating-point to half-precision) | semantic_ir/tests/fmmla_widening_8-bit_floating-point_to_half-precision.rs | 2026-10-16T16:00:00Z
COMPLETED: FMMLA (widening, 8_bit floating-point to single-precision) | semantic_ir/tests/fmmla_widening_8-bit_floating-point_to_single-precision.rs | 2026-10-16T16:00:00Z
COMPLETED: FMMLA (widening, 8-bit floating-point to half-precision) | semantic_ir/tests/fmmla_widening_8_bit_floating_point_to_half_precision.rs | 2026-10-16T16:00:00Z
COMPLETED: FMMLA (widening, 8-bit floating-point to single-precision) | semantic_ir/tests/fmmla_widening_8_bit_floating_point_to_single_precision.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOP4A (widening, 2-way, FP16 to FP32) | semantic_ir/tests/sme_fmop4a_widening_2_way_fp16_to_fp32.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOP4A (widening, 2-way, FP8 to FP16) | semantic_ir/tests/sme_fmop4a_widening_2_way_fp8_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOP4A (widening, 4-way) | semantic_ir/tests/sme_fmop4a_widening_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOPA (widening, 2-way, FP16 to FP32) | semantic_ir/tests/sme_fmopa_widening_2_way_fp16_to_fp32.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOPA (widening, 2-way, FP8 to FP16) | semantic_ir/tests/sme_fmopa_widening_2_way_fp8_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOPA (widening, 4-way) | semantic_ir/tests/sme_fmopa_widening_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOV (general) | semantic_ir/tests/fmov_general.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOV (scalar, immediate) | semantic_ir/tests/fmov_scalar_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: FMOV (vector, immediate) | semantic_ir/tests/fmov_vector_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: FMUL (by element) | semantic_ir/tests/fmul_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FMUL (multiple and single vector) | semantic_ir/tests/sme_fmul_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FMUL (multiple vectors) | semantic_ir/tests/sme_fmul_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FMULX (by element) | semantic_ir/tests/fmulx_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: FNMADD | semantic_ir/tests/fnmadd.rs | 2026-10-16T16:00:00Z
COMPLETED: FNMLA | semantic_ir/tests/fnmla_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FNMLS | semantic_ir/tests/fnmls_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FNMSUB | semantic_ir/tests/fnmsub.rs | 2026-10-16T16:00:00Z
COMPLETED: FNMUL (scalar) | semantic_ir/tests/fnmul_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRECPX | semantic_ir/tests/frecpx.rs | 2026-10-16T16:00:00Z
COMPLETED: FRECPX | semantic_ir/tests/frecpx_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT32X | semantic_ir/tests/frint32x_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT32X (scalar) | semantic_ir/tests/frint32x_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT32X (vector) | semantic_ir/tests/frint32x_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT32Z | semantic_ir/tests/frint32z_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT32Z | semantic_ir/tests/frint32z_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT32Z (scalar) | semantic_ir/tests/frint32z_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT32Z (vector) | semantic_ir/tests/frint32z_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT64X | semantic_ir/tests/frint64x_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT64X (scalar) | semantic_ir/tests/frint64x_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT64X (vector) | semantic_ir/tests/frint64x_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT64Z | semantic_ir/tests/frint64z_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT64Z | semantic_ir/tests/frint64z_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT64Z (scalar) | semantic_ir/tests/frint64z_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINT64Z (vector) | semantic_ir/tests/frint64z_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTA | semantic_ir/tests/sme_frinta.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTA | semantic_ir/tests/frinta_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTA (scalar) | semantic_ir/tests/frinta_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTA (vector) | semantic_ir/tests/frinta_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTI | semantic_ir/tests/frinti_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTI (scalar) | semantic_ir/tests/frinti_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTI (vector) | semantic_ir/tests/frinti_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTM | semantic_ir/tests/frintm_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTM | semantic_ir/tests/sme_frintm.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTM (scalar) | semantic_ir/tests/frintm_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTM (vector) | semantic_ir/tests/frintm_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTN | semantic_ir/tests/frintn_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTN | semantic_ir/tests/sme_frintn.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTN (scalar) | semantic_ir/tests/frintn_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTN (vector) | semantic_ir/tests/frintn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTP | semantic_ir/tests/frintp_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTP | semantic_ir/tests/sme_frintp.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTP (scalar) | semantic_ir/tests/frintp_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTP (vector) | semantic_ir/tests/frintp_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTX | semantic_ir/tests/frintx_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTX (scalar) | semantic_ir/tests/frintx_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTX (vector) | semantic_ir/tests/frintx_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTZ | semantic_ir/tests/frintz_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTZ (scalar) | semantic_ir/tests/frintz_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: FRINTZ (vector) | semantic_ir/tests/frintz_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FSCALE | semantic_ir/tests/fscale.rs | 2026-10-16T16:00:00Z
COMPLETED: FSCALE | semantic_ir/tests/fscale_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: FSCALE (multiple and single vector) | semantic_ir/tests/sme_fscale_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FSCALE (multiple vectors) | semantic_ir/tests/sme_fscale_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FSQRT (vector) | semantic_ir/tests/fsqrt_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: FSUBR | semantic_ir/tests/fsubr_sve_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: FSUBR | semantic_ir/tests/fsubr_sve_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: FSUBR | semantic_ir/tests/fsubr_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FTMAD | semantic_ir/tests/ftmad_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FTMAD | semantic_ir/tests/ftrig_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FTMOPA (widening, 2-way, FP16 to FP32) | semantic_ir/tests/sme_ftmopa_widening_2_way_fp16_to_fp32.rs | 2026-10-16T16:00:00Z
COMPLETED: FTMOPA (widening, 2-way, FP8 to FP16) | semantic_ir/tests/sme_ftmopa_widening_2_way_fp8_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: FTMOPA (widening, 4-way) | semantic_ir/tests/sme_ftmopa_widening_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: FTSMUL | semantic_ir/tests/ftsmul_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FTSSEL | semantic_ir/tests/ftssel_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: FVDOT (FP16 to FP32) | semantic_ir/tests/sme_fvdot_fp16_to_fp32.rs | 2026-10-16T16:00:00Z
COMPLETED: FVDOT (FP8 to FP16) | semantic_ir/tests/sme_fvdot_fp8_to_fp16.rs | 2026-10-16T16:00:00Z
COMPLETED: HISTCNT | semantic_ir/tests/histcnt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: HISTCNT | semantic_ir/tests/histcnt_histseg_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: HISTSEG | semantic_ir/tests/histseg_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: INCB | semantic_ir/tests/incb_scalar_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: INCB | semantic_ir/tests/incb.rs | 2026-10-16T16:00:00Z
COMPLETED: INCD | semantic_ir/tests/incd.rs | 2026-10-16T16:00:00Z
COMPLETED: INCH | semantic_ir/tests/inch.rs | 2026-10-16T16:00:00Z
COMPLETED: INCP | semantic_ir/tests/incp_scalar_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: INCW | semantic_ir/tests/incw.rs | 2026-10-16T16:00:00Z
COMPLETED: INDEX | semantic_ir/tests/index_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: INS (element) | semantic_ir/tests/ins_element.rs | 2026-10-16T16:00:00Z
COMPLETED: INS (element) | semantic_ir/tests/simd_fp_ins_element.rs | 2026-10-16T16:00:00Z
COMPLETED: INS (general) | semantic_ir/tests/simd_fp_ins_general.rs | 2026-10-16T16:00:00Z
COMPLETED: INS (general) | semantic_ir/tests/ins_general.rs | 2026-10-16T16:00:00Z
COMPLETED: INSR (scalar) | semantic_ir/tests/insr_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LASTA | semantic_ir/tests/last_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1 (multiple structures) | semantic_ir/tests/simd_fp_ld1_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1 (multiple structures) | semantic_ir/tests/ld1_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1 (single structure) | semantic_ir/tests/ld1_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1 (single structure) | semantic_ir/tests/simd_fp_ld1_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1B | semantic_ir/tests/ld1b_sve_contiguous.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1B | semantic_ir/tests/ld1b_sve_contiguous_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1B | semantic_ir/tests/ld1b_sve_gather.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1B | semantic_ir/tests/ld1b_sve_gather_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1B (scalar plus immediate, strided registers) | semantic_ir/tests/sme_ld1b_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1B (scalar plus scalar, strided registers) | semantic_ir/tests/sme_ld1b_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1B (scalar plus scalar, tile slice) | semantic_ir/tests/sme_ld1b_scalar_plus_scalar_tile_slice.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1D | semantic_ir/tests/ld1d_sve_gather_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1D | semantic_ir/tests/ld1d_sve_gather.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1D | semantic_ir/tests/ld1d_sve_contiguous_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1D (scalar plus immediate, strided registers) | semantic_ir/tests/sme_ld1d_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1D (scalar plus scalar, strided registers) | semantic_ir/tests/sme_ld1d_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1D (scalar plus scalar, tile slice) | semantic_ir/tests/sme_ld1d_scalar_plus_scalar_tile_slice.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1H | semantic_ir/tests/ld1h_sve_contiguous_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1H | semantic_ir/tests/ld1h_sve_gather.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1H | semantic_ir/tests/ld1h_sve_gather_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1H (scalar plus immediate, strided registers) | semantic_ir/tests/sme_ld1h_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1H (scalar plus scalar, strided registers) | semantic_ir/tests/sme_ld1h_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1H (scalar plus scalar, tile slice) | semantic_ir/tests/sme_ld1h_scalar_plus_scalar_tile_slice.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1Q | semantic_ir/tests/ld1q_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1R | semantic_ir/tests/simd_fp_ld1r.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1R | semantic_ir/tests/ld1r.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1RB | semantic_ir/tests/ld1r_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1RSB | semantic_ir/tests/ld1rsb_rsh_rsw_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SB | semantic_ir/tests/ld1sb_sh_sw_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SH (vector plus immediate) | semantic_ir/tests/ld1sh_sve_gather_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SH (vector plus scalar) | semantic_ir/tests/ld1sh_sve_gather.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SW (vector plus immediate) | semantic_ir/tests/ld1sw_sve_gather_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SW (vector plus scalar) | semantic_ir/tests/ld1sw_sve_gather.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1W | semantic_ir/tests/ld1w_sve_contiguous_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1W | semantic_ir/tests/ld1w_sve_gather_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1W | semantic_ir/tests/ld1w_sve_gather.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1W (scalar plus immediate, strided registers) | semantic_ir/tests/sme_ld1w_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1W (scalar plus scalar, strided registers) | semantic_ir/tests/sme_ld1w_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1W (scalar plus scalar, tile slice) | semantic_ir/tests/sme_ld1w_scalar_plus_scalar_tile_slice.rs | 2026-10-16T16:00:00Z
COMPLETED: LD2 (multiple structures) | semantic_ir/tests/ld2_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: LD2 (multiple structures) | semantic_ir/tests/simd_fp_ld2_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: LD2 (single structure) | semantic_ir/tests/ld2_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: LD2 (single structure) | semantic_ir/tests/simd_fp_ld2_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: LD2B | semantic_ir/tests/ld2b_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD2D | semantic_ir/tests/ld2d_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD2H | semantic_ir/tests/ld2h_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD2Q | semantic_ir/tests/ld2q_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD2R | semantic_ir/tests/simd_fp_ld2r.rs | 2026-10-16T16:00:00Z
COMPLETED: LD2R | semantic_ir/tests/ld2r.rs | 2026-10-16T16:00:00Z
COMPLETED: LD2W | semantic_ir/tests/ld2w_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD3 (multiple structures) | semantic_ir/tests/simd_fp_ld3_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: LD3 (multiple structures) | semantic_ir/tests/ld3_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: LD3 (single structure) | semantic_ir/tests/simd_fp_ld3_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: LD3 (single structure) | semantic_ir/tests/ld3_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: LD3D | semantic_ir/tests/ld3d_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD3H | semantic_ir/tests/ld3h_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD3Q | semantic_ir/tests/ld3q_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD3R | semantic_ir/tests/simd_fp_ld3r.rs | 2026-10-16T16:00:00Z
COMPLETED: LD3R | semantic_ir/tests/ld3r.rs | 2026-10-16T16:00:00Z
COMPLETED: LD3W | semantic_ir/tests/ld3w_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD4 (multiple structures) | semantic_ir/tests/ld4_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: LD4 (multiple structures) | semantic_ir/tests/simd_fp_ld4_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: LD4 (single structure) | semantic_ir/tests/ld4_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: LD4 (single structure) | semantic_ir/tests/simd_fp_ld4_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: LD4D | semantic_ir/tests/ld4d_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD4H | semantic_ir/tests/ld4h_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD4Q | semantic_ir/tests/ld4q_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LD4R | semantic_ir/tests/simd_fp_ld4r.rs | 2026-10-16T16:00:00Z
COMPLETED: LD4R | semantic_ir/tests/ld4r.rs | 2026-10-16T16:00:00Z
COMPLETED: LD4W | semantic_ir/tests/ld4w_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAP1 (SIMD&FP) | semantic_ir/tests/simd_fp_ldap1_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAP1 (SIMD&FP) | semantic_ir/tests/ldap1_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAPUR (SIMD&FP) | semantic_ir/tests/simd_fp_ldapur_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAPUR (SIMD&FP) | semantic_ir/tests/ldapur_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDBFADD | semantic_ir/tests/ldbfadd_ldbfadda_ldbfaddal_ldbfaddl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDBFADD, LDBFADDA, LDBFADDAL, LDBFADDL | semantic_ir/tests/simd_fp_ldbfadd_ldbfadda_ldbfaddal_ldbfaddl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDBFMAX | semantic_ir/tests/ldbfmax_ldbfmaxa_ldbfmaxal_ldbfmaxl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDBFMAX, LDBFMAXA, LDBFMAXAL, LDBFMAXL | semantic_ir/tests/simd_fp_ldbfmax_ldbfmaxa_ldbfmaxal_ldbfmaxl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDBFMAXNM | semantic_ir/tests/ldbfmaxnm_ldbfmaxnma_ldbfmaxnmal_ldbfmaxnml.rs | 2026-10-16T16:00:00Z
COMPLETED: LDBFMAXNM, LDBFMAXNMA, LDBFMAXNMAL, LDBFMAXNML | semantic_ir/tests/simd_fp_ldbfmaxnm_ldbfmaxnma_ldbfmaxnmal_ldbfmaxnml.rs | 2026-10-16T16:00:00Z
COMPLETED: LDBFMIN | semantic_ir/tests/ldbfmin_ldbfmina_ldbfminal_ldbfminl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDBFMIN, LDBFMINA, LDBFMINAL, LDBFMINL | semantic_ir/tests/simd_fp_ldbfmin_ldbfmina_ldbfminal_ldbfminl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDBFMINNM | semantic_ir/tests/ldbfminnm_ldbfminnma_ldbfminnmal_ldbfminnml.rs | 2026-10-16T16:00:00Z
COMPLETED: LDBFMINNM, LDBFMINNMA, LDBFMINNMAL, LDBFMINNML | semantic_ir/tests/simd_fp_ldbfminnm_ldbfminnma_ldbfminnmal_ldbfminnml.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFADD | semantic_ir/tests/ldfadd_ldfadda_ldfaddal_ldfaddl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFADD, LDFADDA, LDFADDAL, LDFADDL | semantic_ir/tests/simd_fp_ldfadd_ldfadda_ldfaddal_ldfaddl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFF1B | semantic_ir/tests/ldff1b_sve_gather.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFF1D | semantic_ir/tests/ldff1d_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFMAX | semantic_ir/tests/ldfmax_ldfmaxa_ldfmaxal_ldfmaxl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFMAX, LDFMAXA, LDFMAXAL, LDFMAXL | semantic_ir/tests/simd_fp_ldfmax_ldfmaxa_ldfmaxal_ldfmaxl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFMAXNM | semantic_ir/tests/ldfmaxnm_ldfmaxnma_ldfmaxnmal_ldfmaxnml.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFMAXNM, LDFMAXNMA, LDFMAXNMAL, LDFMAXNML | semantic_ir/tests/simd_fp_ldfmaxnm_ldfmaxnma_ldfmaxnmal_ldfmaxnml.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFMIN | semantic_ir/tests/ldfmin_ldfmina_ldfminal_ldfminl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFMIN, LDFMINA, LDFMINAL, LDFMINL | semantic_ir/tests/simd_fp_ldfmin_ldfmina_ldfminal_ldfminl.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFMINNM | semantic_ir/tests/ldfminnm_ldfminnma_ldfminnmal_ldfminnml.rs | 2026-10-16T16:00:00Z
COMPLETED: LDFMINNM, LDFMINNMA, LDFMINNMAL, LDFMINNML | semantic_ir/tests/simd_fp_ldfminnm_ldfminnma_ldfminnmal_ldfminnml.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNP (SIMD&FP) | semantic_ir/tests/ldnp_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNP (SIMD&FP) | semantic_ir/tests/simd_fp_ldnp_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1B (scalar plus immediate) | semantic_ir/tests/ldnt1b_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1B (scalar plus immediate, strided registers) | semantic_ir/tests/sme_ldnt1b_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1B (scalar plus scalar, strided registers) | semantic_ir/tests/sme_ldnt1b_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1D | semantic_ir/tests/ldnt1d_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1D (scalar plus immediate, strided registers) | semantic_ir/tests/sme_ldnt1d_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1D (scalar plus scalar, strided registers) | semantic_ir/tests/sme_ldnt1d_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1H (scalar plus immediate, strided registers) | semantic_ir/tests/sme_ldnt1h_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1H (scalar plus scalar, strided registers) | semantic_ir/tests/sme_ldnt1h_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1SH (scalar plus scalar) | semantic_ir/tests/ldnt1sh_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1SH (scalar plus scalar) | semantic_ir/tests/ldnt1h_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1SW (scalar plus scalar) | semantic_ir/tests/ldnt1sw_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1W (scalar plus immediate) | semantic_ir/tests/ldnt1w_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1W (scalar plus immediate, strided registers) | semantic_ir/tests/sme_ldnt1w_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1W (scalar plus scalar, strided registers) | semantic_ir/tests/sme_ldnt1w_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LDP (SIMD&FP) | semantic_ir/tests/ldp_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDP (SIMD&FP) | semantic_ir/tests/simd_fp_ldp_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDR (immediate, SIMD&FP) | semantic_ir/tests/simd_fp_ldr_immediate_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDR (immediate, SIMD&FP) | semantic_ir/tests/ldr_immediate_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDR (literal, SIMD&FP) | semantic_ir/tests/simd_fp_ldr_literal_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDR (literal, SIMD&FP) | semantic_ir/tests/ldr_literal_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDR (register, SIMD&FP) | semantic_ir/tests/ldr_register_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDR (register, SIMD&FP) | semantic_ir/tests/simd_fp_ldr_register_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTNP (SIMD&FP) | semantic_ir/tests/simd_fp_ldtnp_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTNP (SIMD&FP) | semantic_ir/tests/ldtnp_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTP (SIMD&FP) | semantic_ir/tests/simd_fp_ldtp_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTP (SIMD&FP) | semantic_ir/tests/ldtp_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDUR (SIMD&FP) | semantic_ir/tests/ldur_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDUR (SIMD&FP) | semantic_ir/tests/simd_fp_ldur_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: LSL | semantic_ir/tests/lsl_sve_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: LSL | semantic_ir/tests/lsl_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: LSL | semantic_ir/tests/lsl_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: LSL | semantic_ir/tests/lsl_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LSL | semantic_ir/tests/lsl_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: LSLR | semantic_ir/tests/lslr_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: LSLV | semantic_ir/tests/lslv.rs | 2026-10-16T16:00:00Z
COMPLETED: LSR | semantic_ir/tests/lsr_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LSR | semantic_ir/tests/lsr_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: LSR | semantic_ir/tests/lsr_sve_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: LSR | semantic_ir/tests/lsr_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: LSR | semantic_ir/tests/lsr_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: LSRR | semantic_ir/tests/lsrr_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: LSRV | semantic_ir/tests/lsrv.rs | 2026-10-16T16:00:00Z
COMPLETED: LUTI2 | semantic_ir/tests/simd_fp_luti2.rs | 2026-10-16T16:00:00Z
COMPLETED: LUTI2 | semantic_ir/tests/luti2.rs | 2026-10-16T16:00:00Z
COMPLETED: LUTI2 (four registers) | semantic_ir/tests/sme_luti2_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LUTI2 (two registers) | semantic_ir/tests/sme_luti2_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: LUTI4 | semantic_ir/tests/luti4.rs | 2026-10-16T16:00:00Z
COMPLETED: LUTI4 | semantic_ir/tests/simd_fp_luti4.rs | 2026-10-16T16:00:00Z
COMPLETED: LUTI4 (four registers, 16-bit and 32-bit) | semantic_ir/tests/sme_luti4_four_registers_16_bit_and_32_bit.rs | 2026-10-16T16:00:00Z
COMPLETED: LUTI4 (four registers, 8-bit) | semantic_ir/tests/sme_luti4_four_registers_8_bit.rs | 2026-10-16T16:00:00Z
COMPLETED: LUTI4 (two registers) | semantic_ir/tests/sme_luti4_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MATCH | semantic_ir/tests/match_nmatch_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: MLA | semantic_ir/tests/mla_indexed_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: MLA | semantic_ir/tests/mla_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: MLA (by element) | semantic_ir/tests/mla_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: MLA (by element) | semantic_ir/tests/simd_fp_mla_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: MLA (vector) | semantic_ir/tests/mla_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: MLS | semantic_ir/tests/mls_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: MLS (by element) | semantic_ir/tests/simd_fp_mls_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: MLS (by element) | semantic_ir/tests/mls_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: MLS (vector) | semantic_ir/tests/mls_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV | semantic_ir/tests/mov_movz.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV | semantic_ir/tests/mov_movn.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV | semantic_ir/tests/mov_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV | semantic_ir/tests/mov_sp.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV | semantic_ir/tests/mov_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (array to vector, four registers) | semantic_ir/tests/sme_mov_array_to_vector_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (array to vector, two registers) | semantic_ir/tests/sme_mov_array_to_vector_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (element) | semantic_ir/tests/simd_fp_mov_element.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (element) | semantic_ir/tests/mov_element.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (from general) | semantic_ir/tests/simd_fp_mov_from_general.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (from general) | semantic_ir/tests/mov_from_general.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (scalar) | semantic_ir/tests/simd_fp_mov_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (scalar) | semantic_ir/tests/mov_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (tile to vector, four registers) | semantic_ir/tests/sme_mov_tile_to_vector_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (tile to vector, two registers) | semantic_ir/tests/sme_mov_tile_to_vector_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (to general) | semantic_ir/tests/simd_fp_mov_to_general.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (to general) | semantic_ir/tests/mov_to_general.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (vector to array, four registers) | semantic_ir/tests/sme_mov_vector_to_array_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (vector to array, two registers) | semantic_ir/tests/sme_mov_vector_to_array_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (vector to tile, four registers) | semantic_ir/tests/sme_mov_vector_to_tile_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (vector to tile, two registers) | semantic_ir/tests/sme_mov_vector_to_tile_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (vector) | semantic_ir/tests/simd_fp_mov_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: MOV (vector) | semantic_ir/tests/mov_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVA (array to vector, four registers) | semantic_ir/tests/sme_mova_array_to_vector_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVA (array to vector, two registers) | semantic_ir/tests/sme_mova_array_to_vector_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVA (tile to vector, four registers) | semantic_ir/tests/sme_mova_tile_to_vector_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVA (tile to vector, two registers) | semantic_ir/tests/sme_mova_tile_to_vector_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVA (vector to array, four registers) | semantic_ir/tests/sme_mova_vector_to_array_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVA (vector to array, two registers) | semantic_ir/tests/sme_mova_vector_to_array_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVA (vector to tile, four registers) | semantic_ir/tests/sme_mova_vector_to_tile_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVA (vector to tile, two registers) | semantic_ir/tests/sme_mova_vector_to_tile_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVAZ (array to vector, four registers) | semantic_ir/tests/sme_movaz_array_to_vector_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVAZ (array to vector, two registers) | semantic_ir/tests/sme_movaz_array_to_vector_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVAZ (tile to vector, four registers) | semantic_ir/tests/sme_movaz_tile_to_vector_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVAZ (tile to vector, two registers) | semantic_ir/tests/sme_movaz_tile_to_vector_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVI | semantic_ir/tests/simd_fp_movi.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVI | semantic_ir/tests/movi.rs | 2026-10-16T16:00:00Z
COMPLETED: MOVPRFX (unpredicated) | semantic_ir/tests/movprfx_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: MSR | semantic_ir/tests/msr_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: MSR | semantic_ir/tests/msr_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: MUL (by element) | semantic_ir/tests/simd_fp_mul_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: MUL (by element) | semantic_ir/tests/mul_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: MUL (vector) | semantic_ir/tests/mul_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: MVNI | semantic_ir/tests/simd_fp_mvni.rs | 2026-10-16T16:00:00Z
COMPLETED: MVNI | semantic_ir/tests/mvni.rs | 2026-10-16T16:00:00Z
COMPLETED: NAND | semantic_ir/tests/nand_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: NANDS | semantic_ir/tests/nands_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: NEG (vector) | semantic_ir/tests/neg_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: NOR | semantic_ir/tests/nor_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: NORS | semantic_ir/tests/nors_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: NOT | semantic_ir/tests/not_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: NOT | semantic_ir/tests/simd_fp_not.rs | 2026-10-16T16:00:00Z
COMPLETED: NOT | semantic_ir/tests/not_sve_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: NOT | semantic_ir/tests/not.rs | 2026-10-16T16:00:00Z
COMPLETED: ORN | semantic_ir/tests/orn_sve_predicates.rs | 2026-10-16T16:00:00Z
COMPLETED: ORN | semantic_ir/tests/orn_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ORN | semantic_ir/tests/orn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ORN (vector) | semantic_ir/tests/simd_fp_orn_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ORQV | semantic_ir/tests/orqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ORR | semantic_ir/tests/orr_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: ORR | semantic_ir/tests/orr_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ORR | semantic_ir/tests/orr_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ORR | semantic_ir/tests/orr_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ORR | semantic_ir/tests/orr_sve_predicates.rs | 2026-10-16T16:00:00Z
COMPLETED: ORR (vector, immediate) | semantic_ir/tests/orr_vector_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: ORR (vector, immediate) | semantic_ir/tests/simd_fp_orr_vector_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: ORR (vector, register) | semantic_ir/tests/simd_fp_orr_vector_register.rs | 2026-10-16T16:00:00Z
COMPLETED: ORR (vector, register) | semantic_ir/tests/orr_vector_register.rs | 2026-10-16T16:00:00Z
COMPLETED: ORRS | semantic_ir/tests/orrs_sve_predicates.rs | 2026-10-16T16:00:00Z
COMPLETED: ORV | semantic_ir/tests/orv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ORV | semantic_ir/tests/orv_sve_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: PACDA | semantic_ir/tests/pacda.rs | 2026-10-16T16:00:00Z
COMPLETED: PACDB | semantic_ir/tests/pacdb.rs | 2026-10-16T16:00:00Z
COMPLETED: PACIA | semantic_ir/tests/pacia.rs | 2026-10-16T16:00:00Z
COMPLETED: PACIB | semantic_ir/tests/pacib.rs | 2026-10-16T16:00:00Z
COMPLETED: PFALSE | semantic_ir/tests/pfalse_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PFIRST | semantic_ir/tests/pfirst_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PMUL | semantic_ir/tests/simd_fp_pmul.rs | 2026-10-16T16:00:00Z
COMPLETED: PMUL | semantic_ir/tests/pmul.rs | 2026-10-16T16:00:00Z
COMPLETED: PMUL | semantic_ir/tests/pmul_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: PMULL, PMULL2 | semantic_ir/tests/simd_fp_pmull_pmull2.rs | 2026-10-16T16:00:00Z
COMPLETED: PMULL, PMULL2 | semantic_ir/tests/pmull_pmull2.rs | 2026-10-16T16:00:00Z
COMPLETED: PMULLB | semantic_ir/tests/pmullb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PMULLT | semantic_ir/tests/pmullt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PNEXT | semantic_ir/tests/pnext_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PRFB (scalar plus immediate) | semantic_ir/tests/prfb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PRFH (scalar plus immediate) | semantic_ir/tests/prfh_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PRFW (scalar plus immediate) | semantic_ir/tests/prfw_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PSB | semantic_ir/tests/psb.rs | 2026-10-16T16:00:00Z
COMPLETED: PSEL | semantic_ir/tests/psel_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PTEST | semantic_ir/tests/ptest_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PTRUE | semantic_ir/tests/ptrue_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PTRUE | semantic_ir/tests/ptrue_predicate_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: PTRUES | semantic_ir/tests/ptrues_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: RADDHN, RADDHN2 | semantic_ir/tests/raddhn_raddhn2.rs | 2026-10-16T16:00:00Z
COMPLETED: RADDHN, RADDHN2 | semantic_ir/tests/simd_fp_raddhn_raddhn2.rs | 2026-10-16T16:00:00Z
COMPLETED: RADDHNT | semantic_ir/tests/raddhnt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: RAX1 | semantic_ir/tests/simd_fp_rax1.rs | 2026-10-16T16:00:00Z
COMPLETED: RAX1 | semantic_ir/tests/rax1.rs | 2026-10-16T16:00:00Z
COMPLETED: RBIT (vector) | semantic_ir/tests/simd_fp_rbit_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: RDFFR (predicated) | semantic_ir/tests/rdffr_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: RDVL | semantic_ir/tests/rdvl_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: RET | semantic_ir/tests/ret.rs | 2026-10-16T16:00:00Z
COMPLETED: REV16 (vector) | semantic_ir/tests/simd_fp_rev16_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: REV32 (vector) | semantic_ir/tests/simd_fp_rev32_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: REVB | semantic_ir/tests/revb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ROR | semantic_ir/tests/ror_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: ROR | semantic_ir/tests/ror_immediate_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ROR | semantic_ir/tests/ror_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: ROR | semantic_ir/tests/ror_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: RORV | semantic_ir/tests/rorv.rs | 2026-10-16T16:00:00Z
COMPLETED: RSHRN, RSHRN2 | semantic_ir/tests/rshrn_rshn2.rs | 2026-10-16T16:00:00Z
COMPLETED: RSHRN, RSHRN2 | semantic_ir/tests/rshrn_rshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: RSHRN, RSHRN2 | semantic_ir/tests/simd_fp_rshrn_rshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: RSUBHN, RSUBHN2 | semantic_ir/tests/rsubhn_rsubhn2.rs | 2026-10-16T16:00:00Z
COMPLETED: RSUBHN, RSUBHN2 | semantic_ir/tests/simd_fp_rsubhn_rsubhn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SABA | semantic_ir/tests/saba.rs | 2026-10-16T16:00:00Z
COMPLETED: SABA | semantic_ir/tests/simd_fp_saba.rs | 2026-10-16T16:00:00Z
COMPLETED: SABAL, SABAL2 | semantic_ir/tests/simd_fp_sabal_sabal2.rs | 2026-10-16T16:00:00Z
COMPLETED: SABAL, SABAL2 | semantic_ir/tests/sabal_sabal2.rs | 2026-10-16T16:00:00Z
COMPLETED: SABALB | semantic_ir/tests/sabalb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SABD | semantic_ir/tests/simd_fp_sabd.rs | 2026-10-16T16:00:00Z
COMPLETED: SABD | semantic_ir/tests/sabd.rs | 2026-10-16T16:00:00Z
COMPLETED: SABD (SVE predicated) | semantic_ir/tests/sabd_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SABDL, SABDL2 | semantic_ir/tests/sabdl_sabdl2.rs | 2026-10-16T16:00:00Z
COMPLETED: SABDL, SABDL2 | semantic_ir/tests/simd_fp_sabdl_sabdl2.rs | 2026-10-16T16:00:00Z
COMPLETED: SABDLB | semantic_ir/tests/sabdlb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SADALB | semantic_ir/tests/sadalb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SADALP | semantic_ir/tests/simd_fp_sadalp.rs | 2026-10-16T16:00:00Z
COMPLETED: SADALP | semantic_ir/tests/sadalp.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDL, SADDL2 | semantic_ir/tests/simd_fp_saddl_saddl2.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDL, SADDL2 | semantic_ir/tests/saddl_saddl2.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDLB | semantic_ir/tests/saddlb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDLBT | semantic_ir/tests/saddlbt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDLT | semantic_ir/tests/saddlt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDLV | semantic_ir/tests/saddlv.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDLV | semantic_ir/tests/simd_fp_saddlv.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDQV | semantic_ir/tests/saddqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDW, SADDW2 | semantic_ir/tests/simd_fp_saddw_saddw2.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDW, SADDW2 | semantic_ir/tests/saddw_saddw2.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDWB | semantic_ir/tests/saddwb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SADDWT | semantic_ir/tests/saddwt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SBCLB | semantic_ir/tests/sbclb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SBCLT | semantic_ir/tests/sbclt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SBFM | semantic_ir/tests/sbfm.rs | 2026-10-16T16:00:00Z
COMPLETED: SCLAMP | semantic_ir/tests/sclamp_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (merging) | semantic_ir/tests/scvtf_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (scalar SIMD&FP) | semantic_ir/tests/simd_fp_scvtf_scalar_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (scalar SIMD&FP) | semantic_ir/tests/scvtf_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (scalar, fixed-point) | semantic_ir/tests/scvtf_scalar_fixed-point.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (scalar, fixed-point) | semantic_ir/tests/simd_fp_scvtf_scalar_fixed_point.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (scalar, integer) | semantic_ir/tests/simd_fp_scvtf_scalar_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (scalar, integer) | semantic_ir/tests/scvtf_scalar_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (vector, fixed-point) | semantic_ir/tests/scvtf_vector_fixed-point.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (vector, fixed-point) | semantic_ir/tests/simd_fp_scvtf_vector_fixed_point.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (vector, integer) | semantic_ir/tests/scvtf_vector_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: SCVTF (vector, integer) | semantic_ir/tests/simd_fp_scvtf_vector_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: SDIVR | semantic_ir/tests/sdivr_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: SDOT (2-way, multiple and indexed vector) | semantic_ir/tests/sme_sdot_2_way_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SDOT (2-way, multiple and single vector) | semantic_ir/tests/sme_sdot_2_way_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SDOT (2-way, multiple vectors) | semantic_ir/tests/sme_sdot_2_way_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SDOT (2-way, vectors) | semantic_ir/tests/sdot_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SDOT (4-way, multiple and indexed vector) | semantic_ir/tests/sme_sdot_4_way_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SDOT (4-way, multiple and single vector) | semantic_ir/tests/sme_sdot_4_way_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SDOT (4-way, multiple vectors) | semantic_ir/tests/sme_sdot_4_way_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SDOT (by element) | semantic_ir/tests/sdot_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SDOT (by element) | semantic_ir/tests/simd_fp_sdot_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SDOT (vector) | semantic_ir/tests/sdot_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SDOT (vector) | semantic_ir/tests/simd_fp_sdot_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SEL | semantic_ir/tests/sel_vectors_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SEL | semantic_ir/tests/sel_sve_predicates.rs | 2026-10-16T16:00:00Z
COMPLETED: setf | semantic_ir/tests/setf.rs | 2026-10-16T16:00:00Z
COMPLETED: SETFFR | semantic_ir/tests/setffr_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SETGP | semantic_ir/tests/setgp.rs | 2026-10-16T16:00:00Z
COMPLETED: SETGPN | semantic_ir/tests/setgpn.rs | 2026-10-16T16:00:00Z
COMPLETED: SETGPT | semantic_ir/tests/setgpt.rs | 2026-10-16T16:00:00Z
COMPLETED: SETGPTN | semantic_ir/tests/setgptn.rs | 2026-10-16T16:00:00Z
COMPLETED: SETP | semantic_ir/tests/setp.rs | 2026-10-16T16:00:00Z
COMPLETED: SETPN | semantic_ir/tests/setpn.rs | 2026-10-16T16:00:00Z
COMPLETED: SETPT | semantic_ir/tests/setpt.rs | 2026-10-16T16:00:00Z
COMPLETED: SETPTN | semantic_ir/tests/setptn.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1C | semantic_ir/tests/simd_fp_sha1c.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1C | semantic_ir/tests/sha1c.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1H | semantic_ir/tests/simd_fp_sha1h.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1H | semantic_ir/tests/sha1h.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1M | semantic_ir/tests/sha1m.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1M | semantic_ir/tests/simd_fp_sha1m.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1P | semantic_ir/tests/sha1p.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1P | semantic_ir/tests/simd_fp_sha1p.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1SU0 | semantic_ir/tests/sha1su0.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1SU0 | semantic_ir/tests/simd_fp_sha1su0.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1SU1 | semantic_ir/tests/sha1su1.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA1SU1 | semantic_ir/tests/simd_fp_sha1su1.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA256H | semantic_ir/tests/sha256h.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA256H | semantic_ir/tests/simd_fp_sha256h.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA256H2 | semantic_ir/tests/sha256h2.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA256H2 | semantic_ir/tests/simd_fp_sha256h2.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA256SU0 | semantic_ir/tests/sha256su0.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA256SU0 | semantic_ir/tests/simd_fp_sha256su0.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA256SU1 | semantic_ir/tests/simd_fp_sha256su1.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA256SU1 | semantic_ir/tests/sha256su1.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA512H | semantic_ir/tests/simd_fp_sha512h.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA512H | semantic_ir/tests/sha512h.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA512H2 | semantic_ir/tests/sha512h2.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA512H2 | semantic_ir/tests/simd_fp_sha512h2.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA512SU0 | semantic_ir/tests/simd_fp_sha512su0.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA512SU0 | semantic_ir/tests/sha512su0.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA512SU1 | semantic_ir/tests/simd_fp_sha512su1.rs | 2026-10-16T16:00:00Z
COMPLETED: SHA512SU1 | semantic_ir/tests/sha512su1.rs | 2026-10-16T16:00:00Z
COMPLETED: SHL | semantic_ir/tests/simd_fp_shl.rs | 2026-10-16T16:00:00Z
COMPLETED: SHL | semantic_ir/tests/shl.rs | 2026-10-16T16:00:00Z
COMPLETED: SHLL, SHLL2 | semantic_ir/tests/simd_fp_shll_shll2.rs | 2026-10-16T16:00:00Z
COMPLETED: SHLL, SHLL2 | semantic_ir/tests/shll_shll2.rs | 2026-10-16T16:00:00Z
COMPLETED: SHRN, SHRN2 | semantic_ir/tests/shrn_shrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SHRN, SHRN2 | semantic_ir/tests/simd_fp_shrn_shrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SLI | semantic_ir/tests/simd_fp_sli.rs | 2026-10-16T16:00:00Z
COMPLETED: SLI | semantic_ir/tests/sli.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3PARTW1 | semantic_ir/tests/sm3partw1.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3PARTW1 | semantic_ir/tests/simd_fp_sm3partw1.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3PARTW2 | semantic_ir/tests/sm3partw2.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3PARTW2 | semantic_ir/tests/simd_fp_sm3partw2.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3SS1 | semantic_ir/tests/simd_fp_sm3ss1.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3SS1 | semantic_ir/tests/sm3ss1.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3TT1A | semantic_ir/tests/sm3tt1a.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3TT1A | semantic_ir/tests/simd_fp_sm3tt1a.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3TT1B | semantic_ir/tests/simd_fp_sm3tt1b.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3TT1B | semantic_ir/tests/sm3tt1b.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3TT2A | semantic_ir/tests/simd_fp_sm3tt2a.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3TT2A | semantic_ir/tests/sm3tt2a.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3TT2B | semantic_ir/tests/simd_fp_sm3tt2b.rs | 2026-10-16T16:00:00Z
COMPLETED: SM3TT2B | semantic_ir/tests/sm3tt2b.rs | 2026-10-16T16:00:00Z
COMPLETED: SM4E | semantic_ir/tests/simd_fp_sm4e.rs | 2026-10-16T16:00:00Z
COMPLETED: SM4E | semantic_ir/tests/sm4e.rs | 2026-10-16T16:00:00Z
COMPLETED: SM4EKEY | semantic_ir/tests/simd_fp_sm4ekey.rs | 2026-10-16T16:00:00Z
COMPLETED: SM4EKEY | semantic_ir/tests/sm4ekey.rs | 2026-10-16T16:00:00Z
COMPLETED: SMADDL | semantic_ir/tests/smaddl.rs | 2026-10-16T16:00:00Z
COMPLETED: SMAX (multiple and single vector) | semantic_ir/tests/sme_smax_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMAX (multiple vectors) | semantic_ir/tests/sme_smax_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SMAXP | semantic_ir/tests/smaxp.rs | 2026-10-16T16:00:00Z
COMPLETED: SMAXP | semantic_ir/tests/simd_fp_smaxp.rs | 2026-10-16T16:00:00Z
COMPLETED: SMAXP | semantic_ir/tests/smaxp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SMAXQV | semantic_ir/tests/smaxqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SMIN (multiple and single vector) | semantic_ir/tests/sme_smin_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMIN (multiple vectors) | semantic_ir/tests/sme_smin_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SMINP | semantic_ir/tests/sminp.rs | 2026-10-16T16:00:00Z
COMPLETED: SMINP | semantic_ir/tests/simd_fp_sminp.rs | 2026-10-16T16:00:00Z
COMPLETED: SMINP | semantic_ir/tests/sminp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SMINQV | semantic_ir/tests/sminqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLAL (multiple and indexed vector) | semantic_ir/tests/sme_smlal_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLAL (multiple and single vector) | semantic_ir/tests/sme_smlal_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLAL (multiple vectors) | semantic_ir/tests/sme_smlal_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLAL, SMLAL2 (by element) | semantic_ir/tests/simd_fp_smlal_smlal2_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLAL, SMLAL2 (by element) | semantic_ir/tests/smlal_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLAL, SMLAL2 (vector) | semantic_ir/tests/simd_fp_smlal_smlal2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLAL, SMLAL2 (vector) | semantic_ir/tests/smlal_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLALL (multiple and indexed vector) | semantic_ir/tests/sme_smlall_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLALL (multiple and single vector) | semantic_ir/tests/sme_smlall_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLALL (multiple vectors) | semantic_ir/tests/sme_smlall_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLSL (multiple and indexed vector) | semantic_ir/tests/sme_smlsl_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLSL (multiple and single vector) | semantic_ir/tests/sme_smlsl_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLSL (multiple vectors) | semantic_ir/tests/sme_smlsl_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLSL, SMLSL2 (by element) | semantic_ir/tests/simd_fp_smlsl_smlsl2_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLSL, SMLSL2 (by element) | semantic_ir/tests/smlsl_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLSL, SMLSL2 (vector) | semantic_ir/tests/smlsl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLSL, SMLSL2 (vector) | semantic_ir/tests/simd_fp_smlsl_smlsl2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLSLL (multiple and indexed vector) | semantic_ir/tests/sme_smlsll_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLSLL (multiple and single vector) | semantic_ir/tests/sme_smlsll_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMLSLL (multiple vectors) | semantic_ir/tests/sme_smlsll_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SMMLA (vector) | semantic_ir/tests/simd_fp_smmla_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMMLA (vector) | semantic_ir/tests/smmla_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMOD | semantic_ir/tests/smod_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: SMOP4A (2-way) | semantic_ir/tests/sme_smop4a_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: SMOP4A (4-way) | semantic_ir/tests/sme_smop4a_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: SMOP4S (2-way) | semantic_ir/tests/sme_smop4s_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: SMOP4S (4-way) | semantic_ir/tests/sme_smop4s_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: SMOPA (2-way) | semantic_ir/tests/sme_smopa_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: SMOPA (4-way) | semantic_ir/tests/sme_smopa_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: SMOPS (2-way) | semantic_ir/tests/sme_smops_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: SMOPS (4-way) | semantic_ir/tests/sme_smops_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: SMOV | semantic_ir/tests/smov.rs | 2026-10-16T16:00:00Z
COMPLETED: SMOV | semantic_ir/tests/simd_fp_smov.rs | 2026-10-16T16:00:00Z
COMPLETED: SMULL, SMULL2 (by element) | semantic_ir/tests/simd_fp_smull_smull2_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SMULL, SMULL2 (by element) | semantic_ir/tests/smull_smulll2_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SMULL, SMULL2 (vector) | semantic_ir/tests/simd_fp_smull_smull2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SMULL, SMULL2 (vector) | semantic_ir/tests/smull_smulll2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQABS | semantic_ir/tests/simd_fp_sqabs.rs | 2026-10-16T16:00:00Z
COMPLETED: SQABS | semantic_ir/tests/sqabs.rs | 2026-10-16T16:00:00Z
COMPLETED: SQABS | semantic_ir/tests/sqabs_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: SQCVT (four registers) | semantic_ir/tests/sme_sqcvt_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: SQCVT (two registers) | semantic_ir/tests/sme_sqcvt_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: SQCVTU (four registers) | semantic_ir/tests/sme_sqcvtu_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: SQCVTU (two registers) | semantic_ir/tests/sme_sqcvtu_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: SQCVTUN | semantic_ir/tests/sme_sqcvtun.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMLAL, SQDMLAL2 (by element) | semantic_ir/tests/simd_fp_sqdmlal_sqdmlal2_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMLAL, SQDMLAL2 (by element) | semantic_ir/tests/sqdmlal_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMLAL, SQDMLAL2 (vector) | semantic_ir/tests/simd_fp_sqdmlal_sqdmlal2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMLAL, SQDMLAL2 (vector) | semantic_ir/tests/sqdmlal_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMLSL, SQDMLSL2 (by element) | semantic_ir/tests/sqdmlsl_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMLSL, SQDMLSL2 (by element) | semantic_ir/tests/simd_fp_sqdmlsl_sqdmlsl2_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMLSL, SQDMLSL2 (vector) | semantic_ir/tests/sqdmlsl_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMLSL, SQDMLSL2 (vector) | semantic_ir/tests/simd_fp_sqdmlsl_sqdmlsl2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULH (by element) | semantic_ir/tests/sqdmulh_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULH (by element) | semantic_ir/tests/simd_fp_sqdmulh_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULH (indexed) | semantic_ir/tests/sqdmulh_sve_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULH (multiple and single vector) | semantic_ir/tests/sme_sqdmulh_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULH (multiple vectors) | semantic_ir/tests/sme_sqdmulh_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULH (vector) | semantic_ir/tests/sqdmulh_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULH (vector) | semantic_ir/tests/simd_fp_sqdmulh_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULH (vectors) | semantic_ir/tests/sqdmulh_sve_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULL, SQDMULL2 (by element) | semantic_ir/tests/simd_fp_sqdmull_sqdmull2_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULL, SQDMULL2 (by element) | semantic_ir/tests/sqdmull_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULL, SQDMULL2 (vector) | semantic_ir/tests/simd_fp_sqdmull_sqdmull2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQDMULL, SQDMULL2 (vector) | semantic_ir/tests/sqdmull_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQNEG | semantic_ir/tests/simd_fp_sqneg.rs | 2026-10-16T16:00:00Z
COMPLETED: SQNEG | semantic_ir/tests/sqneg.rs | 2026-10-16T16:00:00Z
COMPLETED: SQNEG | semantic_ir/tests/sqneg_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMLAH (by element) | semantic_ir/tests/simd_fp_sqrdmlah_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMLAH (by element) | semantic_ir/tests/sqrdmlah_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMLAH (vector) | semantic_ir/tests/sqrdmlah_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMLAH (vector) | semantic_ir/tests/simd_fp_sqrdmlah_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMLAH (vectors) | semantic_ir/tests/sqrdmlah_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMLSH (by element) | semantic_ir/tests/sqrdmlsh_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMLSH (by element) | semantic_ir/tests/simd_fp_sqrdmlsh_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMLSH (vector) | semantic_ir/tests/simd_fp_sqrdmlsh_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMLSH (vector) | semantic_ir/tests/sqrdmlsh_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMLSH (vectors) | semantic_ir/tests/sqrdmlsh_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMULH (by element) | semantic_ir/tests/simd_fp_sqrdmulh_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMULH (by element) | semantic_ir/tests/sqrdmulh_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMULH (vector) | semantic_ir/tests/simd_fp_sqrdmulh_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRDMULH (vector) | semantic_ir/tests/sqrdmulh_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHL | semantic_ir/tests/sqrshl_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHL | semantic_ir/tests/simd_fp_sqrshl.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHL | semantic_ir/tests/sqrshl.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHLR | semantic_ir/tests/sqrshlr_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHR (four registers) | semantic_ir/tests/sme_sqrshr_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHR (two registers) | semantic_ir/tests/sme_sqrshr_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHRN | semantic_ir/tests/sme_sqrshrn.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHRN, SQRSHRN2 | semantic_ir/tests/simd_fp_sqrshrn_sqrshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHRN, SQRSHRN2 | semantic_ir/tests/sqrshrn_sqrshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHRU (four registers) | semantic_ir/tests/sme_sqrshru_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHRU (two registers) | semantic_ir/tests/sme_sqrshru_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHRUN | semantic_ir/tests/sme_sqrshrun.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHRUN, SQRSHRUN2 | semantic_ir/tests/simd_fp_sqrshrun_sqrshrun2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQRSHRUN, SQRSHRUN2 | semantic_ir/tests/sqrshrun_sqrshrun2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHL (immediate) | semantic_ir/tests/sqshl_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHL (immediate) | semantic_ir/tests/simd_fp_sqshl_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHL (register) | semantic_ir/tests/sqshl_register.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHL (register) | semantic_ir/tests/simd_fp_sqshl_register.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHL (vectors) | semantic_ir/tests/sqshl_sve_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHLR (vectors) | semantic_ir/tests/sqshlr_sve_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHLU (immediate) | semantic_ir/tests/sqshlu_sve_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHRN, SQSHRN2 | semantic_ir/tests/sqshrn_sqshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHRN, SQSHRN2 | semantic_ir/tests/simd_fp_sqshrn_sqshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHRUN, SQSHRUN2 | semantic_ir/tests/sqshrun_sqshrun2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQSHRUN, SQSHRUN2 | semantic_ir/tests/simd_fp_sqshrun_sqshrun2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTN, SQXTN2 | semantic_ir/tests/simd_fp_sqxtn_sqxtn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTN, SQXTN2 | semantic_ir/tests/sqxtn_sqxtn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTUN, SQXTUN2 | semantic_ir/tests/sqxtun_sqxtun2.rs | 2026-10-16T16:00:00Z
COMPLETED: SQXTUN, SQXTUN2 | semantic_ir/tests/simd_fp_sqxtun_sqxtun2.rs | 2026-10-16T16:00:00Z
COMPLETED: SRHADD | semantic_ir/tests/srhadd.rs | 2026-10-16T16:00:00Z
COMPLETED: SRHADD | semantic_ir/tests/simd_fp_srhadd.rs | 2026-10-16T16:00:00Z
COMPLETED: SRI | semantic_ir/tests/sri.rs | 2026-10-16T16:00:00Z
COMPLETED: SRI | semantic_ir/tests/simd_fp_sri.rs | 2026-10-16T16:00:00Z
COMPLETED: SRSHL | semantic_ir/tests/srshl.rs | 2026-10-16T16:00:00Z
COMPLETED: SRSHL | semantic_ir/tests/srshl_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SRSHL | semantic_ir/tests/simd_fp_srshl.rs | 2026-10-16T16:00:00Z
COMPLETED: SRSHL (multiple and single vector) | semantic_ir/tests/sme_srshl_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SRSHL (multiple vectors) | semantic_ir/tests/sme_srshl_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SRSRA | semantic_ir/tests/simd_fp_srsra.rs | 2026-10-16T16:00:00Z
COMPLETED: SRSRA | semantic_ir/tests/srsra.rs | 2026-10-16T16:00:00Z
COMPLETED: SSHLL, SSHLL2 | semantic_ir/tests/simd_fp_sshll_sshll2.rs | 2026-10-16T16:00:00Z
COMPLETED: SSHLL, SSHLL2 | semantic_ir/tests/sshll_sshll2.rs | 2026-10-16T16:00:00Z
COMPLETED: SSHLLB | semantic_ir/tests/sshll_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SSUBL, SSUBL2 | semantic_ir/tests/simd_fp_ssubl_ssubl2.rs | 2026-10-16T16:00:00Z
COMPLETED: SSUBL, SSUBL2 | semantic_ir/tests/ssubl_ssubl2.rs | 2026-10-16T16:00:00Z
COMPLETED: SSUBLB | semantic_ir/tests/ssublb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SSUBLBT | semantic_ir/tests/ssublbt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SSUBLT | semantic_ir/tests/ssublt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SSUBLTB | semantic_ir/tests/ssubltb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SSUBW, SSUBW2 | semantic_ir/tests/simd_fp_ssubw_ssubw2.rs | 2026-10-16T16:00:00Z
COMPLETED: SSUBW, SSUBW2 | semantic_ir/tests/ssubw_ssubw2.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1 (multiple structures) | semantic_ir/tests/st1_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1 (multiple structures) | semantic_ir/tests/simd_fp_st1_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1 (single structure) | semantic_ir/tests/simd_fp_st1_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1 (single structure) | semantic_ir/tests/st1_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1B (scalar plus immediate) | semantic_ir/tests/st1b_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1B (scalar plus immediate, strided registers) | semantic_ir/tests/sme_st1b_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1B (scalar plus scalar, strided registers) | semantic_ir/tests/sme_st1b_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1B (scalar plus scalar, tile slice) | semantic_ir/tests/sme_st1b_scalar_plus_scalar_tile_slice.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1D (scalar plus immediate, strided registers) | semantic_ir/tests/sme_st1d_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1D (scalar plus scalar) | semantic_ir/tests/st1d_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1D (scalar plus scalar, strided registers) | semantic_ir/tests/sme_st1d_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1D (scalar plus scalar, tile slice) | semantic_ir/tests/sme_st1d_scalar_plus_scalar_tile_slice.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1H (scalar plus immediate) | semantic_ir/tests/st1h_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1H (scalar plus immediate, strided registers) | semantic_ir/tests/sme_st1h_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1H (scalar plus scalar, strided registers) | semantic_ir/tests/sme_st1h_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1H (scalar plus scalar, tile slice) | semantic_ir/tests/sme_st1h_scalar_plus_scalar_tile_slice.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1Q | semantic_ir/tests/sme_st1q.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1W (scalar plus immediate, strided registers) | semantic_ir/tests/sme_st1w_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1W (scalar plus scalar) | semantic_ir/tests/st1w_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1W (scalar plus scalar, strided registers) | semantic_ir/tests/sme_st1w_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: ST1W (scalar plus scalar, tile slice) | semantic_ir/tests/sme_st1w_scalar_plus_scalar_tile_slice.rs | 2026-10-16T16:00:00Z
COMPLETED: ST2 (multiple structures) | semantic_ir/tests/st2_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: ST2 (multiple structures) | semantic_ir/tests/simd_fp_st2_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: ST2 (single structure) | semantic_ir/tests/st2_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: ST2 (single structure) | semantic_ir/tests/simd_fp_st2_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: ST2D | semantic_ir/tests/st2d_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST2H | semantic_ir/tests/st2h_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST2Q | semantic_ir/tests/st2q_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST2W | semantic_ir/tests/st2w_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST3 (multiple structures) | semantic_ir/tests/simd_fp_st3_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: ST3 (multiple structures) | semantic_ir/tests/st3_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: ST3 (single structure) | semantic_ir/tests/st3_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: ST3 (single structure) | semantic_ir/tests/simd_fp_st3_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: ST3D | semantic_ir/tests/st3d_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST3H | semantic_ir/tests/st3h_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST3Q | semantic_ir/tests/st3q_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST3W | semantic_ir/tests/st3w_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST4 (multiple structures) | semantic_ir/tests/simd_fp_st4_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: ST4 (multiple structures) | semantic_ir/tests/st4_multiple_structures.rs | 2026-10-16T16:00:00Z
COMPLETED: ST4 (single structure) | semantic_ir/tests/simd_fp_st4_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: ST4 (single structure) | semantic_ir/tests/st4_single_structure.rs | 2026-10-16T16:00:00Z
COMPLETED: ST4D | semantic_ir/tests/st4d_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST4H | semantic_ir/tests/st4h_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST4Q | semantic_ir/tests/st4q_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ST4W | semantic_ir/tests/st4w_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: STADD | semantic_ir/tests/stadd.rs | 2026-10-16T16:00:00Z
COMPLETED: STADDB | semantic_ir/tests/staddb.rs | 2026-10-16T16:00:00Z
COMPLETED: STADDH | semantic_ir/tests/staddh.rs | 2026-10-16T16:00:00Z
COMPLETED: STBFADD, STBFADDL | semantic_ir/tests/simd_fp_stbfadd_stbfaddl.rs | 2026-10-16T16:00:00Z
COMPLETED: STBFADD, STBFADDL | semantic_ir/tests/stbfadd_stbfaddl.rs | 2026-10-16T16:00:00Z
COMPLETED: STBFMAX, STBFMAXL | semantic_ir/tests/stbfmax_stbfmaxl.rs | 2026-10-16T16:00:00Z
COMPLETED: STBFMAX, STBFMAXL | semantic_ir/tests/simd_fp_stbfmax_stbfmaxl.rs | 2026-10-16T16:00:00Z
COMPLETED: STBFMAXNM, STBFMAXNML | semantic_ir/tests/simd_fp_stbfmaxnm_stbfmaxnml.rs | 2026-10-16T16:00:00Z
COMPLETED: STBFMAXNM, STBFMAXNML | semantic_ir/tests/stbfmaxnm_stbfmaxnml.rs | 2026-10-16T16:00:00Z
COMPLETED: STBFMIN, STBFMINL | semantic_ir/tests/stbfmin_ldbfminl.rs | 2026-10-16T16:00:00Z
COMPLETED: STBFMIN, STBFMINL | semantic_ir/tests/stbfmin_stbfminl.rs | 2026-10-16T16:00:00Z
COMPLETED: STBFMIN, STBFMINL | semantic_ir/tests/simd_fp_stbfmin_stbfminl.rs | 2026-10-16T16:00:00Z
COMPLETED: STBFMINNM, STBFMINNML | semantic_ir/tests/stbfminnm_stbfminnml.rs | 2026-10-16T16:00:00Z
COMPLETED: STBFMINNM, STBFMINNML | semantic_ir/tests/simd_fp_stbfminnm_stbfminnml.rs | 2026-10-16T16:00:00Z
COMPLETED: STCLR | semantic_ir/tests/stclr.rs | 2026-10-16T16:00:00Z
COMPLETED: STCLRB | semantic_ir/tests/stclrb.rs | 2026-10-16T16:00:00Z
COMPLETED: STCLRH | semantic_ir/tests/stclrh.rs | 2026-10-16T16:00:00Z
COMPLETED: STEOR | semantic_ir/tests/steor.rs | 2026-10-16T16:00:00Z
COMPLETED: STEORB | semantic_ir/tests/steorb.rs | 2026-10-16T16:00:00Z
COMPLETED: STEORH | semantic_ir/tests/steorh.rs | 2026-10-16T16:00:00Z
COMPLETED: STFADD, STFADDL | semantic_ir/tests/stfadd_stfaddl.rs | 2026-10-16T16:00:00Z
COMPLETED: STFADD, STFADDL | semantic_ir/tests/simd_fp_stfadd_stfaddl.rs | 2026-10-16T16:00:00Z
COMPLETED: STFMAX, STFMAXL | semantic_ir/tests/simd_fp_stfmax_stfmaxl.rs | 2026-10-16T16:00:00Z
COMPLETED: STFMAX, STFMAXL | semantic_ir/tests/stfmax_stfmaxl.rs | 2026-10-16T16:00:00Z
COMPLETED: STFMAXNM, STFMAXNML | semantic_ir/tests/simd_fp_stfmaxnm_stfmaxnml.rs | 2026-10-16T16:00:00Z
COMPLETED: STFMAXNM, STFMAXNML | semantic_ir/tests/stfmaxnm_stfmaxnml.rs | 2026-10-16T16:00:00Z
COMPLETED: STFMIN, STFMINL | semantic_ir/tests/stfmin_stfminl.rs | 2026-10-16T16:00:00Z
COMPLETED: STFMIN, STFMINL | semantic_ir/tests/simd_fp_stfmin_stfminl.rs | 2026-10-16T16:00:00Z
COMPLETED: STFMINNM, STFMINNML | semantic_ir/tests/stfminnm_stfminnml.rs | 2026-10-16T16:00:00Z
COMPLETED: STFMINNM, STFMINNML | semantic_ir/tests/simd_fp_stfminnm_stfminnml.rs | 2026-10-16T16:00:00Z
COMPLETED: STL1 (SIMD&FP) | semantic_ir/tests/stl1_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: STL1 (SIMD&FP) | semantic_ir/tests/simd_fp_stl1_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: STLUR (SIMD&FP) | semantic_ir/tests/simd_fp_stlur_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: STLUR (SIMD&FP) | semantic_ir/tests/stlur_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: STMOPA (2-way) | semantic_ir/tests/sme_stmopa_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: STMOPA (4-way) | semantic_ir/tests/sme_stmopa_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: STNP (SIMD&FP) | semantic_ir/tests/stnp_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: STNP (SIMD&FP) | semantic_ir/tests/simd_fp_stnp_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1B (scalar plus immediate) | semantic_ir/tests/stnt1b_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1B (scalar plus immediate, strided registers) | semantic_ir/tests/sme_stnt1b_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1B (scalar plus scalar, strided registers) | semantic_ir/tests/sme_stnt1b_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1D | semantic_ir/tests/stnt1d_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1D (scalar plus immediate, strided registers) | semantic_ir/tests/sme_stnt1d_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1D (scalar plus scalar, strided registers) | semantic_ir/tests/sme_stnt1d_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1H (scalar plus immediate) | semantic_ir/tests/stnt1h_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1H (scalar plus immediate, strided registers) | semantic_ir/tests/sme_stnt1h_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1H (scalar plus scalar, strided registers) | semantic_ir/tests/sme_stnt1h_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1W (scalar plus immediate) | semantic_ir/tests/stnt1w_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1W (scalar plus immediate, strided registers) | semantic_ir/tests/sme_stnt1w_scalar_plus_immediate_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: STNT1W (scalar plus scalar, strided registers) | semantic_ir/tests/sme_stnt1w_scalar_plus_scalar_strided_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: STP (SIMD&FP) | semantic_ir/tests/simd_fp_stp_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: STP (SIMD&FP) | semantic_ir/tests/stp_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: STR (immediate, SIMD&FP) | semantic_ir/tests/str_immediate_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: STR (immediate, SIMD&FP) | semantic_ir/tests/simd_fp_str_immediate_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: STR (register, SIMD&FP) | semantic_ir/tests/simd_fp_str_register_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: STR (register, SIMD&FP) | semantic_ir/tests/str_register_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: STSET | semantic_ir/tests/stset.rs | 2026-10-16T16:00:00Z
COMPLETED: STSETB | semantic_ir/tests/stsetb.rs | 2026-10-16T16:00:00Z
COMPLETED: STSETH | semantic_ir/tests/stseth.rs | 2026-10-16T16:00:00Z
COMPLETED: STSMAX | semantic_ir/tests/stsmax.rs | 2026-10-16T16:00:00Z
COMPLETED: STSMAXB | semantic_ir/tests/stsmaxb.rs | 2026-10-16T16:00:00Z
COMPLETED: STSMAXH | semantic_ir/tests/stsmaxh.rs | 2026-10-16T16:00:00Z
COMPLETED: STSMIN | semantic_ir/tests/stsmin.rs | 2026-10-16T16:00:00Z
COMPLETED: STSMINB | semantic_ir/tests/stsminb.rs | 2026-10-16T16:00:00Z
COMPLETED: STSMINH | semantic_ir/tests/stsminh.rs | 2026-10-16T16:00:00Z
COMPLETED: STTADD | semantic_ir/tests/sttadd.rs | 2026-10-16T16:00:00Z
COMPLETED: STTCLR | semantic_ir/tests/sttclr.rs | 2026-10-16T16:00:00Z
COMPLETED: STTNP (SIMD&FP) | semantic_ir/tests/simd_fp_sttnp_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: STTNP (SIMD&FP) | semantic_ir/tests/sttnp_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: STTP (SIMD&FP) | semantic_ir/tests/simd_fp_sttp_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: STTP (SIMD&FP) | semantic_ir/tests/sttp_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: STTSET | semantic_ir/tests/sttset.rs | 2026-10-16T16:00:00Z
COMPLETED: STUMAX | semantic_ir/tests/stumax.rs | 2026-10-16T16:00:00Z
COMPLETED: STUMAXB | semantic_ir/tests/stumaxb.rs | 2026-10-16T16:00:00Z
COMPLETED: STUMAXH | semantic_ir/tests/stumaxh.rs | 2026-10-16T16:00:00Z
COMPLETED: STUMIN | semantic_ir/tests/stumin.rs | 2026-10-16T16:00:00Z
COMPLETED: STUMINB | semantic_ir/tests/stuminb.rs | 2026-10-16T16:00:00Z
COMPLETED: STUMINH | semantic_ir/tests/stuminh.rs | 2026-10-16T16:00:00Z
COMPLETED: STUMINL | semantic_ir/tests/stuminl.rs | 2026-10-16T16:00:00Z
COMPLETED: STUR (SIMD&FP) | semantic_ir/tests/stur_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: STUR (SIMD&FP) | semantic_ir/tests/simd_fp_stur_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: SUB | semantic_ir/tests/sub_immediate_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SUB | semantic_ir/tests/sub_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: SUB | semantic_ir/tests/sub_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: SUB | semantic_ir/tests/sub_ext_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: SUB | semantic_ir/tests/sub_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: SUB (to array, array and multiple vectors) | semantic_ir/tests/sme_sub_to_array_array_and_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SUB (to array, multiple and single vector) | semantic_ir/tests/sme_sub_to_array_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SUB (to array, multiple vectors) | semantic_ir/tests/sme_sub_to_array_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: SUB (vector) | semantic_ir/tests/sub_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SUBHN, SUBHN2 | semantic_ir/tests/subhn_subhn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SUBHN, SUBHN2 | semantic_ir/tests/simd_fp_subhn_subhn2.rs | 2026-10-16T16:00:00Z
COMPLETED: SUBS | semantic_ir/tests/subs_ext_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: SUBS | semantic_ir/tests/subs_shifted.rs | 2026-10-16T16:00:00Z
COMPLETED: SUBS | semantic_ir/tests/subs_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: SUDOT (4-way, multiple and indexed vector) | semantic_ir/tests/sme_sudot_4_way_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SUDOT (4-way, multiple and single vector) | semantic_ir/tests/sme_sudot_4_way_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SUDOT (by element) | semantic_ir/tests/sudot_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SUDOT (by element) | semantic_ir/tests/simd_fp_sudot_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: SUDOT (SVE) | semantic_ir/tests/sudot_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SUMLALL (multiple and indexed vector) | semantic_ir/tests/sme_sumlall_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SUMLALL (multiple and single vector) | semantic_ir/tests/sme_sumlall_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: SUMOPA (4-way) | semantic_ir/tests/sme_sumopa_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: SUNPKHI | semantic_ir/tests/sunpkhi_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SUNPKLO | semantic_ir/tests/sunpklo_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SUQADD | semantic_ir/tests/simd_fp_suqadd.rs | 2026-10-16T16:00:00Z
COMPLETED: SUQADD | semantic_ir/tests/suqadd.rs | 2026-10-16T16:00:00Z
COMPLETED: SUQADD (SVE) | semantic_ir/tests/suqadd_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: SVDOT (2-way) | semantic_ir/tests/sme_svdot_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: SVDOT (4-way) | semantic_ir/tests/sme_svdot_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: SXTL, SXTL2 | semantic_ir/tests/simd_fp_sxtl_sxtl2.rs | 2026-10-16T16:00:00Z
COMPLETED: TBL | semantic_ir/tests/simd_fp_tbl.rs | 2026-10-16T16:00:00Z
COMPLETED: TBL | semantic_ir/tests/tbl.rs | 2026-10-16T16:00:00Z
COMPLETED: TBX | semantic_ir/tests/tbx.rs | 2026-10-16T16:00:00Z
COMPLETED: TBX | semantic_ir/tests/simd_fp_tbx.rs | 2026-10-16T16:00:00Z
COMPLETED: TRN1 | semantic_ir/tests/simd_fp_trn1.rs | 2026-10-16T16:00:00Z
COMPLETED: TRN1 | semantic_ir/tests/trn1.rs | 2026-10-16T16:00:00Z
COMPLETED: TRN1 | semantic_ir/tests/trn1_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: TRN2 | semantic_ir/tests/trn2.rs | 2026-10-16T16:00:00Z
COMPLETED: TRN2 | semantic_ir/tests/simd_fp_trn2.rs | 2026-10-16T16:00:00Z
COMPLETED: TST | semantic_ir/tests/tst_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: TST | semantic_ir/tests/tst_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: UABA | semantic_ir/tests/simd_fp_uaba.rs | 2026-10-16T16:00:00Z
COMPLETED: UABA | semantic_ir/tests/uaba.rs | 2026-10-16T16:00:00Z
COMPLETED: UABAL, UABAL2 | semantic_ir/tests/simd_fp_uabal_uabal2.rs | 2026-10-16T16:00:00Z
COMPLETED: UABAL, UABAL2 | semantic_ir/tests/uabal_uabal2.rs | 2026-10-16T16:00:00Z
COMPLETED: UABALB | semantic_ir/tests/uabalb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UABALT | semantic_ir/tests/uabalt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UABD | semantic_ir/tests/uabd.rs | 2026-10-16T16:00:00Z
COMPLETED: UABD | semantic_ir/tests/simd_fp_uabd.rs | 2026-10-16T16:00:00Z
COMPLETED: UABD (SVE) | semantic_ir/tests/uabd_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UABDL, UABDL2 | semantic_ir/tests/simd_fp_uabdl_uabdl2.rs | 2026-10-16T16:00:00Z
COMPLETED: UABDL, UABDL2 | semantic_ir/tests/uabdl_uabdl2.rs | 2026-10-16T16:00:00Z
COMPLETED: UABDLB (SVE) | semantic_ir/tests/uabdlb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UABDLT (SVE) | semantic_ir/tests/uabdlt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UADALP | semantic_ir/tests/uadalp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UADALP | semantic_ir/tests/uadalp.rs | 2026-10-16T16:00:00Z
COMPLETED: UADALP | semantic_ir/tests/udalp.rs | 2026-10-16T16:00:00Z
COMPLETED: UADALP | semantic_ir/tests/simd_fp_uadalp.rs | 2026-10-16T16:00:00Z
COMPLETED: UADALP (SVE) | semantic_ir/tests/udalp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDL, UADDL2 | semantic_ir/tests/uaddl_uaddl2.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDL, UADDL2 | semantic_ir/tests/simd_fp_uaddl_uaddl2.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDLB (SVE) | semantic_ir/tests/uaddlb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDLBT (SVE) | semantic_ir/tests/uaddlbt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDLT (SVE) | semantic_ir/tests/uaddlt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDLV (SVE) | semantic_ir/tests/uaddlv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDV (SVE scalar) | semantic_ir/tests/uaddv_sve_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDV (SVE) | semantic_ir/tests/uaddv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDW, UADDW2 | semantic_ir/tests/uaddw_uaddw2.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDW, UADDW2 | semantic_ir/tests/simd_fp_uaddw_uaddw2.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDWB (SVE) | semantic_ir/tests/uaddwb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UADDWT (SVE) | semantic_ir/tests/uaddwt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UBFM | semantic_ir/tests/ubfm.rs | 2026-10-16T16:00:00Z
COMPLETED: UC VT F (scalar, fixed-point) | semantic_ir/tests/ucvtf_scalar_fixed-point.rs | 2026-10-16T16:00:00Z
COMPLETED: UC VT F (scalar, integer) | semantic_ir/tests/ucvtf_scalar_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: UC VT F (scalar, SIMD&FP) | semantic_ir/tests/ucvtf_scalar_simd_simd_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: UC VT F (vector, fixed-point) | semantic_ir/tests/ucvtf_vector_fixed-point.rs | 2026-10-16T16:00:00Z
COMPLETED: UC VT F (vector, integer) | semantic_ir/tests/ucvtf_vector_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: UCLAMP (SVE) | semantic_ir/tests/uclamp_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UCVTF (scalar SIMD&FP) | semantic_ir/tests/simd_fp_ucvtf_scalar_simdfp.rs | 2026-10-16T16:00:00Z
COMPLETED: UCVTF (scalar, fixed-point) | semantic_ir/tests/simd_fp_ucvtf_scalar_fixed_point.rs | 2026-10-16T16:00:00Z
COMPLETED: UCVTF (scalar, integer) | semantic_ir/tests/simd_fp_ucvtf_scalar_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: UCVTF (vector, fixed-point) | semantic_ir/tests/simd_fp_ucvtf_vector_fixed_point.rs | 2026-10-16T16:00:00Z
COMPLETED: UCVTF (vector, integer) | semantic_ir/tests/simd_fp_ucvtf_vector_integer.rs | 2026-10-16T16:00:00Z
COMPLETED: UDIV (SVE unpredicated) | semantic_ir/tests/udiv_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: UDIV (SVE) | semantic_ir/tests/udiv_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: UDIVR | semantic_ir/tests/udivr_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: UDOT (2-way, multiple and indexed vector) | semantic_ir/tests/sme_udot_2_way_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UDOT (2-way, multiple and single vector) | semantic_ir/tests/sme_udot_2_way_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UDOT (2-way, multiple vectors) | semantic_ir/tests/sme_udot_2_way_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: UDOT (4-way, multiple and indexed vector) | semantic_ir/tests/sme_udot_4_way_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UDOT (4-way, multiple and single vector) | semantic_ir/tests/sme_udot_4_way_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UDOT (4-way, multiple vectors) | semantic_ir/tests/sme_udot_4_way_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: UDOT (by element) | semantic_ir/tests/simd_fp_udot_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: UDOT (by element) | semantic_ir/tests/udot_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: UDOT (SVE) | semantic_ir/tests/udot_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UDOT (vector) | semantic_ir/tests/udot_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UDOT (vector) | semantic_ir/tests/simd_fp_udot_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMADDL | semantic_ir/tests/umaddl.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAX (multiple and single vector) | semantic_ir/tests/sme_umax_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAX (multiple vectors) | semantic_ir/tests/sme_umax_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAX (register) | semantic_ir/tests/umax_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAX (SVE predicated) | semantic_ir/tests/umax_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAXP | semantic_ir/tests/umaxp.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAXP | semantic_ir/tests/simd_fp_umaxp.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAXQV (SVE) | semantic_ir/tests/umaxqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAXV (SVE predicated) | semantic_ir/tests/umaxv_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAXV (SVE scalable) | semantic_ir/tests/umaxv_sve_scalable.rs | 2026-10-16T16:00:00Z
COMPLETED: UMAXV (SVE) | semantic_ir/tests/umaxv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UMIN (multiple and single vector) | semantic_ir/tests/sme_umin_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMIN (multiple vectors) | semantic_ir/tests/sme_umin_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: UMIN (register) | semantic_ir/tests/umin_reg.rs | 2026-10-16T16:00:00Z
COMPLETED: UMIN (SVE predicated) | semantic_ir/tests/umin_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: UMINP | semantic_ir/tests/uminp.rs | 2026-10-16T16:00:00Z
COMPLETED: UMINP | semantic_ir/tests/simd_fp_uminp.rs | 2026-10-16T16:00:00Z
COMPLETED: UMINQV (SVE) | semantic_ir/tests/uminqv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UMINV (SVE predicated) | semantic_ir/tests/uminv_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: UMINV (SVE) | semantic_ir/tests/uminv_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLAL (multiple and indexed vector) | semantic_ir/tests/sme_umlal_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLAL (multiple and single vector) | semantic_ir/tests/sme_umlal_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLAL (multiple vectors) | semantic_ir/tests/sme_umlal_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLAL, UMLAL2 (by element) | semantic_ir/tests/simd_fp_umlal_umlal2_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLAL, UMLAL2 (vector) | semantic_ir/tests/simd_fp_umlal_umlal2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLALL (multiple and indexed vector) | semantic_ir/tests/sme_umlall_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLALL (multiple and single vector) | semantic_ir/tests/sme_umlall_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLALL (multiple vectors) | semantic_ir/tests/sme_umlall_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLSL (multiple and indexed vector) | semantic_ir/tests/sme_umlsl_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLSL (multiple and single vector) | semantic_ir/tests/sme_umlsl_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLSL (multiple vectors) | semantic_ir/tests/sme_umlsl_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLSL, UMLSL2 (by element) | semantic_ir/tests/simd_fp_umlsl_umlsl2_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLSL, UMLSL2 (vector) | semantic_ir/tests/simd_fp_umlsl_umlsl2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLSLL (multiple and indexed vector) | semantic_ir/tests/sme_umlsll_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLSLL (multiple and single vector) | semantic_ir/tests/sme_umlsll_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMLSLL (multiple vectors) | semantic_ir/tests/sme_umlsll_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: UMMLA (vector) | semantic_ir/tests/ummla_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMMLA (vector) | semantic_ir/tests/simd_fp_ummla_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOD | semantic_ir/tests/umod_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOP4A (2-way) | semantic_ir/tests/sme_umop4a_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOP4A (4-way) | semantic_ir/tests/sme_umop4a_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOP4S (2-way) | semantic_ir/tests/sme_umop4s_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOP4S (4-way) | semantic_ir/tests/sme_umop4s_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOPA (2-way) | semantic_ir/tests/sme_umopa_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOPA (4-way) | semantic_ir/tests/sme_umopa_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOPS (2-way) | semantic_ir/tests/sme_umops_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOPS (4-way) | semantic_ir/tests/sme_umops_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOV | semantic_ir/tests/simd_fp_umov.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOV | semantic_ir/tests/umov.rs | 2026-10-16T16:00:00Z
COMPLETED: UMOV (SVE) | semantic_ir/tests/umov_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UMULH (SVE unpredicated) | semantic_ir/tests/umulh_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: UMULL (SVE) | semantic_ir/tests/umull_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UMULL, UMULL2 (by element) | semantic_ir/tests/simd_fp_umull_umull2_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: UMULL, UMULL2 (vector) | semantic_ir/tests/simd_fp_umull_umull2_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: UNEG (SVE predicated) | semantic_ir/tests/uneg_sve_predicated.rs | 2026-10-16T16:00:00Z
COMPLETED: UQADD (SVE) | semantic_ir/tests/uqadd_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UQCVT (four registers) | semantic_ir/tests/sme_uqcvt_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: UQCVT (two registers) | semantic_ir/tests/sme_uqcvt_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: UQRSHL | semantic_ir/tests/uqrshl.rs | 2026-10-16T16:00:00Z
COMPLETED: UQRSHL | semantic_ir/tests/simd_fp_uqrshl.rs | 2026-10-16T16:00:00Z
COMPLETED: UQRSHR (four registers) | semantic_ir/tests/sme_uqrshr_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: UQRSHR (two registers) | semantic_ir/tests/sme_uqrshr_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: UQRSHRN | semantic_ir/tests/sme_uqrshrn.rs | 2026-10-16T16:00:00Z
COMPLETED: UQRSHRN, UQRSHRN2 | semantic_ir/tests/uqrshrn_uqrshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: UQRSHRN, UQRSHRN2 | semantic_ir/tests/simd_fp_uqrshrn_uqrshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHL (immediate) | semantic_ir/tests/uqshl_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHL (immediate) | semantic_ir/tests/simd_fp_uqshl_immediate.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHL (register) | semantic_ir/tests/uqshl_register.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHL (register) | semantic_ir/tests/simd_fp_uqshl_register.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHL (SVE) | semantic_ir/tests/uqshl_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHRN (SVE) | semantic_ir/tests/uqshrnb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHRN, UQSHRN2 | semantic_ir/tests/simd_fp_uqshrn_uqshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHRN, UQSHRN2 | semantic_ir/tests/uqshrn_uqshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSHRNT (SVE) | semantic_ir/tests/uqshrnt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UQSUB (SVE) | semantic_ir/tests/uqsub_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UQXTN, UQXTN2 | semantic_ir/tests/uqxtn_uqxtn2.rs | 2026-10-16T16:00:00Z
COMPLETED: UQXTN, UQXTN2 | semantic_ir/tests/simd_fp_uqxtn_uqxtn2.rs | 2026-10-16T16:00:00Z
COMPLETED: URECPE | semantic_ir/tests/urecpe.rs | 2026-10-16T16:00:00Z
COMPLETED: URECPE | semantic_ir/tests/simd_fp_urecpe.rs | 2026-10-16T16:00:00Z
COMPLETED: URECPE (SVE) | semantic_ir/tests/urecpe_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: URHADD | semantic_ir/tests/urhadd.rs | 2026-10-16T16:00:00Z
COMPLETED: URHADD | semantic_ir/tests/simd_fp_urhadd.rs | 2026-10-16T16:00:00Z
COMPLETED: URHADD (SVE) | semantic_ir/tests/urhadd_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: URSHL | semantic_ir/tests/simd_fp_urshl.rs | 2026-10-16T16:00:00Z
COMPLETED: URSHL | semantic_ir/tests/urshl.rs | 2026-10-16T16:00:00Z
COMPLETED: URSHL (multiple and single vector) | semantic_ir/tests/sme_urshl_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: URSHL (multiple vectors) | semantic_ir/tests/sme_urshl_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: URSHL (SVE) | semantic_ir/tests/urshl_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: URSHLR | semantic_ir/tests/urshrl_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: URSHR (merging) | semantic_ir/tests/urshr_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: URSQRTE | semantic_ir/tests/ursqrte.rs | 2026-10-16T16:00:00Z
COMPLETED: URSQRTE | semantic_ir/tests/simd_fp_ursqrte.rs | 2026-10-16T16:00:00Z
COMPLETED: URSQRTE (SVE) | semantic_ir/tests/ursqrte_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: URSRA | semantic_ir/tests/ursra.rs | 2026-10-16T16:00:00Z
COMPLETED: URSRA | semantic_ir/tests/simd_fp_ursra.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (4-way, multiple and indexed vector) | semantic_ir/tests/sme_usdot_4_way_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (4-way, multiple and single vector) | semantic_ir/tests/sme_usdot_4_way_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (4-way, multiple vectors) | semantic_ir/tests/sme_usdot_4_way_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (by element) | semantic_ir/tests/usdot_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (by element) | semantic_ir/tests/simd_fp_usdot_by_element.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (vector) | semantic_ir/tests/simd_fp_usdot_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (vector) | semantic_ir/tests/usdot_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (vectors) | semantic_ir/tests/usdot_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: USHLL, USHLL2 | semantic_ir/tests/simd_fp_ushll_ushll2.rs | 2026-10-16T16:00:00Z
COMPLETED: USHLL, USHLL2 | semantic_ir/tests/ushll_ushll2.rs | 2026-10-16T16:00:00Z
COMPLETED: USHLLB | semantic_ir/tests/ushll_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: USHRN, USHRN2 | semantic_ir/tests/ushrn_ushrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: USMLALL (multiple and indexed vector) | semantic_ir/tests/sme_usmlall_multiple_and_indexed_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USMLALL (multiple and single vector) | semantic_ir/tests/sme_usmlall_multiple_and_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USMLALL (multiple vectors) | semantic_ir/tests/sme_usmlall_multiple_vectors.rs | 2026-10-16T16:00:00Z
COMPLETED: USMMLA (vector) | semantic_ir/tests/usmmla_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USMMLA (vector) | semantic_ir/tests/simd_fp_usmmla_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: USMOPA (4-way) | semantic_ir/tests/sme_usmopa_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: USQADD | semantic_ir/tests/usqadd.rs | 2026-10-16T16:00:00Z
COMPLETED: USQADD | semantic_ir/tests/simd_fp_usqadd.rs | 2026-10-16T16:00:00Z
COMPLETED: USQADD (SVE) | semantic_ir/tests/usqadd_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: USUBL, USUBL2 | semantic_ir/tests/usubl_usubl2.rs | 2026-10-16T16:00:00Z
COMPLETED: USUBL, USUBL2 | semantic_ir/tests/simd_fp_usubl_usubl2.rs | 2026-10-16T16:00:00Z
COMPLETED: USUBLB | semantic_ir/tests/usublb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: USUBLT | semantic_ir/tests/usublt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: USUBW, USUBW2 | semantic_ir/tests/simd_fp_usubw_usubw2.rs | 2026-10-16T16:00:00Z
COMPLETED: USUBW, USUBW2 | semantic_ir/tests/usubw_usubw2.rs | 2026-10-16T16:00:00Z
COMPLETED: USUBWB | semantic_ir/tests/usubwb_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: USUBWT | semantic_ir/tests/usubwt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UTMOPA (2-way) | semantic_ir/tests/sme_utmopa_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UTMOPA (4-way) | semantic_ir/tests/sme_utmopa_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UUNPKHI | semantic_ir/tests/uunpkhi_lo_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UVDOT (2-way) | semantic_ir/tests/sme_uvdot_2_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UVDOT (4-way) | semantic_ir/tests/sme_uvdot_4_way.rs | 2026-10-16T16:00:00Z
COMPLETED: UXTL, UXTL2 | semantic_ir/tests/simd_fp_uxtl_uxtl2.rs | 2026-10-16T16:00:00Z
COMPLETED: UZP (four registers) | semantic_ir/tests/sme_uzp_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: UZP (two registers) | semantic_ir/tests/sme_uzp_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: UZP1 | semantic_ir/tests/uzp1_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: UZP1 | semantic_ir/tests/simd_fp_uzp1.rs | 2026-10-16T16:00:00Z
COMPLETED: UZP1 | semantic_ir/tests/uzp1.rs | 2026-10-16T16:00:00Z
COMPLETED: UZP1 (SVE) | semantic_ir/tests/uzp1_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UZP2 | semantic_ir/tests/uzp2.rs | 2026-10-16T16:00:00Z
COMPLETED: UZP2 | semantic_ir/tests/simd_fp_uzp2.rs | 2026-10-16T16:00:00Z
COMPLETED: UZP2 (SVE) | semantic_ir/tests/uzp2_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: UZPQ1 | semantic_ir/tests/uzpq_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VCGE (scalar) | semantic_ir/tests/vcge_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VCGE (vector) | semantic_ir/tests/vcge_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VCGT (scalar) | semantic_ir/tests/vcgt_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VCGT (vector) | semantic_ir/tests/vcgt_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VCVT (F16 to F32) | semantic_ir/tests/vcvt_f16_f32.rs | 2026-10-16T16:00:00Z
COMPLETED: VCVT (F32 to F16) | semantic_ir/tests/vcvt_f32_f16.rs | 2026-10-16T16:00:00Z
COMPLETED: VCVT (F32/F64) | semantic_ir/tests/vcvt_f32_f64.rs | 2026-10-16T16:00:00Z
COMPLETED: VCVT (int to FP) | semantic_ir/tests/vcvt_int_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: VCVTA (FP to int) | semantic_ir/tests/vcvta_fp_int.rs | 2026-10-16T16:00:00Z
COMPLETED: VCVTM (FP to int) | semantic_ir/tests/vcvtm_fp_int.rs | 2026-10-16T16:00:00Z
COMPLETED: VCVTN (FP to int) | semantic_ir/tests/vcvtn_fp_int.rs | 2026-10-16T16:00:00Z
COMPLETED: VCVTX (FP to int) | semantic_ir/tests/vcvtx_fp_int.rs | 2026-10-16T16:00:00Z
COMPLETED: VCVTZ (FP to int) | semantic_ir/tests/vcvtz_fp_int.rs | 2026-10-16T16:00:00Z
COMPLETED: VDIV (scalar) | semantic_ir/tests/vdiv_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VDIV (vector) | semantic_ir/tests/vdiv_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VELVT (SVE) | semantic_ir/tests/velvt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VEXT | semantic_ir/tests/vext.rs | 2026-10-16T16:00:00Z
COMPLETED: VFMA (scalar) | semantic_ir/tests/vfma_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VFMA (vector) | semantic_ir/tests/vfma_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VFMS (scalar) | semantic_ir/tests/vfms_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VFMS (vector) | semantic_ir/tests/vfms_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VFNMA (scalar) | semantic_ir/tests/vfnma_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VFNMA (vector) | semantic_ir/tests/vfnma_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VFNMS (scalar) | semantic_ir/tests/vfnms_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VFNMS (vector) | semantic_ir/tests/vfnms_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VMAXNM (scalar) | semantic_ir/tests/vmaxnm_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VMAXNM (vector) | semantic_ir/tests/vmaxnm_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VMINNM (scalar) | semantic_ir/tests/vminnm_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VMINNM (vector) | semantic_ir/tests/vminnm_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VMOV (scalar) | semantic_ir/tests/vmov_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VMOV (vector) | semantic_ir/tests/vmov_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VMSR (SVE) | semantic_ir/tests/vmsr_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VMUL (scalar) | semantic_ir/tests/vmul_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VMUL (vector) | semantic_ir/tests/vmul_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VNEG (scalar) | semantic_ir/tests/vneg_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VNEG (SVE) | semantic_ir/tests/vneg_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VNEG (vector) | semantic_ir/tests/vneg_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VORR (vector) | semantic_ir/tests/vorr_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VRINTA (FP) | semantic_ir/tests/vrinta_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: VRINTM (FP) | semantic_ir/tests/vrintm_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: VRINTN (FP) | semantic_ir/tests/vrintn_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: VRINTP (FP) | semantic_ir/tests/vrintp_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: VRINTX (FP) | semantic_ir/tests/vrintx_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: VRINTZ (FP) | semantic_ir/tests/vrintz_fp.rs | 2026-10-16T16:00:00Z
COMPLETED: VSHLL, VSHLL2 | semantic_ir/tests/vshll_vshll2.rs | 2026-10-16T16:00:00Z
COMPLETED: VSHRN, VSHRN2 | semantic_ir/tests/vshrn_vshrn2.rs | 2026-10-16T16:00:00Z
COMPLETED: VSQRT (vector) | semantic_ir/tests/vsqrt_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: VSUB (scalar) | semantic_ir/tests/vsub_scalar.rs | 2026-10-16T16:00:00Z
COMPLETED: VSUB (SVE) | semantic_ir/tests/vsub_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: VSUB (vector) | semantic_ir/tests/vsub_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: WACC | semantic_ir/tests/wacc_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: WHILEGE | semantic_ir/tests/whilege_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: WHILEGT | semantic_ir/tests/whilegt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: WHILEHI | semantic_ir/tests/whilehi_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: WHILEHS | semantic_ir/tests/whilehs_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: WHILELE | semantic_ir/tests/whilele_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: WHILELO | semantic_ir/tests/whilelo_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: WHILELS | semantic_ir/tests/whilels_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: WHILELT | semantic_ir/tests/whilelt_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: WHILERW | semantic_ir/tests/whilerw_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: WHILEWR | semantic_ir/tests/whilewr_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: WRFFR | semantic_ir/tests/wrffr_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: XTN, XTN2 | semantic_ir/tests/simd_fp_xtn_xtn2.rs | 2026-10-16T16:00:00Z
COMPLETED: XTN, XTN2 | semantic_ir/tests/xtn_xtn2.rs | 2026-10-16T16:00:00Z
COMPLETED: ZERO (double-vector) | semantic_ir/tests/sme_zero_double_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ZERO (quad-vector) | semantic_ir/tests/sme_zero_quad_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ZERO (single-vector) | semantic_ir/tests/sme_zero_single_vector.rs | 2026-10-16T16:00:00Z
COMPLETED: ZIP (four registers) | semantic_ir/tests/sme_zip_four_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: ZIP (two registers) | semantic_ir/tests/sme_zip_two_registers.rs | 2026-10-16T16:00:00Z
COMPLETED: ZIP1 | semantic_ir/tests/zip1.rs | 2026-10-16T16:00:00Z
COMPLETED: ZIP1 | semantic_ir/tests/zip1_sve_unpredicated.rs | 2026-10-16T16:00:00Z
COMPLETED: ZIP1 | semantic_ir/tests/simd_fp_zip1.rs | 2026-10-16T16:00:00Z
COMPLETED: ZIP1 (SVE) | semantic_ir/tests/zip1_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ZIP2 | semantic_ir/tests/zip2.rs | 2026-10-16T16:00:00Z
COMPLETED: ZIP2 | semantic_ir/tests/simd_fp_zip2.rs | 2026-10-16T16:00:00Z
COMPLETED: ZIP2 (SVE) | semantic_ir/tests/zip2_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: ZIPQ1 | semantic_ir/tests/zipq_sve.rs | 2026-10-16T16:00:00Z
COMPLETED: CAS | semantic_ir/tests/cas.rs | 2026-10-16T16:00:00Z
COMPLETED: CASA | semantic_ir/tests/cas.rs | 2026-10-16T16:00:00Z
COMPLETED: CASAL | semantic_ir/tests/cas.rs | 2026-10-16T16:00:00Z
COMPLETED: CASL | semantic_ir/tests/cas.rs | 2026-10-16T16:00:00Z
COMPLETED: CASH | semantic_ir/tests/cash.rs | 2026-10-16T16:00:00Z
COMPLETED: CASAH | semantic_ir/tests/cash.rs | 2026-10-16T16:00:00Z
COMPLETED: CASALH | semantic_ir/tests/cash.rs | 2026-10-16T16:00:00Z
COMPLETED: CASLH | semantic_ir/tests/cash.rs | 2026-10-16T16:00:00Z
COMPLETED: CASP | semantic_ir/tests/casp.rs | 2026-10-16T16:00:00Z
COMPLETED: CASPA | semantic_ir/tests/casp.rs | 2026-10-16T16:00:00Z
COMPLETED: CASPAL | semantic_ir/tests/casp.rs | 2026-10-16T16:00:00Z
COMPLETED: CASPL | semantic_ir/tests/casp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSET | semantic_ir/tests/ldset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETA | semantic_ir/tests/ldset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETAL | semantic_ir/tests/ldset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETL | semantic_ir/tests/ldset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETB | semantic_ir/tests/ldsetb.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETAB | semantic_ir/tests/ldsetb.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETALB | semantic_ir/tests/ldsetb.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETLB | semantic_ir/tests/ldsetb.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETH | semantic_ir/tests/ldseth.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETAH | semantic_ir/tests/ldseth.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETALH | semantic_ir/tests/ldseth.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETLH | semantic_ir/tests/ldseth.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTCLR | semantic_ir/tests/ldtclr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTCLRA | semantic_ir/tests/ldtclr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTCLRAL | semantic_ir/tests/ldtclr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTCLRL | semantic_ir/tests/ldtclr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTSET | semantic_ir/tests/ldtset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTSETA | semantic_ir/tests/ldtset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTSETAL | semantic_ir/tests/ldtset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTSETL | semantic_ir/tests/ldtset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDXR | semantic_ir/tests/ldxr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAXR | semantic_ir/tests/ldxr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDXRH | semantic_ir/tests/ldxr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDXP | semantic_ir/tests/ldxp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAXP | semantic_ir/tests/ldxp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTXR | semantic_ir/tests/ldtxr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAPR | semantic_ir/tests/ldapr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAPRH | semantic_ir/tests/ldapr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAPURSH | semantic_ir/tests/ldapr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAR | semantic_ir/tests/ldar.rs | 2026-10-16T16:00:00Z
COMPLETED: LDARH | semantic_ir/tests/ldar.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAXRH | semantic_ir/tests/ldaxrh.rs | 2026-10-16T16:00:00Z
COMPLETED: LDG | semantic_ir/tests/ldg.rs | 2026-10-16T16:00:00Z
COMPLETED: LDGM | semantic_ir/tests/ldgm.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNF1D | semantic_ir/tests/ldnf1d.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNF1H | semantic_ir/tests/ldnf1d.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNF1W | semantic_ir/tests/ldnf1d.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SH | semantic_ir/tests/ldsh_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SW | semantic_ir/tests/ldsw_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: SVDOT (2-way, indexed) | semantic_ir/tests/svdot_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: SVDOT (2-way, vectors) | semantic_ir/tests/svdot_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (indexed) | semantic_ir/tests/udot_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (2-way, vectors) | semantic_ir/tests/udot_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: MADPT | semantic_ir/tests/madpt.rs | 2026-10-16T16:00:00Z
COMPLETED: CAS | semantic_ir/tests/cas.rs | 2026-10-16T16:00:00Z
COMPLETED: CASA | semantic_ir/tests/cas.rs | 2026-10-16T16:00:00Z
COMPLETED: CASAL | semantic_ir/tests/cas.rs | 2026-10-16T16:00:00Z
COMPLETED: CASL | semantic_ir/tests/cas.rs | 2026-10-16T16:00:00Z
COMPLETED: CASH | semantic_ir/tests/cash.rs | 2026-10-16T16:00:00Z
COMPLETED: CASAH | semantic_ir/tests/cash.rs | 2026-10-16T16:00:00Z
COMPLETED: CASALH | semantic_ir/tests/cash.rs | 2026-10-16T16:00:00Z
COMPLETED: CASLH | semantic_ir/tests/cash.rs | 2026-10-16T16:00:00Z
COMPLETED: CASP | semantic_ir/tests/casp.rs | 2026-10-16T16:00:00Z
COMPLETED: CASPA | semantic_ir/tests/casp.rs | 2026-10-16T16:00:00Z
COMPLETED: CASPAL | semantic_ir/tests/casp.rs | 2026-10-16T16:00:00Z
COMPLETED: CASPL | semantic_ir/tests/casp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSET | semantic_ir/tests/ldset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETA | semantic_ir/tests/ldset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETAL | semantic_ir/tests/ldset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETL | semantic_ir/tests/ldset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETB | semantic_ir/tests/ldsetb.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETAB | semantic_ir/tests/ldsetb.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETALB | semantic_ir/tests/ldsetb.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETLB | semantic_ir/tests/ldsetb.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETH | semantic_ir/tests/ldseth.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETAH | semantic_ir/tests/ldseth.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETALH | semantic_ir/tests/ldseth.rs | 2026-10-16T16:00:00Z
COMPLETED: LDSETLH | semantic_ir/tests/ldseth.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTCLR | semantic_ir/tests/ldtclr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTCLRA | semantic_ir/tests/ldtclr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTCLRAL | semantic_ir/tests/ldtclr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTCLRL | semantic_ir/tests/ldtclr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTSET | semantic_ir/tests/ldtset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTSETA | semantic_ir/tests/ldtset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTSETAL | semantic_ir/tests/ldtset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTSETL | semantic_ir/tests/ldtset.rs | 2026-10-16T16:00:00Z
COMPLETED: LDXR | semantic_ir/tests/ldxr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAXR | semantic_ir/tests/ldxr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDXRH | semantic_ir/tests/ldxr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDXP | semantic_ir/tests/ldxp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAXP | semantic_ir/tests/ldxp.rs | 2026-10-16T16:00:00Z
COMPLETED: LDTXR | semantic_ir/tests/ldtxr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAPR | semantic_ir/tests/ldapr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAPRH | semantic_ir/tests/ldapr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAPURSH | semantic_ir/tests/ldapr.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAR | semantic_ir/tests/ldar.rs | 2026-10-16T16:00:00Z
COMPLETED: LDARH | semantic_ir/tests/ldar.rs | 2026-10-16T16:00:00Z
COMPLETED: LDAXRH | semantic_ir/tests/ldaxrh.rs | 2026-10-16T16:00:00Z
COMPLETED: LDG | semantic_ir/tests/ldg.rs | 2026-10-16T16:00:00Z
COMPLETED: LDGM | semantic_ir/tests/ldgm.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNF1D | semantic_ir/tests/ldnf1d.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNF1H | semantic_ir/tests/ldnf1d.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNF1W | semantic_ir/tests/ldnf1d.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SH | semantic_ir/tests/ldsh_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SW | semantic_ir/tests/ldsw_imm.rs | 2026-10-16T16:00:00Z
COMPLETED: SVDOT (2-way, indexed) | semantic_ir/tests/svdot_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: SVDOT (2-way, vectors) | semantic_ir/tests/svdot_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (indexed) | semantic_ir/tests/udot_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: USDOT (2-way, vectors) | semantic_ir/tests/udot_indexed.rs | 2026-10-16T16:00:00Z
COMPLETED: MADPT | semantic_ir/tests/madpt.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SH | semantic_ir/tests/ld1sh_vec.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SW | semantic_ir/tests/ld1sw_vec.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1D | semantic_ir/tests/ld1d_vec.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1H | semantic_ir/tests/ld1h_vec.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1W | semantic_ir/tests/ld1w_vec.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1H | semantic_ir/tests/ldnt1h_vec.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1W | semantic_ir/tests/ldnt1w_vec.rs | 2026-10-16T16:00:00Z





COMPLETED: CFVT | semantic_ir/tests/cfvt.rs | 2026-10-16T16:00:00Z
COMPLETED: CVT | semantic_ir/tests/cvt.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SH | semantic_ir/tests/ld1b.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SW | semantic_ir/tests/ld1w_vec.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1D | semantic_ir/tests/ld1d_vec.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1H | semantic_ir/tests/ldnt1h_vec.rs | 2026-10-16T16:00:00Z
COMPLETED: LDNT1W | semantic_ir/tests/ldnt1w_vec.rs | 2026-10-16T16:00:00Z
COMPLETED: CFVT | semantic_ir/tests/cfvt.rs | 2026-10-16T16:00:00Z
COMPLETED: CVT | semantic_ir/tests/cvt.rs | 2026-10-16T16:00:00Z
COMPLETED: LD1SH | semantic_ir/tests/ld1b.rs | 2026-10-16T16:00:00Z
COMPLETED: CFBS | semantic_ir/tests/cabs.rs | 2026-10-16T16:00:00Z
COMPLETED: FCMP | semantic_ir/tests/fcmp.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLA | semantic_ir/tests/fmla.rs | 2026-10-16T16:00:00Z
COMPLETED: FMLS | semantic_ir/tests/fmls.rs | 2026-10-16T16:00:00Z
COMPLETED: CBLE | semantic_ir/tests/cblas.rs | 2026-10-16T16:00:00Z
COMPLETED: CBLT | semantic_ir/tests/cbls.rs | 2026-10-16T16:00:00Z
COMPLETED: CBLT | semantic_ir/tests/cblt.rs | 2026-10-16T16:00:00Z
COMPLETED: CBLE | semantic_ir/tests/cblt_eq.rs | 2026-10-16T16:00:00Z
COMPLETED: CMLT | semantic_ir/tests/cmlt.rs | 2026-10-16T16:00:00Z
COMPLETED: CMLS | semantic_ir/tests/cmls.rs | 2026-10-16T16:00:00Z
COMPLETED: CMPE | semantic_ir/tests/cmpe.rs | 2026-10-16T16:00:00Z
COMPLETED: CMPEQ | semantic_ir/tests/cmpeq.rs | 2026-10-16T16:00:00Z
COMPLETED: CMLT | semantic_ir/tests/cmplt.rs | 2026-10-16T16:00:00Z
COMPLETED: ADDHA | semantic_ir/tests/addha.rs | 2026-10-17T00:00:00Z
COMPLETED: ADDVA | semantic_ir/tests/addva.rs | 2026-10-17T00:00:00Z
COMPLETED: ADDSPL | semantic_ir/tests/addspl.rs | 2026-10-17T00:00:00Z
COMPLETED: ADDSVL | semantic_ir/tests/addsvl.rs | 2026-10-17T00:00:00Z
COMPLETED: BLRAAZ | semantic_ir/tests/blraaz.rs | 2026-10-17T00:00:00Z
COMPLETED: BRAAZ | semantic_ir/tests/braaz.rs | 2026-10-17T00:00:00Z
COMPLETED: BRKA | semantic_ir/tests/brka.rs | 2026-10-17T00:00:00Z
COMPLETED: BGRP | semantic_ir/tests/bgrp.rs | 2026-10-17T00:00:00Z
COMPLETED: BFXIL | semantic_ir/tests/bfxil.rs | 2026-10-17T00:00:00Z
COMPLETED: CNTB | semantic_ir/tests/cntb.rs | 2026-10-17T00:00:00Z
COMPLETED: CPYFE | semantic_ir/tests/cpyfe.rs | 2026-10-17T00:00:00Z
COMPLETED: BF2CVT | semantic_ir/tests/bf2cvt.rs | 2026-10-17T00:00:00Z
COMPLETED: BMOPA | semantic_ir/tests/bmopa.rs | 2026-10-17T01:00:00Z
COMPLETED: BMOPS | semantic_ir/tests/bmops.rs | 2026-10-17T01:00:00Z
COMPLETED: CNTD | semantic_ir/tests/cntd.rs | 2026-10-17T01:00:00Z
COMPLETED: CNTH | semantic_ir/tests/cnth.rs | 2026-10-17T01:00:00Z
COMPLETED: CNTW | semantic_ir/tests/cntw.rs | 2026-10-17T01:00:00Z
COMPLETED: BFI | semantic_ir/tests/bfi.rs | 2026-10-17T01:00:00Z
COMPLETED: BFDOT | semantic_ir/tests/bfdot.rs | 2026-10-17T01:00:00Z
COMPLETED: CBBLE | semantic_ir/tests/cbb.rs | 2026-10-17T02:00:00Z
COMPLETED: CBHLE | semantic_ir/tests/cbh.rs | 2026-10-17T02:00:00Z
COMPLETED: BLRAB | semantic_ir/tests/blrab.rs | 2026-10-17T02:00:00Z
COMPLETED: BRAB | semantic_ir/tests/brab.rs | 2026-10-17T02:00:00Z
COMPLETED: CASAB | semantic_ir/tests/casab.rs | 2026-10-17T02:00:00Z
COMPLETED: CASALB | semantic_ir/tests/casalb.rs | 2026-10-17T03:00:00Z
COMPLETED: CASALT | semantic_ir/tests/casalt.rs | 2026-10-17T03:00:00Z
COMPLETED: CASAT | semantic_ir/tests/casat.rs | 2026-10-17T03:00:00Z
COMPLETED: CASB | semantic_ir/tests/casb.rs | 2026-10-17T03:00:00Z
COMPLETED: CASLB | semantic_ir/tests/caslb.rs | 2026-10-17T03:00:00Z
COMPLETED: CASLT | semantic_ir/tests/caslt.rs | 2026-10-17T03:00:00Z
COMPLETED: CAST | semantic_ir/tests/cast.rs | 2026-10-17T03:00:00Z
COMPLETED: CBBLO | semantic_ir/tests/cbblo.rs | 2026-10-17T04:00:00Z
COMPLETED: CBBLS | semantic_ir/tests/cbbs.rs | 2026-10-17T04:00:00Z
COMPLETED: CBBLT | semantic_ir/tests/cbbt.rs | 2026-10-17T04:00:00Z
COMPLETED: CBHLO | semantic_ir/tests/cbhlo.rs | 2026-10-17T04:00:00Z
COMPLETED: CBHLS | semantic_ir/tests/cbhls.rs | 2026-10-17T04:00:00Z
COMPLETED: CBHLT | semantic_ir/tests/cbhlt.rs | 2026-10-17T04:00:00Z
COMPLETED: BFC | semantic_ir/tests/bfc.rs | 2026-10-17T04:00:00Z
COMPLETED: BFCVTNT | semantic_ir/tests/bfcvtnt.rs | 2026-10-17T04:00:00Z
COMPLETED: AUTDZA | semantic_ir/tests/autdza.rs | 2026-10-17T05:00:00Z
COMPLETED: AUTDZB | semantic_ir/tests/autdzb.rs | 2026-10-17T05:00:00Z
COMPLETED: AUTIA1716 | semantic_ir/tests/autia1716.rs | 2026-10-17T05:00:00Z
COMPLETED: AUTIASP | semantic_ir/tests/autiasp.rs | 2026-10-17T05:00:00Z
COMPLETED: AUTIAZ | semantic_ir/tests/autiaz.rs | 2026-10-17T05:00:00Z
COMPLETED: AUTIB1716 | semantic_ir/tests/autib1716.rs | 2026-10-17T05:00:00Z
COMPLETED: AUTIBSP | semantic_ir/tests/autibsp.rs | 2026-10-17T05:00:00Z
COMPLETED: AUTIBZ | semantic_ir/tests/autibz.rs | 2026-10-17T05:00:00Z
COMPLETED: AUTIZA | semantic_ir/tests/autiza.rs | 2026-10-17T05:00:00Z
COMPLETED: AUTIZB | semantic_ir/tests/autizb.rs | 2026-10-17T05:00:00Z
COMPLETED: BFMMLA | semantic_ir/tests/bfmmla.rs | 2026-10-17T06:00:00Z
COMPLETED: BFMOPA | semantic_ir/tests/bfmopa.rs | 2026-10-17T06:00:00Z
COMPLETED: BFMOPS | semantic_ir/tests/bfmops.rs | 2026-10-17T06:00:00Z
COMPLETED: BSL1N | semantic_ir/tests/bsl1n.rs | 2026-10-17T06:00:00Z
COMPLETED: BSL2N | semantic_ir/tests/bsl2n.rs | 2026-10-17T06:00:00Z
COMPLETED: AND | semantic_ir/tests/andp.rs | 2026-10-17T06:00:00Z
COMPLETED: BIC | semantic_ir/tests/bicp.rs | 2026-10-17T06:00:00Z
COMPLETED: CPYE | semantic_ir/tests/cpye.rs | 2026-10-17T06:00:00Z
COMPLETED: CPYET | semantic_ir/tests/cpyet.rs | 2026-10-17T06:00:00Z
COMPLETED: CPYEN | semantic_ir/tests/cpyen.rs | 2026-10-17T07:00:00Z
COMPLETED: CPYERN | semantic_ir/tests/cpyern.rs | 2026-10-17T07:00:00Z
COMPLETED: CPYERT | semantic_ir/tests/cpyert.rs | 2026-10-17T07:00:00Z
COMPLETED: CPYERTN | semantic_ir/tests/cpyertn.rs | 2026-10-17T07:00:00Z
COMPLETED: CPYERTRN | semantic_ir/tests/cpyertrn.rs | 2026-10-17T07:00:00Z
COMPLETED: CPYERTWN | semantic_ir/tests/cpyertwn.rs | 2026-10-17T07:00:00Z
COMPLETED: CPYETN | semantic_ir/tests/cpyetn.rs | 2026-10-17T07:00:00Z
COMPLETED: CPYETRN | semantic_ir/tests/cpyetrn.rs | 2026-10-17T07:00:00Z
COMPLETED: CNTD | semantic_ir/tests/cntd.rs | 2026-10-17T08:00:00Z
COMPLETED: CNTH | semantic_ir/tests/cnth.rs | 2026-10-17T08:00:00Z
COMPLETED: CNTW | semantic_ir/tests/cntw.rs | 2026-10-17T08:00:00Z
COMPLETED: BFVDOT | semantic_ir/tests/bfvdot.rs | 2026-10-17T08:00:00Z
COMPLETED: BF1CVT | semantic_ir/tests/bf1cvt.rs | 2026-10-17T08:00:00Z
COMPLETED: BF1CVTL | semantic_ir/tests/bf1cvtl.rs | 2026-10-17T08:00:00Z
COMPLETED: BF1CVTL2 | semantic_ir/tests/bf1cvtl2.rs | 2026-10-17T08:00:00Z
COMPLETED: BF2CVTL | semantic_ir/tests/bf2cvtl.rs | 2026-10-17T08:00:00Z
COMPLETED: BF1CVTLT | semantic_ir/tests/bf1cvtt.rs | 2026-10-17T09:00:00Z
COMPLETED: BF2CVTLT | semantic_ir/tests/bf2cvtt.rs | 2026-10-17T09:00:00Z
COMPLETED: BFCVT | semantic_ir/tests/bfcvt.rs | 2026-10-17T09:00:00Z
COMPLETED: BFCVTN | semantic_ir/tests/bfcvtn.rs | 2026-10-17T09:00:00Z
COMPLETED: BFCVTN2 | semantic_ir/tests/bfcvtn2.rs | 2026-10-17T09:00:00Z
COMPLETED: ADDHA | semantic_ir/tests/addha.rs | 2026-10-17T09:00:00Z
COMPLETED: ADDVA | semantic_ir/tests/addva.rs | 2026-10-17T09:00:00Z
COMPLETED: ADDHNB | semantic_ir/tests/adhnb.rs | 2026-10-17T09:00:00Z
COMPLETED: ADDHNT | semantic_ir/tests/adhnt.rs | 2026-10-17T09:00:00Z
COMPLETED: CBHLE | semantic_ir/tests/cbhl.rs | 2026-10-17T10:00:00Z
COMPLETED: CBLO | semantic_ir/tests/cblo.rs | 2026-10-17T10:00:00Z
COMPLETED: CBLT | semantic_ir/tests/cblt.rs | 2026-10-17T10:00:00Z
COMPLETED: CMEQ | semantic_ir/tests/cmeq.rs | 2026-10-17T10:00:00Z
COMPLETED: CMGE | semantic_ir/tests/cmge.rs | 2026-10-17T10:00:00Z
COMPLETED: CMGT | semantic_ir/tests/cmgt.rs | 2026-10-17T10:00:00Z
COMPLETED: CMHI | semantic_ir/tests/cmhi.rs | 2026-10-17T10:00:00Z
COMPLETED: CMHS | semantic_ir/tests/cmhs.rs | 2026-10-17T10:00:00Z
COMPLETED: CMLE | semantic_ir/tests/cmle.rs | 2026-10-17T11:00:00Z
COMPLETED: CMLT | semantic_ir/tests/cmlt.rs | 2026-10-17T11:00:00Z
COMPLETED: CMLO | semantic_ir/tests/cmlo.rs | 2026-10-17T11:00:00Z
COMPLETED: CMLS | semantic_ir/tests/cmls.rs | 2026-10-17T11:00:00Z
COMPLETED: CMNE | semantic_ir/tests/cmne.rs | 2026-10-17T11:00:00Z
COMPLETED: CNT | semantic_ir/tests/cnt.rs | 2026-10-17T11:00:00Z
COMPLETED: DUP | semantic_ir/tests/dup.rs | 2026-10-17T11:00:00Z
COMPLETED: EOR | semantic_ir/tests/eorv.rs | 2026-10-17T11:00:00Z
COMPLETED: FADD | semantic_ir/tests/fadd.rs | 2026-10-17T11:00:00Z
COMPLETED: FSUB | semantic_ir/tests/fsub.rs | 2026-10-17T12:00:00Z
COMPLETED: FMUL | semantic_ir/tests/fmul.rs | 2026-10-17T12:00:00Z
COMPLETED: FDIV | semantic_ir/tests/fdiv.rs | 2026-10-17T12:00:00Z
COMPLETED: FMAX | semantic_ir/tests/fmax.rs | 2026-10-17T12:00:00Z
COMPLETED: FMIN | semantic_ir/tests/fmin.rs | 2026-10-17T12:00:00Z
COMPLETED: FNEG | semantic_ir/tests/fneg.rs | 2026-10-17T12:00:00Z
COMPLETED: FSQRT | semantic_ir/tests/fsqrt.rs | 2026-10-17T12:00:00Z
COMPLETED: FCMP | semantic_ir/tests/fcmpv.rs | 2026-10-17T12:00:00Z
COMPLETED: FCMLA | semantic_ir/tests/fcmla.rs | 2026-10-17T13:00:00Z
COMPLETED: FMLA | semantic_ir/tests/fmlav.rs | 2026-10-17T13:00:00Z
COMPLETED: FMLS | semantic_ir/tests/fmlsv.rs | 2026-10-17T13:00:00Z
COMPLETED: FMMLA | semantic_ir/tests/fmmla.rs | 2026-10-17T13:00:00Z
COMPLETED: ABS | semantic_ir/tests/absv.rs | 2026-10-17T13:00:00Z
COMPLETED: NEG | semantic_ir/tests/negv.rs | 2026-10-17T13:00:00Z
COMPLETED: SADDLP | semantic_ir/tests/saddlp.rs | 2026-10-17T13:00:00Z
COMPLETED: UADDLP | semantic_ir/tests/uaddlp.rs | 2026-10-17T13:00:00Z
COMPLETED: SHLL | semantic_ir/tests/shll.rs | 2026-10-17T14:00:00Z
COMPLETED: USHLL | semantic_ir/tests/ushll.rs | 2026-10-17T14:00:00Z
COMPLETED: SQSHL | semantic_ir/tests/sqshl.rs | 2026-10-17T14:00:00Z
COMPLETED: UQSHL | semantic_ir/tests/uqshl.rs | 2026-10-17T14:00:00Z
COMPLETED: SQSHRN | semantic_ir/tests/sqshrn.rs | 2026-10-17T14:00:00Z
COMPLETED: UQSHRN | semantic_ir/tests/uqshrn.rs | 2026-10-17T14:00:00Z
COMPLETED: SQXTN | semantic_ir/tests/sqxtn.rs | 2026-10-17T14:00:00Z
COMPLETED: UQXTN | semantic_ir/tests/uqxtn.rs | 2026-10-17T14:00:00Z
COMPLETED: SQSHLU | semantic_ir/tests/sqshlu.rs | 2026-10-17T15:00:00Z
COMPLETED: UQSHLU | semantic_ir/tests/uqshlu.rs | 2026-10-17T15:00:00Z
COMPLETED: SMAX | semantic_ir/tests/smaxv.rs | 2026-10-17T15:00:00Z
COMPLETED: SMIN | semantic_ir/tests/sminv.rs | 2026-10-17T15:00:00Z
COMPLETED: UMAX | semantic_ir/tests/umaxv.rs | 2026-10-17T15:00:00Z
COMPLETED: UMIN | semantic_ir/tests/uminv.rs | 2026-10-17T15:00:00Z
COMPLETED: SHL | semantic_ir/tests/shlv.rs | 2026-10-17T16:00:00Z
COMPLETED: SHLL | semantic_ir/tests/shllv.rs | 2026-10-17T16:00:00Z
COMPLETED: USHLL | semantic_ir/tests/ushllv.rs | 2026-10-17T16:00:00Z
COMPLETED: SHL | semantic_ir/tests/shlp.rs | 2026-10-17T16:00:00Z
COMPLETED: SHR | semantic_ir/tests/shrv.rs | 2026-10-17T16:00:00Z
COMPLETED: USHR | semantic_ir/tests/usrv.rs | 2026-10-17T16:00:00Z
COMPLETED: SABDL | semantic_ir/tests/sabdl.rs | 2026-10-17T16:00:00Z
COMPLETED: UABDL | semantic_ir/tests/uabdl.rs | 2026-10-17T16:00:00Z
COMPLETED: SABDL2 | semantic_ir/tests/sabdl2.rs | 2026-10-17T17:00:00Z
COMPLETED: UABDL2 | semantic_ir/tests/uabdl2.rs | 2026-10-17T17:00:00Z
COMPLETED: SADALP | semantic_ir/tests/sadalp.rs | 2026-10-17T17:00:00Z
COMPLETED: UADALP | semantic_ir/tests/udalp.rs | 2026-10-17T17:00:00Z
COMPLETED: SQSHLL | semantic_ir/tests/sqshll.rs | 2026-10-17T17:00:00Z
COMPLETED: UQSHLL | semantic_ir/tests/uqshll.rs | 2026-10-17T17:00:00Z
COMPLETED: SQSHLLB | semantic_ir/tests/sqshllb.rs | 2026-10-17T17:00:00Z
COMPLETED: UQSHLLB | semantic_ir/tests/uqshllb.rs | 2026-10-17T17:00:00Z
COMPLETED: SQSHLLT | semantic_ir/tests/sqshllt.rs | 2026-10-17T17:00:00Z
COMPLETED: SQSHLLT2 | semantic_ir/tests/sqshllt2.rs | 2026-10-17T18:00:00Z
COMPLETED: UQSHLLT | semantic_ir/tests/uqshllt.rs | 2026-10-17T18:00:00Z
COMPLETED: UQSHLLT2 | semantic_ir/tests/uqshllt2.rs | 2026-10-17T18:00:00Z
COMPLETED: SSHL | semantic_ir/tests/sshl.rs | 2026-10-17T18:00:00Z
COMPLETED: USHL | semantic_ir/tests/ushl.rs | 2026-10-17T18:00:00Z
COMPLETED: SSHR | semantic_ir/tests/sshr.rs | 2026-10-17T18:00:00Z
COMPLETED: USHR | semantic_ir/tests/usr.rs | 2026-10-17T18:00:00Z
COMPLETED: SABDL | semantic_ir/tests/sabdlv.rs | 2026-10-17T18:00:00Z
COMPLETED: UABDL | semantic_ir/tests/uabdlv.rs | 2026-10-17T18:00:00Z
COMPLETED: SQSHRN2 | semantic_ir/tests/sqshrn2.rs | 2026-10-17T19:00:00Z
COMPLETED: UQSHRN2 | semantic_ir/tests/uqshrn2.rs | 2026-10-17T19:00:00Z
COMPLETED: SQXTN2 | semantic_ir/tests/sqxtn2.rs | 2026-10-17T19:00:00Z
COMPLETED: UQXTN2 | semantic_ir/tests/uqxtn2.rs | 2026-10-17T19:00:00Z
COMPLETED: SSHR2 | semantic_ir/tests/sshr2.rs | 2026-10-17T19:00:00Z
COMPLETED: USHR2 | semantic_ir/tests/usr2.rs | 2026-10-17T19:00:00Z
COMPLETED: SSHRN | semantic_ir/tests/sshrn.rs | 2026-10-17T19:00:00Z
COMPLETED: USHRN | semantic_ir/tests/usrn.rs | 2026-10-17T19:00:00Z
COMPLETED: SQSHRN2B | semantic_ir/tests/sqshrn2b.rs | 2026-10-17T20:00:00Z
COMPLETED: UQSHRN2B | semantic_ir/tests/uqshrn2b.rs | 2026-10-17T20:00:00Z
COMPLETED: SQXTN2B | semantic_ir/tests/sqxtn2b.rs | 2026-10-17T20:00:00Z
COMPLETED: UQXTN2B | semantic_ir/tests/uqxtn2b.rs | 2026-10-17T20:00:00Z
COMPLETED: SQSHLT | semantic_ir/tests/sqshlt.rs | 2026-10-17T20:00:00Z
COMPLETED: UQSHLT | semantic_ir/tests/uqshlt.rs | 2026-10-17T20:00:00Z
COMPLETED: SQSHNT | semantic_ir/tests/sqshnt.rs | 2026-10-17T20:00:00Z
COMPLETED: UQSHNT | semantic_ir/tests/uqshnt.rs | 2026-10-17T20:00:00Z
COMPLETED: SRSHR | semantic_ir/tests/srshr.rs | 2026-10-17T21:00:00Z
COMPLETED: URSHR | semantic_ir/tests/urshr.rs | 2026-10-17T21:00:00Z
COMPLETED: SRSHR2 | semantic_ir/tests/srshr2.rs | 2026-10-17T21:00:00Z
COMPLETED: URSHR2 | semantic_ir/tests/urshr2.rs | 2026-10-17T21:00:00Z
COMPLETED: SRSHRN | semantic_ir/tests/srshrn.rs | 2026-10-17T21:00:00Z
COMPLETED: URSHRN | semantic_ir/tests/urshrn.rs | 2026-10-17T21:00:00Z
COMPLETED: SRSHRN2 | semantic_ir/tests/srshrn2.rs | 2026-10-17T21:00:00Z
COMPLETED: URSHRN2 | semantic_ir/tests/urshrn2.rs | 2026-10-17T21:00:00Z
COMPLETED: ADDHN2 | semantic_ir/tests/adhn2.rs | 2026-10-17T22:30:00Z
COMPLETED: BF2CVTL2 | semantic_ir/tests/bf2cvtl2.rs | 2026-10-17T22:30:00Z
COMPLETED: BRKAS | semantic_ir/tests/brkas.rs | 2026-10-17T22:30:00Z
COMPLETED: BRKB | semantic_ir/tests/brkb.rs | 2026-10-17T22:30:00Z
COMPLETED: BRKBS | semantic_ir/tests/brkbs.rs | 2026-10-17T22:30:00Z
COMPLETED: BRKN | semantic_ir/tests/brkn.rs | 2026-10-17T22:30:00Z
COMPLETED: BRKNS | semantic_ir/tests/brkns.rs | 2026-10-17T22:30:00Z
COMPLETED: BRKPA | semantic_ir/tests/brkpa.rs | 2026-10-17T22:30:00Z
COMPLETED: BRKPAS | semantic_ir/tests/brkpas.rs | 2026-10-17T22:30:00Z
COMPLETED: BRKPB | semantic_ir/tests/brkpb.rs | 2026-10-17T22:30:00Z
COMPLETED: BRKPBS | semantic_ir/tests/brkpbs.rs | 2026-10-17T22:30:00Z
COMPLETED: CTERMEQ | semantic_ir/tests/ctermeq.rs | 2026-10-17T22:30:00Z
COMPLETED: CTERMNE | semantic_ir/tests/ctermne.rs | 2026-10-17T22:30:00Z
COMPLETED: DECB | semantic_ir/tests/decb.rs | 2026-10-17T22:30:00Z
COMPLETED: DECD | semantic_ir/tests/decd.rs | 2026-10-17T22:30:00Z
COMPLETED: DECH | semantic_ir/tests/dech.rs | 2026-10-17T22:30:00Z
COMPLETED: DUPM | semantic_ir/tests/dupm.rs | 2026-10-17T23:00:00Z
COMPLETED: DUPQ | semantic_ir/tests/dupq.rs | 2026-10-17T23:00:00Z
COMPLETED: EORBT | semantic_ir/tests/eorbt.rs | 2026-10-17T23:00:00Z
COMPLETED: EORS | semantic_ir/tests/eors.rs | 2026-10-17T23:00:00Z
COMPLETED: EORTB | semantic_ir/tests/eortb.rs | 2026-10-17T23:00:00Z
COMPLETED: F1CVTL2 | semantic_ir/tests/f1cvtl2.rs | 2026-10-17T23:00:00Z
COMPLETED: F2CVT | semantic_ir/tests/f2cvt.rs | 2026-10-17T23:00:00Z
COMPLETED: F2CVTL | semantic_ir/tests/f2cvtl.rs | 2026-10-17T23:00:00Z
COMPLETED: F2CVTL2 | semantic_ir/tests/f2cvtl2.rs | 2026-10-17T23:00:00Z
COMPLETED: F2CVTLT | semantic_ir/tests/f2cvtt.rs | 2026-10-17T23:00:00Z
COMPLETED: FACLE | semantic_ir/tests/facle.rs | 2026-10-17T23:00:00Z
COMPLETED: FACLT | semantic_ir/tests/faclt.rs | 2026-10-17T23:00:00Z
COMPLETED: FCVTX | semantic_ir/tests/fcvtx.rs | 2026-10-17T23:00:00Z
COMPLETED: FCVTXNT | semantic_ir/tests/fcvtxt.rs | 2026-10-17T23:00:00Z
COMPLETED: FMSB | semantic_ir/tests/fmsb.rs | 2026-10-17T23:00:00Z
COMPLETED: FNMAD | semantic_ir/tests/fnmad.rs | 2026-10-17T23:00:00Z
COMPLETED: FNMSB | semantic_ir/tests/fnmsb.rs | 2026-10-17T23:00:00Z
COMPLETED: LD1RD | semantic_ir/tests/ld1rd.rs | 2026-10-17T23:30:00Z
COMPLETED: LD1RH | semantic_ir/tests/ld1rh.rs | 2026-10-17T23:30:00Z
COMPLETED: LD1RSH | semantic_ir/tests/ld1rsh.rs | 2026-10-17T23:30:00Z
COMPLETED: LD1RSW | semantic_ir/tests/ld1rsw.rs | 2026-10-17T23:30:00Z
COMPLETED: LD1RW | semantic_ir/tests/ld1rw.rs | 2026-10-17T23:30:00Z
COMPLETED: LDNF1B | semantic_ir/tests/ldnf1b.rs | 2026-10-17T23:30:00Z
COMPLETED: LDNF1SB | semantic_ir/tests/ldnf1sb.rs | 2026-10-17T23:30:00Z
COMPLETED: LDNF1SH | semantic_ir/tests/ldnf1sh.rs | 2026-10-17T23:30:00Z
COMPLETED: LDNF1SW | semantic_ir/tests/ldnf1sw.rs | 2026-10-17T23:30:00Z
COMPLETED: LDNT1SB | semantic_ir/tests/ldnt1sb.rs | 2026-10-17T23:30:00Z
COMPLETED: LDNT1SH | semantic_ir/tests/ldnt1sh.rs | 2026-10-17T23:30:00Z
COMPLETED: LDNT1SW | semantic_ir/tests/ldnt1sw.rs | 2026-10-17T23:30:00Z
COMPLETED: MAD | semantic_ir/tests/mad.rs | 2026-10-17T23:30:00Z
COMPLETED: MLAPT | semantic_ir/tests/mlapt.rs | 2026-10-17T23:30:00Z
COMPLETED: MSB | semantic_ir/tests/msb.rs | 2026-10-17T23:30:00Z
COMPLETED: NBSL | semantic_ir/tests/nbsl.rs | 2026-10-17T23:45:00Z
COMPLETED: NMATCH | semantic_ir/tests/nmatch.rs | 2026-10-17T23:45:00Z
COMPLETED: NOTS | semantic_ir/tests/nots.rs | 2026-10-17T23:45:00Z
COMPLETED: ORNS | semantic_ir/tests/orns.rs | 2026-10-17T23:45:00Z
COMPLETED: PMLAL | semantic_ir/tests/pmlal.rs | 2026-10-17T23:45:00Z
COMPLETED: PUNPKHI | semantic_ir/tests/punpkhi.rs | 2026-10-17T23:45:00Z
COMPLETED: PUNPKLO | semantic_ir/tests/punpklo.rs | 2026-10-17T23:45:00Z
COMPLETED: RADDHNB | semantic_ir/tests/raddhnb.rs | 2026-10-17T23:45:00Z
COMPLETED: RDFFRS | semantic_ir/tests/rdffrs.rs | 2026-10-17T23:45:00Z
COMPLETED: RDSVL | semantic_ir/tests/rdsvl.rs | 2026-10-17T23:45:00Z
COMPLETED: REVD | semantic_ir/tests/revd.rs | 2026-10-17T23:45:00Z
COMPLETED: REVH | semantic_ir/tests/revh.rs | 2026-10-17T23:45:00Z
COMPLETED: REVW | semantic_ir/tests/revw.rs | 2026-10-17T23:45:00Z
COMPLETED: RSHRNB | semantic_ir/tests/rshrnb.rs | 2026-10-17T23:45:00Z
COMPLETED: RSHRNT | semantic_ir/tests/rshrnt.rs | 2026-10-17T23:45:00Z
COMPLETED: RSUBHNB | semantic_ir/tests/rsubhnb.rs | 2026-10-17T23:45:00Z
COMPLETED: RSUBHNT | semantic_ir/tests/rsubhnt.rs | 2026-10-17T23:45:00Z
COMPLETED: SABALT | semantic_ir/tests/sabalt.rs | 2026-10-18T00:00:00Z
COMPLETED: SABDLT | semantic_ir/tests/sabdlt.rs | 2026-10-18T00:00:00Z
COMPLETED: SHRNB | semantic_ir/tests/shrnb.rs | 2026-10-18T00:00:00Z
COMPLETED: SHRNT | semantic_ir/tests/shrnt.rs | 2026-10-18T00:00:00Z
COMPLETED: SHSUBR | semantic_ir/tests/shsubr.rs | 2026-10-18T00:00:00Z
COMPLETED: SMMLA | semantic_ir/tests/smmla.rs | 2026-10-18T00:00:00Z
COMPLETED: SPLICE | semantic_ir/tests/splice.rs | 2026-10-18T00:00:00Z
COMPLETED: SQCADD | semantic_ir/tests/sqcadd.rs | 2026-10-18T00:00:00Z
COMPLETED: SQCVTN | semantic_ir/tests/sqcvtn.rs | 2026-10-18T00:00:00Z
COMPLETED: SQDECB | semantic_ir/tests/sqdecb.rs | 2026-10-18T00:00:00Z
COMPLETED: SQDMLALBT | semantic_ir/tests/sqdmlalbt.rs | 2026-10-18T00:00:00Z
COMPLETED: SQDMLSLBT | semantic_ir/tests/sqdmlslbt.rs | 2026-10-18T00:00:00Z
COMPLETED: SQINCB | semantic_ir/tests/sqincb.rs | 2026-10-18T00:00:00Z
COMPLETED: SQRSHRNB | semantic_ir/tests/sqrshrnbb.rs | 2026-10-18T00:00:00Z
COMPLETED: SQRSHRNT | semantic_ir/tests/sqrshrnbt.rs | 2026-10-18T00:00:00Z
COMPLETED: SQRSHRUNB | semantic_ir/tests/sqrshrunb.rs | 2026-10-18T00:00:00Z
COMPLETED: SQRSHRUNT | semantic_ir/tests/sqrshrunt.rs | 2026-10-18T00:00:00Z
COMPLETED: SSHRNB | semantic_ir/tests/sshrnb.rs | 2026-10-18T00:15:00Z
COMPLETED: SSHRNT | semantic_ir/tests/sshrnt.rs | 2026-10-18T00:15:00Z
COMPLETED: SSUBWB | semantic_ir/tests/ssubwb.rs | 2026-10-18T00:15:00Z
COMPLETED: SSUBWT | semantic_ir/tests/ssubwt.rs | 2026-10-18T00:15:00Z
COMPLETED: SUBHNB | semantic_ir/tests/subhnb.rs | 2026-10-18T00:15:00Z
COMPLETED: SUBHNT | semantic_ir/tests/subhnt.rs | 2026-10-18T00:15:00Z
COMPLETED: SUDOT | semantic_ir/tests/sudot.rs | 2026-10-18T00:15:00Z
COMPLETED: SUMOP4A | semantic_ir/tests/sumop4a.rs | 2026-10-18T00:15:00Z
COMPLETED: SUMOP4S | semantic_ir/tests/sumop4s.rs | 2026-10-18T00:30:00Z
COMPLETED: SUMOPS | semantic_ir/tests/sumops.rs | 2026-10-18T00:30:00Z
COMPLETED: SUNPK | semantic_ir/tests/sunpk.rs | 2026-10-18T00:30:00Z
COMPLETED: SUTMOPA | semantic_ir/tests/sutmopa.rs | 2026-10-18T00:30:00Z
COMPLETED: SUVDOT | semantic_ir/tests/suvdot.rs | 2026-10-18T00:30:00Z
COMPLETED: TBLQ | semantic_ir/tests/tblq.rs | 2026-10-18T00:30:00Z
COMPLETED: TBXQ | semantic_ir/tests/tbxq.rs | 2026-10-18T00:30:00Z
COMPLETED: UABDLB | semantic_ir/tests/uabdlb.rs | 2026-10-18T00:30:00Z
COMPLETED: UABDLT | semantic_ir/tests/uabdlt.rs | 2026-10-18T00:30:00Z
COMPLETED: UADDLB | semantic_ir/tests/uaddlb.rs | 2026-10-18T00:30:00Z
COMPLETED: UADDLT | semantic_ir/tests/uaddlt.rs | 2026-10-18T00:30:00Z
COMPLETED: UADDV | semantic_ir/tests/uaddv.rs | 2026-10-18T01:00:00Z
COMPLETED: UADDWB | semantic_ir/tests/uaddwb.rs | 2026-10-18T01:00:00Z
COMPLETED: UADDWT | semantic_ir/tests/uaddwt.rs | 2026-10-18T01:15:00Z
COMPLETED: UCLAMP | semantic_ir/tests/uclamp.rs | 2026-10-18T01:15:00Z
COMPLETED: UHSUBR | semantic_ir/tests/uhsubr.rs | 2026-10-18T01:30:00Z
COMPLETED: UMAXQV | semantic_ir/tests/umaxqv.rs | 2026-10-18T01:30:00Z
COMPLETED: UMINQV | semantic_ir/tests/uminqv.rs | 2026-10-18T01:45:00Z
COMPLETED: UMMLA | semantic_ir/tests/ummla.rs | 2026-10-18T01:45:00Z
COMPLETED: UQCVTN | semantic_ir/tests/uqcvtn.rs | 2026-10-18T02:00:00Z
COMPLETED: UQDECB | semantic_ir/tests/uqdecb.rs | 2026-10-18T02:00:00Z
COMPLETED: UQINCB | semantic_ir/tests/uqincb.rs | 2026-10-18T02:15:00Z
COMPLETED: UQRSHLR | semantic_ir/tests/uqrshlr.rs | 2026-10-18T02:15:00Z
COMPLETED: UQRSHRNB | semantic_ir/tests/uqrshnbb.rs | 2026-10-18T02:30:00Z
COMPLETED: UQRSHRNT | semantic_ir/tests/uqrshnbt.rs | 2026-10-18T02:30:00Z
COMPLETED: UQSHLR | semantic_ir/tests/uqshrl.rs | 2026-10-18T02:30:00Z
COMPLETED: UQSHRNB | semantic_ir/tests/uqshrnb.rs | 2026-10-18T02:30:00Z
COMPLETED: UQSHRNT | semantic_ir/tests/uqshrnt.rs | 2026-10-18T02:30:00Z
COMPLETED: UQSUBR | semantic_ir/tests/uqsubr.rs | 2026-10-18T02:30:00Z
COMPLETED: UQXTNB | semantic_ir/tests/uqxtnb.rs | 2026-10-18T02:30:00Z
COMPLETED: UQXTNT | semantic_ir/tests/uqxtnt.rs | 2026-10-18T02:30:00Z
COMPLETED: USHLLT | semantic_ir/tests/ushllt.rs | 2026-10-18T02:30:00Z
COMPLETED: USMMLA | semantic_ir/tests/usmmla.rs | 2026-10-18T02:30:00Z
COMPLETED: USMOP4A | semantic_ir/tests/usmop4a.rs | 2026-10-18T02:30:00Z
COMPLETED: USMOP4S | semantic_ir/tests/usmop4s.rs | 2026-10-18T02:30:00Z
COMPLETED: USMOPS | semantic_ir/tests/usmops.rs | 2026-10-18T02:30:00Z
COMPLETED: USTMOPA | semantic_ir/tests/ustmopa.rs | 2026-10-18T02:30:00Z
COMPLETED: USVDOT | semantic_ir/tests/suvdot.rs | 2026-10-18T02:30:00Z
COMPLETED: UUNPK | semantic_ir/tests/uunpk.rs | 2026-10-18T02:30:00Z
COMPLETED: UUNPKLO | semantic_ir/tests/uunpklo.rs | 2026-10-18T02:30:00Z
COMPLETED: UZPQ2 | semantic_ir/tests/uzpq2.rs | 2026-10-18T02:30:00Z
COMPLETED: ZIPQ2 | semantic_ir/tests/zipq2.rs | 2026-10-18T02:30:00Z
COMPLETED: ADC | semantic_ir/tests/adc.rs | 2026-10-18T03:00:00Z
COMPLETED: ADCS | semantic_ir/tests/adcs.rs | 2026-10-18T03:00:00Z
COMPLETED: ADD | semantic_ir/tests/add.rs | 2026-10-18T03:00:00Z
COMPLETED: ADDS | semantic_ir/tests/adds.rs | 2026-10-18T03:00:00Z
COMPLETED: ADDHN | semantic_ir/tests/adhn.rs | 2026-10-18T03:00:00Z
COMPLETED: AESD | semantic_ir/tests/aesd.rs | 2026-10-18T03:00:00Z
COMPLETED: AESE | semantic_ir/tests/aese.rs | 2026-10-18T03:00:00Z
COMPLETED: AND | semantic_ir/tests/andp.rs | 2026-10-18T03:00:00Z
COMPLETED: AND | semantic_ir/tests/andv.rs | 2026-10-18T03:00:00Z
HEARTBEAT: 2026-10-18T03:00:00Z
COMPLETED_COUNT: 2980
COMPLETED: ANDS | semantic_ir/tests/ands.rs | 2026-10-18T03:15:00Z
COMPLETED: ASR | semantic_ir/tests/asr.rs | 2026-10-18T03:15:00Z
COMPLETED: ASR | semantic_ir/tests/asrr.rs | 2026-10-18T03:15:00Z
COMPLETED: ASR | semantic_ir/tests/asrv.rs | 2026-10-18T03:15:00Z
COMPLETED: AUTDA | semantic_ir/tests/autda.rs | 2026-10-18T03:15:00Z
COMPLETED: AUTDB | semantic_ir/tests/autdb.rs | 2026-10-18T03:15:00Z
COMPLETED: AUTIA | semantic_ir/tests/autia.rs | 2026-10-18T03:15:00Z
COMPLETED: AUTIB | semantic_ir/tests/autib.rs | 2026-10-18T03:15:00Z
COMPLETED: BIC | semantic_ir/tests/bic.rs | 2026-10-18T03:15:00Z
COMPLETED: BIC | semantic_ir/tests/bicr.rs | 2026-10-18T03:15:00Z
HEARTBEAT: 2026-10-18T03:15:00Z
COMPLETED_COUNT: 2990
COMPLETED: FVDOTB | semantic_ir/tests/fvdotb.rs | 2026-10-18T03:30:00Z
COMPLETED: FVDOTT | semantic_ir/tests/fvdott.rs | 2026-10-18T03:30:00Z
COMPLETED: LASTP | semantic_ir/tests/lastp.rs | 2026-10-18T03:30:00Z
COMPLETED: SQSHLR | semantic_ir/tests/sqshr.rs | 2026-10-18T03:30:00Z
COMPLETED: SQSHRNB | semantic_ir/tests/sqshrnb.rs | 2026-10-18T03:30:00Z
COMPLETED: SQSHRNT | semantic_ir/tests/sqshrnt.rs | 2026-10-18T03:30:00Z
COMPLETED: SQSHRUNB | semantic_ir/tests/sqshrunb.rs | 2026-10-18T03:30:00Z
COMPLETED: SQSHRUNT | semantic_ir/tests/sqshrunt.rs | 2026-10-18T03:30:00Z
COMPLETED: SQXTNB | semantic_ir/tests/sqxtnb.rs | 2026-10-18T03:30:00Z
COMPLETED: SQXTNT | semantic_ir/tests/sqxtnt.rs | 2026-10-18T03:30:00Z
COMPLETED: BIT | semantic_ir/tests/bit.rs | 2026-10-18T03:45:00Z
COMPLETED: BIF | semantic_ir/tests/bif.rs | 2026-10-18T03:45:00Z
COMPLETED: BSL | semantic_ir/tests/bsl.rs | 2026-10-18T03:45:00Z
COMPLETED: MLA | semantic_ir/tests/mla.rs | 2026-10-18T03:45:00Z
COMPLETED: MLS | semantic_ir/tests/mls.rs | 2026-10-18T03:45:00Z
COMPLETED: MUL | semantic_ir/tests/mul.rs | 2026-10-18T03:45:00Z
COMPLETED: PMULL | semantic_ir/tests/pmull.rs | 2026-10-18T03:45:00Z
COMPLETED: UQSHLR | semantic_ir/tests/uqshrl.rs | 2026-10-18T03:45:00Z
COMPLETED: UQSHRNB | semantic_ir/tests/uqshrnb.rs | 2026-10-18T03:45:00Z
COMPLETED: UQSHRNT | semantic_ir/tests/uqshrnt.rs | 2026-10-18T03:45:00Z
COMPLETED: UQSUBR | semantic_ir/tests/uqsubr.rs | 2026-10-18T03:45:00Z
COMPLETED: UQXTNB | semantic_ir/tests/uqxtnb.rs | 2026-10-18T03:45:00Z
COMPLETED: DUPM | semantic_ir/tests/dupm.rs | 2026-10-18T04:00:00Z
COMPLETED: DUPQ | semantic_ir/tests/dupq.rs | 2026-10-18T04:00:00Z
COMPLETED: F1CVTL2 | semantic_ir/tests/f1cvtl2.rs | 2026-10-18T04:00:00Z
COMPLETED: F2CVT | semantic_ir/tests/f2cvt.rs | 2026-10-18T04:00:00Z
COMPLETED: F2CVTL | semantic_ir/tests/f2cvtl.rs | 2026-10-18T04:00:00Z
COMPLETED: F2CVTL2 | semantic_ir/tests/f2cvtl2.rs | 2026-10-18T04:00:00Z
COMPLETED: F2CVTLT | semantic_ir/tests/f2cvtt.rs | 2026-10-18T04:00:00Z
COMPLETED: FACLE | semantic_ir/tests/facle.rs | 2026-10-18T04:00:00Z
COMPLETED: FACLT | semantic_ir/tests/faclt.rs | 2026-10-18T04:00:00Z
COMPLETED: FCVTX | semantic_ir/tests/fcvtx.rs | 2026-10-18T04:00:00Z
COMPLETED: FCVTXNT | semantic_ir/tests/fcvtxnt.rs | 2026-10-18T04:00:00Z
COMPLETED: FMSB | semantic_ir/tests/fmsb.rs | 2026-10-18T04:00:00Z
COMPLETED: FNMAD | semantic_ir/tests/fnmad.rs | 2026-10-18T04:00:00Z
COMPLETED: FNMSB | semantic_ir/tests/fnmsb.rs | 2026-10-18T04:00:00Z
COMPLETED: UADDV | semantic_ir/tests/uaddv.rs | 2026-10-18T04:15:00Z
COMPLETED: UADDWB | semantic_ir/tests/uaddwb.rs | 2026-10-18T04:15:00Z
COMPLETED: UADDWT | semantic_ir/tests/uaddwt.rs | 2026-10-18T04:15:00Z
COMPLETED: UCLAMP | semantic_ir/tests/uclamp.rs | 2026-10-18T04:15:00Z
COMPLETED: UHSUBR | semantic_ir/tests/uhsubr.rs | 2026-10-18T04:15:00Z
COMPLETED: UMAXQV | semantic_ir/tests/umaxqv.rs | 2026-10-18T04:15:00Z
COMPLETED: UMINQV | semantic_ir/tests/uminqv.rs | 2026-10-18T04:15:00Z
COMPLETED: UMMLA | semantic_ir/tests/ummla.rs | 2026-10-18T04:15:00Z
COMPLETED: UQCVTN | semantic_ir/tests/uqcvtn.rs | 2026-10-18T04:15:00Z
COMPLETED: UQDECB | semantic_ir/tests/uqdecb.rs | 2026-10-18T04:15:00Z
COMPLETED: UQINCB | semantic_ir/tests/uqincb.rs | 2026-10-18T04:15:00Z
COMPLETED: UQRSHLR | semantic_ir/tests/uqrshlr.rs | 2026-10-18T04:15:00Z
COMPLETED: UQRSHRNB | semantic_ir/tests/uqrshnbb.rs | 2026-10-18T04:15:00Z
COMPLETED: UQRSHRNT | semantic_ir/tests/uqrshnbt.rs | 2026-10-18T04:15:00Z
COMPLETED: UQXTNT | semantic_ir/tests/uqxtnt.rs | 2026-10-18T04:30:00Z
COMPLETED: USHLLT | semantic_ir/tests/ushllt.rs | 2026-10-18T04:30:00Z
COMPLETED: USMMLA | semantic_ir/tests/usmmla.rs | 2026-10-18T04:30:00Z
COMPLETED: USMOP4A | semantic_ir/tests/usmop4a.rs | 2026-10-18T04:30:00Z
COMPLETED: USMOP4S | semantic_ir/tests/usmop4s.rs | 2026-10-18T04:30:00Z
COMPLETED: USMOPS | semantic_ir/tests/usmops.rs | 2026-10-18T04:30:00Z
COMPLETED: USTMOPA | semantic_ir/tests/ustmopa.rs | 2026-10-18T04:30:00Z
COMPLETED: USVDOT | semantic_ir/tests/suvdot.rs | 2026-10-18T04:30:00Z
COMPLETED: UUNPK | semantic_ir/tests/uunpk.rs | 2026-10-18T04:30:00Z
COMPLETED: UUNPKLO | semantic_ir/tests/uunpklo.rs | 2026-10-18T04:30:00Z
COMPLETED: UZPQ2 | semantic_ir/tests/uzpq2.rs | 2026-10-18T04:30:00Z
COMPLETED: ZIPQ2 | semantic_ir/tests/zipq2.rs | 2026-10-18T04:30:00Z
COMPLETED: SQSUBR | semantic_ir/tests/sqsubr.rs | 2026-10-18T04:45:00Z
COMPLETED: SQXTUNB | semantic_ir/tests/sqxtunb.rs | 2026-10-18T04:45:00Z
COMPLETED: SQXTUNT | semantic_ir/tests/sqxtunt.rs | 2026-10-18T04:45:00Z
COMPLETED: SRSHLR | semantic_ir/tests/srshr.rs | 2026-10-18T04:45:00Z
COMPLETED: SSHLLT | semantic_ir/tests/sshllt.rs | 2026-10-18T04:45:00Z
HEARTBEAT: 2026-10-18T04:45:00Z
COMPLETED_COUNT: 3045
