# Create Assembler from scratch!

class Assembler:
  # 引数はアセンブリファイルのパス
  # 読み込んだアセンブリをリスト形式で保管する
  def __init__(self, read_path, write_path) -> None:
    with open(read_path, mode='r') as f:
      assembly = f.read().splitlines()
    self.assembly = [i.strip().split()[0] for i in assembly if i[0:2] != '//' and i != '']
    self.write_path = write_path

  # それぞれのコマンドをバイナリに変換し、リストとして保持
  # 最後に保存したリストをファイルとして書き込み
  def main(self):
    hack_list = []
    parser = Parser(self.assembly)
    table = SymbolTable()

    table_address = 0

    # 1回目のパス
    while parser.hasMoreCommands():
      command_type = parser.commandType()

      if command_type == 'C_COMMAND' or command_type == 'A_COMMAND':
        table_address += 1
      # L_COMMANDのときにシンボル解決を行う
      else:
        symbol = parser.symbol()
        table.addEntry(symbol, table_address)

      parser.advance()

    # Parserを新しく作り直す
    parser = Parser(self.assembly)

    next_ram = 16

    # 2回目のパス
    while parser.hasMoreCommands():
      # まずは命令の種類を把握する
      command_type = parser.commandType()

      # C命令だったときの処理
      if command_type == 'C_COMMAND':
        code = Code(
          parser.dest(),
          parser.comp(),
          parser.jump()
        )
        bin = '111' + code.comp() + code.dest() + code.jump()
        hack_list.append(bin)

      # A命令だったときの処理
      if command_type == 'A_COMMAND':
        symbol = parser.symbol()
        # A命令のvalueが数字のとき
        if symbol[0].isdecimal():
          num = int(symbol)
          bin = '0' + format(num, '015b')
        # A命令のvalueが変数のとき
        else:
          # 変数がすでにTableに入っているときはそれを読み込む
          if table.contains(symbol):
            address = table.getAddress(symbol)
          # Tableに変数が含まれてないときは新たに書き込む（シンボル解決）
          else:
            table.addEntry(symbol, next_ram)
            address = next_ram
            next_ram += 1
          bin = '0' + format(address, '015b')
        hack_list.append(bin)

      parser.advance()

    with open(self.write_path, mode='w') as f:
      f.write('\n'.join(hack_list))


class Parser:
  # コンストラクタ/初期化
  def __init__(self, file) -> None:
    self.file = file
    self.length = len(self.file)
    self.current = 0

  def hasMoreCommands(self):
    return self.current < self.length

  def advance(self):
    self.current += 1

  def commandType(self):
    if (self.file[self.current][0] == '@'): return 'A_COMMAND'
    if (self.file[self.current][0] == '('): return 'L_COMMAND'
    return 'C_COMMAND'

  # A_COMMANDまたはL_COMMANDのときに呼び出される
  def symbol(self):
    # A_COMMANDのとき
    if (self.file[self.current][0] == '@'):
      return self.file[self.current][1:]
    # L_COMMANDのとき
    else:
      return self.file[self.current][1:-1]

  # C_COMMANDのときに呼び出される
  def dest(self):
    split_equal = self.file[self.current].split('=')
    if (len(split_equal) == 1): return 'null'
    return split_equal[0]

  # C_COMMANDのときに呼び出される
  def comp(self):
    split_equal = self.file[self.current].split('=')
    split_semicolon = self.file[self.current].split(';')
    if (len(split_equal) != 1):
      comp = split_equal[1]
    else:
      comp = split_semicolon[0]
    return comp

  # C_COMMANDのときに呼び出される
  def jump(self):
    split_semicolon = self.file[self.current].split(';')
    if (len(split_semicolon) == 1): return 'null'
    return split_semicolon[1]


class Code:
  def __init__(self, dest_mnemonic, comp_mnemonic, jump_mnemonic) -> None:
    self.dest_mnemonic = dest_mnemonic
    self.comp_mnemonic = comp_mnemonic
    self.jump_mnemonic = jump_mnemonic

  def dest(self):
    if (self.dest_mnemonic == 'M'): return '001'
    if (self.dest_mnemonic == 'D'): return '010'
    if (self.dest_mnemonic == 'MD'): return '011'
    if (self.dest_mnemonic == 'A'): return '100'
    if (self.dest_mnemonic == 'AM'): return '101'
    if (self.dest_mnemonic == 'AD'): return '110'
    if (self.dest_mnemonic == 'AMD'): return '111'
    return '000' # mnimonic == 'null'

  def comp(self):
    if (self.comp_mnemonic == '1'): return '0111111'
    if (self.comp_mnemonic == '-1'): return '0111010'
    if (self.comp_mnemonic == 'D'): return '0001100'
    if (self.comp_mnemonic == 'A'): return '0110000'
    if (self.comp_mnemonic == '!D'): return '0001101'
    if (self.comp_mnemonic == '!A'): return '0110001'
    if (self.comp_mnemonic == '-D'): return '0001111'
    if (self.comp_mnemonic == '-A'): return '0110011'
    if (self.comp_mnemonic == 'D+1'): return '0011111'
    if (self.comp_mnemonic == 'A+1'): return '0110111'
    if (self.comp_mnemonic == 'D-1'): return '0001110'
    if (self.comp_mnemonic == 'A-1'): return '0110010'
    if (self.comp_mnemonic == 'D+A'): return '0000010'
    if (self.comp_mnemonic == 'D-A'): return '0010011'
    if (self.comp_mnemonic == 'A-D'): return '0000111'
    if (self.comp_mnemonic == 'D&A'): return '0000000'
    if (self.comp_mnemonic == 'D|A'): return '0010101'
    if (self.comp_mnemonic == 'M'): return '1110000'
    if (self.comp_mnemonic == '!M'): return '1110001'
    if (self.comp_mnemonic == '-M'): return '1110011'
    if (self.comp_mnemonic == 'M+1'): return '1110111'
    if (self.comp_mnemonic == 'M-1'): return '1110010'
    if (self.comp_mnemonic == 'D+M'): return '1000010'
    if (self.comp_mnemonic == 'D-M'): return '1010011'
    if (self.comp_mnemonic == 'M-D'): return '1000111'
    if (self.comp_mnemonic == 'D&M'): return '1000000'
    if (self.comp_mnemonic == 'D|M'): return '1010101'
    return '0101010' # mnimonic == '0'

  def jump(self):
    if (self.jump_mnemonic == 'JGT'): return '001'
    if (self.jump_mnemonic == 'JEQ'): return '010'
    if (self.jump_mnemonic == 'JGE'): return '011'
    if (self.jump_mnemonic == 'JLT'): return '100'
    if (self.jump_mnemonic == 'JNE'): return '101'
    if (self.jump_mnemonic == 'JLE'): return '110'
    if (self.jump_mnemonic == 'JMP'): return '111'
    return '000' # mnimonic == 'null'


class SymbolTable:
  def __init__(self) -> None:
    self.table = {
      'SP': 0,
      'LCL': 1,
      'ARG': 2,
      'THIS': 3,
      'THAT': 4,
      'R0': 0,
      'R1': 1,
      'R2': 2,
      'R3': 3,
      'R4': 4,
      'R5': 5,
      'R6': 6,
      'R7': 7,
      'R8': 8,
      'R9': 9,
      'R10': 10,
      'R11': 11,
      'R12': 12,
      'R13': 13,
      'R14': 14,
      'R15': 15,
      'SCREEN': 16384,
      'KBD': 24576
    }

  def addEntry(self, symbol, address):
    self.table[symbol] = address

  def contains(self, symbol):
    return symbol in self.table

  def getAddress(self, symbol):
    return self.table[symbol]


if __name__ == "__main__":
  asm_1 = Assembler('./add/Add.asm', './add/Add.hack')
  asm_1.main()
  asm_2 = Assembler('./max/Max.asm', './max/Max.hack')
  asm_2.main()
  asm_3 = Assembler('./max/MaxL.asm', './max/MaxL.hack')
  asm_3.main()
  asm_4 = Assembler('./pong/Pong.asm', './pong/Pong.hack')
  asm_4.main()
  asm_5 = Assembler('./pong/PongL.asm', './pong/PongL.hack')
  asm_5.main()
  asm_6 = Assembler('./rect/Rect.asm', './rect/Rect.hack')
  asm_6.main()
  asm_7 = Assembler('./rect/RectL.asm', './rect/RectL.hack')
  asm_7.main()
