## BFCVT

Single-precision convert to BFloat16 (scalar)

This instruction converts the single-precision floating-point value in the 32-bit SIMD&FP source register to BFloat16 format and writes the result in the 16-bit SIMD&FP destination register.

ID_AA64ISAR1_EL1.BF16 indicates whether this instruction is supported.

### <span id="iclass_single_precision_to_bfloat16"></span>Single-precision to BFloat16<span style="font-size:smaller;">\
(FEAT_BF16) </span>

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td class="droppedname">M</td>
<td></td>
<td class="droppedname">S</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">ftype</td>
<td></td>
<td colspan="6" class="droppedname">opcode</td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="BFCVT_BS_floatdp1"></span>

BFCVT [\<Hd\>](#Hd "Is the 16-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Sn\>](#Sn "Is the 32-bit name of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_BF16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Hd&gt;</td>
<td><span id="Hd"></span>
<p>Is the 16-bit name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sn&gt;</td>
<td><span id="Sn"></span>
<p>Is the 32-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let operand : bits(32) = V{}(n); let merge : boolean = [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()); var result : bits(128) = if merge then [V](shared_pseudocode.html#accessor_V_2){128}(d) else Zeros{128}; result\[0+:16\] = [FPConvertBF](shared_pseudocode.html#func_FPConvertBF_2)(operand, FPCR()); [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>
