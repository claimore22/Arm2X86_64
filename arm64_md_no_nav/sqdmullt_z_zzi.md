## SQDMULLT (indexed)

Signed saturating doubling multiply by indexed element (top)

This instruction multiplies the odd-numbered signed elements within each 128-bit segment of the first source vector by the specified element in the corresponding second source vector segment, and then doubles and places the results in overlapping double-width elements of the destination vector register. Each result element is saturated to the double-width N-bit element's signed integer range -2<sup>(N-1)</sup> to (2<sup>(N-1)</sup>)-1.

The elements within the second source vector are specified using an immediate index that selects the same element position within each 128-bit vector segment. The index range is from 0 to one less than the number of elements per 128-bit segment.

It has encodings from 2 classes: [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

### <span id="iclass_32_bit"></span>32-bit<span style="font-size:smaller;">\
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
<td colspan="2" class="lr">i3h</td>
<td colspan="3" class="lr">Zm</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">i3l</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
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
<td></td>
<td class="droppedname">T</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="sqdmullt_z_zzi_s"></span>

SQDMULLT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").S, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").H, [\<Zm\>](#Zm__4 "For the "32-bit" variant: is the name of the second source scalable vector register Z0-Z7, encoded in the "Zm" field.").H\[[\<imm\>](#imm__78 "For the "32-bit" variant: is the element index, in the range 0 to 7, encoded in the "i3h:i3l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 16; let index : integer = UInt(i3h::i3l); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let sel : integer = 1;

### <span id="iclass_64_bit"></span>64-bit<span style="font-size:smaller;">\
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
<td class="lr">i2h</td>
<td colspan="4" class="lr">Zm</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">i2l</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
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
<td></td>
<td class="droppedname">T</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="sqdmullt_z_zzi_d"></span>

SQDMULLT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").D, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").S, [\<Zm\>](#Zm__2 "For the "64-bit" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").S\[[\<imm\>](#imm__88 "For the "64-bit" variant: is the element index, in the range 0 to 3, encoded in the "i2h:i2l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let index : integer = UInt(i2h::i2l); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let sel : integer = 1;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd&gt;</td>
<td><span id="Zd"></span>
<p>Is the name of the destination scalable vector register, encoded in the "Zd" field.</p></td>
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
<p>For the "32-bit" variant: is the name of the second source scalable vector register Z0-Z7, encoded in the "Zm" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zm__2"></span>
<p>For the "64-bit" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__78"></span>
<p>For the "32-bit" variant: is the element index, in the range 0 to 7, encoded in the "i3h:i3l" fields.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__88"></span>
<p>For the "64-bit" variant: is the element index, in the range 0 to 3, encoded in the "i2h:i2l" fields.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (2 \* esize); let eltspersegment : integer = 128 DIV (2 \* esize); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let s : integer = e - (e MOD eltspersegment); let element1 : integer = SInt(operand1\[(2 \* e + sel)\*:esize\]); let element2 : integer = SInt(operand2\[(2 \* s + index)\*:esize\]); let res : integer = 2 \* element1 \* element2; result\[e\*:(2\*esize)\] = [SignedSat](shared_pseudocode.html#func_SignedSat_2){2\*esize}(res); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>
