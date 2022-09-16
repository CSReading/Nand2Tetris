include("../jackanalyzer/jackanalyzer.jl")


function main()

    #dirs = ["Seven", "ConvertToBin"]
    dirs = ["ConvertToBin"]
    for dir ∈ dirs
        analyze("nicetak/projects/11/$dir", export_token = false, export_parsed = false)
    end
    
end

main()


