code(c::CommandA) = string(c.num, base = 2, pad = 16)


function code(c::CommandC)

    dict_comp = Dict(
        "0"   => "0101010",
        "1"   => "0111111",
        "-1"  => "0111010",
        "D"   => "0001100",
        "A"   => "0110000",
        "!D"  => "0001101",
        "!A"  => "0110001",
        "-D"  => "0001111",
        "-A"  => "0110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",
        "M"   => "1110000",
        "!M"  => "1110001",
        "-M"  => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101"
    )

    dict_dest = Dict(
        "null" => "000",
        "M"    => "001",
        "D"    => "010",
        "MD"   => "011",
        "A"    => "100",
        "AM"   => "101",
        "AD"   => "110",
        "AMD"  => "111",
        "" => "000"
    )

    dict_jump = Dict(
        "null" => "000",
        "JGT"  => "001",
        "JEQ"  => "010",
        "JGE"  => "011",
        "JLT"  => "100",
        "JNE"  => "101",
        "JLE"  => "110",
        "JMP"  => "111",
        "" => "000"
    )

    s_comp = dict_comp[c.comp]
    s_dest = dict_dest[c.dest]
    s_jump = dict_jump[c.jump]

    return "111$s_comp$s_dest$s_jump"

end

function code!(asm::ASM)

    for command ∈ asm.commands
        push!(asm.bins, code(command))
    end

end