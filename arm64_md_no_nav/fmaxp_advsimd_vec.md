## FMAXP (vector)

Floating-point maximum pairwise (vector)

This instruction creates a vector by concatenating the vector elements of the first source SIMD&FP register after the vector elements of the second source SIMD&FP register, reads each pair of adjacent vector elements from the concatenated vector, writes the larger of each pair of values into a vector, and writes the vector to the destination SIMD&FP register. All the values in this instruction are floating-point values.

When FPCR.AH is 0, the behavior is as follows for each pairwise operation:

- Negative zero compares less than positive zero.
- When FPCR.DN is 0, if either element is a NaN, the result is a quiet NaN.
- When FPCR.DN is 1, if either element is a NaN, the result is Default NaN.

When FPCR.AH is 1, the behavior is as follows for each pairwise operation:

- If both elements are zeros, regardless of the sign of either zero, the result is the second element.
- If either element is a NaN, regardless of the value of FPCR.DN, the result is the second element.

This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see <span class="armarm-xref" data-linkend="ARMARM_BEIJDDAG">Floating-point exceptions and exception traps</span>.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

It has encodings from 2 classes: [Half-precision](#iclass_half_precision) and [Single-precision and double-precision](#iclass_single_precision_and_double_precision)

### <span id="iclass_half_precision"></span>Half-precision<span style="font-size:smaller;">\
(FEAT_AdvSIMD && FEAT_FP16) </span>

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
<td class="lr">Q</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">o1</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="2"></td>
<td colspan="3" class="droppedname">opcode</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="FMAXP_asimdsamefp16_only"></span>

FMAXP [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__4 "For the "Half-precision" variant: is an arrangement specifier, "), [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").[\<T\>](#T_option__4 "For the "Half-precision" variant: is an arrangement specifier, "), [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").[\<T\>](#T_option__4 "For the "Half-precision" variant: is an arrangement specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) \|\| !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 16; let datasize : integer{} = 64 \<\< UInt(Q); let elements : integer = datasize DIV esize;

### <span id="iclass_single_precision_and_double_precision"></span>Single-precision and double-precision<span style="font-size:smaller;">\
(FEAT_AdvSIMD) </span>

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
<td class="lr">Q</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">sz</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">o1</td>
<td></td>
<td></td>
<td colspan="5"></td>
<td colspan="5" class="droppedname">opcode</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="FMAXP_asimdsame_only"></span>

FMAXP [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__19 "For the "Single-precision and double-precision" variant: is an arrangement specifier, "), [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").[\<T\>](#T_option__19 "For the "Single-precision and double-precision" variant: is an arrangement specifier, "), [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").[\<T\>](#T_option__19 "For the "Single-precision and double-precision" variant: is an arrangement specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sz::Q == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 32 \<\< UInt(sz); let datasize : integer{} = 64 \<\< UInt(Q); let elements : integer = datasize DIV esize;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Vd&gt;</td>
<td><span id="Vd"></span>
<p>Is the name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T_option__4"></span>
<p>For the "Half-precision" variant: is an arrangement specifier, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">4H</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">8H</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="T_option__19"></span>
<p>For the "Single-precision and double-precision" variant: is an arrangement specifier, encoded in “(sz :: Q)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">sz</th>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="bitfield">0</td>
<td class="symbol">2S</td>
</tr>
<tr>
<td class="bitfield">0</td>
<td class="bitfield">1</td>
<td class="symbol">4S</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="bitfield">0</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="bitfield">1</td>
<td class="symbol">2D</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vn&gt;</td>
<td><span id="Vn__2"></span>
<p>Is the name of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vm&gt;</td>
<td><span id="Vm"></span>
<p>Is the name of the second SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(m); var result : bits(datasize); let concat : bits(2\*datasize) = operand2::operand1; var element1 : bits(esize); var element2 : bits(esize); for e = 0 to elements-1 do element1 = concat\[(2\*e)\*:esize\]; element2 = concat\[((2\*e)+1)\*:esize\]; result\[e\*:esize\] = [FPMax](shared_pseudocode.html#func_FPMax_4){esize}(element1, element2, FPCR()); end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;

</div>
