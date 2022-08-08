using Parameters

abstract type VMCommand end

struct C_ARITHMETIC <: VMCommand

    command::String
    cnt_bool::Int64

end

struct C_PUSH <: VMCommand

    arg1::String
    arg2::String
    filename::String

end

struct C_POP <: VMCommand

    arg1::String
    arg2::String
    filename::String

end

struct C_LABEL <: VMCommand

    label::String
    funcname::String

end

struct C_GOTO <: VMCommand

    dest::String
    funcname::String

end

struct C_IF <: VMCommand

    dest::String
    funcname::String

end

struct C_FUNCTION <: VMCommand

    func::String
    k::Int64

end

struct C_CALL <: VMCommand

    func::String
    n::Int64
    filename::String
    cnt_call::Int64
    
end

struct C_RETURN <: VMCommand end

@with_kw struct VM

    filename::String
    lines::Vector{String}
    commands::Vector{VMCommand} = [C_CALL("Sys.init", 0, filename, 0)]
    asms::Vector{String} = [
        """
        @256
        D=A
        @SP
        M=D
        """
    ]
    
end

function parse!(vm::VM)

    cnt_bool = 0
    cnt_call = 0
    funcname = ""


    for line ∈ vm.lines
        
        command = split(line, " ")[begin]

        if command ∈ ["add", "sub", "and", "or", "neg", "not"]
            push!(vm.commands, C_ARITHMETIC(command, cnt_bool))
        elseif command ∈ ["eq", "lt", "gt"]
            push!(vm.commands, C_ARITHMETIC(command, cnt_bool))
            cnt_bool += 1
        elseif command == "push"
            command, arg1, arg2 = split(line, " ")
            push!(vm.commands, C_PUSH(arg1, arg2, vm.filename))
        elseif command == "pop"
            command, arg1, arg2 = split(line, " ")
            push!(vm.commands, C_POP(arg1, arg2, vm.filename))
        elseif command == "label"
            command, label = split(line, " ")
            push!(vm.commands, C_LABEL(label, funcname))
        elseif command == "goto"
            command, dest = split(line, " ")
            push!(vm.commands, C_GOTO(dest, funcname))
        elseif command == "if-goto"
            command, dest = split(line, " ")
            push!(vm.commands, C_IF(dest, funcname))
        elseif command == "call"
            command, func, str_n = split(line, " ")
            push!(vm.commands, C_CALL(func, parse(Int64, str_n), vm.filename, cnt_call))
            cnt_call += 1
        elseif command == "function"
            command, func, str_n = split(line, " ")
            push!(vm.commands, C_FUNCTION(func, parse(Int64, str_n)))
            funcname = "$func\$"
        elseif command == "return"
            push!(vm.commands, C_RETURN())
        end
    end

end