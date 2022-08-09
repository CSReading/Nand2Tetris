import re

class Parser:
  def __init__(self, file):
    self.commands = open(file).readlines()
    self.command_number = -1

  def has_more_commands(self):
    return self.command_number < len(self.commands) - 1

  def advance(self):
    self.command_number += 1
    self.command = self._strip_white_space(self._strip_comments(self.commands[self.command_number]))

  def command_type(self):
    if '@' in self.command:
      return 'A_COMMAND'
    elif '=' in self.command or ';' in self.command:
      return 'C_COMMAND'
    elif '(' in self.command and ')' in self.command:
      return 'L_COMMAND'

  def symbol(self):
    if '(' in self.command and ')' in self.command:
      return re.findall(r'\(([.$:\w]+)\)', self.command)[0]
    elif '@' in self.command:
      return self.command.split('@')[1]

  def dest(self):
    if '=' in self.command:
      return self.command.split('=')[0]
    else:
      return ''

  def comp(self):
    partial = re.sub(r'.*=', '', self.command)
    return re.sub(r';.*', '', partial)

  def jump(self):
    if ';' in self.command:
      return self.command.split(';')[-1]
    else:
      return ''

  def _strip_comments(self, line):
    return line.split('//')[0]

  def _strip_white_space(self, line):
    return ''.join(line.split())