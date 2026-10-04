## SWPP, SWPPA, SWPPAL, SWPPL

Swap quadword in memory

This instruction atomically loads a 128-bit quadword from a memory location, and stores the value held in a pair of registers back to the same memory location. The value initially loaded from memory is returned in the same pair of registers.

- <span class="asm-code">SWPPA</span> and <span class="asm-code">SWPPAL</span> load from memory with acquire semantics.
- <span class="asm-code">SWPPL</span> and <span class="asm-code">SWPPAL</span> store to memory with release semantics.
- <span class="asm-code">SWPP</span> has neither acquire nor release semantics.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_LSE128) </span>

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">A</td>
<td class="lr">R</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rt2</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td></td>
<td class="droppedname">S</td>
<td colspan="6"></td>
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

#### Encoding for the SWPP variant

<span id="SWPP_128_memop_128"></span> Applies when <span class="bitdiff"> (A == 0 && R == 0)</span>

SWPP [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the SWPPA variant

<span id="SWPPA_128_memop_128"></span> Applies when <span class="bitdiff"> (A == 1 && R == 0)</span>

SWPPA [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the SWPPAL variant

<span id="SWPPAL_128_memop_128"></span> Applies when <span class="bitdiff"> (A == 1 && R == 1)</span>

SWPPAL [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the SWPPL variant

<span id="SWPPL_128_memop_128"></span> Applies when <span class="bitdiff"> (A == 0 && R == 1)</span>

SWPPL [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_LSE128) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Rt == '11111' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Rt2 == '11111' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let t2 : integer{} = UInt(Rt2); let n : integer{} = UInt(Rn); let acquire : boolean = A == '1'; let release : boolean = R == '1'; let tagchecked : boolean = n != 31; var rt_unknown : boolean = FALSE; if t == t2 then let c : [Constraint](shared_pseudocode.html#type_Constraint) = ConstrainUnpredictable([Unpredictable_LSE128OVERLAP](shared_pseudocode.html#enum_Unpredictable_LSE128OVERLAP)); assert c IN {[Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> rt_unknown = TRUE; // result is UNKNOWN when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end;

<div class="encoding-notes">

For information about the <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span> behavior of this instruction, see <span class="armarm-xref" data-linkend="CHDJDBHJ">Architectural Constraints on UNPREDICTABLE behaviors</span>, and particularly <span class="armarm-xref" data-linkend="128_bit_atomic_memory_operations">128-bit Atomic Memory Operations</span>.

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xt1&gt;</td>
<td><span id="Xt1OrXZR"></span>
<p>Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt2&gt;</td>
<td><span id="Xt2OrXZR"></span>
<p>Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field.</p></td>
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

<div class="ps">

### Operation

var address : bits(64); let value1 : bits(64) = X{}(t); let value2 : bits(64) = X{}(t2); var data : bits(128); var store_value : bits(128); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescAtomicOp](shared_pseudocode.html#func_CreateAccDescAtomicOp_9)([MemAtomicOp_SWP](shared_pseudocode.html#enum_MemAtomicOp_SWP), acquire, release, tagchecked, privileged, t, t2, t, t2); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; store_value = if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then value1::value2 else value2::value1; let comparevalue : bits(128) = ARBITRARY : bits(128); // Irrelevant when not executing CAS data = [MemAtomic](shared_pseudocode.html#func_MemAtomic_5){128}(address, comparevalue, store_value, accdesc); if rt_unknown then data = ARBITRARY : bits(128); end; if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then [X](shared_pseudocode.html#accessor_X_2){64}(t) = data\[127:64\]; [X](shared_pseudocode.html#accessor_X_2){64}(t2) = data\[63:0\]; else [X](shared_pseudocode.html#accessor_X_2){64}(t) = data\[63:0\]; [X](shared_pseudocode.html#accessor_X_2){64}(t2) = data\[127:64\]; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
