
include("parse.jl")

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
            @FALSE_$(c.cnt_bool)
            D;$(d_sym[c.command])
            @SP
            A=M-1
            M=-1
            @CONTINUE_$(c.cnt_bool)
            0;JMP
            (FALSE_$(c.cnt_bool))
            @SP
            A=M-1
            M=0
            (CONTINUE_$(c.cnt_bool))
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

function code(c::C_LABEL)

    s = """
        ($(c.funcname)$(c.label))
        """

    return s
end

function code(c::C_GOTO)

    s = """
        @$(c.funcname)$(c.dest)
        0;JMP
        """

    return s
end

function code(c::C_IF)

    s = """
        @SP
        M=M-1
        A=M
        D=M
        @$(c.funcname)$(c.dest)
        D;JNE
        """

    return s
end

function code(c::C_FUNCTION)
    
    s = "($(c.func))\n"

    for _ = 1:c.k
        s *= """
            @0
            D=A
            @SP
            A=M
            M=D 
            @SP
            M=M+1
            """
    end

    return s
end


function code(c::C_RETURN)

    s = """
        // R13 <- *(LCL - 5)
        @LCL
        D=M
        @5
        A=D-A
        D=M
        @R13
        M=D
        // *ARG <- pop()
        @SP
        A=M-1
        D=M
        @ARG
        A=M
        M=D
        // SP <- ARG + 1
        D=A+1
        @SP
        M=D
        // THAT <- *(LCL-1); LCL--
        @LCL
        AM=M-1
        D=M
        @THAT
        M=D
        // THIS <- *(LCL-1); LCL--
        @LCL
        AM=M-1
        D=M
        @THIS
        M=D
        // ARG <- *(LCL-1); LCL--
        @LCL
        AM=M-1
        D=M
        @ARG
        M=D
        // LCL <- *(LCL-1); LCL--
        @LCL
        AM=M-1
        D=M
        @LCL
        M=D
        // goto R13
        @R13
        A=M
        0;JMP
        """
    
    return s
end


function code(c::C_CALL)

    s = """
        @$(c.filename).RET_$(c.cnt_call)
        D=A
        @SP
        A=M
        M=D
        @SP
        M=M+1"""

    for address ∈ ["LCL", "ARG", "THIS", "THAT"]
        
        s = """
            $s
            // push $address
            @$address
            D=M
            @SP
            A=M
            M=D
            @SP
            M=M+1"""
    end

    s = """
        $s
        // LCL <- SP
        @SP
        D=M
        @LCL
        M=D
        // ARG <- SP-n-5
        @$(c.n + 5)
        D=D-A
        @ARG
        M=D
        // goto func
        @$(c.func)
        0;JMP
        // (return-address)
        ($(c.filename).RET_$(c.cnt_call))
        """

    return s
end

function code!(vm::VM)

    for command ∈ vm.commands
        push!(vm.asms, code(command))
    end

end