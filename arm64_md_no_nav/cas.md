## CAS, CASA, CASAL, CASL

Compare and swap word or doubleword in memory

This instruction reads a 32-bit word or 64-bit doubleword from memory, and compares it against the value held in a first register. If the comparison is equal, the value in a second register is written to memory. If the comparison is not equal, the architecture permits writing the value read from the location to memory. If the write is performed, the read and write occur atomically such that no other modification of the memory location can take place between the read and write.

- If the destination register is not one of WZR or XZR, <span class="asm-code">CASA</span> and <span class="asm-code">CASAL</span> load from memory with acquire semantics.
- <span class="asm-code">CASL</span> and <span class="asm-code">CASAL</span> store to memory with release semantics.
- <span class="asm-code">CAS</span> has neither acquire nor release semantics.

The architecture permits that the data read clears any exclusive monitors associated with that location, even if the compare subsequently fails.

If the instruction generates a synchronous Data Abort, the register which is compared and loaded, that is \<Ws\>, or \<Xs\>, is restored to the value held in the register before the instruction was executed.

For a <span class="asm-code">CAS</span> or <span class="asm-code">CASA</span> instruction, when \<Ws\> or \<Xs\> specifies the same register as \<Wt\> or \<Xt\>, this signals to the memory system that an additional subsequent <span class="asm-code">CAS</span>, <span class="asm-code">CASA</span>, <span class="asm-code">CASAL</span>, or <span class="asm-code">CASL</span> access to the specified location is expected to occur in the near future in the absence of taking an exception or exceptional software conditions. The memory system can respond by taking actions that are expected to enable the subsequent <span class="asm-code">CAS</span>, <span class="asm-code">CASA</span>, <span class="asm-code">CASAL</span>, or <span class="asm-code">CASL</span> access to succeed when it does occur.

A code sequence starting with a <span class="asm-code">CAS</span> or <span class="asm-code">CASA</span> instruction for which \<Ws\> or \<Xs\> specifies the same register as \<Wt\> or \<Xt\>, and ending with a subsequent <span class="asm-code">CAS</span>, <span class="asm-code">CASA</span>, <span class="asm-code">CASAL</span>, or <span class="asm-code">CASL</span> to the same location, exhibits the following properties for best performance when the location may be accessed concurrently, on one or more other PEs:

- The sequence does not contain any direct system register writes, address translation instructions, cache or TLB maintenance operations, exception producing instructions, exception returns, or <span class="asm-code">ISB</span> barriers.
- The execution of the sequence includes 32 or fewer instructions.
- The value provided in \<Ws\> or \<Xs\> of the first <span class="asm-code">CAS</span> or <span class="asm-code">CASA</span> is a value likely to result in the comparison failing. A failing comparison result may lead to better performance due to the hardware not performing a write to memory.

<div class="note">

------------------------------------------------------------------------

#### Note

For a <span class="asm-code">CAS</span> or <span class="asm-code">CASA</span> instruction, when \<Ws\> or \<Xs\> specifies the same register as \<Wt\> or \<Xt\>, the value in memory is not modified, because the <span class="asm-code">CAS</span> or <span class="asm-code">CASA</span> either fails its compare or writes the same value back to memory.

For performance reasons, Arm recommends that software not use a <span class="asm-code">CAS</span> or <span class="asm-code">CASA</span> instruction when \<Ws\> or \<Xs\> specifies the same register as \<Wt\> or \<Xt\> in a routine that includes a subsequent <span class="asm-code">CAS</span>, <span class="asm-code">CASA</span>, <span class="asm-code">CASAL</span>, or <span class="asm-code">CASL</span> for any of the following purposes:

- To load the value that will be compared to conditionally determine whether or not to execute the subsequent <span class="asm-code">CAS</span>, <span class="asm-code">CASA</span>, <span class="asm-code">CASAL</span>, or <span class="asm-code">CASL</span>.
- As a form of prefetch executed between the instruction loading the value that will be compared and the subsequent compare instruction that conditionally determines whether or not to execute the subsequent <span class="asm-code">CAS</span>, <span class="asm-code">CASA</span>, <span class="asm-code">CASAL</span>, or <span class="asm-code">CASL</span>.

This guidance does not apply when \<Ws\> or \<Xs\> specifies the same register as \<Wt\> or \<Xt\> for a <span class="asm-code">CAS</span> or <span class="asm-code">CASA</span> instruction and the compare between the <span class="asm-code">CAS</span> or <span class="asm-code">CASA</span> and the subsequent <span class="asm-code">CAS</span>, <span class="asm-code">CASA</span>, <span class="asm-code">CASAL</span>, or <span class="asm-code">CASL</span> is intended to detect an exceptional software condition such that the result of the compare does not prevent the subsequent <span class="asm-code">CAS</span>, <span class="asm-code">CASA</span>, <span class="asm-code">CASAL</span>, or <span class="asm-code">CASL</span> from executing under normal operating conditions.

------------------------------------------------------------------------

</div>

For more information about memory ordering semantics, see <span class="armarm-xref" data-linkend="ARMARM_BEIHCHEF">Load-Acquire, Store-Release</span>.

For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

### <span id="iclass_no_offset"></span>No offset<span style="font-size:smaller;">\
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
<td class="l">1</td>
<td class="r">x</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">L</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rs</td>
<td class="lr">o0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="7"></td>
<td></td>
<td></td>
<td colspan="5"></td>
<td></td>
<td colspan="5" class="droppedname">Rt2</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit CAS variant

<span id="CAS_C32_comswap"></span> Applies when <span class="bitdiff"> (size == 10 && L == 0 && o0 == 0)</span>

CAS [\<Ws\>](#WsOrWZR__3 "Is the 32-bit name of the general-purpose register to be compared and loaded, encoded in the "Rs" field."), [\<Wt\>](#WtOrWZR__3 "Is the 32-bit name of the general-purpose register to be conditionally stored, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

<div class="encoding">

#### Encoding for the 32-bit CASA variant

<span id="CASA_C32_comswap"></span> Applies when <span class="bitdiff"> (size == 10 && L == 1 && o0 == 0)</span>

CASA [\<Ws\>](#WsOrWZR__3 "Is the 32-bit name of the general-purpose register to be compared and loaded, encoded in the "Rs" field."), [\<Wt\>](#WtOrWZR__3 "Is the 32-bit name of the general-purpose register to be conditionally stored, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

<div class="encoding">

#### Encoding for the 32-bit CASAL variant

<span id="CASAL_C32_comswap"></span> Applies when <span class="bitdiff"> (size == 10 && L == 1 && o0 == 1)</span>

CASAL [\<Ws\>](#WsOrWZR__3 "Is the 32-bit name of the general-purpose register to be compared and loaded, encoded in the "Rs" field."), [\<Wt\>](#WtOrWZR__3 "Is the 32-bit name of the general-purpose register to be conditionally stored, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

<div class="encoding">

#### Encoding for the 32-bit CASL variant

<span id="CASL_C32_comswap"></span> Applies when <span class="bitdiff"> (size == 10 && L == 0 && o0 == 1)</span>

CASL [\<Ws\>](#WsOrWZR__3 "Is the 32-bit name of the general-purpose register to be compared and loaded, encoded in the "Rs" field."), [\<Wt\>](#WtOrWZR__3 "Is the 32-bit name of the general-purpose register to be conditionally stored, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit CAS variant

<span id="CAS_C64_comswap"></span> Applies when <span class="bitdiff"> (size == 11 && L == 0 && o0 == 0)</span>

CAS [\<Xs\>](#XsOrXZR__4 "Is the 64-bit name of the general-purpose register to be compared and loaded, encoded in the "Rs" field."), [\<Xt\>](#XtOrXZR__10 "Is the 64-bit name of the general-purpose register to be conditionally stored, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit CASA variant

<span id="CASA_C64_comswap"></span> Applies when <span class="bitdiff"> (size == 11 && L == 1 && o0 == 0)</span>

CASA [\<Xs\>](#XsOrXZR__4 "Is the 64-bit name of the general-purpose register to be compared and loaded, encoded in the "Rs" field."), [\<Xt\>](#XtOrXZR__10 "Is the 64-bit name of the general-purpose register to be conditionally stored, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit CASAL variant

<span id="CASAL_C64_comswap"></span> Applies when <span class="bitdiff"> (size == 11 && L == 1 && o0 == 1)</span>

CASAL [\<Xs\>](#XsOrXZR__4 "Is the 64-bit name of the general-purpose register to be compared and loaded, encoded in the "Rs" field."), [\<Xt\>](#XtOrXZR__10 "Is the 64-bit name of the general-purpose register to be conditionally stored, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

<div class="encoding">

#### Encoding for the 64-bit CASL variant

<span id="CASL_C64_comswap"></span> Applies when <span class="bitdiff"> (size == 11 && L == 0 && o0 == 1)</span>

CASL [\<Xs\>](#XsOrXZR__4 "Is the 64-bit name of the general-purpose register to be compared and loaded, encoded in the "Rs" field."), [\<Xt\>](#XtOrXZR__10 "Is the 64-bit name of the general-purpose register to be conditionally stored, encoded in the "Rt" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#0}\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_LSE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let s : integer{} = UInt(Rs); let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let datasize : integer{} = 8 \<\< UInt(size); let regsize : integer{} = if datasize == 64 then 64 else 32; let acquire : boolean = L == '1' && s != 31; let release : boolean = o0 == '1'; let tagchecked : boolean = n != 31;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Ws&gt;</td>
<td><span id="WsOrWZR__3"></span>
<p>Is the 32-bit name of the general-purpose register to be compared and loaded, encoded in the "Rs" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wt&gt;</td>
<td><span id="WtOrWZR__3"></span>
<p>Is the 32-bit name of the general-purpose register to be conditionally stored, encoded in the "Rt" field.</p></td>
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
<td>&lt;Xs&gt;</td>
<td><span id="XsOrXZR__4"></span>
<p>Is the 64-bit name of the general-purpose register to be compared and loaded, encoded in the "Rs" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__10"></span>
<p>Is the 64-bit name of the general-purpose register to be conditionally stored, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

var address : bits(64); var comparevalue : bits(datasize); var newvalue : bits(datasize); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescAtomicOp](shared_pseudocode.html#func_CreateAccDescAtomicOp_7)([MemAtomicOp_CAS](shared_pseudocode.html#enum_MemAtomicOp_CAS), acquire, release, tagchecked, privileged, t, s); comparevalue = [X](shared_pseudocode.html#accessor_X_2){datasize}(s); newvalue = [X](shared_pseudocode.html#accessor_X_2){datasize}(t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let data : bits(datasize) = MemAtomic{}(address, comparevalue, newvalue, accdesc); [X](shared_pseudocode.html#accessor_X_2){regsize}(s) = ZeroExtend{regsize}(data);

</div>
