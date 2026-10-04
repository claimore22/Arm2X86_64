## RCWSSETP, RCWSSETPA, RCWSSETPAL, RCWSSETPL

Read check write software atomic bit set on quadword in memory

This instruction atomically loads a 128-bit quadword from memory, performs a bitwise OR with the value held in a pair of registers on it, and conditionally stores the result back to memory. Storing of the result back to memory is conditional on RCW Checks and RCWS Checks. The value initially loaded from memory is returned in the same pair of registers. If the RCW Checks fail or the RCWS Checks fail, the architecture permits writing the value read from the location to memory. If the write is performed, the read and the write occur atomically such that no other modification of the memory location can take place between the read and the write. This instruction updates the condition flags based on the result of the RCW Checks and RCWS Checks.

- <span class="asm-code">RCWSSETPA</span> and <span class="asm-code">RCWSSETPAL</span> load from memory with acquire semantics.
- <span class="asm-code">RCWSSETPL</span> and <span class="asm-code">RCWSSETPAL</span> store to memory with release semantics.
- <span class="asm-code">RCWSSETP</span> has neither acquire nor release semantics.

<div class="note">

------------------------------------------------------------------------

#### Note

This instruction is for performing atomic updates of translation table entries and not for general use.

------------------------------------------------------------------------

</div>

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_D128 && FEAT_THE) </span>

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
<td class="lr">1</td>
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
<td>1</td>
<td class="r">1</td>
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

#### Encoding for the RCWSSETP variant

<span id="RCWSSETP_128_memop_128"></span> Applies when <span class="bitdiff"> (A == 0 && R == 0)</span>

RCWSSETP [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the RCWSSETPA variant

<span id="RCWSSETPA_128_memop_128"></span> Applies when <span class="bitdiff"> (A == 1 && R == 0)</span>

RCWSSETPA [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the RCWSSETPAL variant

<span id="RCWSSETPAL_128_memop_128"></span> Applies when <span class="bitdiff"> (A == 1 && R == 1)</span>

RCWSSETPAL [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the RCWSSETPL variant

<span id="RCWSSETPL_128_memop_128"></span> Applies when <span class="bitdiff"> (A == 0 && R == 1)</span>

RCWSSETPL [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_D128) \|\| !IsFeatureImplemented(FEAT_THE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Rt == '11111' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Rt2 == '11111' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let t2 : integer{} = UInt(Rt2); let n : integer{} = UInt(Rn); let soft : boolean = TRUE; let acquire : boolean = A == '1'; let release : boolean = R == '1'; let tagchecked : boolean = n != 31; var rt_unknown : boolean = FALSE; if t == t2 then let c : [Constraint](shared_pseudocode.html#type_Constraint) = ConstrainUnpredictable([Unpredictable_LSE128OVERLAP](shared_pseudocode.html#enum_Unpredictable_LSE128OVERLAP)); assert c IN {[Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN), [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF), [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP)}; case c of when [Constraint_UNKNOWN](shared_pseudocode.html#enum_Constraint_UNKNOWN) =\> rt_unknown = TRUE; // result is UNKNOWN when [Constraint_UNDEF](shared_pseudocode.html#enum_Constraint_UNDEF) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when [Constraint_NOP](shared_pseudocode.html#enum_Constraint_NOP) =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; end;

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

if \![IsD128Enabled](shared_pseudocode.html#func_IsD128Enabled_1)([PSTATE](shared_pseudocode.html#global_PSTATE).EL) then Undefined(); end; var address : bits(64); var value1 : bits(64); var value2 : bits(64); var newdata : bits(128); var readdata : bits(128); var nzcv : bits(4); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescRCW](shared_pseudocode.html#func_CreateAccDescRCW_9)([MemAtomicOp_ORR](shared_pseudocode.html#enum_MemAtomicOp_ORR), soft, acquire, release, tagchecked, t, t2, t, t2); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; value1 = [X](shared_pseudocode.html#accessor_X_2){64}(t); value2 = [X](shared_pseudocode.html#accessor_X_2){64}(t2); newdata = if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then value1::value2 else value2::value1; let compdata : bits(128) = ARBITRARY : bits(128); // Irrelevant when not executing CAS (nzcv, readdata) = [MemAtomicRCW](shared_pseudocode.html#func_MemAtomicRCW_5){128}(address, compdata, newdata, accdesc); PSTATE.\[N,Z,C,V\] = nzcv; if rt_unknown then readdata = ARBITRARY : bits(128); end; if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then [X](shared_pseudocode.html#accessor_X_2){64}(t) = readdata\[127:64\]; [X](shared_pseudocode.html#accessor_X_2){64}(t2) = readdata\[63:0\]; else [X](shared_pseudocode.html#accessor_X_2){64}(t) = readdata\[63:0\]; [X](shared_pseudocode.html#accessor_X_2){64}(t2) = readdata\[127:64\]; end;

</div>
