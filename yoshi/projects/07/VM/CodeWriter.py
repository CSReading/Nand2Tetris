class CodeWriter:
    def __init__(self):
        pass

    def setFileName(self, dest):
        self.outfile = open(dest, "w")

    def writeArithmetic(self, command):
        trans=""
        if command == "add":
            trans += "@SP\n"
            trans += "AM=M-1\n"
            trans += "D=M\n"
            trans += "@SP\n"
            trans += "AM=M-1\n"
            trans += "M=D+M\n"
            trans += "@SP\n"
            trans += "M=M+1\n"

        self.outfile.write(trans)

    def writePushPop(self, command, segment, index):
        trans=""
        if command == "push":
            if segment == "constant":
                trans += "@" + index + "\n"
                trans += "D=A\n"
                trans += "@SP\n"
                trans += "A=M\n"
                trans += "M=D\n"
                trans += "@SP\n"
                trans += "M=M+1\n"

        self.outfile.write(trans)
    
    def close(self):
        self.outfile.close()