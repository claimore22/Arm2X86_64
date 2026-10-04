## FCVTZU (scalar, fixed-point)

Floating-point convert to unsigned fixed-point, rounding toward zero (scalar)

This instruction converts the floating-point value in the SIMD&FP source register to a 32-bit or 64-bit fixed-point unsigned integer using the Round towards Zero rounding mode, and writes the result to the general-purpose destination register.

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
<td class="lr">sf</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="2" class="lr">ftype</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="6" class="lr">scale</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">S</td>
<td colspan="5"></td>
<td colspan="2"></td>
<td></td>
<td colspan="2" class="droppedname">rmode</td>
<td colspan="3" class="droppedname">opcode</td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Half-precision to 32-bit variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="FCVTZU_32H_float2fix"></span> Applies when <span class="bitdiff"> (sf == 0 && ftype == 11)</span>

FCVTZU [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Hn\>](#Hn__2 "Is the 16-bit name of the SIMD&FP source register, encoded in the "Rn" field."), \#[\<fbits\>](#fbits__2 "For the "Double-precision to 32-bit", "Half-precision to 32-bit", and "Single-precision to 32-bit" variants: is the number of bits after the binary point in the fixed-point destination, in the range 1 to 32, encoded as 64 minus "scale".")

</div>

<div class="encoding">

#### Encoding for the Half-precision to 64-bit variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="FCVTZU_64H_float2fix"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 11)</span>

FCVTZU [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Hn\>](#Hn__2 "Is the 16-bit name of the SIMD&FP source register, encoded in the "Rn" field."), \#[\<fbits\>](#fbits__4 "For the "Double-precision to 64-bit", "Half-precision to 64-bit", and "Single-precision to 64-bit" variants: is the number of bits after the binary point in the fixed-point destination, in the range 1 to 64, encoded as 64 minus "scale".")

</div>

<div class="encoding">

#### Encoding for the Single-precision to 32-bit variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FCVTZU_32S_float2fix"></span> Applies when <span class="bitdiff"> (sf == 0 && ftype == 00)</span>

FCVTZU [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Sn\>](#Sn "Is the 32-bit name of the SIMD&FP source register, encoded in the "Rn" field."), \#[\<fbits\>](#fbits__2 "For the "Double-precision to 32-bit", "Half-precision to 32-bit", and "Single-precision to 32-bit" variants: is the number of bits after the binary point in the fixed-point destination, in the range 1 to 32, encoded as 64 minus "scale".")

</div>

<div class="encoding">

#### Encoding for the Single-precision to 64-bit variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FCVTZU_64S_float2fix"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 00)</span>

FCVTZU [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Sn\>](#Sn "Is the 32-bit name of the SIMD&FP source register, encoded in the "Rn" field."), \#[\<fbits\>](#fbits__4 "For the "Double-precision to 64-bit", "Half-precision to 64-bit", and "Single-precision to 64-bit" variants: is the number of bits after the binary point in the fixed-point destination, in the range 1 to 64, encoded as 64 minus "scale".")

</div>

<div class="encoding">

#### Encoding for the Double-precision to 32-bit variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FCVTZU_32D_float2fix"></span> Applies when <span class="bitdiff"> (sf == 0 && ftype == 01)</span>

FCVTZU [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Dn\>](#Dn "Is the 64-bit name of the SIMD&FP source register, encoded in the "Rn" field."), \#[\<fbits\>](#fbits__2 "For the "Double-precision to 32-bit", "Half-precision to 32-bit", and "Single-precision to 32-bit" variants: is the number of bits after the binary point in the fixed-point destination, in the range 1 to 32, encoded as 64 minus "scale".")

</div>

<div class="encoding">

#### Encoding for the Double-precision to 64-bit variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FCVTZU_64D_float2fix"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 01)</span>

FCVTZU [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Dn\>](#Dn "Is the 64-bit name of the SIMD&FP source register, encoded in the "Rn" field."), \#[\<fbits\>](#fbits__4 "For the "Double-precision to 64-bit", "Half-precision to 64-bit", and "Single-precision to 64-bit" variants: is the number of bits after the binary point in the fixed-point destination, in the range 1 to 64, encoded as 64 minus "scale".")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sf == '0' && scale\[5\] == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let intsize : integer{} = 32 \<\< UInt(sf); let decode_fltsize : integer{} = 8 \<\< UInt(ftype XOR '10'); let fracbits : integer = 64 - UInt(scale); let rounding : [FPRounding](shared_pseudocode.html#type_FPRounding) = [FPRounding_ZERO](shared_pseudocode.html#enum_FPRounding_ZERO); let unsigned : boolean = TRUE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wd&gt;</td>
<td><span id="WdOrWZR"></span>
<p>Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Hn&gt;</td>
<td><span id="Hn__2"></span>
<p>Is the 16-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;fbits&gt;</td>
<td><span id="fbits__2"></span>
<p>For the "Double-precision to 32-bit", "Half-precision to 32-bit", and "Single-precision to 32-bit" variants: is the number of bits after the binary point in the fixed-point destination, in the range 1 to 32, encoded as 64 minus "scale".</p></td>
</tr>
<tr>
<td></td>
<td><span id="fbits__4"></span>
<p>For the "Double-precision to 64-bit", "Half-precision to 64-bit", and "Single-precision to 64-bit" variants: is the number of bits after the binary point in the fixed-point destination, in the range 1 to 64, encoded as 64 minus "scale".</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR__6"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sn&gt;</td>
<td><span id="Sn"></span>
<p>Is the 32-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dn&gt;</td>
<td><span id="Dn"></span>
<p>Is the 64-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let fltval : bits(decode_fltsize) = V{}(n); [X](shared_pseudocode.html#accessor_X_2){intsize}(d) = [FPToFixed](shared_pseudocode.html#func_FPToFixed_7){intsize, decode_fltsize}(fltval, fracbits, unsigned, FPCR(), rounding);

</div>

### Operational information

If FEAT_SME is implemented and the PE is in Streaming SVE mode, then any subsequent instruction which is dependent on the general-purpose register written by this instruction might be significantly delayed.
