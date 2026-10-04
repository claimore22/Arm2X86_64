## BFMMLA (widening)

BFloat16 matrix multiply-accumulate to single-precision

This instruction multiplies the 2x4 matrix of BFloat16 values held in the first source vector by the 4x2 matrix of BFloat16 values in the second source vector.

If FEAT_EBF16 is not implemented or FPCR.EBF is 0, this instruction:

- Calculates the intermediate single-precision results as a sum of two unfused sum-of-products using two pairs of adjacent BFloat16 values from each of the source vectors, and then rounds the results before accumulation into the 2x2 matrix of single-precision results in the destination vector. This is equivalent to accumulating a 4-way dot product per destination element.
- Uses the non-IEEE 754 Round-to-Odd rounding mode, which forces bit 0 of an inexact result to 1, and rounds an overflow to an appropriately signed Infinity.
- Flushes denormalized inputs and results to zero, as if FPCR.{FZ, FIZ} is {1, 1}.
- Honors FPCR.AH when generating a default NaN result, otherwise disables alternative floating point behaviors, as if FPCR.AH is 0.

If FEAT_EBF16 is implemented and FPCR.EBF is 1, this instruction:

- Calculates the intermediate single-precision results as a sum of two fused sum-of-products using two pairs of adjacent BFloat16 values from each of the source vectors, and then rounds the results before accumulation into the 2x2 matrix of single-precision results in the destination vector. This is equivalent to accumulating a 4-way dot product per destination element.
- Follows all other floating-point behaviors that apply to single-precision arithmetic, as governed by FPCR.RMode, FPCR.FZ, FPCR.AH, and FPCR.FIZ.

Irrespective of FEAT_EBF16 and FPCR.EBF, this instruction:

- Does not modify the cumulative FPSR exception bits (IDC, IXC, UFC, OFC, DZC, and IOC).
- Disables trapped floating-point exceptions, as if the FPCR trap enable bits (IDE, IXE, UFE, OFE, DZE, and IOE) are all zero.
- Generates only the default NaN, as if FPCR.DN is 1.

ID_AA64ISAR1_EL1.BF16 indicates whether this instruction is supported.

### <span id="iclass_advanced_simd"></span>Advanced SIMD<span style="font-size:smaller;">\
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
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td class="droppedname">Q</td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td colspan="5"></td>
<td></td>
<td colspan="4" class="droppedname">opcode</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="BFMMLA_asimdsame2_E"></span>

BFMMLA [\<Vd\>](#Vd__3 "Is the name of the SIMD&FP third source and destination register, encoded in the "Rd" field.").4S, [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").8H, [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").8H

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_BF16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Vd&gt;</td>
<td><span id="Vd__3"></span>
<p>Is the name of the SIMD&amp;FP third source and destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vn&gt;</td>
<td><span id="Vn__2"></span>
<p>Is the name of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vm&gt;</td>
<td><span id="Vm"></span>
<p>Is the name of the second SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let op1 : bits(128) = V{}(n); let op2 : bits(128) = V{}(m); let acc : bits(128) = V{}(d); [V](shared_pseudocode.html#accessor_V_2){128}(d) = [BFMatMulAddH](shared_pseudocode.html#func_BFMatMulAddH_4)(acc, op1, op2, FPCR());

</div>

### Operational information

Arm expects that the BFMMLA instruction will deliver a peak BFloat16 multiply throughput that is at least as high as can be achieved using two BFDOT (vector) instructions, with a goal that it should have significantly higher throughput.
