## FSCALE

Floating-point adjust exponent by vector

This instruction multiplies the floating-point elements of the first source vector by 2.0 to the power of the signed integer values in the corresponding elements of the second source vector, and places the results in the corresponding elements of the destination vector.

It has encodings from 2 classes: [Half-precision](#iclass_half_precision) and [Single-precision and double-precision](#iclass_single_precision_and_double_precision)

### <span id="iclass_half_precision"></span>Half-precision<span style="font-size:smaller;">\
(FEAT_FP8) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
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
<td class="droppedname">a</td>
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

<span id="FSCALE_asimdsamefp16_only"></span>

FSCALE [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__4 "For the "Half-precision" variant: is an arrangement specifier, "), [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").[\<T\>](#T_option__4 "For the "Half-precision" variant: is an arrangement specifier, "), [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").[\<T\>](#T_option__4 "For the "Half-precision" variant: is an arrangement specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 16; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = if Q == '1' then 128 else 64; let elements : integer = datasize DIV esize;

### <span id="iclass_single_precision_and_double_precision"></span>Single-precision and double-precision<span style="font-size:smaller;">\
(FEAT_FP8) </span>

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
<td class="l">1</td>
<td class="r">x</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
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
<td colspan="2" class="droppedname">size</td>
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

<span id="FSCALE_asimdsame_only"></span>

FSCALE [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__19 "For the "Single-precision and double-precision" variant: is an arrangement specifier, "), [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").[\<T\>](#T_option__19 "For the "Single-precision and double-precision" variant: is an arrangement specifier, "), [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").[\<T\>](#T_option__19 "For the "Single-precision and double-precision" variant: is an arrangement specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Q == '0' && size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = if Q == '1' then 128 else 64; let elements : integer = datasize DIV esize;

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
<p>For the "Single-precision and double-precision" variant: is an arrangement specifier, encoded in “(size[0] :: Q)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size[0]</th>
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

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(m); var result : bits(datasize); for e = 0 to elements-1 do let op1 : bits(esize) = operand1\[e\*:esize\]; let op2 : integer = SInt(operand2\[e\*:esize\]); result\[e\*:esize\] = [FPScale](shared_pseudocode.html#func_FPScale_4){esize}(op1, op2, FPCR()); end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;

</div>
