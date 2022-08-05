module VMTranslator

export vmtranslate

include("parser.jl")
include("code.jl")

function vmtranslate_file(filename::String)

    
    basename = split(split(filename, ".vm")[begin], "/")[end]
    dirname = split(filename, "$basename.vm")[begin]
    
    lines = [split(line, "//")[begin] for line ∈ readlines(filename) if length(line) > 0 && line[begin] ≠ '/']
    vm = VM(filename=basename, lines=lines)

    parse!(vm)
    code!(vm)

    open("$dirname$basename.asm", "w") do f

        write(f, join(vm.asms, ""))
        
    end;

end

function vmtranslate(pathname::String)

    if occursin(".vm", pathname)
        vmtranslate_file(pathname)
    else
        vmfiles = [file for file ∈ readdir(pathname, join = true) if occursin(".vm", file)]

        for vmfile ∈ vmfiles
            vmtranslate_file(vmfile)
        end

    end

end

end