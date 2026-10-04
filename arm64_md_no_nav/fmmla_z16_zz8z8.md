## FMMLA (widening, FP8 to FP16)

8-bit floating-point matrix multiply-accumulate to half-precision

This instruction multiplies the 2×4 matrix of 8-bit floating-point values held in each 64-bit segment of the first source vector by the 4×2 matrix of 8-bit floating-point values in the corresponding segment of the second source vector. The intermediate half-precision results calculated as a fused sum-of-products using four adjacent 8-bit floating-point values from each of the source vectors, are scaled by 2<sup>-UInt(FPMR.LSCALE\[3:0\])</sup>, before being destructively added without intermediate rounding to the 2×2 half-precision matrix in the corresponding segment of the destination vector. This is equivalent to accumulating 4-way dot product per destination element.

The 8-bit floating-point encoding format for the elements of the first source vector and the second source vector is selected by FPMR.F8S1 and FPMR.F8S2 respectively.

This instruction is unpredicated.

This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

### <span id="iclass_sve2"></span>SVE2<span style="font-size:smaller;">\
(FEAT_SVE2 && FEAT_F8F16MM) </span>

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
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zda</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td class="droppedname">op</td>
<td></td>
<td colspan="5"></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmmla_z16_zz8z8_"></span>

FMMLA [\<Zda\>](#Zda "Is the name of the third source and destination scalable vector register, encoded in the "Zda" field.").H, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").B, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2) \|\| !IsFeatureImplemented(FEAT_F8F16MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

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
<td><span id="Zm"></span>
<p>Is the name of the second source scalable vector register, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 64; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for s = 0 to segments-1 do let op1 : bits(64) = operand1\[s\*:(64)\]; let op2 : bits(64) = operand2\[s\*:(64)\]; let addend : bits(64) = operand3\[s\*:(64)\]; let way : integer{} = 4; result\[s\*:(64)\] = [FP8MatMulAddFP](shared_pseudocode.html#func_FP8MatMulAddFP_7){64}(addend, op1, op2, way, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;

</div>

### Operational information

This instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> must be unpredicated.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
