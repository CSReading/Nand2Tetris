import sys
from Parser import Parser
from CodeWriter import CodeWriter

def main():
    path = sys.argv[1]
    parser = Parser(path + ".vm")
    writer = CodeWriter()
    writer.setFileName(path + ".asm")

    while parser.hasMoreCommands():
        parser.advance()

        cType = parser.commandType()
        if cType == "push" or cType == "pop":
            writer.writePushPop(cType, parser.arg1(), parser.arg2())
        elif cType == "math":
            writer.writeArithmetic(parser.command[0])

    writer.close()

if __name__=='__main__':
    main()
