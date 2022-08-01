# Create Virtual Machine from scratch!
# 教科書とは実装をちょっと変えており、VMtranslatorはディレクトリを引数に取らずファイルごとに作り直す

import os
import sys
import pathlib
from turtle import st

class VMtranslator:
  def __init__(self, read_path) -> None:
    with open(read_path, mode='r') as f:
      vm = f.read().splitlines()
    self.vm = [i for i in vm if i[0:2] != '//' and i != '']
    root, _ = os.path.splitext(read_path)
    self.write_path = root + '.asm'
    self.filename = os.path.splitext(os.path.basename(read_path))[0]

  def main(self):
    # Parserを新しく作る
    parser = Parser(self.vm)

    # Code Writerを新しく作る
    code_writer = CodeWriter(self.write_path)
    code_writer.setFileName(self.filename)

    while parser.hasMoreCommands():
      command_type = parser.commandType()

      # 算術コマンドのときの処理
      if command_type == 'C_ARITHMETIC':
        code_writer.writeArithmetic(parser.command())

      # メモリアクセスコマンドのときの処理
      if command_type == 'C_PUSH' or command_type == 'C_POP':
        code_writer.writePushPop(
          parser.command(),
          parser.segment(),
          parser.index()
        )

      parser.advance()

    code_writer.close()

class Parser:
  # コンストラクタ
  def __init__(self, file) -> None:
    self.file = file
    self.length = len(self.file)
    self.current = 0

  def hasMoreCommands(self):
    return self.current < self.length

  def advance(self):
    self.current += 1

  def commandType(self):
    command = self.file[self.current].split()
    if len(command) == 1:
      if command[0] == 'return': return 'C_RETURN'
      return 'C_ARITHMETIC'
    elif len(command) == 2:
      if command[0] == 'label': return 'C_LABEL'
      if command[0] == 'goto': return 'C_GOTO'
      return 'C_IF'
    else:
      if command[0] == 'push': return 'C_PUSH'
      if command[0] == 'pop': return 'C_POP'
      if command[0] == 'function': return 'C_FUNCTION'
      return 'C_CALL'

  def command(self):
    command = self.file[self.current].split()
    return command[0]

  def segment(self):
    command = self.file[self.current].split()
    return command[1]

  def index(self):
    command = self.file[self.current].split()
    return int(command[2])

class CodeWriter:
  def __init__(self, write_path) -> None:
    self.f = open(write_path, mode="w", encoding="utf-8")
    self.fileName = ''
    self.label = 1

  def setFileName(self, fileName):
    self.fileName = fileName

  def writeArithmetic(self, command):
    if command == 'add':
      asm_codes = [
        '@SP',
        'A=M-1',
        'A=A-1',
        'D=M',
        '@SP',
        'A=M-1',
        'D=D+M',
        '@SP',
        'A=M-1',
        'A=A-1',
        'M=D',
        '@SP',
        'M=M-1',
      ]
    if command == 'sub':
      asm_codes = [
        '@SP',
        'A=M-1',
        'A=A-1',
        'D=M',
        '@SP',
        'A=M-1',
        'D=D-M',
        '@SP',
        'A=M-1',
        'A=A-1',
        'M=D',
        '@SP',
        'M=M-1',
      ]
    if command == 'neg':
      asm_codes = [
        '@SP',
        'A=M-1',
        'D=M',
        '@SP',
        'A=M-1',
        'M=-D',
      ]
    if command == 'eq':
      asm_codes = [
        '@SP',
        'A=M-1',
        'A=A-1',
        'D=M',
        '@SP',
        'A=M-1',
        'D=D-M',
        '@TRUE_' + str(self.label),
        'D;JEQ',
        '@SP',
        'A=M-1',
        'A=A-1',
        'M=0',
        '@END_' + str(self.label),
        '0;JMP',
        '(TRUE_{})'.format(str(self.label)),
        '@SP',
        'A=M-1',
        'A=A-1',
        'M=-1',
        '@END_' + str(self.label),
        '0;JMP',
        '(END_{})'.format(str(self.label)),
        '@SP',
        'M=M-1',
      ]
    if command == 'gt':
      asm_codes=[
        '@SP',
        'A=M-1',
        'A=A-1',
        'D=M',
        '@SP',
        'A=M-1',
        'D=D-M',
        '@TRUE_' + str(self.label),
        'D;JGT',
        '@SP',
        'A=M-1',
        'A=A-1',
        'M=0',
        '@END_' + str(self.label),
        '0;JMP',
        '(TRUE_{})'.format(str(self.label)),
        '@SP',
        'A=M-1',
        'A=A-1',
        'M=-1',
        '@END_' + str(self.label),
        '0;JMP',
        '(END_{})'.format(str(self.label)),
        '@SP',
        'M=M-1',
      ]
    if command == 'lt':
      asm_codes=[
        '@SP',
        'A=M-1',
        'A=A-1',
        'D=M',
        '@SP',
        'A=M-1',
        'D=D-M',
        '@TRUE_' + str(self.label),
        'D;JLT',
        '@SP',
        'A=M-1',
        'A=A-1',
        'M=0',
        '@END_' + str(self.label),
        '0;JMP',
        '(TRUE_{})'.format(str(self.label)),
        '@SP',
        'A=M-1',
        'A=A-1',
        'M=-1',
        '@END_' + str(self.label),
        '0;JMP',
        '(END_{})'.format(str(self.label)),
        '@SP',
        'M=M-1',
      ]
    if command == 'and':
      asm_codes = [
        '@SP',
        'A=M-1',
        'A=A-1',
        'D=M',
        '@SP',
        'A=M-1',
        'D=D&M',
        '@SP',
        'A=M-1',
        'A=A-1',
        'M=D',
        '@SP',
        'M=M-1',
      ]
    if command == 'or':
      asm_codes = [
        '@SP',
        'A=M-1',
        'A=A-1',
        'D=M',
        '@SP',
        'A=M-1',
        'D=D|M',
        '@SP',
        'A=M-1',
        'A=A-1',
        'M=D',
        '@SP',
        'M=M-1',
      ]
    if command == 'not':
      asm_codes = [
        '@SP',
        'A=M-1',
        'M=!M',
      ]

    self.f.write('\n'.join(asm_codes))
    self.f.write('\n')
    self.label += 1

  def writePushPop(self, command, segment, index):
    segment_sybmol = ''
    if segment == 'local':
      segment_sybmol = 'LCL'
    if segment == 'argument':
      segment_sybmol = 'ARG'
    if segment == 'this':
      segment_sybmol = 'THIS'
    if segment == 'that':
      segment_sybmol = 'THAT'

    if command == 'push':
      if segment in ['local', 'argument', 'this', 'that']:
        asm_codes = [
          '@{}'.format(str(index)),
          'D=A',
          '@{}'.format(segment_sybmol),
          'A=M+D',
          'D=M',
          '@SP',
          'A=M',
          'M=D',
          '@SP',
          'M=M+1',
        ]
      if segment in ['pointer', 'temp']:
        asm_codes = [
          '@{}'.format(str(index)),
          'D=A',
          '@{}'.format('3' if segment == 'pointer' else '5'),
          'A=D+A',
          'D=M',
          '@SP',
          'A=M',
          'M=D',
          '@SP',
          'M=M+1',
        ]
      if segment == 'constant':
        asm_codes = [
          '@{}'.format(str(index)),
          'D=A',
          '@SP',
          'A=M',
          'M=D',
          '@SP',
          'M=M+1',
        ]
      if segment == 'static':
        asm_codes = [
          '@{}.{}'.format(self.fileName, index),
          'D=M',
          '@SP',
          'A=M',
          'M=D',
          '@SP',
          'M=M+1',
        ]

    if command == 'pop':
      if segment in ['local', 'argument', 'this', 'that']:
        asm_codes = [
          '@{}'.format(str(index)),
          'D=A',
          '@{}'.format(segment_sybmol),
          'D=M+D',
          '@R13',
          'M=D',
          '@SP',
          'A=M-1',
          'D=M',
          '@R13',
          'A=M',
          'M=D',
          '@SP',
          'M=M-1',
        ]
      if segment in ['pointer', 'temp']:
        asm_codes = [
          '@{}'.format(str(index)),
          'D=A',
          '@{}'.format('3' if segment == 'pointer' else '5'),
          'D=D+A',
          '@R13',
          'M=D',
          '@SP',
          'A=M-1',
          'D=M',
          '@R13',
          'A=M',
          'M=D',
          '@SP',
          'M=M-1',
        ]
      if segment == 'static':
        asm_codes = [
          '@SP',
          'A=M-1',
          'D=M',
          '@{}.{}'.format(self.fileName, str(index)),
          'M=D',
          '@SP',
          'M=M-1',
        ]

    self.f.write('\n'.join(asm_codes))
    self.f.write('\n')
    self.label += 1

  def close(self):
    self.f.close()


if __name__ == "__main__":
  """
  このプログラムは`python VMtranslator.py source`というコマンドで走らせる。
  例: python VMtranslator.py StackArithmetic

  ただしそれが暗黙的知識なっているのは良くないので、
  本来であればargparseモジュールやシェルスクリプトなどを用いて下記直す必要がある。

  sourceが.vmファイルなのかディレクトリ名なのかの判定も行う。
  ディレクトリであればそのディレクトに走らせる.vmファイルすべてに対してVM変換を行う。
  """
  read_path_list = []
  arg_path = './{}'.format(sys.argv[1])
  if os.path.isfile(arg_path + '.vm'):
    read_path_list.append(arg_path + '.vm')
  else:
    path = pathlib.Path(arg_path)
    for file in path.glob('**/*.vm'):
      read_path_list.append('./' + str(file))

  for read_path in read_path_list:
    translator = VMtranslator(read_path)
    translator.main()
