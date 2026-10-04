## USMLALL (multiple and indexed vector)

Multi-vector unsigned by signed 8-bit integer multiply-add by indexed element to 32-bit integer

This instruction multiplies each unsigned 8-bit element in the one, two, or four first source vectors by each signed 8-bit indexed element of the second source vector, widens each product to 32 bits, and destructively adds these values to the corresponding 32-bit elements of the ZA quad-vector groups.

The elements within the second source vector are specified using an immediate element index that selects the same element position within each 128-bit vector segment. The element index range is from 0 to one less than the number of elements per 128-bit segment.

The quad-vector group within all of, each half of, or each quarter of the ZA array is selected by the sum of the vector select register and offset range, modulo all, half, or quarter the number of ZA array vectors.

The vector group symbol, VGx2 or VGx4, indicates that the ZA operand consists of two or four ZA quad-vector groups respectively. The vector group symbol is preferred for disassembly, but optional in assembler source code.

This instruction is unpredicated.

It has encodings from 3 classes: [One ZA quad-vector](#iclass_one_za_quad_vector) , [Two ZA quad-vectors](#iclass_two_za_quad_vectors) and [Four ZA quad-vectors](#iclass_four_za_quad_vectors)

### <span id="iclass_one_za_quad_vector"></span>One ZA quad-vector<span style="font-size:smaller;">\
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
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">i4h</td>
<td colspan="2" class="lr">Rv</td>
<td colspan="3" class="lr">i4l</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
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
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">U</td>
<td class="droppedname">S</td>
<td class="droppedname">op</td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="usmlall_za_zzi_s"></span>

USMLALL ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__3 "For the "One ZA quad-vector" variant: is the first vector select offset, encoded as "off2" field times 4."):[\<offs4\>](#offs4 "For the "One ZA quad-vector" variant: is the fourth vector select offset, encoded as "off2" field times 4 plus 3.")\], [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").B, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").B\[[\<index\>](#index__24 "For the "Four ZA quad-vectors of 32-bit elements", "One ZA quad-vector of 32-bit elements", and "Two ZA quad-vectors of 32-bit elements" variants: is the element index, in the range 0 to 15, encoded in the "i4h:i4l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off2::'00'); let index : integer = UInt(i4h::i4l); let nreg : integer{} = 1;

### <span id="iclass_two_za_quad_vectors"></span>Two ZA quad-vectors<span style="font-size:smaller;">\
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
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="lr">0</td>
<td colspan="2" class="lr">i4h</td>
<td colspan="4" class="lr">Zn</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">i4l</td>
<td class="lr">o1</td>
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
<td class="droppedname">op</td>
<td class="droppedname">U</td>
<td class="droppedname">S</td>
<td colspan="2"></td>
<td></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="usmlall_za_zzi_s2xi"></span>

USMLALL ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__4 "For the "Four ZA quad-vectors" and "Two ZA quad-vectors" variants: is the first vector select offset, encoded as "o1" field times 4."):[\<offs4\>](#offs4__2 "For the "Four ZA quad-vectors" and "Two ZA quad-vectors" variants: is the fourth vector select offset, encoded as "o1" field times 4 plus 3."){, VGx2}\], { [\<Zn1\>](#Zn1__2 "For the "Two ZA quad-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 2.").B-[\<Zn2\>](#Zn2__2 "Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" times 2 plus 1.").B }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").B\[[\<index\>](#index__24 "For the "Four ZA quad-vectors of 32-bit elements", "One ZA quad-vector of 32-bit elements", and "Two ZA quad-vectors of 32-bit elements" variants: is the element index, in the range 0 to 15, encoded in the "i4h:i4l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn::'0'); let m : integer = UInt('0'::Zm); let offset : integer = UInt(o1::'00'); let index : integer = UInt(i4h::i4l); let nreg : integer{} = 2;

### <span id="iclass_four_za_quad_vectors"></span>Four ZA quad-vectors<span style="font-size:smaller;">\
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
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">1</td>
<td colspan="2" class="lr">Rv</td>
<td class="lr">0</td>
<td colspan="2" class="lr">i4h</td>
<td colspan="3" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">i4l</td>
<td class="lr">o1</td>
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
<td></td>
<td class="droppedname">op</td>
<td class="droppedname">U</td>
<td class="droppedname">S</td>
<td colspan="2"></td>
<td></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="usmlall_za_zzi_s4xi"></span>

USMLALL ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__4 "For the "Four ZA quad-vectors" and "Two ZA quad-vectors" variants: is the first vector select offset, encoded as "o1" field times 4."):[\<offs4\>](#offs4__2 "For the "Four ZA quad-vectors" and "Two ZA quad-vectors" variants: is the fourth vector select offset, encoded as "o1" field times 4 plus 3."){, VGx4}\], { [\<Zn1\>](#Zn1__3 "For the "Four ZA quad-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 4.").B-[\<Zn4\>](#Zn4__2 "Is the name of the fourth scalable vector register of the first source multi-vector group, encoded as "Zn" times 4 plus 3.").B }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").B\[[\<index\>](#index__24 "For the "Four ZA quad-vectors of 32-bit elements", "One ZA quad-vector of 32-bit elements", and "Two ZA quad-vectors of 32-bit elements" variants: is the element index, in the range 0 to 15, encoded in the "i4h:i4l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn::'00'); let m : integer = UInt('0'::Zm); let offset : integer = UInt(o1::'00'); let index : integer = UInt(i4h::i4l); let nreg : integer{} = 4;

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
<td><span id="offs1__3"></span>
<p>For the "One ZA quad-vector" variant: is the first vector select offset, encoded as "off2" field times 4.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs1__4"></span>
<p>For the "Four ZA quad-vectors" and "Two ZA quad-vectors" variants: is the first vector select offset, encoded as "o1" field times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;offs4&gt;</td>
<td><span id="offs4"></span>
<p>For the "One ZA quad-vector" variant: is the fourth vector select offset, encoded as "off2" field times 4 plus 3.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs4__2"></span>
<p>For the "Four ZA quad-vectors" and "Two ZA quad-vectors" variants: is the fourth vector select offset, encoded as "o1" field times 4 plus 3.</p></td>
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
<td><span id="index__24"></span>
<p>Is the element index, in the range 0 to 15, encoded in the "i4h:i4l" fields.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__2"></span>
<p>For the "Two ZA quad-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 2.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zn1__3"></span>
<p>For the "Four ZA quad-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 4.</p></td>
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

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV nreg; let eltspersegment : integer = 128 DIV esize; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; var result : bits(VL); vec = vec - (vec MOD 4); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(n+r); let operand2 : bits(VL) = Z{}(m); for i = 0 to 3 do let operand3 : bits(VL) = ZAvector{}(vec + i); for e = 0 to elements-1 do let segmentbase : integer = e - (e MOD eltspersegment); let s : integer = 4 \* segmentbase + index; let element1 : integer = UInt(operand1\[(4 \* e + i)\*:(esize DIV 4)\]); let element2 : integer = SInt(operand2\[s\*:(esize DIV 4)\]); let product : bits(esize) = (element1 \* element2)\[esize-1:0\]; result\[e\*:esize\] = operand3\[e\*:esize\] + product; end; [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec + i) = result; end; vec = vec + vstride; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
