class Parser:
    def __init__(self, source):
        self.file = open(source)
        self.command = []
        self.advanceReachedEOF = False
        self.cType = {
            "sub" : "math",
            "add" : "math",
            "neg" : "math",
            "eq"  : "math",
            "gt"  : "math",
            "lt"  : "math",
            "and" : "math",
            "or"  : "math",
            "not" : "math",
            "push" : "push",
            "pop"  : "pop",
            "EOF"  : "EOF",
        }

    def hasMoreCommands(self):
        position = self.file.tell()
        self.advance()
        self.file.seek(position)
        return not self.advanceReachedEOF

    def advance(self):
        thisLine = self.file.readline()
        if thisLine == "":
            self.advanceReachedEOF = True
        else:
            splitLine = thisLine.split("/")[0].strip()

            if splitLine == "":
                self.advance()
            else:
                self.command = splitLine.split()

    def commandType(self):
        return self.cType.get(self.command[0], "invalid cType")

    def arg1(self):
        return self.command[1]
    
    def arg2(self):
        return self.command[2]