# module JackAnalyzer

# export analyze

include("tokenize.jl")

function analyze(pathname::String; export_token = false)

    if occursin(".jack", pathname)

        file_to_jack(pathname, export_token = export_token)
    
    else
        
        jackfiles = [file for file ∈ readdir(pathname) if occursin(".jack", file)]

        for file ∈ jackfiles

            file_to_jack("$pathname/$file", export_token = export_token)

        end

    end

end


function file_to_jack(filename::String; export_token = false)

    basename = split(split(filename, ".jack")[begin], "/")[end]
    dirname = split(filename, "$basename.jack")[begin]

    jack = Jack(filename=basename)


    # Omit Comment
    lines = omit_comment(readlines(filename))

    for line ∈ lines
        
        if length(line) > 0
            push!(jack.lines, line)
        end

    end
    
    # Tokenize
    tokenize!(jack)


    # Export 
    if export_token

        open("$dirname$(basename)T.xml", "w") do f
            write(f, join(jack.str_tokens, "\n"))
        end

        println("$basename.jack is tokenized.")
    end

end

function omit_comment(lines)

    line_joined = join(lines, "\n")
    line_joined_omitted = replace(line_joined, r"\/\*[\s\S]*?\*\/" => "")

    return [replace(line, r"\/\/.*$" => "") for line ∈ split(line_joined_omitted, "\n")]

end

# end
