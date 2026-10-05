# AI_PROGRESS — worker heartbeat + ledger

<!-- Supervisor parses: IN_PROGRESS, COMPLETED, HEARTBEAT, CHECKPOINT, NEED_CONTINUE -->
<!-- Worker: update HEARTBEAT at least every 5 min while active. Atomic write. -->

HEARTBEAT: 2026-10-05T13:50:00Z
IN_PROGRESS:
COMPLETED_COUNT: 398

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