## CDOT (indexed)

Complex integer dot product by indexed element

This instruction delimits the source vectors into pairs of 8-bit or 16-bit signed integer complex numbers. Within each pair, the complex numbers in the first source vector are multiplied by the corresponding complex numbers in the second source vector and the resulting wide real or wide imaginary part of the product is accumulated into a 32-bit or 64-bit destination vector element that overlaps all four of the elements that comprise a pair of complex number values in the first source vector.

As a result each instruction implicitly deinterleaves the real and imaginary components of their complex number inputs, so that the destination vector accumulates 4×wide real sums or 4×wide imaginary sums.

The complex numbers in the second source vector are rotated by 0, 90, 180 or 270 degrees in the direction from the positive real axis towards the positive imaginary axis, when considered in polar representation, by performing the following transformations prior to the dot product operations:

- If the rotation is \#0, the imaginary parts of the complex numbers in the second source vector are negated. The destination vector therefore accumulates the real parts of a complex dot product.
- If the rotation is \#90, the real and imaginary parts of the complex numbers the second source vector are swapped. The destination vector therefore accumulates the imaginary parts of a complex dot product.
- If the rotation is \#180, there is no transformation. The destination vector therefore accumulates the real parts of a complex conjugate dot product.
- If the rotation is \#270, the real parts of the complex numbers in the second source vector are negated and then swapped with the imaginary parts. The destination vector therefore accumulates the imaginary parts of a complex conjugate dot product.

The indexed form of these instructions select a single pair of complex numbers within each 128-bit segment of the second source vector as the multiplier for all pairs of complex numbers within the corresponding 128-bit segment of the first source vector. The complex number pairs within the second source vector are specified using an immediate index that selects the same complex number pair position within each 128-bit vector segment. The index range is from 0 to one less than the number of complex number pairs per 128-bit segment.

Each complex number is represented in a vector register as an even/odd pair of elements with the real part in the even-numbered element and the imaginary part in the odd-numbered element.

It has encodings from 2 classes: [8-bit to 32-bit](#iclass_8_bit_to_32_bit) and [16-bit to 64-bit](#iclass_16_bit_to_64_bit)

### <span id="iclass_8_bit_to_32_bit"></span>8-bit to 32-bit<span style="font-size:smaller;">\
(FEAT_SVE2 \|\| FEAT_SME) </span>

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
<td class="r">0</td>
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
<td>1</td>
<td>0</td>
<td class="r">0</td>
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

<span id="cdot_z_zzzi_s"></span>

CDOT [\<Zda\>](#Zda "Is the name of the third source and destination scalable vector register, encoded in the "Zda" field.").S, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").B, [\<Zm\>](#Zm__4 "For the "8-bit to 32-bit" variant: is the name of the second source scalable vector register Z0-Z7, encoded in the "Zm" field.").B\[[\<imm\>](#imm__40 "For the "8-bit to 32-bit" variant: is the immediate index of a group of four 8-bit values within each 128-bit vector segment, in the range 0 to 3, encoded in the "i2" field.")\], [\<const\>](#const__4 "Is the const specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let index : integer = UInt(i2); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let sel_a : integer = UInt(rot\[0\]); let sel_b : integer = UInt(NOT(rot\[0\])); let sub_i : boolean = (rot\[0\] == rot\[1\]);

### <span id="iclass_16_bit_to_64_bit"></span>16-bit to 64-bit<span style="font-size:smaller;">\
(FEAT_SVE2 \|\| FEAT_SME) </span>

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
<td class="r">0</td>
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
<td>1</td>
<td>0</td>
<td class="r">0</td>
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

<span id="cdot_z_zzzi_d"></span>

CDOT [\<Zda\>](#Zda "Is the name of the third source and destination scalable vector register, encoded in the "Zda" field.").D, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").H, [\<Zm\>](#Zm__2 "For the "16-bit to 64-bit" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").H\[[\<imm\>](#imm__39 "For the "16-bit to 64-bit" variant: is the immediate index of a group of four 16-bit values within each 128-bit vector segment, in the range 0 to 1, encoded in the "i1" field.")\], [\<const\>](#const__4 "Is the const specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let index : integer = UInt(i1); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let sel_a : integer = UInt(rot\[0\]); let sel_b : integer = UInt(NOT(rot\[0\])); let sub_i : boolean = (rot\[0\] == rot\[1\]);

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
<p>For the "8-bit to 32-bit" variant: is the name of the second source scalable vector register Z0-Z7, encoded in the "Zm" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zm__2"></span>
<p>For the "16-bit to 64-bit" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__40"></span>
<p>For the "8-bit to 32-bit" variant: is the immediate index of a group of four 8-bit values within each 128-bit vector segment, in the range 0 to 3, encoded in the "i2" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__39"></span>
<p>For the "16-bit to 64-bit" variant: is the immediate index of a group of four 16-bit values within each 128-bit vector segment, in the range 0 to 1, encoded in the "i1" field.</p></td>
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

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let eltspersegment : integer = 128 DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do let segmentbase : integer = e - (e MOD eltspersegment); let s : integer = segmentbase + index; var res : bits(esize) = operand3\[e\*:esize\]; for i = 0 to 1 do let elt1_r : integer = SInt(operand1\[(4 \* e + 2 \* i + 0)\*:(esize DIV 4)\]); let elt1_i : integer = SInt(operand1\[(4 \* e + 2 \* i + 1)\*:(esize DIV 4)\]); let elt2_a : integer = SInt(operand2\[(4 \* s + 2 \* i + sel_a)\*:(esize DIV 4)\]); let elt2_b : integer = SInt(operand2\[(4 \* s + 2 \* i + sel_b)\*:(esize DIV 4)\]); if sub_i then res = (res + (elt1_r \* elt2_a)) - (elt1_i \* elt2_b); else res = res + (elt1_r \* elt2_a) + (elt1_i \* elt2_b); end; end; result\[e\*:esize\] = res; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;

</div>

### Operational information

This instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> must be unpredicated.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
