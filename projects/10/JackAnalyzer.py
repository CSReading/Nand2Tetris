import re

class JackAnalyzer:
  def __init__(self, read_path):
    self.read_path = read_path
    self.write_path = read_path[:-5] + '.xml'
    with open(self.read_path, mode='r') as f:
      jack = f.read().splitlines()
    jack_lines = [deleteComments(i.strip()) for i in jack if i[0:2] != '//' and i[0:2] != '/*' and i[0:2] != ' *' and i[0:6] != '   /**' and i[0:7] != '    /**']
    tokens = []
    for line in jack_lines:
      x = re.split(r'(".*")', line)
      for y in x:
        if y == '':
          break
        if y[0] == '"':
          tokens.append(y)
        else:
          z = re.split(r'(\W)', y)
          tokens.extend([i for i in z if i != '' and i != ' '])
    self.tokens = tokens

  def main(self):
    xml_codes = self.compile()

    with open(self.write_path, mode="w", encoding="utf-8") as f:
      f.write('\n'.join(xml_codes))

  def compile(self):
    tokenizer = JackTokenizer(self.tokens)
    xml_codes = []
    xml_codes.append('<class>')
    xml_codes.append(self.compile_line(tokenizer)) # <keyword> class </keyword>
    xml_codes.append(self.compile_line(tokenizer)) # <identifier> Main </identifier>
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> { </symbol>

    xml_codes.extend(self.compileClassVarDec(tokenizer))
    xml_codes.extend(self.compileSubroutine(tokenizer))

    xml_codes.append(self.compile_line(tokenizer)) # <symbol> } </symbol>
    xml_codes.append('</class>')
    return xml_codes

  def compile_line(self, tokenizer):
    token_type = tokenizer.tokenType()
    if token_type == 'stringConstant':
      token = tokenizer.token()[1:-1]
    elif tokenizer.token() in ['<', '>', '&']:
      if tokenizer.token() == '<':
        token = '&lt;'
      elif tokenizer.token() == '>':
        token = '&gt;'
      else:
        token = '&amp;'
    else:
      token = tokenizer.token()
    tokenizer.advance()
    return "<{}> {} </{}>".format(token_type, token, token_type)

  def compileClassVarDec(self, tokenizer):
    if tokenizer.token() in ['static', 'field']:
      xml_codes = []
      xml_codes.append('<classVarDec>')
      while True:
        if tokenizer.token() == ';':
          break
        xml_codes.append(self.compile_line(tokenizer))
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> ; </symbol>
      xml_codes.append('</classVarDec>')
      xml_codes.extend(self.compileClassVarDec(tokenizer))  # classVarDecが連続するとき
      return xml_codes
    else:
      return []

  def compileSubroutine(self, tokenizer):
    if tokenizer.token() in ['constructor', 'function', 'method']:
      xml_codes = []
      xml_codes.append('<subroutineDec>')
      xml_codes.append(self.compile_line(tokenizer)) # <keyword> function </keyword>
      xml_codes.append(self.compile_line(tokenizer)) # <keyword> void </keyword>
      xml_codes.append(self.compile_line(tokenizer)) # <identifier> main </identifier>
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> ( </symbol>
      xml_codes.extend(self.compileParameterList(tokenizer))
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> ) </symbol>
      xml_codes.extend(self.compileSubroutineBody(tokenizer))
      xml_codes.append('</subroutineDec>')
      xml_codes.extend(self.compileSubroutine(tokenizer)) # subroutineDecが連続するとき
      return xml_codes
    else:
      return []

  def compileParameterList(self, tokenizer):
    xml_codes = []
    xml_codes.append('<parameterList>')

    while True:
      if tokenizer.token() == ')':
        break
      xml_codes.append(self.compile_line(tokenizer))

    xml_codes.append('</parameterList>')
    return xml_codes

  def compileSubroutineBody(self, tokenizer):
    xml_codes = []
    xml_codes.append('<subroutineBody>')
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> { </symbol>

    while tokenizer.token() == 'var':
      xml_codes.extend(self.compileVarDec(tokenizer))

    xml_codes.extend(self.compileStatements(tokenizer))

    xml_codes.append(self.compile_line(tokenizer)) # <symbol> } </symbol>
    xml_codes.append('</subroutineBody>')
    return xml_codes

  def compileVarDec(self, tokenizer):
    xml_codes = []
    xml_codes.append('<varDec>')
    while True:
      if tokenizer.token() == ';':
        break
      xml_codes.append(self.compile_line(tokenizer))
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> ; </symbol>
    xml_codes.append('</varDec>')
    return xml_codes

  def compileStatements(self, tokenizer):
    xml_codes = []
    xml_codes.append('<statements>')

    while tokenizer.token() in ['do', 'let', 'while', 'return', 'if']:
      if tokenizer.token() == 'do':
        xml_codes.extend(self.compileDo(tokenizer))
      if tokenizer.token() == 'let':
        xml_codes.extend(self.compileLet(tokenizer))
      if tokenizer.token() == 'while':
        xml_codes.extend(self.compileWhile(tokenizer))
      if tokenizer.token() == 'return':
        xml_codes.extend(self.compileReturn(tokenizer))
      if tokenizer.token() == 'if':
        xml_codes.extend(self.compileIf(tokenizer))

    xml_codes.append('</statements>')
    return xml_codes

  def compileDo(self, tokenizer):
    xml_codes = []
    xml_codes.append('<doStatement>')
    xml_codes.append(self.compile_line(tokenizer)) # <keyword> do </keyword>
    xml_codes.extend(self.compileSubroutineCall(tokenizer))
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> ; </symbol>
    xml_codes.append('</doStatement>')
    return xml_codes

  def compileLet(self, tokenizer):
    xml_codes = []
    xml_codes.append('<letStatement>')
    xml_codes.append(self.compile_line(tokenizer)) # <keyword> let </keyword>
    xml_codes.append(self.compile_line(tokenizer)) # <identifier> game </identifier>

    if tokenizer.token() == '[':
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> [ </symbol>
      xml_codes.extend(self.compileExpression(tokenizer))
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> ] </symbol>

    xml_codes.append(self.compile_line(tokenizer)) # <symbol> = </symbol>
    xml_codes.extend(self.compileExpression(tokenizer))
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> ; </symbol>
    xml_codes.append('</letStatement>')
    return xml_codes

  def compileWhile(self, tokenizer):
    xml_codes = []
    xml_codes.append('<whileStatement>')
    xml_codes.append(self.compile_line(tokenizer)) # <keyword> while </keyword>
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> ( </symbol>
    xml_codes.extend(self.compileExpression(tokenizer))
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> ) </symbol>
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> { </symbol>
    xml_codes.extend(self.compileStatements(tokenizer))
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> } </symbol>
    xml_codes.append('</whileStatement>')
    return xml_codes

  def compileReturn(self, tokenizer):
    xml_codes = []
    xml_codes.append('<returnStatement>')
    xml_codes.append(self.compile_line(tokenizer)) # <keyword> return </keyword>
    if tokenizer.token() != ';':
      xml_codes.extend(self.compileExpression(tokenizer))
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> ; </symbol>
    xml_codes.append('</returnStatement>')
    return xml_codes

  def compileIf(self, tokenizer):
    xml_codes = []
    xml_codes.append('<ifStatement>')
    xml_codes.append(self.compile_line(tokenizer)) # <keyword> if </keyword>
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> ( </symbol>
    xml_codes.extend(self.compileExpression(tokenizer))
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> ) </symbol>
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> { </symbol>
    xml_codes.extend(self.compileStatements(tokenizer))
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> } </symbol>
    if tokenizer.token() == 'else':
      xml_codes.append(self.compile_line(tokenizer)) # <keyword> else </keyword>
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> { </symbol>
      xml_codes.extend(self.compileStatements(tokenizer))
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> } </symbol>
    xml_codes.append('</ifStatement>')
    return xml_codes

  def compileExpression(self, tokenizer):
    xml_codes = []
    xml_codes.append('<expression>')
    xml_codes.extend(self.compileTerm(tokenizer))

    while tokenizer.token() in ['+', '-', '*', '/', '&', '|', '<', '>', '=']:
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> / </symbol>
      xml_codes.extend(self.compileTerm(tokenizer))
    xml_codes.append('</expression>')
    return xml_codes

  def compileTerm(self, tokenizer):
    xml_codes = []
    xml_codes.append('<term>')

    if tokenizer.tokenType() in ['keyword', 'integerConstant', 'stringConstant']:
      xml_codes.append(self.compile_line(tokenizer))

    elif tokenizer.token() == '(':
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> ( </symbol>
      xml_codes.extend(self.compileExpression(tokenizer))
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> ) </symbol>

    elif tokenizer.next_token() in ['.']:
      xml_codes.extend(self.compileSubroutineCall(tokenizer))

    elif tokenizer.token() in ['-', '~']:
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> - </symbol>
      xml_codes.extend(self.compileTerm(tokenizer))

    else:
      xml_codes.append(self.compile_line(tokenizer)) # <identifier> a </identifier>
      if tokenizer.token() in['(', '[']:
        xml_codes.append(self.compile_line(tokenizer)) # <symbol> ( </symbol>
        xml_codes.extend(self.compileExpression(tokenizer))
        xml_codes.append(self.compile_line(tokenizer)) # <symbol> ) </symbol>

    xml_codes.append('</term>')
    return xml_codes

  def compileSubroutineCall(self, tokenizer):
    xml_codes = []
    xml_codes.append(self.compile_line(tokenizer)) # <identifier> Output </identifier>
    if tokenizer.token() == '.':
      xml_codes.append(self.compile_line(tokenizer)) # <symbol> . </symbol>
      xml_codes.append(self.compile_line(tokenizer)) # <identifier> printInt </identifier>
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> ( </symbol>
    xml_codes.extend(self.compileExpressionList(tokenizer))
    xml_codes.append(self.compile_line(tokenizer)) # <symbol> ) </symbol>
    return xml_codes

  def compileExpressionList(self, tokenizer):
    xml_codes = []
    xml_codes.append('<expressionList>')
    while True:
      if tokenizer.token() != ')':
        if tokenizer.token() == ',':
          xml_codes.append(self.compile_line(tokenizer))
        xml_codes.extend(self.compileExpression(tokenizer))
      else:
        break
    xml_codes.append('</expressionList>')
    return xml_codes


class JackTokenizer:
  def __init__(self, tokens):
    self.tokens = tokens
    self.length = len(self.tokens)
    self.current = 0

  def hadMoreTokens(self):
    return self.current < self.length

  def advance(self):
    self.current += 1

  def token(self):
    return self.tokens[self.current]

  def next_token(self):
    return self.tokens[self.current + 1]

  def tokenType(self):
    if self.tokens[self.current] in keyword_list:
      return 'keyword'
    if self.tokens[self.current] in symbol_list:
      return 'symbol'
    if re.match(r'\d+', self.tokens[self.current]) != None:
      return 'integerConstant'
    if self.tokens[self.current][0] == '"':
      return 'stringConstant'
    return 'identifier'


keyword_list = [
  'class', 'constructor', 'function', 'method', 'field', 'static', 'var',
  'int', 'char', 'boolean', 'void', 'true', 'false', 'null', 'this',
  'let', 'do', 'if', 'else', 'while', 'return'
]

symbol_list = [
  '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&',
  '|', '<', '>', '=', '~'
]

# パースの際にコメントを削除して純粋なコマンドだけを返す
def deleteComments(command):
  split_command = command.strip().split()
  if '//' in split_command:
    idx = split_command.index('//')
    split_command = [split_command[i] for i in range(idx)]
  split_command = ' '.join(split_command)
  return split_command

dir_files = [
  "ArrayTest/Main.jack",
  "ExpressionLessSquare/Main.jack",
  "ExpressionLessSquare/Square.jack",
  "ExpressionLessSquare/SquareGame.jack",
  "Square/Main.jack",
  "Square/Square.jack",
  "Square/SquareGame.jack"
]

if __name__ == "__main__":
  for file in dir_files:
    jack_analyzer = JackAnalyzer("./{}".format(file))
    jack_analyzer.main()
