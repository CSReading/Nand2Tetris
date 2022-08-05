# Create Virtual Machine from scratch!
# Chapter8のVMはchapter7のコードに追加する形で作成する

import os
import sys
import pathlib

class VMtranslator:
  def __init__(self, read_path) -> None:
    self.read_path = read_path
    with open(self.read_path, mode='r') as f:
      vm = f.read().splitlines()
    self.vm = [deleteComments(i) for i in vm if i[0:2] != '//' and i != '']
    root, _ = os.path.splitext(self.read_path)
    self.write_path = root + '.asm'
    self.filename = os.path.splitext(os.path.basename(self.read_path))[0]

  def main(self):
    # Parserを新しく作る
    parser = Parser(self.vm)

    # Code Writerを新しく作る
    code_writer = CodeWriter(self.write_path)
    code_writer.setFileName(self.filename)

    # ブートストラップコード
    # Class1.vm, Class2.vm, Main.vmは関数の最初じゃないのでスタックポインタを初期化しない
    if self.filename not in ["Class1", "Class2", "Main"]:
      code_writer.bootStrap()
    # Sys.vmのときはSys.initをcallしなきゃいけない
    if self.filename == "Sys":
      code_writer.writeCall("Sys.init", '0')

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

      # labelコマンドのときの処理
      if command_type == 'C_LABEL':
        code_writer.writeLabel(
          parser.segment()
        )

      # gotoコマンドのときの処理
      if command_type == 'C_GOTO':
        code_writer.writeGoto(
          parser.segment()
        )

      # if-gotoコマンドのときの処理
      if command_type == 'C_IF':
        code_writer.writeIf(
          parser.segment()
        )

      # callコマンドのときの処理
      if command_type == 'C_CALL':
        code_writer.writeCall(
          parser.segment(),
          parser.index()
        )

      # functionコマンドのときの処理
      if command_type == 'C_FUNCTION':
        code_writer.writeFunction(
          parser.segment(),
          parser.index()
        )

      # returnコマンドのときの処理
      if command_type == 'C_RETURN':
        code_writer.writeReturn()

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
    segment_symbol = ''
    if segment == 'local':
      segment_symbol = 'LCL'
    if segment == 'argument':
      segment_symbol = 'ARG'
    if segment == 'this':
      segment_symbol = 'THIS'
    if segment == 'that':
      segment_symbol = 'THAT'

    if command == 'push':
      if segment in ['local', 'argument', 'this', 'that']:
        asm_codes = [
          '@{}'.format(str(index)),
          'D=A',
          '@{}'.format(segment_symbol),
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
          '@{}'.format(segment_symbol),
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

  def writeLabel(self, label):
    asm_codes = [
      '({})'.format(label),
    ]
    self.f.write('\n'.join(asm_codes))
    self.f.write('\n')
    self.label += 1

  def writeGoto(self, label):
    asm_codes = [
      '@{}'.format(label),
      '0;JMP'
    ]
    self.f.write('\n'.join(asm_codes))
    self.f.write('\n')
    self.label += 1

  def writeIf(self, label):
    asm_codes = [
      '@SP',
      'A=M-1',
      'D=M',
      '@SP',
      'M=M-1',
      '@{}'.format(label),
      'D;JNE',
    ]
    self.f.write('\n'.join(asm_codes))
    self.f.write('\n')
    self.label += 1

  def writeCall(self, functionName, numArgs):
    numArgs = int(numArgs)
    new_label = 'RETURN_LABEL_' + str(self.label)
    asm_codes = [
      '@{}'.format(new_label),
      'D=A',
      '@SP',
      'A=M',
      'M=D',
      '@SP',
      'M=M+1',

      '@LCL',
      'D=M',
      '@SP',
      'A=M',
      'M=D',
      '@SP',
      'M=M+1',

      '@ARG',
      'D=M',
      '@SP',
      'A=M',
      'M=D',
      '@SP',
      'M=M+1',

      '@THIS',
      'D=M',
      '@SP',
      'A=M',
      'M=D',
      '@SP',
      'M=M+1',

      '@THAT',
      'D=M',
      '@SP',
      'A=M',
      'M=D',
      '@SP',
      'M=M+1',

      '@SP',
      'D=M',
      '@{}'.format(str(5 + numArgs)),
      'D=D-A',
      '@ARG',
      'M=D',

      '@SP',
      'D=M',
      '@LCL',
      'M=D',

      '@{}'.format(functionName),
      '0;JMP',

      '({})'.format(new_label)
    ]
    self.f.write('\n'.join(asm_codes))
    self.f.write('\n')
    self.label +=1

  def writeReturn(self):
    asm_codes = [
      '@LCL',
      'D=M',
      '@R13',
      'M=D',

      '@5',
      'D=A',
      '@R13',
      'A=M-D',
      'D=M',
      '@R14',
      'M=D',

      '@SP',
      'A=M-1',
      'D=M',
      '@ARG',
      'A=M',
      'M=D',

      '@ARG',
      'D=M',
      '@SP',
      'M=D+1',

      '@R13',
      'A=M-1',
      'D=M',
      '@THAT',
      'M=D',

      '@R13',
      'D=M',
      '@2',
      'A=D-A',
      'D=M',
      '@THIS',
      'M=D',

      '@R13',
      'D=M',
      '@3',
      'A=D-A',
      'D=M',
      '@ARG',
      'M=D',

      '@R13',
      'D=M',
      '@4',
      'A=D-A',
      'D=M',
      '@LCL',
      'M=D',

      '@R14',
      'A=M',
      '0;JMP'
    ]
    self.f.write('\n'.join(asm_codes))
    self.f.write('\n')
    self.label +=1

  def writeFunction(self, functionName, numLocals):
    numLocals = int(numLocals)
    asm_codes = ['({})'.format(functionName)]
    for _ in range(numLocals):
      asm_codes.extend([
        '@SP',
        'A=M',
        'M=0',
        '@SP',
        'M=M+1'
      ])
    self.f.write('\n'.join(asm_codes))
    self.f.write('\n')
    self.label += 1

  # スタックポインタの初期化
  def bootStrap(self):
    asm_codes = [
      '@256',
      'D=A',
      '@SP',
      'M=D',
    ]
    self.f.write('\n'.join(asm_codes))
    self.f.write('\n')

  def close(self):
    self.f.close()

# パースの際にコメントを削除して純粋なコマンドだけを返す
def deleteComments(command):
  split_command = command.strip().split()
  if '//' in split_command:
    idx = split_command.index('//')
    split_command = [split_command[i] for i in range(idx)]
  split_command = ' '.join(split_command)
  return split_command

if __name__ == "__main__":
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

  # Sys.asmとMain.asmを連結してFibonacciElement.asmを作る
  with open('./FunctionCalls/FibonacciElement/Sys.asm', mode='r') as f_fe_sys:
    fe_asm = f_fe_sys.read().splitlines()
  with open('./FunctionCalls/FibonacciElement/Main.asm', mode='r') as f_fe_main:
    fe_asm_main = f_fe_main.read().splitlines()
  fe_asm.extend(fe_asm_main)
  with open('./FunctionCalls/FibonacciElement/FibonacciElement.asm', mode='w') as f_fe:
    f_fe.write('\n'.join(fe_asm))

  # Sys.asmとClass1.asmとClass2.asmを連結してStaticsTest.asmを作る
  with open('./FunctionCalls/StaticsTest/Sys.asm', mode='r') as f_st_sys:
    st_asm = f_st_sys.read().splitlines()
  with open('./FunctionCalls/StaticsTest/Class1.asm', mode='r') as f_st_c1:
    st_asm_c1 = f_st_c1.read().splitlines()
  st_asm.extend(st_asm_c1)
  with open('./FunctionCalls/StaticsTest/Class2.asm', mode='r') as f_st_c2:
    st_asm_c2 = f_st_c2.read().splitlines()
  st_asm.extend(st_asm_c2)
  with open('./FunctionCalls/StaticsTest/StaticsTest.asm', mode='w') as f_st:
    f_st.write('\n'.join(st_asm))

  # Sys.vmをNestedCall.asmにRenameする
  with open('./FunctionCalls/NestedCall/Sys.asm', mode='r') as f_nc:
    nc_asm = f_nc.read().splitlines()
  with open('./FunctionCalls/NestedCall/NestedCall.asm', mode='w') as f:
    f.write('\n'.join(nc_asm))
