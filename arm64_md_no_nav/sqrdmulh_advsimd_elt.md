## SQRDMULH (by element)

Signed saturating rounding doubling multiply returning high half (by element)

This instruction multiplies each vector element in the first source SIMD&FP register by the specified vector element of the second source SIMD&FP register, doubles the results, places the most significant half of the final results into a vector, and writes the vector to the destination SIMD&FP register.

The results are rounded. For truncated results, see <span class="armarm-xref" data-linkend="ARMARM_A64.instructions.SQDMULH_advsimd_elt">SQDMULH</span>.

If any of the results overflows, they are saturated. If saturation occurs, the cumulative saturation bit FPSR.QC is set.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

It has encodings from 2 classes: [Scalar](#iclass_scalar) and [Vector](#iclass_vector)

### <span id="iclass_scalar"></span>Scalar<span style="font-size:smaller;">\
(FEAT_AdvSIMD) </span>

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
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">L</td>
<td class="lr">M</td>
<td colspan="4" class="lr">Rm</td>
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">H</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td colspan="4"></td>
<td colspan="3"></td>
<td class="droppedname">op</td>
<td></td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SQRDMULH_asisdelem_R"></span>

SQRDMULH [\<V\>](#V_option__8 "Is a width specifier, ")[\<d\>](#d "Is the number of the SIMD&FP destination register, encoded in the "Rd" field."), [\<V\>](#V_option__8 "Is a width specifier, ")[\<n\>](#n__2 "Is the number of the first SIMD&FP source register, encoded in the "Rn" field."), V[\<m\>](#Vm__5 "Is the number of the second SIMD&FP source register, ").[\<Ts\>](#Ts_option__5 "Is an element size specifier, ")\[[\<index\>](#index_option "Is the element index, ")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let idxdsize : integer{} = 64 \<\< UInt(H); var index : integer; var Rmhi : bit; case size of when '01' =\> index = UInt(H::L::M); Rmhi = '0'; when '10' =\> index = UInt(H::L); Rmhi = M; otherwise =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let m : integer = UInt(Rmhi::Rm); let esize : integer{} = 8 \<\< UInt(size); let datasize : integer{} = esize; let elements : integer = 1; let round : boolean = TRUE;

### <span id="iclass_vector"></span>Vector<span style="font-size:smaller;">\
(FEAT_AdvSIMD) </span>

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
<td class="lr">Q</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">L</td>
<td class="lr">M</td>
<td colspan="4" class="lr">Rm</td>
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">H</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td colspan="4"></td>
<td colspan="3"></td>
<td class="droppedname">op</td>
<td></td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SQRDMULH_asimdelem_R"></span>

SQRDMULH [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__18 "Is an arrangement specifier, "), [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").[\<T\>](#T_option__18 "Is an arrangement specifier, "), V[\<m\>](#Vm__5 "Is the number of the second SIMD&FP source register, ").[\<Ts\>](#Ts_option__5 "Is an element size specifier, ")\[[\<index\>](#index_option "Is the element index, ")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let idxdsize : integer{} = 64 \<\< UInt(H); var index : integer; var Rmhi : bit; case size of when '01' =\> index = UInt(H::L::M); Rmhi = '0'; when '10' =\> index = UInt(H::L); Rmhi = M; otherwise =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let m : integer = UInt(Rmhi::Rm); let esize : integer{} = 8 \<\< UInt(size); let datasize : integer{} = 64 \<\< UInt(Q); let elements : integer = datasize DIV esize; let round : boolean = TRUE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;V&gt;</td>
<td><span id="V_option__8"></span>
<p>Is a width specifier, encoded in “size”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;V&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">RESERVED</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;d&gt;</td>
<td><span id="d"></span>
<p>Is the number of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;n&gt;</td>
<td><span id="n__2"></span>
<p>Is the number of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;m&gt;</td>
<td><span id="Vm__5"></span>
<p>Is the number of the second SIMD&amp;FP source register, encoded in “(size :: M :: Rm)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;m&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">UInt('0' :: Rm)</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">UInt(M :: Rm)</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">RESERVED</td>
</tr>
</tbody>
</table>
Restricted to 0-15 when element size &lt;Ts&gt; is H.</td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Ts&gt;</td>
<td><span id="Ts_option__5"></span>
<p>Is an element size specifier, encoded in “size”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;Ts&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">RESERVED</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;index&gt;</td>
<td><span id="index_option"></span>
<p>Is the element index, encoded in “(size :: H :: L :: M)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;index&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">UInt(H :: L :: M)</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">UInt(H :: L)</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">RESERVED</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vd&gt;</td>
<td><span id="Vd"></span>
<p>Is the name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T_option__18"></span>
<p>Is an arrangement specifier, encoded in “(size :: Q)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="bitfield">x</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="bitfield">0</td>
<td class="symbol">4H</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="bitfield">1</td>
<td class="symbol">8H</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="bitfield">0</td>
<td class="symbol">2S</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="bitfield">1</td>
<td class="symbol">4S</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="bitfield">x</td>
<td class="symbol">RESERVED</td>
</tr>
</tbody>
</table></td>
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

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(idxdsize) = V{}(m); var result : bits(datasize); var element1 : integer; var element2 : integer; var product : integer; var sat : boolean; element2 = SInt(operand2\[index\*:esize\]); for e = 0 to elements-1 do element1 = SInt(operand1\[e\*:esize\]); product = 2 \* element1 \* element2; product = [RShr](shared_pseudocode.html#func_RShr_3)(product, esize, round); // The following only saturates if element1 and element2 equal -(2^(esize-1)) (result\[e\*:esize\], sat) = [SignedSatQ](shared_pseudocode.html#func_SignedSatQ_2){esize}(product); if sat then FPSR().QC = '1'; end; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
