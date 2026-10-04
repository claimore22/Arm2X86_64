## BF1CVT, BF2CVT

Multi-vector 8-bit floating-point convert to BFloat16

This instruction converts each 8-bit floating-point element of the source vector to BFloat16 while downscaling the value, and places the results in the corresponding 16-bit elements of the destination vectors. BF1CVT scales the values by 2<sup>-UInt(FPMR.LSCALE\[5:0\])</sup>. BF2CVT scales the values by 2<sup>-UInt(FPMR.LSCALE2\[5:0\])</sup>. The 8-bit floating-point encoding format for BF1CVT is selected by FPMR.F8S1. The 8-bit floating-point encoding format for BF2CVT is selected by FPMR.F8S2.

This instruction is unpredicated.

It has encodings from 2 classes: [BF1CVT](#iclass_bf1cvt) and [BF2CVT](#iclass_bf2cvt)

### <span id="iclass_bf1cvt"></span>BF1CVT<span style="font-size:smaller;">\
(FEAT_SME2 && FEAT_FP8) </span>

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
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="4" class="lr">Zd</td>
<td class="lr">0</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="4"></td>
<td class="droppedname">L</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="bf1cvt_mz2_z8_"></span>

BF1CVT { [\<Zd1\>](#Zd1 "For the "Two registers" variant: is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 2.").H-[\<Zd2\>](#Zd2 "For the "Consecutive" variant: is the name of the second scalable vector register of the destination multi-vector group, encoded as "Zd" times 2 plus 1.").H }, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) \|\| !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let d : integer = UInt(Zd:: '0'); let issrc2 : boolean = FALSE;

### <span id="iclass_bf2cvt"></span>BF2CVT<span style="font-size:smaller;">\
(FEAT_SME2 && FEAT_FP8) </span>

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
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="4" class="lr">Zd</td>
<td class="lr">0</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="4"></td>
<td class="droppedname">L</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="bf2cvt_mz2_z8_"></span>

BF2CVT { [\<Zd1\>](#Zd1 "For the "Two registers" variant: is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 2.").H-[\<Zd2\>](#Zd2 "For the "Consecutive" variant: is the name of the second scalable vector register of the destination multi-vector group, encoded as "Zd" times 2 plus 1.").H }, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) \|\| !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let d : integer = UInt(Zd:: '0'); let issrc2 : boolean = TRUE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd1&gt;</td>
<td><span id="Zd1"></span>
<p>Is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 2.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zd2&gt;</td>
<td><span id="Zd2"></span>
<p>Is the name of the second scalable vector register of the destination multi-vector group, encoded as "Zd" times 2 plus 1.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn&gt;</td>
<td><span id="Zn"></span>
<p>Is the name of the source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 8; let operand : bits(VL) = Z{}(n); var result : bits(2\*VL); for e = 0 to elements-1 do let element : bits(8) = operand\[e\*:8\]; result\[e\*:16\] = [FP8ConvertBF](shared_pseudocode.html#func_FP8ConvertBF_4)(element, issrc2, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+0) = result\[VL-1:0\]; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+1) = result\[2\*VL-1:VL\];

</div>
