## FCVTPU (vector)

Floating-point convert to unsigned integer, rounding toward plus infinity (vector)

This instruction converts a scalar or each element in a vector from a floating-point value to an unsigned integer value using the Round towards Plus Infinity rounding mode, and writes the result to the SIMD&FP destination register.

This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see <span class="armarm-xref" data-linkend="ARMARM_BEIJDDAG">Floating-point exceptions and exception traps</span>.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

It has encodings from 4 classes: [Scalar half-precision](#iclass_scalar_half_precision) , [Scalar single-precision and double-precision](#iclass_scalar_single_precision_and_double_precision) , [Vector half-precision](#iclass_vector_half_precision) and [Vector single-precision and double-precision](#iclass_vector_single_precision_and_double_precision)

### <span id="iclass_scalar_half_precision"></span>Scalar half-precision<span style="font-size:smaller;">\
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
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">o2</td>
<td colspan="4"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td class="droppedname">o1</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="FCVTPU_asisdmiscfp16_R"></span>

FCVTPU [\<Hd\>](#Hd "Is the 16-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Hn\>](#Hn__2 "Is the 16-bit name of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) \|\| !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 16; let datasize : integer{} = esize; let elements : integer = 1; let rounding : [FPRounding](shared_pseudocode.html#type_FPRounding) = [FPDecodeRounding](shared_pseudocode.html#func_FPDecodeRounding_1)(o1::o2); let unsigned : boolean = TRUE;

### <span id="iclass_scalar_single_precision_and_double_precision"></span>Scalar single-precision and double-precision<span style="font-size:smaller;">\
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
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">sz</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">o2</td>
<td></td>
<td colspan="5"></td>
<td colspan="4"></td>
<td class="droppedname">o1</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="FCVTPU_asisdmisc_R"></span>

FCVTPU [\<V\>](#V_option__9 "Is a width specifier, ")[\<d\>](#d "Is the number of the SIMD&FP destination register, encoded in the "Rd" field."), [\<V\>](#V_option__9 "Is a width specifier, ")[\<n\>](#n__3 "Is the number of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 32 \<\< UInt(sz); let datasize : integer{} = esize; let elements : integer = 1; let rounding : [FPRounding](shared_pseudocode.html#type_FPRounding) = [FPDecodeRounding](shared_pseudocode.html#func_FPDecodeRounding_1)(o1::o2); let unsigned : boolean = TRUE;

### <span id="iclass_vector_half_precision"></span>Vector half-precision<span style="font-size:smaller;">\
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
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
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
<td class="droppedname">o2</td>
<td colspan="4"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td class="droppedname">o1</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="FCVTPU_asimdmiscfp16_R"></span>

FCVTPU [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__4 "For the "Vector half-precision" variant: is an arrangement specifier, "), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").[\<T\>](#T_option__4 "For the "Vector half-precision" variant: is an arrangement specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) \|\| !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 16; let datasize : integer{} = 64 \<\< UInt(Q); let elements : integer = datasize DIV esize; let rounding : [FPRounding](shared_pseudocode.html#type_FPRounding) = [FPDecodeRounding](shared_pseudocode.html#func_FPDecodeRounding_1)(o1::o2); let unsigned : boolean = TRUE;

### <span id="iclass_vector_single_precision_and_double_precision"></span>Vector single-precision and double-precision<span style="font-size:smaller;">\
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
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">sz</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
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
<td class="droppedname">o2</td>
<td></td>
<td colspan="5"></td>
<td colspan="4"></td>
<td class="droppedname">o1</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="FCVTPU_asimdmisc_R"></span>

FCVTPU [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__19 "For the "Vector single-precision and double-precision" variant: is an arrangement specifier, "), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").[\<T\>](#T_option__19 "For the "Vector single-precision and double-precision" variant: is an arrangement specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sz::Q == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 32 \<\< UInt(sz); let datasize : integer{} = 64 \<\< UInt(Q); let elements : integer = datasize DIV esize; let rounding : [FPRounding](shared_pseudocode.html#type_FPRounding) = [FPDecodeRounding](shared_pseudocode.html#func_FPDecodeRounding_1)(o1::o2); let unsigned : boolean = TRUE;

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
<td>&lt;Hn&gt;</td>
<td><span id="Hn__2"></span>
<p>Is the 16-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;V&gt;</td>
<td><span id="V_option__9"></span>
<p>Is a width specifier, encoded in “sz”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">sz</th>
<th class="symbol">&lt;V&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">D</td>
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
<td><span id="n__3"></span>
<p>Is the number of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
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
<td><span id="T_option__4"></span>
<p>For the "Vector half-precision" variant: is an arrangement specifier, encoded in “Q”:</p>
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
<td><span id="T_option__19"></span>
<p>For the "Vector single-precision and double-precision" variant: is an arrangement specifier, encoded in “(sz :: Q)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">sz</th>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="bitfield">0</td>
<td class="symbol">2S</td>
</tr>
<tr>
<td class="bitfield">0</td>
<td class="bitfield">1</td>
<td class="symbol">4S</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="bitfield">0</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="bitfield">1</td>
<td class="symbol">2D</td>
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
<td><span id="Vn"></span>
<p>Is the name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if elements == 1 && IsFeatureImplemented(FEAT_FPRCVT) then [AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); else [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); end; let operand : bits(datasize) = V{}(n); let merge : boolean = elements == 1 && [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()); var result : bits(128) = if merge then [V](shared_pseudocode.html#accessor_V_2){128}(d) else Zeros{128}; let fracbits : integer = 0; var element : bits(esize); for e = 0 to elements-1 do element = operand\[e\*:esize\]; result\[e\*:esize\] = [FPToFixed](shared_pseudocode.html#func_FPToFixed_7){esize, esize}(element, fracbits, unsigned, FPCR(), rounding); end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>
