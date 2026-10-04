## RCWSSET, RCWSSETA, RCWSSETAL, RCWSSETL

Read check write software atomic bit set on doubleword in memory

This instruction atomically loads a 64-bit doubleword from memory, performs a bitwise OR with the value held in a register on it, and conditionally stores the result back to memory. Storing of the result back to memory is conditional on RCW Checks and RCWS Checks. The value initially loaded from memory is returned in the destination register. If the RCW Checks fail or the RCWS Checks fail, the architecture permits writing the value read from the location to memory. If the write is performed, the read and the write occur atomically such that no other modification of the memory location can take place between the read and the write. This instruction updates the condition flags based on the result of the RCW Checks and RCWS Checks.

- If the destination register is not XZR, <span class="asm-code">RCWSSETA</span> and <span class="asm-code">RCWSSETAL</span> load from memory with acquire semantics.
- <span class="asm-code">RCWSSETL</span> and <span class="asm-code">RCWSSETAL</span> store to memory with release semantics.
- <span class="asm-code">RCWSSET</span> has neither acquire nor release semantics.

<div class="note">

------------------------------------------------------------------------

#### Note

This instruction is for performing atomic updates of translation table entries and not for general use.

------------------------------------------------------------------------

</div>

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_THE) </span>

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

#### Encoding for the RCWSSET variant

<span id="RCWSSET_64_memop"></span> Applies when <span class="bitdiff"> (A == 0 && R == 0)</span>

RCWSSET [\<Xs\>](#XsOrXZR "Is the 64-bit name of the general-purpose register to be stored, encoded in the "Rs" field."), [\<Xt\>](#XtOrXZR__8 "Is the 64-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the RCWSSETA variant

<span id="RCWSSETA_64_memop"></span> Applies when <span class="bitdiff"> (A == 1 && R == 0)</span>

RCWSSETA [\<Xs\>](#XsOrXZR "Is the 64-bit name of the general-purpose register to be stored, encoded in the "Rs" field."), [\<Xt\>](#XtOrXZR__8 "Is the 64-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the RCWSSETAL variant

<span id="RCWSSETAL_64_memop"></span> Applies when <span class="bitdiff"> (A == 1 && R == 1)</span>

RCWSSETAL [\<Xs\>](#XsOrXZR "Is the 64-bit name of the general-purpose register to be stored, encoded in the "Rs" field."), [\<Xt\>](#XtOrXZR__8 "Is the 64-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the RCWSSETL variant

<span id="RCWSSETL_64_memop"></span> Applies when <span class="bitdiff"> (A == 0 && R == 1)</span>

RCWSSETL [\<Xs\>](#XsOrXZR "Is the 64-bit name of the general-purpose register to be stored, encoded in the "Rs" field."), [\<Xt\>](#XtOrXZR__8 "Is the 64-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_THE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let s : integer{} = UInt(Rs); let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let soft : boolean = TRUE; let acquire : boolean = A == '1' && t != 31; let release : boolean = R == '1'; let tagchecked : boolean = n != 31;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xs&gt;</td>
<td><span id="XsOrXZR"></span>
<p>Is the 64-bit name of the general-purpose register to be stored, encoded in the "Rs" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__8"></span>
<p>Is the 64-bit name of the general-purpose register to be loaded, encoded in the "Rt" field.</p></td>
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

if [IsD128Enabled](shared_pseudocode.html#func_IsD128Enabled_1)([PSTATE](shared_pseudocode.html#global_PSTATE).EL) then Undefined(); end; var address : bits(64); let newdata : bits(64) = X{}(s); var readdata : bits(64); var nzcv : bits(4); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescRCW](shared_pseudocode.html#func_CreateAccDescRCW_7)([MemAtomicOp_ORR](shared_pseudocode.html#enum_MemAtomicOp_ORR), soft, acquire, release, tagchecked, t, s); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let compdata : bits(64) = ARBITRARY : bits(64); // Irrelevant when not executing CAS (nzcv, readdata) = [MemAtomicRCW](shared_pseudocode.html#func_MemAtomicRCW_5){64}(address, compdata, newdata, accdesc); PSTATE.\[N,Z,C,V\] = nzcv; [X](shared_pseudocode.html#accessor_X_2){64}(t) = readdata; // Return the old value when t!=31

</div>
