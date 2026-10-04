## LDR (register, SIMD&FP)

Load SIMD&FP register (register offset)

This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from a base register value and an offset register value. The offset can be optionally shifted and extended.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

### <span id="iclass_simd_fp_registers"></span>SIMD&FP registers<span style="font-size:smaller;">\
(FEAT_FP) </span>

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
<td colspan="2" class="lr">size</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">x</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td colspan="3" class="lr">option</td>
<td class="lr">S</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="2"></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 8-bit with extended register offset variant

<span id="LDR_B_ldst_regoff"></span> Applies when <span class="bitdiff"> (size == 00 && opc == 01 && option != 011)</span>

LDR [\<Bt\>](#Bt "Is the 8-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), ([\<Wm\>](#WmOrWZR "When option<0> is set to 0, is the 32-bit name of the general-purpose index register, encoded in the "Rm" field.")\|[\<Xm\>](#XmOrXZR__2 "When option<0> is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field.")), [\<extend\>](#extend_option "For the "8-bit with extended register offset" variant: is the index extend specifier, ") {[\<amount\>](#amount "For the "8-bit with extended register offset" and "8-bit with shifted register offset" variants: is the index shift amount, it must be #0, encoded in "S" as 0 if omitted, or as 1 if present.")}\]

</div>

<div class="encoding">

#### Encoding for the 8-bit with shifted register offset variant

<span id="LDR_BL_ldst_regoff"></span> Applies when <span class="bitdiff"> (size == 00 && opc == 01 && option == 011)</span>

LDR [\<Bt\>](#Bt "Is the 8-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), [\<Xm\>](#XmOrXZR__2 "When option<0> is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field."){, LSL [\<amount\>](#amount "For the "8-bit with extended register offset" and "8-bit with shifted register offset" variants: is the index shift amount, it must be #0, encoded in "S" as 0 if omitted, or as 1 if present.")}\]

</div>

<div class="encoding">

#### Encoding for the 16-bit variant

<span id="LDR_H_ldst_regoff"></span> Applies when <span class="bitdiff"> (size == 01 && opc == 01)</span>

LDR [\<Ht\>](#Ht "Is the 16-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), ([\<Wm\>](#WmOrWZR "When option<0> is set to 0, is the 32-bit name of the general-purpose index register, encoded in the "Rm" field.")\|[\<Xm\>](#XmOrXZR__2 "When option<0> is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field.")){, [\<extend\>](#extend_option__3 "For the "128-bit", "16-bit", "32-bit", and "64-bit" variants: is the index extend/shift specifier, defaulting to LSL, and which must be omitted for the LSL option when <amount> is omitted, ") {[\<amount\>](#amount_option__2 "For the "16-bit" variant: is the index shift amount, optional only when <extend> is not LSL. Where it is permitted to be optional, it defaults to #0. It is ")}}\]

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="LDR_S_ldst_regoff"></span> Applies when <span class="bitdiff"> (size == 10 && opc == 01)</span>

LDR [\<St\>](#St "Is the 32-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), ([\<Wm\>](#WmOrWZR "When option<0> is set to 0, is the 32-bit name of the general-purpose index register, encoded in the "Rm" field.")\|[\<Xm\>](#XmOrXZR__2 "When option<0> is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field.")){, [\<extend\>](#extend_option__3 "For the "128-bit", "16-bit", "32-bit", and "64-bit" variants: is the index extend/shift specifier, defaulting to LSL, and which must be omitted for the LSL option when <amount> is omitted, ") {[\<amount\>](#amount_option__4 "For the "32-bit" variant: is the index shift amount, optional only when <extend> is not LSL. Where it is permitted to be optional, it defaults to #0. It is ")}}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="LDR_D_ldst_regoff"></span> Applies when <span class="bitdiff"> (size == 11 && opc == 01)</span>

LDR [\<Dt\>](#Dt "Is the 64-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), ([\<Wm\>](#WmOrWZR "When option<0> is set to 0, is the 32-bit name of the general-purpose index register, encoded in the "Rm" field.")\|[\<Xm\>](#XmOrXZR__2 "When option<0> is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field.")){, [\<extend\>](#extend_option__3 "For the "128-bit", "16-bit", "32-bit", and "64-bit" variants: is the index extend/shift specifier, defaulting to LSL, and which must be omitted for the LSL option when <amount> is omitted, ") {[\<amount\>](#amount_option__6 "For the "64-bit" variant: is the index shift amount, optional only when <extend> is not LSL. Where it is permitted to be optional, it defaults to #0. It is ")}}\]

</div>

<div class="encoding">

#### Encoding for the 128-bit variant

<span id="LDR_Q_ldst_regoff"></span> Applies when <span class="bitdiff"> (size == 00 && opc == 11)</span>

LDR [\<Qt\>](#Qt "Is the 128-bit name of the SIMD&FP register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), ([\<Wm\>](#WmOrWZR "When option<0> is set to 0, is the 32-bit name of the general-purpose index register, encoded in the "Rm" field.")\|[\<Xm\>](#XmOrXZR__2 "When option<0> is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field.")){, [\<extend\>](#extend_option__3 "For the "128-bit", "16-bit", "32-bit", and "64-bit" variants: is the index extend/shift specifier, defaulting to LSL, and which must be omitted for the LSL option when <amount> is omitted, ") {[\<amount\>](#amount_option "For the "128-bit" variant: is the index shift amount, optional only when <extend> is not LSL. Where it is permitted to be optional, it defaults to #0. It is ")}}\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if option\[1\] == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; // sub-word index if opc\[1\] == '1' && size != '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let scale : integer{} = if opc\[1\] == '1' then 4 else UInt(size); let extend_type : [ExtendType](shared_pseudocode.html#type_ExtendType) = [DecodeRegExtend](shared_pseudocode.html#func_DecodeRegExtend_1)(option); let shift : integer{} = if S == '1' then scale else 0;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Bt&gt;</td>
<td><span id="Bt"></span>
<p>Is the 8-bit name of the SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option"></span>
<p>Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wm&gt;</td>
<td><span id="WmOrWZR"></span>
<p>When option&lt;0&gt; is set to 0, is the 32-bit name of the general-purpose index register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xm&gt;</td>
<td><span id="XmOrXZR__2"></span>
<p>When option&lt;0&gt; is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;extend&gt;</td>
<td><span id="extend_option"></span>
<p>For the "8-bit with extended register offset" variant: is the index extend specifier, encoded in “option”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">option</th>
<th class="symbol">&lt;extend&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">010</td>
<td class="symbol">UXTW</td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="symbol">SXTW</td>
</tr>
<tr>
<td class="bitfield">111</td>
<td class="symbol">SXTX</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="extend_option__3"></span>
<p>For the "128-bit", "16-bit", "32-bit", and "64-bit" variants: is the index extend/shift specifier, defaulting to LSL, and which must be omitted for the LSL option when &lt;amount&gt; is omitted, encoded in “option”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">option</th>
<th class="symbol">&lt;extend&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">010</td>
<td class="symbol">UXTW</td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="symbol">LSL</td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="symbol">SXTW</td>
</tr>
<tr>
<td class="bitfield">111</td>
<td class="symbol">SXTX</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;amount&gt;</td>
<td><span id="amount"></span>
<p>For the "8-bit with extended register offset" and "8-bit with shifted register offset" variants: is the index shift amount, it must be #0, encoded in "S" as 0 if omitted, or as 1 if present.</p></td>
</tr>
<tr>
<td></td>
<td><span id="amount_option__2"></span>
<p>For the "16-bit" variant: is the index shift amount, optional only when &lt;extend&gt; is not LSL. Where it is permitted to be optional, it defaults to #0. It is encoded in “S”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">S</th>
<th class="symbol">&lt;amount&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">#0</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">#1</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="amount_option__4"></span>
<p>For the "32-bit" variant: is the index shift amount, optional only when &lt;extend&gt; is not LSL. Where it is permitted to be optional, it defaults to #0. It is encoded in “S”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">S</th>
<th class="symbol">&lt;amount&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">#0</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">#2</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="amount_option__6"></span>
<p>For the "64-bit" variant: is the index shift amount, optional only when &lt;extend&gt; is not LSL. Where it is permitted to be optional, it defaults to #0. It is encoded in “S”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">S</th>
<th class="symbol">&lt;amount&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">#0</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">#3</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="amount_option"></span>
<p>For the "128-bit" variant: is the index shift amount, optional only when &lt;extend&gt; is not LSL. Where it is permitted to be optional, it defaults to #0. It is encoded in “S”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">S</th>
<th class="symbol">&lt;amount&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">#0</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">#4</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Ht&gt;</td>
<td><span id="Ht"></span>
<p>Is the 16-bit name of the SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;St&gt;</td>
<td><span id="St"></span>
<p>Is the 32-bit name of the SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dt&gt;</td>
<td><span id="Dt"></span>
<p>Is the 64-bit name of the SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Qt&gt;</td>
<td><span id="Qt"></span>
<p>Is the 128-bit name of the SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Shared Decode

let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 8 \<\< scale; let nontemporal : boolean = FALSE; let tagchecked : boolean = TRUE;

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let offset : bits(64) = ExtendReg{}(m, extend_type, shift); var address : bits(64); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescASIMD](shared_pseudocode.html#func_CreateAccDescASIMD_4)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, tagchecked, privileged); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); [V](shared_pseudocode.html#accessor_V_2){datasize}(t) = [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc);

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
