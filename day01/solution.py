elf = 1
while True:
  try:
    cals = input()
    if cals != "":
      print(f"{elf}: {cals}")
    else:
      elf += 1
  except:
    exit()
