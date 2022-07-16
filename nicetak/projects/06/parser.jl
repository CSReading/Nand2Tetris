abstract type Command end

struct CommandA <: Command

    sym::String
    num::Int64

end

struct CommandC <: Command

    dest::String
    comp::String
    jump::String

end

struct CommandL <: Command

    sym::String

end

@with_kw struct ASM

    lines::Vector{String}
    st::SymTable = SymTable()
    commands::Vector{Command} = Vector{Command}()
    bins::Vector{String} = Vector{String}()
    
end

@enum TypeCommand A C L

function command_type(line)

    pf = line[begin]

    if pf == '@'
        return A
    elseif pf ∈ ['0', '1', 'D', 'A', '!', '-', 'M']
        return C
    elseif pf == '('
        return L
    else
        ErrorException("Cannot Identify the Command Type")
    end

end


function parse!(asm::ASM)

    # First Path (Create Sym. Table)
    for line in asm.lines

        if command_type(line) == L
            sym = line[begin+1:end-1]
            push!(asm.st.d, sym => asm.st.cnt_rom)
        else
            asm.st.cnt_rom += 1
        end

    end

    # Second Path

    for line in asm.lines

        if command_type(line) == A

            sym = line[begin+1:end]
            num = tryparse(Int64, sym)

            if isnothing(num) # Symbol

                if !haskey(asm.st.d, sym)
                    
                    push!(asm.st.d, sym => asm.st.cnt_ram)
                    asm.st.cnt_ram += 1

                end

                push!(asm.commands, CommandA(sym, asm.st.d[sym]))

            else
                push!(asm.commands, CommandA("num", num))
            end
        
        elseif command_type(line) == C

            dest, comp, jump = occursin("=", line) ?  (split(line, "=")..., "") : ("", split(line, ";")...)
    
            push!(asm.commands, CommandC(dest, comp, jump))
                
        end

    end

end
