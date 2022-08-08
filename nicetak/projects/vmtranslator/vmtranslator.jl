module VMTranslator

export vmtranslate

include("code.jl")



function file_to_vm(filename::String)
    
    basename = split(split(filename, ".vm")[begin], "/")[end]

    lines = [split(line, "//")[begin] for line ∈ readlines(filename) if length(line) > 0 && line[begin] ≠ '/']
    vm = VM(filename=basename, lines=lines)

    parse!(vm)
    code!(vm)

    return vm

end


function vmtranslate(pathname::String)

    if occursin(".vm", pathname)

        basename = split(split(pathname, ".vm")[begin], "/")[end]
        dirname = split(pathname, "$basename.vm")[begin]

        vm = file_to_vm(pathname)

        open("$dirname$basename.asm", "w") do f

            write(f, join(vm.asms, ""))
            
        end

    else

        basename = split(pathname, "/")[end]
        dirname = split(pathname, basename)[begin]

        vmfiles = [file for file ∈ readdir(pathname) if occursin(".vm", file)]
            
        vms = [file_to_vm("$pathname/$vmfile") for vmfile ∈ vmfiles]
        asms = vcat([vm.asms for vm ∈ vms]...) 

        open("$dirname$basename/$basename.asm", "w") do f

            write(f, join(asms, ""))
            
        end

    end

end

end