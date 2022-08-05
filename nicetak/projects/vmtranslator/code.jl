
include("parser.jl")

function code(c::C_ARITHMETIC)

    d_sym = Dict("add" => "+", "sub" => "-", "and" => "&", "or" => "|",
                 "neg" => "-", "not" => "!",
                 "eq" => "JNE", "lt" => "JGE", "gt" => "JLE")

    s = ""

    if c.command ∈ ["add", "and", "or"]
        s = """
            @SP
            AM=M-1
            D=M
            A=A-1
            M=D$(d_sym[c.command])M
            """
    elseif c.command == "sub"
        s = """
            @SP
            AM=M-1
            D=M
            A=A-1
            M=M-D
            """
    elseif c.command ∈ ["not", "neg"]
        s = """
            @SP
            A=M-1
            M=$(d_sym[c.command])M
            """
    elseif c.command ∈ ["eq", "gt", "lt"]
        s = """
            @SP
            AM=M-1
            D=M
            A=A-1
            D=M-D
            @FALSE_$(c.id)
            D;$(d_sym[c.command])
            @SP
            A=M-1
            M=-1
            @CONTINUE_$(c.id)
            0;JMP
            (FALSE_$(c.id))
            @SP
            A=M-1
            M=0
            (CONTINUE_$(c.id))
            """
    end

    return s
end

function code(c::C_PUSH)
    

    d = Dict("argument" => "ARG", "local" => "LCL",
             "this" => "THIS", "that" => "THAT",
             "pointer" => "THIS", "temp" => "R5")

    s = ""
    s_push = """
             @SP
             A=M
             M=D
             @SP
             M=M+1"""

    if c.arg1 ∈ ["argument", "local", "this", "that"]
        s = """
            @$(d[c.arg1])
            D=M
            @$(c.arg2)
            A=D+A
            D=M
            $s_push
            """
    elseif c.arg1 == "constant"
        s = """
            @$(c.arg2)
            D=A
            $s_push
            """
    elseif c.arg1 == "static"
        s = """
            @$(c.filename).$(c.arg2)
            D=M
            $s_push
            """
    elseif c.arg1 ∈ ["pointer", "temp"]
        s = """
            @$(d[c.arg1])
            D=A
            @$(c.arg2)
            A=D+A
            D=M
            $s_push
            """
    end

    return s
end

function code(c::C_POP)
    

    d = Dict("argument" => "ARG", "local" => "LCL",
             "this" => "THIS", "that" => "THAT",
             "pointer" => "THIS", "temp" => "R5")

    s = ""
    s_pop = """
             @R13
             M=D
             @SP
             AM=M-1
             D=M
             @R13
             A=M
             M=D"""

    if c.arg1 ∈ ["argument", "local", "this", "that"]
        s = """
            @$(d[c.arg1])
            D=M
            @$(c.arg2)
            D=D+A
            $s_pop
            """
    elseif c.arg1 == "static"
        s = """
            @$(c.filename).$(c.arg2)
            D=A
            $s_pop
            """
    elseif c.arg1 ∈ ["pointer", "temp"]
        s = """
            @$(d[c.arg1])
            D=A
            @$(c.arg2)
            D=D+A
            $s_pop
            """
    end

    return s
end


function code!(vm::VM)

    for command ∈ vm.commands
        push!(vm.asms, code(command))
    end

end