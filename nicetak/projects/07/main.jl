include("../vmtranslator/vmtranslator.jl")

using .VMTranslator

function main()

    vmtranslate("nicetak/projects/07/StackArithmetic/SimpleAdd/")
    vmtranslate("nicetak/projects/07/StackArithmetic/StackTest/")
    vmtranslate("nicetak/projects/07/MemoryAccess/BasicTest/")
    vmtranslate("nicetak/projects/07/MemoryAccess/StaticTest/")
    vmtranslate("nicetak/projects/07/MemoryAccess/PointerTest/")

end

main()