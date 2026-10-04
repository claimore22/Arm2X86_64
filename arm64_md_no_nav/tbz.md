## TBZ

Test bit and branch if zero

This instruction compares the value of a test bit with zero, and conditionally branches to a label at a PC-relative offset if the comparison is equal. This instruction provides a hint that this is not a subroutine call or return. This instruction does not affect condition flags.

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
<td class="lr">b5</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="5" class="lr">b40</td>
<td colspan="14" class="lr">imm14</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">op</td>
<td colspan="5"></td>
<td colspan="14"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="TBZ_only_testbranch"></span>

TBZ [\<R\>](#R_option "Is a width specifier, ")[\<t\>](#Rt_option "Is the number [0-30] of the general-purpose register to be tested or the name ZR (31), encoded in the "Rt" field."), \#[\<imm\>](#imm_0_63 "Is the bit number to be tested, in the range 0 to 63, encoded in "b5:b40"."), [\<label\>](#imm14_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range +/-32KB, is encoded as "imm14" times 4.")

</div>

#### Decode for this encoding

let t : integer = UInt(Rt); let datasize : integer{} = 32 \<\< UInt(b5); let bit_pos : integer = UInt(b5::b40); let offset : bits(64) = SignExtend{}(imm14::'00');

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;R&gt;</td>
<td><span id="R_option"></span>
<p>Is a width specifier, encoded in “b5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">b5</th>
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
</table>
In assembler source code an 'X' specifier is always permitted, but a 'W' specifier is only permitted when the bit number is less than 32.</td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;t&gt;</td>
<td><span id="Rt_option"></span>
<p>Is the number [0-30] of the general-purpose register to be tested or the name ZR (31), encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm_0_63"></span>
<p>Is the bit number to be tested, in the range 0 to 63, encoded in "b5:b40".</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;label&gt;</td>
<td><span id="imm14_offset"></span>
<p>Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range +/-32KB, is encoded as "imm14" times 4.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let operand : bits(datasize) = X{}(t); let branch_conditional : boolean = TRUE; if operand\[bit_pos\] == '0' then [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}([PC64](shared_pseudocode.html#func_PC64_0)() + offset, [BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); else [BranchNotTaken](shared_pseudocode.html#func_BranchNotTaken_2)([BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); end;

</div>
