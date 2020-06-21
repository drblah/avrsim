import re

pat = re.compile(r'.*?:\s+(\S{2})\s(\S{2})\s+eor\sR(\d{1,2}), R(\d{1,2})')

input_file = open("eor.dissasm", 'r')

input_array = []
register_Rd = []
register_Rr = []

for line in input_file:
    if "eor" in line:
        m = pat.match(line)  
        input_array.append( int(m.group(1) + m.group(2), base=16) )
        register_Rd.append(int(m.group(3)))
        register_Rr.append(int(m.group(4)))

print("let input_array = {}".format(input_array))
print("let register_rd = {}".format(register_Rd))
print("let register_rr = {}".format(register_Rr))
#print(register_Rd)
#print(register_Rr)