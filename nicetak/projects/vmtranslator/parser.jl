using Parameters

abstract type VMCommand end

struct C_ARITHMETIC <: VMCommand

    command::String
    id::Int64 # Unique id for each command

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

end

struct C_GOTO <: VMCommand

end

struct C_IF <: VMCommand

end

struct C_FUNCTION <: VMCommand

end

struct C_RETURN <: VMCommand

end

struct C_CALL <: VMCommand

end


@with_kw struct VM

    filename::String
    lines::Vector{String}
    commands::Vector{VMCommand} = Vector{VMCommand}()
    asms::Vector{String} = Vector{String}()
    
end


function parse!(vm::VM)

    for (id, line) ∈ enumerate(vm.lines)
        
        command = split(line, " ")[begin]

        if command ∈ ["add", "sub", "and", "or", "neg", "not", "eq", "lt", "gt"]
            push!(vm.commands, C_ARITHMETIC(command, id))
        elseif command == "push"
            command, arg1, arg2 = split(line, " ")
            push!(vm.commands, C_PUSH(arg1, arg2, vm.filename))
        elseif command == "pop"
            command, arg1, arg2 = split(line, " ")
            push!(vm.commands, C_POP(arg1, arg2, vm.filename))
        end
    end

end