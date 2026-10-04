## IC

Instruction cache operation

For more information, see <span class="armarm-xref" data-linkend="ARMARM_BABEJJJE">op0 == 0b01, cache maintenance, TLB maintenance, address translation, prediction restriction, BRBE, Trace Extension, and Guarded Control Stack instructions</span>.

This is an alias of [SYS](sys.html). This means:

- The encodings in this description are named to match the encodings of [SYS](sys.html).
- The description of [SYS](sys.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="3" class="lr">op1</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td colspan="4" class="lr">CRm</td>
<td colspan="3" class="lr">op2</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td class="droppedname">L</td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="4" class="droppedname">CRn</td>
<td colspan="4"></td>
<td colspan="3"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="IC_SYS_CR_systeminstrs"></span>

IC [\<ic_op\>](#ic_op_option "Is an IC operation name, as listed for the IC system instruction pages, "){, [\<Xt\>](#XtOrXZR__2 "Is the 64-bit name of the optional general-purpose source register, defaulting to '11111', encoded in the "Rt" field.")}

is equivalent to

[SYS](sys.html#SYS_CR_systeminstrs) \#[\<op1\>](sys.html#op1), C7, [\<Cm\>](sys.html#Cm), \#[\<op2\>](sys.html#op2){, [\<Xt\>](sys.html#XtOrXZR__2)}

and is the preferred disassembly when <span class="pseudocode">SysOp(op1, '0111', CRm, op2) == Sys_IC</span>.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;ic_op&gt;</td>
<td><span id="ic_op_option"></span>
<p>Is an IC operation name, as listed for the IC system instruction pages, encoded in “(op1 :: CRm :: op2)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">op1</th>
<th class="bitfield">CRm</th>
<th class="bitfield">op2</th>
<th class="symbol">&lt;ic_op&gt;</th>
<th class="symbol">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">0001</td>
<td class="bitfield">000</td>
<td class="symbol">IALLUIS</td>
<td class="description"><p>For more information, see IC IALLUIS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">0101</td>
<td class="bitfield">000</td>
<td class="symbol">IALLU</td>
<td class="description"><p>For more information, see IC IALLU.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">0101</td>
<td class="bitfield">001</td>
<td class="symbol">IVAU</td>
<td class="description"><p>For more information, see IC IVAU.</p></td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__2"></span>
<p>Is the 64-bit name of the optional general-purpose source register, defaulting to '11111', encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [SYS](sys.html) gives the operational pseudocode for this instruction.

</div>
