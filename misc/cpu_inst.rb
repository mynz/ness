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

#pp ams
#pp ops
#pp icodes


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
  
  #p ec

  ec = case ec
  when "" then "Zero"
  when "+1" then "One"
  when "+1or2" then "OneOrTwo"
  end

  #"#{o}, #{am}, #{sz}, #{cl}, #{ec}"

  ret = <<-"EOS"
  InstSpec {  // #{idx}
    code: #{cd},
    opcode: Opcode::#{o},
    addr_mode: AddrMode::#{am},
    size: #{sz},
    cycle: #{cl},
    ext_cycle: ExtCycle::#{ec},
  },
  EOS
  ret
end

256.times { |i|
  e = orderd.assoc(i)
  e = orderd.assoc(0xea) unless e # NOP

  l = fmt(e[1], i)
  #puts l + ",  // #{i}"
  puts l
}
