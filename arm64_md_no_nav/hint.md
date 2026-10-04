## HINT

Hint instruction

This instruction is for the instruction set space that is reserved for architectural hint instructions.

Some encodings described here are not allocated in this revision of the architecture, and behave as NOPs. These encodings might be allocated to other hint functionality in future revisions of the architecture and therefore must not be used by software.

<div class="regdiagram-32">

<table class="regdiagram">
<thead>
<tr>
<th>31</th>
<th>30</th>
<th>29</th>
<th>28</th>
<th>27</th>
<th>26</th>
<th>25</th>
<th>24</th>
<th>23</th>
<th>22</th>
<th>21</th>
<th>20</th>
<th>19</th>
<th>18</th>
<th>17</th>
<th>16</th>
<th>15</th>
<th>14</th>
<th>13</th>
<th>12</th>
<th>11</th>
<th>10</th>
<th>9</th>
<th>8</th>
<th>7</th>
<th>6</th>
<th>5</th>
<th>4</th>
<th>3</th>
<th>2</th>
<th>1</th>
<th>0</th>
</tr>
</thead>
<tbody>
<tr class="firstrow">
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="4" class="lr">CRm</td>
<td colspan="3" class="lr">op2</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="HINT_HM_hints"></span>

HINT \#[\<imm\>](#CRm_op2 "Is a 7-bit unsigned immediate, in the range 0 to 127, encoded in the "CRm:op2" field.  The encodings that are allocated to architectural hint functionality are described in the 'Hints' table in the 'Index by Encoding'.   [note] For allocated encodings of "CRm:op2":    * A disassembler will disassemble the allocated instruction, rather than the HINT instruction.   * An assembler may support assembly of allocated encodings using HINT with the corresponding <imm> value, but it is not required to do so. [/note]")

</div>

#### Decode for this encoding

var op : [SystemHintOp](shared_pseudocode.html#type_SystemHintOp); var stream : boolean; case CRm::op2 of when '0000 000' =\> op = [SystemHintOp_NOP](shared_pseudocode.html#enum_SystemHintOp_NOP); when '0000 001' =\> op = [SystemHintOp_YIELD](shared_pseudocode.html#enum_SystemHintOp_YIELD); when '0000 010' =\> op = [SystemHintOp_WFE](shared_pseudocode.html#enum_SystemHintOp_WFE); when '0000 011' =\> op = [SystemHintOp_WFI](shared_pseudocode.html#enum_SystemHintOp_WFI); when '0000 100' =\> op = [SystemHintOp_SEV](shared_pseudocode.html#enum_SystemHintOp_SEV); when '0000 101' =\> op = [SystemHintOp_SEVL](shared_pseudocode.html#enum_SystemHintOp_SEVL); when '0000 110' =\> if !IsFeatureImplemented(FEAT_DGH) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; op = [SystemHintOp_DGH](shared_pseudocode.html#enum_SystemHintOp_DGH); when '0000 111' =\> See("XPACLRI"); when '0001 xxx' =\> case op2 of when '000' =\> See("PACIA1716"); when '010' =\> See("PACIB1716"); when '100' =\> See("AUTIA1716"); when '110' =\> See("AUTIB1716"); otherwise =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; when '0010 000' =\> if !IsFeatureImplemented(FEAT_RAS) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; op = [SystemHintOp_ESB](shared_pseudocode.html#enum_SystemHintOp_ESB); when '0010 001' =\> if !IsFeatureImplemented(FEAT_SPE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; op = [SystemHintOp_PSB](shared_pseudocode.html#enum_SystemHintOp_PSB); when '0010 010' =\> if !IsFeatureImplemented(FEAT_TRF) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; op = [SystemHintOp_TSB](shared_pseudocode.html#enum_SystemHintOp_TSB); when '0010 011' =\> if !IsFeatureImplemented(FEAT_GCS) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; op = [SystemHintOp_GCSB](shared_pseudocode.html#enum_SystemHintOp_GCSB); when '0010 100' =\> op = [SystemHintOp_CSDB](shared_pseudocode.html#enum_SystemHintOp_CSDB); when '0010 110' =\> if !IsFeatureImplemented(FEAT_CLRBHB) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; op = [SystemHintOp_CLRBHB](shared_pseudocode.html#enum_SystemHintOp_CLRBHB); when '0011 xxx' =\> case op2 of when '000' =\> See("PACIAZ"); when '001' =\> See("PACIASP"); when '010' =\> See("PACIBZ"); when '011' =\> See("PACIBSP"); when '100' =\> See("AUTIAZ"); when '101' =\> See("AUTIASP"); when '110' =\> See("AUTIBZ"); when '111' =\> See("AUTIBSP"); end; when '0100 xx0' =\> if !IsFeatureImplemented(FEAT_BTI) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; // Check branch target compatibility between BTI instruction and PSTATE.BTYPE [SetBTypeCompatible](shared_pseudocode.html#func_SetBTypeCompatible_1)([BTypeCompatible_BTI](shared_pseudocode.html#func_BTypeCompatible_BTI_1)(op2\[2:1\])); op = [SystemHintOp_BTI](shared_pseudocode.html#enum_SystemHintOp_BTI); when '0100 111' =\> See("PACM"); when '0101 000' =\> if !IsFeatureImplemented(FEAT_CHK) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; op = [SystemHintOp_CHKFEAT](shared_pseudocode.html#enum_SystemHintOp_CHKFEAT); when '0110 00x' =\> if !IsFeatureImplemented(FEAT_PCDPHINT) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; stream = op2\[0\] == '1'; op = [SystemHintOp_STSHH](shared_pseudocode.html#enum_SystemHintOp_STSHH); otherwise =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="CRm_op2"></span>
<p>Is a 7-bit unsigned immediate, in the range 0 to 127, encoded in the "CRm:op2" field.</p>
<p>The encodings that are allocated to architectural hint functionality are described in the 'Hints' table in the 'Index by Encoding'.</p>
<div class="note">
<hr />
<h4 id="note">Note</h4>
<p>For allocated encodings of "CRm:op2":</p>
<ul>
<li>A disassembler will disassemble the allocated instruction, rather than the <span class="asm-code">HINT</span> instruction.</li>
<li>An assembler may support assembly of allocated encodings using <span class="asm-code">HINT</span> with the corresponding &lt;imm&gt; value, but it is not required to do so.</li>
</ul>
<hr />
</div></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

case op of when [SystemHintOp_YIELD](shared_pseudocode.html#enum_SystemHintOp_YIELD) =\> Hint_Yield(); when [SystemHintOp_DGH](shared_pseudocode.html#enum_SystemHintOp_DGH) =\> Hint_DGH(); when [SystemHintOp_WFE](shared_pseudocode.html#enum_SystemHintOp_WFE) =\> [Hint_WFE](shared_pseudocode.html#func_Hint_WFE_0)(); when [SystemHintOp_WFI](shared_pseudocode.html#enum_SystemHintOp_WFI) =\> [Hint_WFI](shared_pseudocode.html#func_Hint_WFI_0)(); when [SystemHintOp_SEV](shared_pseudocode.html#enum_SystemHintOp_SEV) =\> SendEvent(); when [SystemHintOp_SEVL](shared_pseudocode.html#enum_SystemHintOp_SEVL) =\> [SendEventLocal](shared_pseudocode.html#func_SendEventLocal_0)(); when [SystemHintOp_ESB](shared_pseudocode.html#enum_SystemHintOp_ESB) =\> SynchronizeErrors(); [AArch64_ESBOperation](shared_pseudocode.html#func_AArch64_ESBOperation_0)(); if [PSTATE](shared_pseudocode.html#global_PSTATE).EL IN {[EL0](shared_pseudocode.html#global_EL0), [EL1](shared_pseudocode.html#global_EL1)} && [EL2Enabled](shared_pseudocode.html#func_EL2Enabled_0)() then [AArch64_vESBOperation](shared_pseudocode.html#func_AArch64_vESBOperation_0)(); elsif IsFeatureImplemented(FEAT_E3DSE) && [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL3](shared_pseudocode.html#global_EL3) then [AArch64_dESBOperation](shared_pseudocode.html#func_AArch64_dESBOperation_0)(); end; TakeUnmaskedSErrorInterrupts(); when [SystemHintOp_PSB](shared_pseudocode.html#enum_SystemHintOp_PSB) =\> if IsFeatureImplemented(FEAT_FGT) && IsFeatureImplemented(FEAT_SPEv1p5) then let trap_to_el2 : boolean = ([PSTATE](shared_pseudocode.html#global_PSTATE).EL IN {[EL0](shared_pseudocode.html#global_EL0), [EL1](shared_pseudocode.html#global_EL1)} && [EL2Enabled](shared_pseudocode.html#func_EL2Enabled_0)() && \![IsInHost](shared_pseudocode.html#func_IsInHost_0)() && (\![HaveEL](shared_pseudocode.html#func_HaveEL_1)([EL3](shared_pseudocode.html#global_EL3)) \|\| SCR_EL3().FGTEn == '1') && HFGITR_EL2().PSBCSYNC == '1'); if trap_to_el2 then let target_el : bits(2) = [EL2](shared_pseudocode.html#global_EL2); let iss : bits(25) = 0x3\[24:0\]; [AArch64_OtherInstrTrap](shared_pseudocode.html#func_AArch64_OtherInstrTrap_2)(target_el, iss); end; end; ProfilingSynchronizationBarrier(); when [SystemHintOp_TSB](shared_pseudocode.html#enum_SystemHintOp_TSB) =\> if IsFeatureImplemented(FEAT_FGT2) && IsFeatureImplemented(FEAT_TRBEv1p1) then let trap_to_el2 : boolean = ([PSTATE](shared_pseudocode.html#global_PSTATE).EL IN {[EL0](shared_pseudocode.html#global_EL0), [EL1](shared_pseudocode.html#global_EL1)} && [EL2Enabled](shared_pseudocode.html#func_EL2Enabled_0)() && \![IsInHost](shared_pseudocode.html#func_IsInHost_0)() && (\![HaveEL](shared_pseudocode.html#func_HaveEL_1)([EL3](shared_pseudocode.html#global_EL3)) \|\| SCR_EL3().FGTEn2 == '1') && HFGITR2_EL2().TSBCSYNC == '1'); if trap_to_el2 then let target_el : bits(2) = [EL2](shared_pseudocode.html#global_EL2); let iss : bits(25) = 0x4\[24:0\]; [AArch64_OtherInstrTrap](shared_pseudocode.html#func_AArch64_OtherInstrTrap_2)(target_el, iss); end; end; [TraceSynchronizationBarrier](shared_pseudocode.html#func_TraceSynchronizationBarrier_0)(); when [SystemHintOp_GCSB](shared_pseudocode.html#enum_SystemHintOp_GCSB) =\> GCSSynchronizationBarrier(); when [SystemHintOp_CHKFEAT](shared_pseudocode.html#enum_SystemHintOp_CHKFEAT) =\> [X](shared_pseudocode.html#accessor_X_2){64}(16) = [AArch64_ChkFeat](shared_pseudocode.html#func_AArch64_ChkFeat_1)([X](shared_pseudocode.html#accessor_X_2){64}(16)); when [SystemHintOp_CSDB](shared_pseudocode.html#enum_SystemHintOp_CSDB) =\> ConsumptionOfSpeculativeDataBarrier(); when [SystemHintOp_CLRBHB](shared_pseudocode.html#enum_SystemHintOp_CLRBHB) =\> Hint_CLRBHB(); when [SystemHintOp_BTI](shared_pseudocode.html#enum_SystemHintOp_BTI) =\> [SetBTypeNext](shared_pseudocode.html#func_SetBTypeNext_1)('00'); when [SystemHintOp_STSHH](shared_pseudocode.html#enum_SystemHintOp_STSHH) =\> Hint_StoreShared(stream); when [SystemHintOp_NOP](shared_pseudocode.html#enum_SystemHintOp_NOP) =\> return; // Do nothing otherwise =\> unreachable; end;

</div>
