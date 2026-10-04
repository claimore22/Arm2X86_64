## RCWCASP, RCWCASPA, RCWCASPAL, RCWCASPL

Read check write compare and swap quadword in memory

This instruction reads a 128-bit quadword from memory, and compares it against the value held in a pair of registers. If the comparison is equal, the value in a second pair of registers is conditionally written to memory. Storing back to memory is conditional on RCW Checks. If the compare fails or the RCW Checks fail, the architecture permits writing the value read from the location to memory. If the write is performed, the read and the write occur atomically such that no other modification of the memory location can take place between the read and the write. This instruction updates the condition flags based on the result of the update of memory.

- <span class="asm-code">RCWCASPA</span> and <span class="asm-code">RCWCASPAL</span> load from memory with acquire semantics.
- <span class="asm-code">RCWCASPL</span> and <span class="asm-code">RCWCASPAL</span> store to memory with release semantics.
- <span class="asm-code">RCWCASP</span> has neither acquire nor release semantics.

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
<td colspan="5" class="lr">Rs</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
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
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the RCWCASP variant

<span id="RCWCASP_C64_rcwcomswappr"></span> Applies when <span class="bitdiff"> (A == 0 && R == 0)</span>

RCWCASP [\<Xs\>](#Xs "Is the 64-bit name of the first general-purpose register to be compared and loaded, encoded in the "Rs" field. <Xs> must be an even-numbered register."), [\<X(s+1)\>](#XsPlus1 "Is the 64-bit name of the second general-purpose register to be compared and loaded."), [\<Xt\>](#Xt "Is the 64-bit name of the first general-purpose register to be conditionally stored, encoded in the "Rt" field. <Xt> must be an even-numbered register."), [\<X(t+1)\>](#XtPlus1__3 "Is the 64-bit name of the second general-purpose register to be conditionally stored."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the RCWCASPA variant

<span id="RCWCASPA_C64_rcwcomswappr"></span> Applies when <span class="bitdiff"> (A == 1 && R == 0)</span>

RCWCASPA [\<Xs\>](#Xs "Is the 64-bit name of the first general-purpose register to be compared and loaded, encoded in the "Rs" field. <Xs> must be an even-numbered register."), [\<X(s+1)\>](#XsPlus1 "Is the 64-bit name of the second general-purpose register to be compared and loaded."), [\<Xt\>](#Xt "Is the 64-bit name of the first general-purpose register to be conditionally stored, encoded in the "Rt" field. <Xt> must be an even-numbered register."), [\<X(t+1)\>](#XtPlus1__3 "Is the 64-bit name of the second general-purpose register to be conditionally stored."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the RCWCASPAL variant

<span id="RCWCASPAL_C64_rcwcomswappr"></span> Applies when <span class="bitdiff"> (A == 1 && R == 1)</span>

RCWCASPAL [\<Xs\>](#Xs "Is the 64-bit name of the first general-purpose register to be compared and loaded, encoded in the "Rs" field. <Xs> must be an even-numbered register."), [\<X(s+1)\>](#XsPlus1 "Is the 64-bit name of the second general-purpose register to be compared and loaded."), [\<Xt\>](#Xt "Is the 64-bit name of the first general-purpose register to be conditionally stored, encoded in the "Rt" field. <Xt> must be an even-numbered register."), [\<X(t+1)\>](#XtPlus1__3 "Is the 64-bit name of the second general-purpose register to be conditionally stored."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the RCWCASPL variant

<span id="RCWCASPL_C64_rcwcomswappr"></span> Applies when <span class="bitdiff"> (A == 0 && R == 1)</span>

RCWCASPL [\<Xs\>](#Xs "Is the 64-bit name of the first general-purpose register to be compared and loaded, encoded in the "Rs" field. <Xs> must be an even-numbered register."), [\<X(s+1)\>](#XsPlus1 "Is the 64-bit name of the second general-purpose register to be compared and loaded."), [\<Xt\>](#Xt "Is the 64-bit name of the first general-purpose register to be conditionally stored, encoded in the "Rt" field. <Xt> must be an even-numbered register."), [\<X(t+1)\>](#XtPlus1__3 "Is the 64-bit name of the second general-purpose register to be conditionally stored."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_D128) \|\| !IsFeatureImplemented(FEAT_THE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Rs\[0\] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Rt\[0\] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let s : integer{} = UInt(Rs); let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let acquire : boolean = A == '1'; let release : boolean = R == '1'; let soft : boolean = FALSE; let tagchecked : boolean = n != 31;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xs&gt;</td>
<td><span id="Xs"></span>
<p>Is the 64-bit name of the first general-purpose register to be compared and loaded, encoded in the "Rs" field. &lt;Xs&gt; must be an even-numbered register.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;X(s+1)&gt;</td>
<td><span id="XsPlus1"></span>
<p>Is the 64-bit name of the second general-purpose register to be compared and loaded.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="Xt"></span>
<p>Is the 64-bit name of the first general-purpose register to be conditionally stored, encoded in the "Rt" field. &lt;Xt&gt; must be an even-numbered register.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;X(t+1)&gt;</td>
<td><span id="XtPlus1__3"></span>
<p>Is the 64-bit name of the second general-purpose register to be conditionally stored.</p></td>
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

if \![IsD128Enabled](shared_pseudocode.html#func_IsD128Enabled_1)([PSTATE](shared_pseudocode.html#global_PSTATE).EL) then Undefined(); end; var address : bits(64); var newdata : bits(128); var compdata : bits(128); var readdata : bits(128); var nzcv : bits(4); let s1 : bits(64) = X{}(s); let s2 : bits(64) = X{}(s+1); let t1 : bits(64) = X{}(t); let t2 : bits(64) = X{}(t+1); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescRCW](shared_pseudocode.html#func_CreateAccDescRCW_9)([MemAtomicOp_CAS](shared_pseudocode.html#enum_MemAtomicOp_CAS), soft, acquire, release, tagchecked, t, t+1, s, s+1); compdata = if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then s1::s2 else s2::s1; newdata = if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then t1::t2 else t2::t1; if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; (nzcv, readdata) = [MemAtomicRCW](shared_pseudocode.html#func_MemAtomicRCW_5){128}(address, compdata, newdata, accdesc); PSTATE.\[N,Z,C,V\] = nzcv; if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then [X](shared_pseudocode.html#accessor_X_2){64}(s) = readdata\[127:64\]; [X](shared_pseudocode.html#accessor_X_2){64}(s+1) = readdata\[63:0\]; else [X](shared_pseudocode.html#accessor_X_2){64}(s) = readdata\[63:0\]; [X](shared_pseudocode.html#accessor_X_2){64}(s+1) = readdata\[127:64\]; end;

</div>
