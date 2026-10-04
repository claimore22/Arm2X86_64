## LDCLRH, LDCLRAH, LDCLRALH, LDCLRLH

Atomic bit clear on halfword

This instruction atomically loads a 16-bit halfword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register.

- If the destination register is not WZR, <span class="asm-code">LDCLRAH</span> and <span class="asm-code">LDCLRALH</span> load from memory with acquire semantics.
- <span class="asm-code">LDCLRLH</span> and <span class="asm-code">LDCLRALH</span> store to memory with release semantics.
- <span class="asm-code">LDCLRH</span> has neither acquire nor release semantics.

For more information about memory ordering semantics, see <span class="armarm-xref" data-linkend="ARMARM_BEIHCHEF">Load-Acquire, Store-Release</span>.

For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

This instruction is used by the alias [STCLRH, STCLRLH](stclrh_ldclrh.html "Atomic bit clear on halfword, without return").

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_LSE) </span>

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
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">A</td>
<td class="lr">R</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rs</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="2"></td>
<td></td>
<td></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">o3</td>
<td colspan="3" class="droppedname">opc</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the No memory ordering variant

<span id="LDCLRH_32_memop"></span> Applies when <span class="bitdiff"> (A == 0 && R == 0)</span>

LDCLRH [\<Ws\>](#WsOrWZR "Is the 32-bit name of the general-purpose register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), [\<Wt\>](#WtOrWZR__2 "Is the 32-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the Acquire variant

<span id="LDCLRAH_32_memop"></span> Applies when <span class="bitdiff"> (A == 1 && R == 0)</span>

LDCLRAH [\<Ws\>](#WsOrWZR "Is the 32-bit name of the general-purpose register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), [\<Wt\>](#WtOrWZR__2 "Is the 32-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the Acquire-release variant

<span id="LDCLRALH_32_memop"></span> Applies when <span class="bitdiff"> (A == 1 && R == 1)</span>

LDCLRALH [\<Ws\>](#WsOrWZR "Is the 32-bit name of the general-purpose register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), [\<Wt\>](#WtOrWZR__2 "Is the 32-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the Release variant

<span id="LDCLRLH_32_memop"></span> Applies when <span class="bitdiff"> (A == 0 && R == 1)</span>

LDCLRLH [\<Ws\>](#WsOrWZR "Is the 32-bit name of the general-purpose register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), [\<Wt\>](#WtOrWZR__2 "Is the 32-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_LSE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let s : integer{} = UInt(Rs); let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let acquire : boolean = A == '1' && Rt != '11111'; let release : boolean = R == '1'; let tagchecked : boolean = n != 31;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Ws&gt;</td>
<td><span id="WsOrWZR"></span>
<p>Is the 32-bit name of the general-purpose register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wt&gt;</td>
<td><span id="WtOrWZR__2"></span>
<p>Is the 32-bit name of the general-purpose register to be loaded, encoded in the "Rt" field.</p></td>
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

</div>

<div class="syntax-notes">

</div>

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [STCLRH, STCLRLH](stclrh_ldclrh.html "Atomic bit clear on halfword, without return") | <span class="pseudocode">A == '0' && Rt == '11111'</span> |

<div class="ps">

### Operation

var address : bits(64); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescAtomicOp](shared_pseudocode.html#func_CreateAccDescAtomicOp_7)([MemAtomicOp_BIC](shared_pseudocode.html#enum_MemAtomicOp_BIC), acquire, release, tagchecked, privileged, t, s); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let comparevalue : bits(16) = ARBITRARY : bits(16); // Irrelevant when not executing CAS let value : bits(16) = [X](shared_pseudocode.html#accessor_X_2){16}(s); let data : bits(16) = MemAtomic{}(address, comparevalue, value, accdesc); if t != 31 then [X](shared_pseudocode.html#accessor_X_2){32}(t) = ZeroExtend{32}(data); end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
