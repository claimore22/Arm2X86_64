## FMOV (vector, immediate)

Floating-point move immediate (vector)

This instruction copies an immediate floating-point constant into every element of the SIMD&FP destination register.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

It has encodings from 2 classes: [Half-precision](#iclass_half_precision) and [Single-precision and double-precision](#iclass_single_precision_and_double_precision)

### <span id="iclass_half_precision"></span>Half-precision<span style="font-size:smaller;">\
(FEAT_AdvSIMD && FEAT_FP16) </span>

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
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">a</td>
<td class="lr">b</td>
<td class="lr">c</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">d</td>
<td class="lr">e</td>
<td class="lr">f</td>
<td class="lr">g</td>
<td class="lr">h</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">op</td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td></td>
<td></td>
<td colspan="4" class="droppedname">cmode</td>
<td class="droppedname">o2</td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="FMOV_asimdimm_H_h"></span>

FMOV [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__4 "For the "Half-precision" variant: is an arrangement specifier, "), \#[\<imm\>](#a_b_c_d_e_f_g_h__2 "Is a signed floating-point constant with 3-bit exponent and normalized 4 bits of precision, encoded in "a:b:c:d:e:f:g:h". For details of the range of constants available and the encoding of <imm>, see x[Modified immediate constants in A64 floating-point instructions](CJAFAFAI).")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) \|\| !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let rd : integer = UInt(Rd); let datasize : integer{} = 64 \<\< UInt(Q); let imm8 : bits(8) = a::b::c::d::e::f::g::h; let imm16 : bits(16) = imm8\[7\]::NOT(imm8\[6\])::Replicate{2}(imm8\[6\])::imm8\[5:0\]::Zeros{6}; let imm : bits(datasize) = Replicate{}(imm16);

### <span id="iclass_single_precision_and_double_precision"></span>Single-precision and double-precision<span style="font-size:smaller;">\
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
<td class="lr">op</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">a</td>
<td class="lr">b</td>
<td class="lr">c</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">d</td>
<td class="lr">e</td>
<td class="lr">f</td>
<td class="lr">g</td>
<td class="lr">h</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td></td>
<td colspan="10"></td>
<td></td>
<td></td>
<td></td>
<td colspan="4" class="droppedname">cmode</td>
<td class="droppedname">o2</td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Single-precision variant

<span id="FMOV_asimdimm_S_s"></span> Applies when <span class="bitdiff"> (op == 0)</span>

FMOV [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__8 "For the "Single-precision" variant: is an arrangement specifier, "), \#[\<imm\>](#a_b_c_d_e_f_g_h__2 "Is a signed floating-point constant with 3-bit exponent and normalized 4 bits of precision, encoded in "a:b:c:d:e:f:g:h". For details of the range of constants available and the encoding of <imm>, see x[Modified immediate constants in A64 floating-point instructions](CJAFAFAI).")

</div>

<div class="encoding">

#### Encoding for the Double-precision variant

<span id="FMOV_asimdimm_D2_d"></span> Applies when <span class="bitdiff"> (Q == 1 && op == 1)</span>

FMOV [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").2D, \#[\<imm\>](#a_b_c_d_e_f_g_h__2 "Is a signed floating-point constant with 3-bit exponent and normalized 4 bits of precision, encoded in "a:b:c:d:e:f:g:h". For details of the range of constants available and the encoding of <imm>, see x[Modified immediate constants in A64 floating-point instructions](CJAFAFAI).")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if cmode::op == '11111' then // FMOV Dn,#imm is in main FP instruction set if Q == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; end; let rd : integer = UInt(Rd); let datasize : integer{} = 64 \<\< UInt(Q); let imm64 : bits(64) = [AdvSIMDExpandImm](shared_pseudocode.html#func_AdvSIMDExpandImm_3)(op, cmode, a::b::c::d::e::f::g::h); let imm : bits(datasize) = Replicate{}(imm64);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

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
<td><span id="T_option__4"></span>
<p>For the "Half-precision" variant: is an arrangement specifier, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">4H</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">8H</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="T_option__8"></span>
<p>For the "Single-precision" variant: is an arrangement specifier, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">2S</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">4S</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="a_b_c_d_e_f_g_h__2"></span>
<p>Is a signed floating-point constant with 3-bit exponent and normalized 4 bits of precision, encoded in "a:b:c:d:e:f:g:h". For details of the range of constants available and the encoding of &lt;imm&gt;, see <span class="armarm-xref" data-linkend="CJAFAFAI">Modified immediate constants in A64 floating-point instructions</span>.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); [V](shared_pseudocode.html#accessor_V_2){datasize}(rd) = imm;

</div>
