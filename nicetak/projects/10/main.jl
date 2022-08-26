include("../jackanalyzer/jackanalyzer.jl")


function main()

    dirs = ["ExpressionLessSquare", "Square", "ArrayTest"]
    for dir ∈ dirs
        analyze("nicetak/projects/10/$dir", export_token = false)
    end
    
end

main()

