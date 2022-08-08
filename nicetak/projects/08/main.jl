include("../vmtranslator/vmtranslator.jl")

using .VMTranslator

function main()

    #vmtranslate("nicetak/projects/08/ProgramFlow/BasicLoop")
    #vmtranslate("nicetak/projects/08/ProgramFlow/FibonacciSeries")
    #vmtranslate("nicetak/projects/08/FUnctionCalls/SimpleFunction")
    vmtranslate("nicetak/projects/08/FUnctionCalls/FibonacciElement")
    vmtranslate("nicetak/projects/08/FUnctionCalls/StaticsTest")
    vmtranslate("nicetak/projects/08/FUnctionCalls/NestedCall")

end

main()