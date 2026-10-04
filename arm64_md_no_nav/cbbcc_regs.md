## CBB\<cc\>

Compare bytes and branch

This instruction compares the byte values in two registers, and conditionally branches to a label at a PC-relative offset if the condition is true. This instruction provides a hint that this is not a subroutine call or return. This instruction does not affect the condition flags.

This instruction is used by the pseudo-instructions [CBBLE](cbble_regs.html "Compare signed less than or equal to bytes and branch"), [CBBLO](cbblo_regs.html "Compare unsigned lower than bytes and branch"), [CBBLS](cbbls_regs.html "Compare unsigned lower than or same as bytes and branch"), and [CBBLT](cbblt_regs.html "Compare signed less than bytes and branch").

### <span id="iclass_branch"></span>Branch<span style="font-size:smaller;">\
(FEAT_CMPBR) </span>

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
<td>1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td colspan="3" class="lr">cc</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="9" class="lr">imm9</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="8"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td></td>
<td class="droppedname">H</td>
<td colspan="9"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Greater than variant

<span id="CBBGT_8_regs"></span> Applies when <span class="bitdiff"> (cc == 000)</span>

CBBGT [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the Greater than or equal variant

<span id="CBBGE_8_regs"></span> Applies when <span class="bitdiff"> (cc == 001)</span>

CBBGE [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the Higher variant

<span id="CBBHI_8_regs"></span> Applies when <span class="bitdiff"> (cc == 010)</span>

CBBHI [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the Higher or same variant

<span id="CBBHS_8_regs"></span> Applies when <span class="bitdiff"> (cc == 011)</span>

CBBHS [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the Equal variant

<span id="CBBEQ_8_regs"></span> Applies when <span class="bitdiff"> (cc == 110)</span>

CBBEQ [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the Not equal variant

<span id="CBBNE_8_regs"></span> Applies when <span class="bitdiff"> (cc == 111)</span>

CBBNE [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_CMPBR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let datasize : integer{} = 8 \<\< UInt(H); let t : integer = UInt(Rt); let m : integer = UInt(Rm); let offset : bits(64) = SignExtend{}(imm9::'00'); var op : [CmpOp](shared_pseudocode.html#type_CmpOp); var unsigned : boolean; case cc of when '000' =\> op = [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT); unsigned = FALSE; when '001' =\> op = [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE); unsigned = FALSE; when '010' =\> op = [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT); unsigned = TRUE; when '011' =\> op = [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE); unsigned = TRUE; when '110' =\> op = [Cmp_EQ](shared_pseudocode.html#enum_Cmp_EQ); unsigned = TRUE; when '111' =\> op = [Cmp_NE](shared_pseudocode.html#enum_Cmp_NE); unsigned = TRUE; otherwise =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wt&gt;</td>
<td><span id="WtOrWZR"></span>
<p>Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wm&gt;</td>
<td><span id="WmOrWZR__2"></span>
<p>Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;label&gt;</td>
<td><span id="imm9_offset"></span>
<p>Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let operand1 : bits(datasize) = X{}(t); let operand2 : bits(datasize) = X{}(m); let branch_conditional : boolean = TRUE; let value1 : integer = if unsigned then UInt(operand1) else SInt(operand1); let value2 : integer = if unsigned then UInt(operand2) else SInt(operand2); var cond : boolean; case op of when [Cmp_EQ](shared_pseudocode.html#enum_Cmp_EQ) =\> cond = value1 == value2; when [Cmp_NE](shared_pseudocode.html#enum_Cmp_NE) =\> cond = value1 != value2; when [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE) =\> cond = value1 \>= value2; when [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT) =\> cond = value1 \> value2; end; if cond then [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}([PC64](shared_pseudocode.html#func_PC64_0)() + offset, [BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); else [BranchNotTaken](shared_pseudocode.html#func_BranchNotTaken_2)([BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); end;

</div>
