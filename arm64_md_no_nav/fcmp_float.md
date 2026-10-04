## FCMP

Floating-point quiet compare (scalar)

This instruction compares the two SIMD&FP source register values, or the first SIMD&FP source register value and zero. It writes the result to the <span class="armarm-xref" data-linkend="ARMARM_PSTATE">PSTATE</span>.{N, Z, C, V} flags.

This instruction raises an Invalid Operation floating-point exception if either or both of the operands is a signaling NaN.

This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see <span class="armarm-xref" data-linkend="ARMARM_BEIJDDAG">Floating-point exceptions and exception traps</span>.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

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
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="2" class="lr">ftype</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="l">0</td>
<td class="r">x</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td class="droppedname">M</td>
<td></td>
<td class="droppedname">S</td>
<td colspan="5"></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="2" class="droppedname">op</td>
<td colspan="4"></td>
<td colspan="5"></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Half-precision variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="FCMP_H_floatcmp"></span> Applies when <span class="bitdiff"> (ftype == 11 && opc == 00)</span>

FCMP [\<Hn\>](#Hn "For the "Half-precision" variant: is the 16-bit name of the first SIMD&FP source register, encoded in the "Rn" field."), [\<Hm\>](#Hm "Is the 16-bit name of the second SIMD&FP source register, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the Half-precision, zero variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="FCMP_HZ_floatcmp"></span> Applies when <span class="bitdiff"> (ftype == 11 && Rm == (00000) && opc == 01)</span>

FCMP [\<Hn\>](#Hn__2 "For the "Half-precision, zero" variant: is the 16-bit name of the SIMD&FP source register, encoded in the "Rn" field."), \#0.0

</div>

<div class="encoding">

#### Encoding for the Single-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FCMP_S_floatcmp"></span> Applies when <span class="bitdiff"> (ftype == 00 && opc == 00)</span>

FCMP [\<Sn\>](#Sn__3 "For the "Single-precision" variant: is the 32-bit name of the first SIMD&FP source register, encoded in the "Rn" field."), [\<Sm\>](#Sm "Is the 32-bit name of the second SIMD&FP source register, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the Single-precision, zero variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FCMP_SZ_floatcmp"></span> Applies when <span class="bitdiff"> (ftype == 00 && Rm == (00000) && opc == 01)</span>

FCMP [\<Sn\>](#Sn "For the "Single-precision, zero" variant: is the 32-bit name of the SIMD&FP source register, encoded in the "Rn" field."), \#0.0

</div>

<div class="encoding">

#### Encoding for the Double-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FCMP_D_floatcmp"></span> Applies when <span class="bitdiff"> (ftype == 01 && opc == 00)</span>

FCMP [\<Dn\>](#Dn__2 "For the "Double-precision" variant: is the 64-bit name of the first SIMD&FP source register, encoded in the "Rn" field."), [\<Dm\>](#Dm "Is the 64-bit name of the second SIMD&FP source register, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the Double-precision, zero variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FCMP_DZ_floatcmp"></span> Applies when <span class="bitdiff"> (ftype == 01 && Rm == (00000) && opc == 01)</span>

FCMP [\<Dn\>](#Dn "For the "Double-precision, zero" variant: is the 64-bit name of the SIMD&FP source register, encoded in the "Rn" field."), \#0.0

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Rn); let m : integer = UInt(Rm); // ignored when opc\<0\> == '1' let datasize : integer{} = 8 \<\< UInt(ftype XOR '10'); let signal_all_nans : boolean = FALSE; let cmp_with_zero : boolean = (opc\[0\] == '1');

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Hn&gt;</td>
<td><span id="Hn"></span>
<p>For the "Half-precision" variant: is the 16-bit name of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Hn__2"></span>
<p>For the "Half-precision, zero" variant: is the 16-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Hm&gt;</td>
<td><span id="Hm"></span>
<p>Is the 16-bit name of the second SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sn&gt;</td>
<td><span id="Sn__3"></span>
<p>For the "Single-precision" variant: is the 32-bit name of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Sn"></span>
<p>For the "Single-precision, zero" variant: is the 32-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sm&gt;</td>
<td><span id="Sm"></span>
<p>Is the 32-bit name of the second SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dn&gt;</td>
<td><span id="Dn__2"></span>
<p>For the "Double-precision" variant: is the 64-bit name of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Dn"></span>
<p>For the "Double-precision, zero" variant: is the 64-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dm&gt;</td>
<td><span id="Dm"></span>
<p>Is the 64-bit name of the second SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = if cmp_with_zero then [FPZero](shared_pseudocode.html#func_FPZero_2){datasize}('0') else [V](shared_pseudocode.html#accessor_V_2){datasize}(m); PSTATE.\[N,Z,C,V\] = [FPCompare](shared_pseudocode.html#func_FPCompare_5){datasize}(operand1, operand2, signal_all_nans, FPCR());

</div>

### Operational information

The IEEE 754 standard specifies that the result of a comparison is precisely one of \<, ==, \> or unordered. If either or both of the operands is a NaN, they are unordered, and all three of (Operand1 \< Operand2), (Operand1 == Operand2) and (Operand1 \> Operand2) are false. An unordered comparison sets the PSTATE condition flags to N=0, Z=0, C=1, and V=1.

If FEAT_SME is implemented and the PE is in Streaming SVE mode, then any subsequent instruction which is dependent on the NZCV condition flags written by this instruction might be significantly delayed.
