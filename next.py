import sys
import shutil

if len(sys.argv) > 1:
    new_problem_file = "src/bin/p{0:03d}.rs".format(int(sys.argv[1]))
    shutil.copy("src/bin/template.rs", new_problem_file)
    print(new_problem_file, " created")