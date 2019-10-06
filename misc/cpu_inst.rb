require "pp"

# http://pgate1.at-ninja.jp/NES_on_FPGA/nes_cpu.htm#clock

txt = open("misc/cpu_inst.txt").read

extra = <<"EOS"
IndirectX     LAX           $A3   2   6
ZeroPage      LAX           $A7   2   3
Absolute      LAX           $AF   3   4
IndirectY     LAX           $B3   2   5
ZeroPageY     LAX           $B7   2   4
AbsoluteY     LAX           $BF   3   4
EOS

txt += extra

table = txt.lines.map { |line|
  l = line.chomp

  next if l.size == 0
  #puts l

  am = l[0..13].strip
  op = l[14..17].strip
  ol = l[18..27].strip
  cd = l[28+1..33].strip.to_i(16)
  ex = l[34..37].strip
  cl = l[38..39].strip

  ec = l.size >= 40 ? l[40..-1].strip : ""
  #pp cl, ec

  [ am, op, ol, cd, ex, cl, ec]

  #cs = l[13..-1].split
  #puts cs.size
  #[am] + cs
}.select {|e| e }

#pp table
pp table[0..5]

ams = table.map {|e|
  e[0]
}.uniq.sort

ops = table.map {|e|
  e[1]
}.uniq.sort

orderd = table.sort {|a, b|
  a[3] <=> b[3]
}.map { |e|
  [e[3], e]
}

def fmt(e, idx)
  o = e[1]
  am = e[0]
  am.delete! ", "
  am = am + "(0)" unless ["Implied", "Accumulator"].include? am

  cd = e[3]
  sz = e[4]
  cl = e[5]
  ec = e[6]

  ec = case ec
  when "" then "Zero"
  when "+1" then "One"
  when "+1or2" then "OneOrTwo"
  when "undefined" then "Undefined"
  end

  #"#{o}, #{am}, #{sz}, #{cl}, #{ec}"

  ret = <<-"EOS"
  InstSpec {  // #{idx}
    code: 0x#{cd.to_s(16)},
    opcode: Opcode::#{o},
    operand: Operand::#{am},
    size: #{sz},
    cycles: #{cl},
    ext_cycles: ExtCycles::#{ec},
  },
  EOS
  ret
end

0xff.times { |i|
  e = orderd.assoc(i)

  unless e
    e = orderd.assoc(0xea).dup

    low = i & 0x0F

    operand, size = case low
    when 0x0 then ["Immediate", 2]
    when 0x4 then ["ZeroPage", 2]
    when 0xa then ["Implied", 1]
    when 0xc then ["Absolute", 3]
    else ["Implied",1]
    end

    e[1][0] = operand
    e[1][1] = "NOP"
    e[1][3] = i
    e[1][4] = size
    e[1][6] = "undefined"
  end

  l = fmt(e[1], i)
  puts l
}
