require "pp"

# http://pgate1.at-ninja.jp/NES_on_FPGA/nes_cpu.htm#clock

txt = open("misc/cpu_inst.txt").read
#pp txt

# "Zero Page,X   ADC $44,X     $75   2   4"

table = txt.lines.map { |line|
  l = line.chomp

  next if l.size == 0
  #puts l

  am = l[0..13].strip
  cs = l[13..-1].split
  #puts cs.size
  [am] + cs
}.select {|e| e }

pp table

ams = table.map {|e|
  e[0]
}.uniq.sort

ops = table.map {|e|
  e[1]
}.uniq.sort

icodes = table.map {|e|
  e[3][1..-1]
}.uniq.sort

pp ams
pp ops
pp icodes
