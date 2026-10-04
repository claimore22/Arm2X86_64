## FMLS (indexed)

Floating-point fused multiply-subtract by indexed element

This instruction multiplies all floating-point elements within each 128-bit segment of the first source vector by the specified element in the corresponding second source vector segment. The products are then destructively subtracted without intermediate rounding from the corresponding elements of the addend and destination vector.

The elements within the second source vector are specified using an immediate index that selects the same element position within each 128-bit vector segment. The index range is from 0 to one less than the number of elements per 128-bit segment. This instruction is unpredicated.

It has encodings from 3 classes: [Half-precision](#iclass_half_precision) , [Single-precision](#iclass_single_precision) and [Double-precision](#iclass_double_precision)

### <span id="iclass_half_precision"></span>Half-precision<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">i3h</td>
<td class="lr">1</td>
<td colspan="2" class="lr">i3l</td>
<td colspan="3" class="lr">Zm</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zda</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td class="droppedname">o2</td>
<td class="droppedname">op</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmls_z_zzzi_h"></span>

FMLS [\<Zda\>](#Zda "Is the name of the third source and destination scalable vector register, encoded in the "Zda" field.").H, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").H, [\<Zm\>](#Zm__4 "For the "Half-precision" and "Single-precision" variants: is the name of the second source scalable vector register Z0-Z7, encoded in the "Zm" field.").H\[[\<imm\>](#imm__36 "For the "Half-precision" variant: is the immediate index, in the range 0 to 7, encoded in the "i3h:i3l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 16; let index : integer = UInt(i3h::i3l); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let op1_neg : boolean = TRUE; let op3_neg : boolean = FALSE;

### <span id="iclass_single_precision"></span>Single-precision<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">i2</td>
<td colspan="3" class="lr">Zm</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zda</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td class="droppedname">o2</td>
<td class="droppedname">op</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmls_z_zzzi_s"></span>

FMLS [\<Zda\>](#Zda "Is the name of the third source and destination scalable vector register, encoded in the "Zda" field.").S, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").S, [\<Zm\>](#Zm__4 "For the "Half-precision" and "Single-precision" variants: is the name of the second source scalable vector register Z0-Z7, encoded in the "Zm" field.").S\[[\<imm\>](#imm__55 "For the "Single-precision" variant: is the immediate index, in the range 0 to 3, encoded in the "i2" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let index : integer = UInt(i2); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let op1_neg : boolean = TRUE; let op3_neg : boolean = FALSE;

### <span id="iclass_double_precision"></span>Double-precision<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">i1</td>
<td colspan="4" class="lr">Zm</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zda</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td></td>
<td colspan="4"></td>
<td colspan="4"></td>
<td class="droppedname">o2</td>
<td class="droppedname">op</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmls_z_zzzi_d"></span>

FMLS [\<Zda\>](#Zda "Is the name of the third source and destination scalable vector register, encoded in the "Zda" field.").D, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").D, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").D\[[\<imm\>](#imm__54 "For the "Double-precision" variant: is the immediate index, in the range 0 to 1, encoded in the "i1" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let index : integer = UInt(i1); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let op1_neg : boolean = TRUE; let op3_neg : boolean = FALSE;

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
<td><span id="Zm__4"></span>
<p>For the "Half-precision" and "Single-precision" variants: is the name of the second source scalable vector register Z0-Z7, encoded in the "Zm" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zm__2"></span>
<p>For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__36"></span>
<p>For the "Half-precision" variant: is the immediate index, in the range 0 to 7, encoded in the "i3h:i3l" fields.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__55"></span>
<p>For the "Single-precision" variant: is the immediate index, in the range 0 to 3, encoded in the "i2" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__54"></span>
<p>For the "Double-precision" variant: is the immediate index, in the range 0 to 1, encoded in the "i1" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let eltspersegment : integer = 128 DIV esize; let op1 : bits(VL) = Z{}(n); let op2 : bits(VL) = Z{}(m); var result : bits(VL) = Z{}(da); for e = 0 to elements-1 do let segmentbase : integer = e - (e MOD eltspersegment); let s : integer = segmentbase + index; let elem2 : bits(esize) = op2\[s\*:esize\]; let elem1 : bits(esize) = (if op1_neg then [FPNeg](shared_pseudocode.html#func_FPNeg_3){esize}(op1\[e\*:esize\], FPCR()) else op1\[e\*:esize\]); let elem3 : bits(esize) = (if op3_neg then [FPNeg](shared_pseudocode.html#func_FPNeg_3){esize}(result\[e\*:esize\], FPCR()) else result\[e\*:esize\]); result\[e\*:esize\] = [FPMulAdd](shared_pseudocode.html#func_FPMulAdd_5){esize}(elem3, elem1, elem2, FPCR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;

</div>

### Operational information

This instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> must be unpredicated.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
