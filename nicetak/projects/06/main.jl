using Parameters

include("sym_table.jl")
include("parser.jl")
include("code.jl")


function assemble(filename::String)

    basename, _ = split(filename, ".asm")

    lines = [replace(split(line, "//")[begin], " " => "") for line ∈ readlines(filename) if length(line) > 0 && line[begin] ≠ '/']
    asm = ASM(lines=lines)

    parse!(asm)
    code!(asm)

    open("$basename.hack", "w") do f

        write(f, join(asm.bins, "\n"))
        
    end;

end

assemble("nicetak/projects/06/add/Add.asm")
assemble("nicetak/projects/06/max/MaxL.asm")
assemble("nicetak/projects/06/rect/RectL.asm")
assemble("nicetak/projects/06/pong/PongL.asm")

## Symbol
assemble("nicetak/projects/06/max/Max.asm")
assemble("nicetak/projects/06/rect/Rect.asm")
assemble("nicetak/projects/06/pong/Pong.asm")