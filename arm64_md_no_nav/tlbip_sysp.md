## TLBIP

TLB invalidate pair operation

TLB invalidate pair operation.

This is an alias of [SYSP](sysp.html). This means:

- The encodings in this description are named to match the encodings of [SYSP](sysp.html).
- The description of [SYSP](sysp.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_system"></span>System<span style="font-size:smaller;">\
(FEAT_SYSINSTR128) </span>

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
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="3" class="lr">op1</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td class="r">x</td>
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

<span id="TLBIP_SYSP_CR_syspairinstrs"></span>

TLBIP [\<tlbip_op\>](#tlbip_op_option "Is a TLBIP operation name, as listed for the TLBIP system pair instruction group, "){, [\<Xt1\>](#Xt1_register "Is the 64-bit name of the first optional general-purpose source register, defaulting to '11111', encoded in the "Rt" field."), [\<Xt2\>](#Xt2 "Is the 64-bit name of the second optional general-purpose source register, defaulting to '11111', encoded as "Rt" +1. Defaults to '11111' if "Rt" = '11111'.")}

is equivalent to

[SYSP](sysp.html#SYSP_CR_syspairinstrs) \#[\<op1\>](sysp.html#op1), [\<Cn\>](sysp.html#Cn), [\<Cm\>](sysp.html#Cm), \#[\<op2\>](sysp.html#op2){, [\<Xt1\>](sysp.html#Xt1_register), [\<Xt2\>](sysp.html#Xt2)}

and is the preferred disassembly when <span class="pseudocode">SysOp128(op1, CRn, CRm, op2) == Sys_TLBIP</span>.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;tlbip_op&gt;</td>
<td><span id="tlbip_op_option"></span>
<p>Is a TLBIP operation name, as listed for the TLBIP system pair instruction group, encoded in “(op1 :: CRn :: CRm :: op2)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">op1</th>
<th class="bitfield">CRn</th>
<th class="bitfield">CRm</th>
<th class="bitfield">op2</th>
<th class="symbol">&lt;tlbip_op&gt;</th>
<th class="symbol">Architectural Feature</th>
<th class="symbol">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0001</td>
<td class="bitfield">001</td>
<td class="symbol">VAE1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE1OS, TLBIP VAE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0001</td>
<td class="bitfield">011</td>
<td class="symbol">VAAE1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAAE1OS, TLBIP VAAE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0001</td>
<td class="bitfield">101</td>
<td class="symbol">VALE1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE1OS, TLBIP VALE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0001</td>
<td class="bitfield">111</td>
<td class="symbol">VAALE1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAALE1OS, TLBIP VAALE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0010</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE1IS, TLBIP RVAE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0010</td>
<td class="bitfield">011</td>
<td class="symbol">RVAAE1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAAE1IS, TLBIP RVAAE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0010</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE1IS, TLBIP RVALE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0010</td>
<td class="bitfield">111</td>
<td class="symbol">RVAALE1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAALE1IS, TLBIP RVAALE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0011</td>
<td class="bitfield">001</td>
<td class="symbol">VAE1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE1IS, TLBIP VAE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0011</td>
<td class="bitfield">011</td>
<td class="symbol">VAAE1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAAE1IS, TLBIP VAAE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0011</td>
<td class="bitfield">101</td>
<td class="symbol">VALE1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE1IS, TLBIP VALE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0011</td>
<td class="bitfield">111</td>
<td class="symbol">VAALE1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAALE1IS, TLBIP VAALE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0101</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE1OS, TLBIP RVAE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0101</td>
<td class="bitfield">011</td>
<td class="symbol">RVAAE1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAAE1OS, TLBIP RVAAE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0101</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE1OS, TLBIP RVALE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0101</td>
<td class="bitfield">111</td>
<td class="symbol">RVAALE1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAALE1OS, TLBIP RVAALE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0110</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE1, TLBIP RVAE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0110</td>
<td class="bitfield">011</td>
<td class="symbol">RVAAE1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAAE1, TLBIP RVAAE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0110</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE1, TLBIP RVALE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0110</td>
<td class="bitfield">111</td>
<td class="symbol">RVAALE1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAALE1, TLBIP RVAALE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0111</td>
<td class="bitfield">001</td>
<td class="symbol">VAE1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE1, TLBIP VAE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0111</td>
<td class="bitfield">011</td>
<td class="symbol">VAAE1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAAE1, TLBIP VAAE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0111</td>
<td class="bitfield">101</td>
<td class="symbol">VALE1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE1, TLBIP VALE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">0111</td>
<td class="bitfield">111</td>
<td class="symbol">VAALE1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAALE1, TLBIP VAALE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0001</td>
<td class="bitfield">001</td>
<td class="symbol">VAE1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE1OS, TLBIP VAE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0001</td>
<td class="bitfield">011</td>
<td class="symbol">VAAE1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAAE1OS, TLBIP VAAE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0001</td>
<td class="bitfield">101</td>
<td class="symbol">VALE1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE1OS, TLBIP VALE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0001</td>
<td class="bitfield">111</td>
<td class="symbol">VAALE1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAALE1OS, TLBIP VAALE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0010</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE1IS, TLBIP RVAE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0010</td>
<td class="bitfield">011</td>
<td class="symbol">RVAAE1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAAE1IS, TLBIP RVAAE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0010</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE1IS, TLBIP RVALE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0010</td>
<td class="bitfield">111</td>
<td class="symbol">RVAALE1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAALE1IS, TLBIP RVAALE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0011</td>
<td class="bitfield">001</td>
<td class="symbol">VAE1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE1IS, TLBIP VAE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0011</td>
<td class="bitfield">011</td>
<td class="symbol">VAAE1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAAE1IS, TLBIP VAAE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0011</td>
<td class="bitfield">101</td>
<td class="symbol">VALE1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE1IS, TLBIP VALE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0011</td>
<td class="bitfield">111</td>
<td class="symbol">VAALE1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAALE1IS, TLBIP VAALE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0101</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE1OS, TLBIP RVAE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0101</td>
<td class="bitfield">011</td>
<td class="symbol">RVAAE1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAAE1OS, TLBIP RVAAE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0101</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE1OS, TLBIP RVALE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0101</td>
<td class="bitfield">111</td>
<td class="symbol">RVAALE1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAALE1OS, TLBIP RVAALE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0110</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE1, TLBIP RVAE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0110</td>
<td class="bitfield">011</td>
<td class="symbol">RVAAE1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAAE1, TLBIP RVAAE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0110</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE1, TLBIP RVALE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0110</td>
<td class="bitfield">111</td>
<td class="symbol">RVAALE1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAALE1, TLBIP RVAALE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0111</td>
<td class="bitfield">001</td>
<td class="symbol">VAE1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE1, TLBIP VAE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0111</td>
<td class="bitfield">011</td>
<td class="symbol">VAAE1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAAE1, TLBIP VAAE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0111</td>
<td class="bitfield">101</td>
<td class="symbol">VALE1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE1, TLBIP VALE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">0111</td>
<td class="bitfield">111</td>
<td class="symbol">VAALE1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAALE1, TLBIP VAALE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0000</td>
<td class="bitfield">001</td>
<td class="symbol">IPAS2E1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2E1IS, TLBIP IPAS2E1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0000</td>
<td class="bitfield">010</td>
<td class="symbol">RIPAS2E1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2E1IS, TLBIP RIPAS2E1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0000</td>
<td class="bitfield">101</td>
<td class="symbol">IPAS2LE1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2LE1IS, TLBIP IPAS2LE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0000</td>
<td class="bitfield">110</td>
<td class="symbol">RIPAS2LE1IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2LE1IS, TLBIP RIPAS2LE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0001</td>
<td class="bitfield">001</td>
<td class="symbol">VAE2OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE2OS, TLBIP VAE2OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0001</td>
<td class="bitfield">101</td>
<td class="symbol">VALE2OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE2OS, TLBIP VALE2OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0010</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE2IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE2IS, TLBIP RVAE2ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0010</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE2IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE2IS, TLBIP RVALE2ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0011</td>
<td class="bitfield">001</td>
<td class="symbol">VAE2IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE2IS, TLBIP VAE2ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0011</td>
<td class="bitfield">101</td>
<td class="symbol">VALE2IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE2IS, TLBIP VALE2ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0100</td>
<td class="bitfield">000</td>
<td class="symbol">IPAS2E1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2E1OS, TLBIP IPAS2E1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0100</td>
<td class="bitfield">001</td>
<td class="symbol">IPAS2E1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2E1, TLBIP IPAS2E1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0100</td>
<td class="bitfield">010</td>
<td class="symbol">RIPAS2E1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2E1, TLBIP RIPAS2E1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0100</td>
<td class="bitfield">011</td>
<td class="symbol">RIPAS2E1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2E1OS, TLBIP RIPAS2E1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0100</td>
<td class="bitfield">100</td>
<td class="symbol">IPAS2LE1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2LE1OS, TLBIP IPAS2LE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0100</td>
<td class="bitfield">101</td>
<td class="symbol">IPAS2LE1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2LE1, TLBIP IPAS2LE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0100</td>
<td class="bitfield">110</td>
<td class="symbol">RIPAS2LE1</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2LE1, TLBIP RIPAS2LE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0100</td>
<td class="bitfield">111</td>
<td class="symbol">RIPAS2LE1OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2LE1OS, TLBIP RIPAS2LE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0101</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE2OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE2OS, TLBIP RVAE2OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0101</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE2OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE2OS, TLBIP RVALE2OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0110</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE2</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE2, TLBIP RVAE2NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0110</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE2</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE2, TLBIP RVALE2NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0111</td>
<td class="bitfield">001</td>
<td class="symbol">VAE2</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE2, TLBIP VAE2NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">0111</td>
<td class="bitfield">101</td>
<td class="symbol">VALE2</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE2, TLBIP VALE2NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0000</td>
<td class="bitfield">001</td>
<td class="symbol">IPAS2E1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2E1IS, TLBIP IPAS2E1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0000</td>
<td class="bitfield">010</td>
<td class="symbol">RIPAS2E1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2E1IS, TLBIP RIPAS2E1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0000</td>
<td class="bitfield">101</td>
<td class="symbol">IPAS2LE1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2LE1IS, TLBIP IPAS2LE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0000</td>
<td class="bitfield">110</td>
<td class="symbol">RIPAS2LE1ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2LE1IS, TLBIP RIPAS2LE1ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0001</td>
<td class="bitfield">001</td>
<td class="symbol">VAE2OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE2OS, TLBIP VAE2OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0001</td>
<td class="bitfield">101</td>
<td class="symbol">VALE2OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE2OS, TLBIP VALE2OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0010</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE2ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE2IS, TLBIP RVAE2ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0010</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE2ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE2IS, TLBIP RVALE2ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0011</td>
<td class="bitfield">001</td>
<td class="symbol">VAE2ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE2IS, TLBIP VAE2ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0011</td>
<td class="bitfield">101</td>
<td class="symbol">VALE2ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE2IS, TLBIP VALE2ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0100</td>
<td class="bitfield">000</td>
<td class="symbol">IPAS2E1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2E1OS, TLBIP IPAS2E1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0100</td>
<td class="bitfield">001</td>
<td class="symbol">IPAS2E1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2E1, TLBIP IPAS2E1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0100</td>
<td class="bitfield">010</td>
<td class="symbol">RIPAS2E1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2E1, TLBIP RIPAS2E1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0100</td>
<td class="bitfield">011</td>
<td class="symbol">RIPAS2E1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2E1OS, TLBIP RIPAS2E1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0100</td>
<td class="bitfield">100</td>
<td class="symbol">IPAS2LE1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2LE1OS, TLBIP IPAS2LE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0100</td>
<td class="bitfield">101</td>
<td class="symbol">IPAS2LE1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP IPAS2LE1, TLBIP IPAS2LE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0100</td>
<td class="bitfield">110</td>
<td class="symbol">RIPAS2LE1NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2LE1, TLBIP RIPAS2LE1NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0100</td>
<td class="bitfield">111</td>
<td class="symbol">RIPAS2LE1OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RIPAS2LE1OS, TLBIP RIPAS2LE1OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0101</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE2OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE2OS, TLBIP RVAE2OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0101</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE2OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE2OS, TLBIP RVALE2OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0110</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE2NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE2, TLBIP RVAE2NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0110</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE2NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE2, TLBIP RVALE2NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0111</td>
<td class="bitfield">001</td>
<td class="symbol">VAE2NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE2, TLBIP VAE2NXS.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">0111</td>
<td class="bitfield">101</td>
<td class="symbol">VALE2NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE2, TLBIP VALE2NXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0001</td>
<td class="bitfield">001</td>
<td class="symbol">VAE3OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE3OS, TLBIP VAE3OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0001</td>
<td class="bitfield">101</td>
<td class="symbol">VALE3OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE3OS, TLBIP VALE3OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0010</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE3IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE3IS, TLBIP RVAE3ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0010</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE3IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE3IS, TLBIP RVALE3ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0011</td>
<td class="bitfield">001</td>
<td class="symbol">VAE3IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE3IS, TLBIP VAE3ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0011</td>
<td class="bitfield">101</td>
<td class="symbol">VALE3IS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE3IS, TLBIP VALE3ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0101</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE3OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE3OS, TLBIP RVAE3OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0101</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE3OS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE3OS, TLBIP RVALE3OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0110</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE3</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE3, TLBIP RVAE3NXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0110</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE3</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE3, TLBIP RVALE3NXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0111</td>
<td class="bitfield">001</td>
<td class="symbol">VAE3</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE3, TLBIP VAE3NXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">0111</td>
<td class="bitfield">101</td>
<td class="symbol">VALE3</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE3, TLBIP VALE3NXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0001</td>
<td class="bitfield">001</td>
<td class="symbol">VAE3OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE3OS, TLBIP VAE3OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0001</td>
<td class="bitfield">101</td>
<td class="symbol">VALE3OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE3OS, TLBIP VALE3OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0010</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE3ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE3IS, TLBIP RVAE3ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0010</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE3ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE3IS, TLBIP RVALE3ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0011</td>
<td class="bitfield">001</td>
<td class="symbol">VAE3ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE3IS, TLBIP VAE3ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0011</td>
<td class="bitfield">101</td>
<td class="symbol">VALE3ISNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE3IS, TLBIP VALE3ISNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0101</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE3OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE3OS, TLBIP RVAE3OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0101</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE3OSNXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE3OS, TLBIP RVALE3OSNXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0110</td>
<td class="bitfield">001</td>
<td class="symbol">RVAE3NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVAE3, TLBIP RVAE3NXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0110</td>
<td class="bitfield">101</td>
<td class="symbol">RVALE3NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP RVALE3, TLBIP RVALE3NXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0111</td>
<td class="bitfield">001</td>
<td class="symbol">VAE3NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VAE3, TLBIP VAE3NXS.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">0111</td>
<td class="bitfield">101</td>
<td class="symbol">VALE3NXS</td>
<td class="feature">FEAT_D128</td>
<td class="description"><p>For more information, see TLBIP VALE3, TLBIP VALE3NXS.</p></td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt1&gt;</td>
<td><span id="Xt1_register"></span>
<p>Is the 64-bit name of the first optional general-purpose source register, defaulting to '11111', encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt2&gt;</td>
<td><span id="Xt2"></span>
<p>Is the 64-bit name of the second optional general-purpose source register, defaulting to '11111', encoded as "Rt" +1. Defaults to '11111' if "Rt" = '11111'.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [SYSP](sysp.html) gives the operational pseudocode for this instruction.

</div>
