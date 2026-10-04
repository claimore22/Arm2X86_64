## PMOV (to predicate)

Move predicate from vector

This instruction copies a packed bitmap, where bit value 0b1 represents TRUE and bit value 0b0 represents FALSE, from a portion of the source vector register to elements of the destination SVE predicate register.

Because the number of bits in an SVE predicate element scales with the vector element size, the behavior varies according to the specified element size.

- When the predicate element specifier is B, each bit \[N\] from the least-significant VL/8 bits in the source vector register is copied to bit \[N\] of the destination predicate register. The portion index, if specified, must be 0.
- When the predicate element specifier is H, each bit \[N\] within the indexed block of VL/16 bits in the source vector register is copied to bit \[N\*2\] of the destination predicate register, and the other bits in the predicate are set to zero. The portion index is in the range 0 to 1, inclusive.
- When the predicate element specifier is S, each bit \[N\] within the indexed block of VL/32 bits in the source vector register is copied to bit \[N\*4\] of the destination predicate register, and the other bits in the predicate are set to zero. The portion index is in the range 0 to 3, inclusive.
- When the predicate element specifier is D, each bit \[N\] within the indexed block of VL/64 bits in the source vector register is copied to bit \[N\*8\] of the destination predicate register, and the other bits in the predicate are set to zero. The portion index is in the range 0 to 7, inclusive.

The portion index is optional, defaulting to 0 if omitted.

It has encodings from 4 classes: [Byte](#iclass_byte) , [Doubleword](#iclass_doubleword) , [Halfword](#iclass_halfword) and [Word](#iclass_word)

### <span id="iclass_byte"></span>Byte<span style="font-size:smaller;">\
(FEAT_SVE2p1 \|\| FEAT_SME2p1) </span>

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
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="2"></td>
<td colspan="2" class="droppedname">opc2</td>
<td></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td></td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pmov_p_zi_b"></span>

PMOV [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").B, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let d : integer = UInt(Pd); let esize : integer{} = 8; let imm : integer = 0;

### <span id="iclass_doubleword"></span>Doubleword<span style="font-size:smaller;">\
(FEAT_SVE2p1 \|\| FEAT_SME2p1) </span>

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
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">i3h</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="2" class="lr">i3l</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pmov_p_zi_d"></span>

PMOV [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").D, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field."){\[[\<imm\>](#imm__83 "For the "Doubleword" variant: is the optional portion index, in the range 0 to 7, defaulting to 0, encoded in the "i3h:i3l" fields.")\]}

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let d : integer = UInt(Pd); let esize : integer{} = 64; let imm : integer = UInt(i3h::i3l);

### <span id="iclass_halfword"></span>Halfword<span style="font-size:smaller;">\
(FEAT_SVE2p1 \|\| FEAT_SME2p1) </span>

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
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">i1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td></td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pmov_p_zi_h"></span>

PMOV [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").H, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field."){\[[\<imm\>](#imm__84 "For the "Halfword" variant: is the optional portion index, in the range 0 to 1, defaulting to 0, encoded in the "i1" field.")\]}

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let d : integer = UInt(Pd); let esize : integer{} = 16; let imm : integer = UInt(i1);

### <span id="iclass_word"></span>Word<span style="font-size:smaller;">\
(FEAT_SVE2p1 \|\| FEAT_SME2p1) </span>

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
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="2" class="lr">i2</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td></td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pmov_p_zi_s"></span>

PMOV [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").S, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field."){\[[\<imm\>](#imm__85 "For the "Word" variant: is the optional portion index, in the range 0 to 3, defaulting to 0, encoded in the "i2" field.")\]}

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let d : integer = UInt(Pd); let esize : integer{} = 32; let imm : integer = UInt(i2);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Pd&gt;</td>
<td><span id="Pd"></span>
<p>Is the name of the destination scalable predicate register, encoded in the "Pd" field.</p></td>
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

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__83"></span>
<p>For the "Doubleword" variant: is the optional portion index, in the range 0 to 7, defaulting to 0, encoded in the "i3h:i3l" fields.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__84"></span>
<p>For the "Halfword" variant: is the optional portion index, in the range 0 to 1, defaulting to 0, encoded in the "i1" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__85"></span>
<p>For the "Word" variant: is the optional portion index, in the range 0 to 3, defaulting to 0, encoded in the "i2" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let operand : bits(VL) = Z{}(n); var result : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do result\[e\*:psize\] = ZeroExtend{psize}(operand\[(elements \* imm) + e\]); end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
