## SHA1M

SHA1 hash update (majority)

SHA1 hash update (majority).

### <span id="iclass_advanced_simd"></span>Advanced SIMD<span style="font-size:smaller;">\
(FEAT_SHA1) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="4"></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td colspan="5"></td>
<td></td>
<td colspan="3" class="droppedname">opcode</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SHA1M_QSV_cryptosha3"></span>

SHA1M [\<Qd\>](#Qd "Is the 128-bit name of the SIMD&FP source and destination, encoded in the "Rd" field."), [\<Sn\>](#Sn__2 "Is the 32-bit name of the second SIMD&FP source register, encoded in the "Rn" field."), [\<Vm\>](#Vm__7 "Is the name of the third SIMD&FP source register, encoded in the "Rm" field.").4S

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SHA1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Qd&gt;</td>
<td><span id="Qd"></span>
<p>Is the 128-bit name of the SIMD&amp;FP source and destination, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sn&gt;</td>
<td><span id="Sn__2"></span>
<p>Is the 32-bit name of the second SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vm&gt;</td>
<td><span id="Vm__7"></span>
<p>Is the name of the third SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); var x : bits(128) = V{}(d); var y : bits(32) = V{}(n); // Note: 32 not 128 bits wide let w : bits(128) = V{}(m); for e = 0 to 3 do let t : bits(32) = [SHAmajority](shared_pseudocode.html#func_SHAmajority_4){32}(x\[63:32\], x\[95:64\], x\[127:96\]); y = y + ROL(x\[31:0\], 5) + t + w\[e\*:32\]; x\[63:32\] = ROL(x\[63:32\], 30); let yx : bits(160) = ROL(y::x, 32); (y, x) = (yx\[159:128\], yx\[127:0\]); end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = x;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
