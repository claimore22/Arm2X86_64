## STFMAXNM, STFMAXNML

Atomic floating-point maximum number, without return

This instruction atomically loads a 16-bit, 32-bit, or 64-bit value from memory, calculates the floating-point maximum number with the value held in a register, and stores the result back to memory.

- <span class="asm-code">STFMAXNML</span> stores to memory with release semantics.
- <span class="asm-code">STFMAXNM</span> has no release semantics.

This instruction:

- Disables alternative floating-point behaviors, as if FPCR.AH is 0.
- Generates only the default NaN, as if FPCR.DN is 1.
- Does not modify the cumulative FPSR exception bits (IDC, IXC, UFC, OFC, DZC, and IOC).
- Disables trapped floating-point exceptions, as if the FPCR trap enable bits (IDE, IXE, UFE, OFE, DZE, and IOE) are all zero.

For more information about memory ordering semantics, see <span class="armarm-xref" data-linkend="ARMARM_BEIHCHEF">Load-Acquire, Store-Release</span>.

For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

### <span id="iclass_floating_point"></span>Floating-point<span style="font-size:smaller;">\
(FEAT_LSFE) </span>

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
<td class="lr">0</td>
<td class="lr">R</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rs</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="2"></td>
<td class="droppedname">A</td>
<td></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">o3</td>
<td colspan="3" class="droppedname">opc</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5" class="droppedname">Rt</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Half-precision no memory ordering variant

<span id="STFMAXNM_16"></span> Applies when <span class="bitdiff"> (size == 01 && R == 0)</span>

STFMAXNM [\<Hs\>](#fpfar_hs "Is the 16-bit name of the SIMD&FP register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the Half-precision release variant

<span id="STFMAXNML_16"></span> Applies when <span class="bitdiff"> (size == 01 && R == 1)</span>

STFMAXNML [\<Hs\>](#fpfar_hs "Is the 16-bit name of the SIMD&FP register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the Single-precision no memory ordering variant

<span id="STFMAXNM_32"></span> Applies when <span class="bitdiff"> (size == 10 && R == 0)</span>

STFMAXNM [\<Ss\>](#fpfar_ss "Is the 32-bit name of the SIMD&FP register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the Single-precision release variant

<span id="STFMAXNML_32"></span> Applies when <span class="bitdiff"> (size == 10 && R == 1)</span>

STFMAXNML [\<Ss\>](#fpfar_ss "Is the 32-bit name of the SIMD&FP register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the Double-precision no memory ordering variant

<span id="STFMAXNM_64"></span> Applies when <span class="bitdiff"> (size == 11 && R == 0)</span>

STFMAXNM [\<Ds\>](#fpfar_ds "Is the 64-bit name of the SIMD&FP register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the Double-precision release variant

<span id="STFMAXNML_64"></span> Applies when <span class="bitdiff"> (size == 11 && R == 1)</span>

STFMAXNML [\<Ds\>](#fpfar_ds "Is the 64-bit name of the SIMD&FP register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_LSFE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let s : integer = UInt(Rs); let n : integer = UInt(Rn); let datasize : integer{} = 8 \<\< UInt(size); let acquire : boolean = FALSE; let release : boolean = R == '1'; let tagchecked : boolean = n != 31;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Hs&gt;</td>
<td><span id="fpfar_hs"></span>
<p>Is the 16-bit name of the SIMD&amp;FP register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field.</p></td>
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
<td>&lt;Ss&gt;</td>
<td><span id="fpfar_ss"></span>
<p>Is the 32-bit name of the SIMD&amp;FP register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Ds&gt;</td>
<td><span id="fpfar_ds"></span>
<p>Is the 64-bit name of the SIMD&amp;FP register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); var address : bits(64); var value : bits(datasize); var data : bits(datasize); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescFPAtomicOp](shared_pseudocode.html#func_CreateAccDescFPAtomicOp_4)([MemAtomicOp_FPMAXNM](shared_pseudocode.html#enum_MemAtomicOp_FPMAXNM), acquire, release, tagchecked); value = [V](shared_pseudocode.html#accessor_V_2){datasize}(s); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let comparevalue : bits(datasize) = ARBITRARY : bits(datasize); // Irrelevant when not executing CAS data = [MemAtomic](shared_pseudocode.html#func_MemAtomic_5){datasize}(address, comparevalue, value, accdesc);

</div>
