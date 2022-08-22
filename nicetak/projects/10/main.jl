include("../jackanalyzer/jackanalyzer.jl")


function main()


    dirs = ["Square", "ArrayTest"]
    for dir ∈ dirs
        analyze("nicetak/projects/10/$dir", export_token = true)
    end
    
end

main()

