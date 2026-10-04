## CTERMEQ, CTERMNE

Compare and terminate loop

This instruction detects termination conditions in serialized vector loops by testing whether the comparison between the scalar source operands holds true. If the comparison is not successful, the instruction tests the state of the !Last condition flag (C), which indicates whether the previous flag-setting predicate instruction selected the last element of the vector partition.

The Z and C condition flags are preserved by this instruction. The N and V condition flags are set as a pair to generate one of the following conditions for a subsequent conditional instruction:

| Condition | N | V | Meaning |
|----|----|----|----|
| GE | 0 | 0 | Continue loop (compare failed and last element not selected) |
| LT | 0 | 1 | Terminate loop (last element selected) |
| LT | 1 | 0 | Terminate loop (compare succeeded) |
| GE | 1 | 1 | Never generated |

The scalar source operands are 32-bit or 64-bit general-purpose registers of the same size.

It has encodings from 2 classes: [Equal](#iclass_equal) and [Not equal](#iclass_not_equal)

### <span id="iclass_equal"></span>Equal<span style="font-size:smaller;">\
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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">sz</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">op</td>
<td></td>
<td></td>
<td colspan="5"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ctermeq_rr_"></span>

CTERMEQ [\<R\>](#R__4 "Is a width specifier, ")[\<n\>](#n__5 "Is the number [0-30] of the source general-purpose register or the name ZR (31), encoded in the "Rn" field."), [\<R\>](#R__4 "Is a width specifier, ")[\<m\>](#m__3 "Is the number [0-30] of the source general-purpose register or the name ZR (31), encoded in the "Rm" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32 \<\< UInt(sz); let n : integer = UInt(Rn); let m : integer = UInt(Rm); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_EQ](shared_pseudocode.html#enum_Cmp_EQ);

### <span id="iclass_not_equal"></span>Not equal<span style="font-size:smaller;">\
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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">sz</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">op</td>
<td></td>
<td></td>
<td colspan="5"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ctermne_rr_"></span>

CTERMNE [\<R\>](#R__4 "Is a width specifier, ")[\<n\>](#n__5 "Is the number [0-30] of the source general-purpose register or the name ZR (31), encoded in the "Rn" field."), [\<R\>](#R__4 "Is a width specifier, ")[\<m\>](#m__3 "Is the number [0-30] of the source general-purpose register or the name ZR (31), encoded in the "Rm" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32 \<\< UInt(sz); let n : integer = UInt(Rn); let m : integer = UInt(Rm); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_NE](shared_pseudocode.html#enum_Cmp_NE);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;R&gt;</td>
<td><span id="R__4"></span>
<p>Is a width specifier, encoded in “sz”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">sz</th>
<th class="symbol">&lt;R&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">X</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;n&gt;</td>
<td><span id="n__5"></span>
<p>Is the number [0-30] of the source general-purpose register or the name ZR (31), encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;m&gt;</td>
<td><span id="m__3"></span>
<p>Is the number [0-30] of the source general-purpose register or the name ZR (31), encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let operand1 : bits(esize) = X{}(n); let operand2 : bits(esize) = X{}(m); let element1 : integer = UInt(operand1); let element2 : integer = UInt(operand2); var term : boolean; case cmp_op of when [Cmp_EQ](shared_pseudocode.html#enum_Cmp_EQ) =\> term = element1 == element2; when [Cmp_NE](shared_pseudocode.html#enum_Cmp_NE) =\> term = element1 != element2; end; if term then [PSTATE](shared_pseudocode.html#global_PSTATE).N = '1'; [PSTATE](shared_pseudocode.html#global_PSTATE).V = '0'; else [PSTATE](shared_pseudocode.html#global_PSTATE).N = '0'; [PSTATE](shared_pseudocode.html#global_PSTATE).V = (NOT [PSTATE](shared_pseudocode.html#global_PSTATE).C); end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
