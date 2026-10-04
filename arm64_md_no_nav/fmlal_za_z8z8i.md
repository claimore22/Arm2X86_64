## FMLAL (multiple and indexed vector, FP8 to FP16)

Multi-vector 8-bit floating-point multiply-add by indexed element to half-precision

This instruction widens all 8-bit floating-point elements in the one, two, or four first source vectors and the indexed element of the second source vector to half-precision format and multiplies the corresponding elements. The intermediate products are scaled by 2<sup>-UInt(FPMR.LSCALE\[3:0\])</sup> before being destructively added without intermediate rounding to the overlapping 16-bit half-precision elements of the ZA double-vector groups.

The 8-bit floating-point elements within the second source vector are specified using a 4-bit immediate index that selects the same element position within each 128-bit vector segment.

The double-vector group within all of, each half of, or each quarter of the ZA array is selected by the sum of the vector select register and offset range, modulo all, half, or quarter the number of ZA array vectors.

The vector group symbol, VGx2 or VGx4, indicates that the ZA operand consists of two or four ZA double-vector groups respectively. The vector group symbol is preferred for disassembly, but optional in assembler source code.

The 8-bit floating-point encoding format for the elements of the first source vector and the second source vector is selected by FPMR.F8S1 and FPMR.F8S2 respectively.

This instruction is unpredicated.

It has encodings from 3 classes: [One ZA double-vector](#iclass_one_za_double_vector) , [Two ZA double-vectors](#iclass_two_za_double_vectors) and [Four ZA double-vectors](#iclass_four_za_double_vectors)

### <span id="iclass_one_za_double_vector"></span>One ZA double-vector<span style="font-size:smaller;">\
(FEAT_SME_F8F16) </span>

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
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">i4A</td>
<td colspan="2" class="lr">Rv</td>
<td class="lr">0</td>
<td colspan="2" class="lr">i4B</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">i4C</td>
<td colspan="3" class="lr">off3</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmlal_za_z8z8i_1"></span>

FMLAL ZA.H\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1 "For the "One ZA double-vector" variant: is the first vector select offset, encoded as "off3" field times 2."):[\<offs2\>](#offs2 "For the "One ZA double-vector" variant: is the second vector select offset, encoded as "off3" field times 2 plus 1.")\], [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").B, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").B\[[\<index\>](#index__23 "For the "One ZA double-vector" variant: is the element index, in the range 0 to 15, encoded in the "i4A:i4B:i4C" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_F8F16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off3::'0'); let index : integer = UInt(i4A::i4B::i4C); let nreg : integer{} = 1;

### <span id="iclass_two_za_double_vectors"></span>Two ZA double-vectors<span style="font-size:smaller;">\
(FEAT_SME_F8F16) </span>

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
<td colspan="2" class="lr">i4h</td>
<td colspan="4" class="lr">Zn</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="2" class="lr">i4l</td>
<td colspan="2" class="lr">off2</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmlal_za_z8z8i_2xi"></span>

FMLAL ZA.H\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the first vector select offset, encoded as "off2" field times 2."):[\<offs2\>](#offs2__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the second vector select offset, encoded as "off2" field times 2 plus 1."){, VGx2}\], { [\<Zn1\>](#Zn1__2 "For the "Two ZA double-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 2.").B-[\<Zn2\>](#Zn2__2 "Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" times 2 plus 1.").B }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").B\[[\<index\>](#index__24 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the element index, in the range 0 to 15, encoded in the "i4h:i4l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_F8F16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn::'0'); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off2::'0'); let index : integer = UInt(i4h::i4l); let nreg : integer{} = 2;

### <span id="iclass_four_za_double_vectors"></span>Four ZA double-vectors<span style="font-size:smaller;">\
(FEAT_SME_F8F16) </span>

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
<td colspan="2" class="lr">i4h</td>
<td colspan="3" class="lr">Zn</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="2" class="lr">i4l</td>
<td colspan="2" class="lr">off2</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmlal_za_z8z8i_4xi"></span>

FMLAL ZA.H\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the first vector select offset, encoded as "off2" field times 2."):[\<offs2\>](#offs2__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the second vector select offset, encoded as "off2" field times 2 plus 1."){, VGx4}\], { [\<Zn1\>](#Zn1__3 "For the "Four ZA double-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 4.").B-[\<Zn4\>](#Zn4__2 "Is the name of the fourth scalable vector register of the first source multi-vector group, encoded as "Zn" times 4 plus 3.").B }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").B\[[\<index\>](#index__24 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the element index, in the range 0 to 15, encoded in the "i4h:i4l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_F8F16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn::'00'); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off2::'0'); let index : integer = UInt(i4h::i4l); let nreg : integer{} = 4;

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
<td><span id="index__23"></span>
<p>For the "One ZA double-vector" variant: is the element index, in the range 0 to 15, encoded in the "i4A:i4B:i4C" fields.</p></td>
</tr>
<tr>
<td></td>
<td><span id="index__24"></span>
<p>For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the element index, in the range 0 to 15, encoded in the "i4h:i4l" fields.</p></td>
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

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV nreg; let eltspersegment : integer = 128 DIV 16; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; var result : bits(VL); vec = vec - (vec MOD 2); for r = 0 to nreg-1 do let op1 : bits(VL) = Z{}(n+r); let op2 : bits(VL) = Z{}(m); for i = 0 to 1 do let op3 : bits(VL) = ZAvector{}(vec + i); for e = 0 to elements-1 do let segmentbase : integer = e - (e MOD eltspersegment); let s : integer = 2 \* segmentbase + index; let elem1 : bits(8) = op1\[(2 \* e + i)\*:(8)\]; let elem2 : bits(8) = op2\[s\*:(8)\]; let elem3 : bits(16) = op3\[e\*:(16)\]; result\[e\*:(16)\] = [FP8MulAddFP](shared_pseudocode.html#func_FP8MulAddFP_6){16}(elem3, elem1, elem2, FPCR(), FPMR()); end; [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec + i) = result; end; vec = vec + vstride; end;

</div>
