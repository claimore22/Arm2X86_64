## FCMLA (indexed)

Floating-point complex multiply-add by indexed element

This instruction multiplies the duplicated real components for rotations 0 and 180, or imaginary components for rotations 90 and 270, of the floating-point complex numbers in each 128-bit segment of the first source vector by the specified complex number in the corresponding the second source vector segment rotated by 0, 90, 180 or 270 degrees in the direction from the positive real axis towards the positive imaginary axis, when considered in polar representation.

This instruction then destructively adds the products to the corresponding components of the complex numbers in the addend and destination vector, without intermediate rounding.

These transformations permit the creation of a variety of multiply-add and multiply-subtract operations on complex numbers by combining two of these instructions with the same vector operands but with rotations that are 90 degrees apart.

Each complex number is represented in a vector register as an even/odd pair of elements with the real part in the even-numbered element and the imaginary part in the odd-numbered element.

The complex numbers within the second source vector are specified using an immediate index that selects the same complex number position within each 128-bit vector segment. The index range is from 0 to one less than the number of complex numbers per 128-bit segment. This instruction is unpredicated.

It has encodings from 2 classes: [Half-precision](#iclass_half_precision) and [Single-precision](#iclass_single_precision)

### <span id="iclass_half_precision"></span>Half-precision<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">i2</td>
<td colspan="3" class="lr">Zm</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="2" class="lr">rot</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zda</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcmla_z_zzzi_h"></span>

FCMLA [\<Zda\>](#Zda "Is the name of the third source and destination scalable vector register, encoded in the "Zda" field.").H, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").H, [\<Zm\>](#Zm__4 "For the "Half-precision" variant: is the name of the second source scalable vector register Z0-Z7, encoded in the "Zm" field.").H\[[\<imm\>](#imm__51 "For the "Half-precision" variant: is the index of a Real and Imaginary pair, in the range 0 to 3, encoded in the "i2" field.")\], [\<const\>](#const__4 "Is the const specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 16; let index : integer = UInt(i2); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let sel_a : integer = UInt(rot\[0\]); let sel_b : integer = UInt(NOT(rot\[0\])); let neg_i : boolean = (rot\[1\] == '1'); let neg_r : boolean = (rot\[0\] != rot\[1\]);

### <span id="iclass_single_precision"></span>Single-precision<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">i1</td>
<td colspan="4" class="lr">Zm</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="2" class="lr">rot</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zda</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td></td>
<td colspan="4"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcmla_z_zzzi_s"></span>

FCMLA [\<Zda\>](#Zda "Is the name of the third source and destination scalable vector register, encoded in the "Zda" field.").S, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").S, [\<Zm\>](#Zm__2 "For the "Single-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").S\[[\<imm\>](#imm__52 "For the "Single-precision" variant: is the index of a Real and Imaginary pair, in the range 0 to 1, encoded in the "i1" field.")\], [\<const\>](#const__4 "Is the const specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let index : integer = UInt(i1); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let sel_a : integer = UInt(rot\[0\]); let sel_b : integer = UInt(NOT(rot\[0\])); let neg_i : boolean = (rot\[1\] == '1'); let neg_r : boolean = (rot\[0\] != rot\[1\]);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zda&gt;</td>
<td><span id="Zda"></span>
<p>Is the name of the third source and destination scalable vector register, encoded in the "Zda" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn&gt;</td>
<td><span id="Zn__2"></span>
<p>Is the name of the first source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zm&gt;</td>
<td><span id="Zm__4"></span>
<p>For the "Half-precision" variant: is the name of the second source scalable vector register Z0-Z7, encoded in the "Zm" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zm__2"></span>
<p>For the "Single-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__51"></span>
<p>For the "Half-precision" variant: is the index of a Real and Imaginary pair, in the range 0 to 3, encoded in the "i2" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__52"></span>
<p>For the "Single-precision" variant: is the index of a Real and Imaginary pair, in the range 0 to 1, encoded in the "i1" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;const&gt;</td>
<td><span id="const__4"></span>
<p>Is the const specifier, encoded in “rot”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">rot</th>
<th class="symbol">&lt;const&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">#0</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">#90</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">#180</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">#270</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let pairs : integer = VL DIV (2 \* esize); let pairspersegment : integer = 128 DIV (2 \* esize); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for p = 0 to pairs-1 do let segmentbase : integer = p - (p MOD pairspersegment); let s : integer = segmentbase + index; var addend_r : bits(esize) = operand3\[(2 \* p + 0)\*:esize\]; var addend_i : bits(esize) = operand3\[(2 \* p + 1)\*:esize\]; let elt1_a : bits(esize) = operand1\[(2 \* p + sel_a)\*:esize\]; var elt2_a : bits(esize) = operand2\[(2 \* s + sel_a)\*:esize\]; var elt2_b : bits(esize) = operand2\[(2 \* s + sel_b)\*:esize\]; if neg_r then elt2_a = [FPNeg](shared_pseudocode.html#func_FPNeg_3){esize}(elt2_a, FPCR()); end; if neg_i then elt2_b = [FPNeg](shared_pseudocode.html#func_FPNeg_3){esize}(elt2_b, FPCR()); end; addend_r = [FPMulAdd](shared_pseudocode.html#func_FPMulAdd_5){esize}(addend_r, elt1_a, elt2_a, FPCR()); addend_i = [FPMulAdd](shared_pseudocode.html#func_FPMulAdd_5){esize}(addend_i, elt1_a, elt2_b, FPCR()); result\[(2 \* p + 0)\*:esize\] = addend_r; result\[(2 \* p + 1)\*:esize\] = addend_i; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;

</div>

### Operational information

This instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> must be unpredicated.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
