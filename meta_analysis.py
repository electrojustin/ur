import os

dumb = 0
smart = 0

for i in range(0, 100):
  if os.system("cargo run ur") == 0:
    dumb += 1
  else:
    smart += 1

print('Dumb AI: ' + str(dumb))
print('Smart AI: ' + str(smart))
