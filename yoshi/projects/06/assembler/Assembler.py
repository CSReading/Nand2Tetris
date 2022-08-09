import Parser, Code, SymbolTable

class Assembler:
    def __init__(self):
        self.symbols = SymbolTable.SymbolTable()
        self.symbol_addr = 16

    