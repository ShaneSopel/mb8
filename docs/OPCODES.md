<table>
    <tr>
        <th>asm</th>
        <th>opcode</th>
        <th>short description</th>
    </tr>
    <tr>
        <td>NOP</td>
        <td>0x0000</td>
        <td>No operation</td>
    </tr>
    <tr>
        <td>HALT</td>
        <td>0x0100</td>
        <td>Stop the execution</td>
    </tr>
    <tr>
        <td>MOV dst src</td>
        <td>0x10AB</td>
        <td>Move data from B register to A register</td>
    </tr>
    <tr>
        <td>ADD dst src</td>
        <td>0x11AB</td>
        <td>Put the sum of A and B registers into A register</td>
    </tr>
    <tr>
        <td>SUB dst src</td>
        <td>0x12AB</td>
        <td>Put the difference of A and B registers into A register</td>
    </tr>
    <tr>
        <td>LDI dst value</td>
        <td>0x2AXX</td>
        <td>Load immediate XX value into A register</td>
    </tr>
</table>
