## LDRSB (register)

Load register signed byte (register)

This instruction calculates an address from a base register value and an offset register value, loads a byte from memory, sign-extends it, and writes it to a register. For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

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
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">x</td>
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
<td colspan="2" class="droppedname">size</td>
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

#### Encoding for the 32-bit with extended register offset variant

<span id="LDRSB_32B_ldst_regoff"></span> Applies when <span class="bitdiff"> (opc == 11 && option != 011)</span>

LDRSB [\<Wt\>](#WtOrWZR__4 "Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), ([\<Wm\>](#WmOrWZR "When option<0> is set to 0, is the 32-bit name of the general-purpose index register, encoded in the "Rm" field.")\|[\<Xm\>](#XmOrXZR__2 "When option<0> is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field.")), [\<extend\>](#extend_option "Is the index extend specifier, ") {[\<amount\>](#amount "Is the index shift amount, it must be #0, encoded in "S" as 0 if omitted, or as 1 if present.")}\]

</div>

<div class="encoding">

#### Encoding for the 32-bit with shifted register offset variant

<span id="LDRSB_32BL_ldst_regoff"></span> Applies when <span class="bitdiff"> (opc == 11 && option == 011)</span>

LDRSB [\<Wt\>](#WtOrWZR__4 "Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), [\<Xm\>](#XmOrXZR__2 "When option<0> is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field."){, LSL [\<amount\>](#amount "Is the index shift amount, it must be #0, encoded in "S" as 0 if omitted, or as 1 if present.")}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit with extended register offset variant

<span id="LDRSB_64B_ldst_regoff"></span> Applies when <span class="bitdiff"> (opc == 10 && option != 011)</span>

LDRSB [\<Xt\>](#XtOrXZR__11 "Is the 64-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), ([\<Wm\>](#WmOrWZR "When option<0> is set to 0, is the 32-bit name of the general-purpose index register, encoded in the "Rm" field.")\|[\<Xm\>](#XmOrXZR__2 "When option<0> is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field.")), [\<extend\>](#extend_option "Is the index extend specifier, ") {[\<amount\>](#amount "Is the index shift amount, it must be #0, encoded in "S" as 0 if omitted, or as 1 if present.")}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit with shifted register offset variant

<span id="LDRSB_64BL_ldst_regoff"></span> Applies when <span class="bitdiff"> (opc == 10 && option == 011)</span>

LDRSB [\<Xt\>](#XtOrXZR__11 "Is the 64-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), [\<Xm\>](#XmOrXZR__2 "When option<0> is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field."){, LSL [\<amount\>](#amount "Is the index shift amount, it must be #0, encoded in "S" as 0 if omitted, or as 1 if present.")}\]

</div>

#### Decode for all variants of this encoding

if option\[1\] == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; // sub-word index let extend_type : [ExtendType](shared_pseudocode.html#type_ExtendType) = [DecodeRegExtend](shared_pseudocode.html#func_DecodeRegExtend_1)(option); let shift : integer{} = 0;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wt&gt;</td>
<td><span id="WtOrWZR__4"></span>
<p>Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
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
<p>Is the index extend specifier, encoded in “option”:</p>
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
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;amount&gt;</td>
<td><span id="amount"></span>
<p>Is the index shift amount, it must be #0, encoded in "S" as 0 if omitted, or as 1 if present.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__11"></span>
<p>Is the 64-bit name of the general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Shared Decode

let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let regsize : integer{} = 64 \>\> UInt(opc\[0\]); let nontemporal : boolean = FALSE; let tagchecked : boolean = TRUE;

</div>

<div class="ps">

### Operation

let offset : bits(64) = ExtendReg{}(m, extend_type, shift); var address : bits(64); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_5)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, privileged, tagchecked, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); let data : bits(8) = [Mem](shared_pseudocode.html#accessor_Mem_3){8}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){regsize}(t) = SignExtend{regsize}(data);

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
