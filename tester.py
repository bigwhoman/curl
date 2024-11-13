import subprocess
for i in range(1,2):
    input_file = open(f"tests/inputs/input{i}.txt")
    
    input_command = input_file.read().split("--")[1]
    print(input_command)
    result = subprocess.run(["cargo", "run", "--", input_command], capture_output=True, text=True)
    temp_output = open(f"tests/temp/temp",'w+')
    temp_output.write(result.stderr)