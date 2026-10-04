## FMLALL (multiple and single vector)

Multi-vector 8-bit floating-point multiply-add by vector to single-precision

This instruction widens all 8-bit floating-point elements in the one, two, or four first source vectors and the second source vector to single-precision format and multiplies the corresponding elements. The intermediate products are scaled by 2<sup>-UInt(FPMR.LSCALE)</sup> before being destructively added without intermediate rounding to the overlapping 32-bit single-precision elements of the ZA quad-vector groups.

The quad-vector group within all of, each half of, or each quarter of the ZA array is selected by the sum of the vector select register and offset range, modulo all, half, or quarter the number of ZA array vectors.

The vector group symbol, VGx2 or VGx4, indicates that the ZA operand consists of two or four ZA quad-vector groups respectively. The vector group symbol is preferred for disassembly, but optional in assembler source code.

The 8-bit floating-point encoding format for the elements of the first source vector and the second source vector is selected by FPMR.F8S1 and FPMR.F8S2 respectively.

This instruction is unpredicated.

It has encodings from 3 classes: [One ZA quad-vector](#iclass_one_za_quad_vector) , [Two ZA quad-vectors](#iclass_two_za_quad_vectors) and [Four ZA quad-vectors](#iclass_four_za_quad_vectors)

### <span id="iclass_one_za_quad_vector"></span>One ZA quad-vector<span style="font-size:smaller;">\
(FEAT_SME_F8F32) </span>

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
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Zn</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="2" class="lr">off2</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmlall_za32_z8z8v_1"></span>

FMLALL ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__3 "For the "One ZA quad-vector" variant: is the first vector select offset, encoded as "off2" field times 4."):[\<offs4\>](#offs4 "For the "One ZA quad-vector" variant: is the fourth vector select offset, encoded as "off2" field times 4 plus 3.")\], [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").B, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_F8F32) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off2::'00'); let nreg : integer{} = 1;

### <span id="iclass_two_za_quad_vectors"></span>Two ZA quad-vectors<span style="font-size:smaller;">\
(FEAT_SME_F8F32) </span>

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
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">o1</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmlall_za32_z8z8v_2x1"></span>

FMLALL ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__4 "For the "Four ZA quad-vectors" and "Two ZA quad-vectors" variants: is the first vector select offset, encoded as "o1" field times 4."):[\<offs4\>](#offs4__2 "For the "Four ZA quad-vectors" and "Two ZA quad-vectors" variants: is the fourth vector select offset, encoded as "o1" field times 4 plus 3."){, VGx2}\], { [\<Zn1\>](#Zn1 "Is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn".").B-[\<Zn2\>](#Zn2 "Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" plus 1 modulo 32.").B }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_F8F32) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn); let m : integer = UInt('0'::Zm); let offset : integer = UInt(o1::'00'); let nreg : integer{} = 2;

### <span id="iclass_four_za_quad_vectors"></span>Four ZA quad-vectors<span style="font-size:smaller;">\
(FEAT_SME_F8F32) </span>

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
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">o1</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmlall_za32_z8z8v_4x1"></span>

FMLALL ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__4 "For the "Four ZA quad-vectors" and "Two ZA quad-vectors" variants: is the first vector select offset, encoded as "o1" field times 4."):[\<offs4\>](#offs4__2 "For the "Four ZA quad-vectors" and "Two ZA quad-vectors" variants: is the fourth vector select offset, encoded as "o1" field times 4 plus 3."){, VGx4}\], { [\<Zn1\>](#Zn1 "Is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn".").B-[\<Zn4\>](#Zn4 "Is the name of the fourth scalable vector register of the first source multi-vector group, encoded as "Zn" plus 3 modulo 32.").B }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_F8F32) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn); let m : integer = UInt('0'::Zm); let offset : integer = UInt(o1::'00'); let nreg : integer{} = 4;

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
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1"></span>
<p>Is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn".</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn2&gt;</td>
<td><span id="Zn2"></span>
<p>Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" plus 1 modulo 32.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn4&gt;</td>
<td><span id="Zn4"></span>
<p>Is the name of the fourth scalable vector register of the first source multi-vector group, encoded as "Zn" plus 3 modulo 32.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 32; let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV nreg; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; var result : bits(VL); vec = vec - (vec MOD 4); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}((n+r) MOD 32); let operand2 : bits(VL) = Z{}(m); for i = 0 to 3 do let operand3 : bits(VL) = ZAvector{}(vec + i); for e = 0 to elements-1 do let element1 : bits(8) = operand1\[(4 \* e + i)\*:8\]; let element2 : bits(8) = operand2\[(4 \* e + i)\*:8\]; let element3 : bits(32) = operand3\[e\*:32\]; result\[e\*:32\] = [FP8MulAddFP](shared_pseudocode.html#func_FP8MulAddFP_6){32}(element3, element1, element2, FPCR(), FPMR()); end; [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec + i) = result; end; vec = vec + vstride; end;

</div>
