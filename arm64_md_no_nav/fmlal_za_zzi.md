## FMLAL (multiple and indexed vector, FP16 to FP32)

Multi-vector half-precision multiply-add by indexed element to single-precision

This instruction widens all 16-bit half-precision elements in the one, two, or four first source vectors and the indexed element of the second source vector to single-precision format, then multiplies the corresponding elements and destructively adds these values without intermediate rounding to the overlapping 32-bit single-precision elements of the ZA double-vector groups.

The half-precision elements within the second source vector are specified using a 3-bit immediate index that selects the same element position within each 128-bit vector segment.

The double-vector group within all of, each half of, or each quarter of the ZA array is selected by the sum of the vector select register and offset range, modulo all, half, or quarter the number of ZA array vectors.

The vector group symbol, VGx2 or VGx4, indicates that the ZA operand consists of two or four ZA double-vector groups respectively. The vector group symbol is preferred for disassembly, but optional in assembler source code.

This instruction follows SME ZA-targeting floating-point behaviors.

This instruction is unpredicated.

It has encodings from 3 classes: [One ZA double-vector](#iclass_one_za_double_vector) , [Two ZA double-vectors](#iclass_two_za_double_vectors) and [Four ZA double-vectors](#iclass_four_za_double_vectors)

### <span id="iclass_one_za_double_vector"></span>One ZA double-vector<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">i3h</td>
<td colspan="2" class="lr">Rv</td>
<td class="lr">1</td>
<td colspan="2" class="lr">i3l</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="3" class="lr">off3</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmlal_za_zzi_1"></span>

FMLAL ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1 "For the "One ZA double-vector" variant: is the first vector select offset, encoded as "off3" field times 2."):[\<offs2\>](#offs2 "For the "One ZA double-vector" variant: is the second vector select offset, encoded as "off3" field times 2 plus 1.")\], [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").H, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").H\[[\<index\>](#index__18 "For the "Four ZA quad-vectors of 64-bit elements", "One ZA quad-vector of 64-bit elements", and "Two ZA quad-vectors of 64-bit elements" variants: is the element index, in the range 0 to 7, encoded in the "i3h:i3l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off3::'0'); let index : integer = UInt(i3h::i3l); let nreg : integer{} = 1;

### <span id="iclass_two_za_double_vectors"></span>Two ZA double-vectors<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="lr">1</td>
<td colspan="2" class="lr">i3h</td>
<td colspan="4" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">i3l</td>
<td colspan="2" class="lr">off2</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td></td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmlal_za_zzi_2xi"></span>

FMLAL ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the first vector select offset, encoded as "off2" field times 2."):[\<offs2\>](#offs2__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the second vector select offset, encoded as "off2" field times 2 plus 1."){, VGx2}\], { [\<Zn1\>](#Zn1__2 "For the "Two ZA double-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 2.").H-[\<Zn2\>](#Zn2__2 "Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" times 2 plus 1.").H }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").H\[[\<index\>](#index__18 "For the "Four ZA quad-vectors of 64-bit elements", "One ZA quad-vector of 64-bit elements", and "Two ZA quad-vectors of 64-bit elements" variants: is the element index, in the range 0 to 7, encoded in the "i3h:i3l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn::'0'); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off2::'0'); let index : integer = UInt(i3h::i3l); let nreg : integer{} = 2;

### <span id="iclass_four_za_double_vectors"></span>Four ZA double-vectors<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">1</td>
<td colspan="2" class="lr">Rv</td>
<td class="lr">1</td>
<td colspan="2" class="lr">i3h</td>
<td colspan="3" class="lr">Zn</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">i3l</td>
<td colspan="2" class="lr">off2</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td></td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmlal_za_zzi_4xi"></span>

FMLAL ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the first vector select offset, encoded as "off2" field times 2."):[\<offs2\>](#offs2__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the second vector select offset, encoded as "off2" field times 2 plus 1."){, VGx4}\], { [\<Zn1\>](#Zn1__3 "For the "Four ZA double-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 4.").H-[\<Zn4\>](#Zn4__2 "Is the name of the fourth scalable vector register of the first source multi-vector group, encoded as "Zn" times 4 plus 3.").H }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").H\[[\<index\>](#index__18 "For the "Four ZA quad-vectors of 64-bit elements", "One ZA quad-vector of 64-bit elements", and "Two ZA quad-vectors of 64-bit elements" variants: is the element index, in the range 0 to 7, encoded in the "i3h:i3l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn::'00'); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off2::'0'); let index : integer = UInt(i3h::i3l); let nreg : integer{} = 4;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wv&gt;</td>
<td><span id="Wv"></span>
<p>Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;offs1&gt;</td>
<td><span id="offs1"></span>
<p>For the "One ZA double-vector" variant: is the first vector select offset, encoded as "off3" field times 2.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs1__2"></span>
<p>For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the first vector select offset, encoded as "off2" field times 2.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;offs2&gt;</td>
<td><span id="offs2"></span>
<p>For the "One ZA double-vector" variant: is the second vector select offset, encoded as "off3" field times 2 plus 1.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs2__2"></span>
<p>For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the second vector select offset, encoded as "off2" field times 2 plus 1.</p></td>
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
<td><span id="Zm__2"></span>
<p>Is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;index&gt;</td>
<td><span id="index__18"></span>
<p>Is the element index, in the range 0 to 7, encoded in the "i3h:i3l" fields.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__2"></span>
<p>For the "Two ZA double-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 2.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zn1__3"></span>
<p>For the "Four ZA double-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn2&gt;</td>
<td><span id="Zn2__2"></span>
<p>Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" times 2 plus 1.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn4&gt;</td>
<td><span id="Zn4__2"></span>
<p>Is the name of the fourth scalable vector register of the first source multi-vector group, encoded as "Zn" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 32; let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV nreg; let eltspersegment : integer = 128 DIV 32; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; var result : bits(VL); vec = vec - (vec MOD 2); for r = 0 to nreg-1 do let op1 : bits(VL) = Z{}(n+r); let op2 : bits(VL) = Z{}(m); for i = 0 to 1 do let op3 : bits(VL) = ZAvector{}(vec + i); for e = 0 to elements-1 do let segmentbase : integer = e - (e MOD eltspersegment); let s : integer = 2 \* segmentbase + index; let elem1 : bits(16) = op1\[(2 \* e + i)\*:(16)\]; let elem2 : bits(16) = op2\[s\*:(16)\]; let elem3 : bits(32) = op3\[e\*:(32)\]; result\[e\*:(32)\] = [FPMulAddH_ZA](shared_pseudocode.html#func_FPMulAddH_ZA_4)(elem3, elem1, elem2, FPCR()); end; [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec + i) = result; end; vec = vec + vstride; end;

</div>
