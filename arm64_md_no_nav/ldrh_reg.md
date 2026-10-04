## LDRH (register)

Load register halfword (register)

This instruction calculates an address from a base register value and an offset register value, loads a halfword from memory, zero-extends it, and writes it to a register. For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

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
<td class="r">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">0</td>
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
<td colspan="2" class="droppedname">size</td>
<td colspan="2"></td>
<td></td>
<td class="droppedname">VR</td>
<td></td>
<td></td>
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

#### Encoding

<span id="LDRH_32_ldst_regoff"></span>

LDRH [\<Wt\>](#WtOrWZR__4 "Is the 32-bit name of the general-purpose register to be transferred, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), ([\<Wm\>](#WmOrWZR "When option<0> is set to 0, is the 32-bit name of the general-purpose index register, encoded in the "Rm" field.")\|[\<Xm\>](#XmOrXZR__2 "When option<0> is set to 1, is the 64-bit name of the general-purpose index register, encoded in the "Rm" field.")){, [\<extend\>](#extend_option__3 "For the "128-bit", "16-bit", "32-bit", and "64-bit" variants: is the index extend/shift specifier, defaulting to LSL, and which must be omitted for the LSL option when <amount> is omitted, ") {[\<amount\>](#amount_option__2 "For the "16-bit" variant: is the index shift amount, optional only when <extend> is not LSL. Where it is permitted to be optional, it defaults to #0. It is ")}}\]

</div>

#### Decode for this encoding

if option\[1\] == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; // sub-word index let extend_type : [ExtendType](shared_pseudocode.html#type_ExtendType) = [DecodeRegExtend](shared_pseudocode.html#func_DecodeRegExtend_1)(option); let shift : integer{} = if S == '1' then 1 else 0;

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
<td><span id="extend_option__3"></span>
<p>Is the index extend/shift specifier, defaulting to LSL, and which must be omitted for the LSL option when &lt;amount&gt; is omitted, encoded in “option”:</p>
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
<td><span id="amount_option__2"></span>
<p>Is the index shift amount, optional only when &lt;extend&gt; is not LSL. Where it is permitted to be optional, it defaults to #0. It is encoded in “S”:</p>
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
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Shared Decode

let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let nontemporal : boolean = FALSE; let tagchecked : boolean = TRUE;

</div>

<div class="ps">

### Operation

let offset : bits(64) = ExtendReg{}(m, extend_type, shift); var address : bits(64); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_5)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, privileged, tagchecked, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); let data : bits(16) = [Mem](shared_pseudocode.html#accessor_Mem_3){16}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){32}(t) = ZeroExtend{32}(data);

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
