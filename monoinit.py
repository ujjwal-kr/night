import os, sys

if os.name == "nt":
    sys.exit(
        "This script is designed to run only on unix based systems.\n"
        "Please use Windows Subsystem for Linux."
    )

# === IMPORT MODULES === #
import typing, argparse, readline, json, subprocess, traceback
from cmd import Cmd

# === COMPLETER === #
class Completer(object):
    def __init__(self, options):
        self.options = sorted(options)
    def complete(self, text, state):
        if state == 0: 
            if text:  
                self.matches = [s for s in self.options if s and s.startswith(text)]
            else:  
                self.matches = self.options[:]
        try: 
            return self.matches[state]
        except IndexError:
            return None

# === GET FILE PATHS FROM GITIGNORE === #
def git_ignore_scan()-> list:
    gitIgnore = False
    if ".gitignore" in os.listdir(PARENT_DIR):gitIgnore = True
    if gitIgnore:
        with open(os.path.join(PARENT_DIR, ".gitignore"),'r') as f:
            ignore = f.read().splitlines()
            ignore = [os.path.join(PARENT_DIR, k) for k in ignore]
        return ignore
    return []

# === GET TODOS === #
def todos(folder: str) -> str:
    def getFiles(path: str) -> list:
        files = []
        for i in os.listdir(path):
            if os.path.isdir(os.path.join(path, i)): files += getFiles(os.path.join(path, i))
            elif (
                b"ascii" in subprocess.run(
                    ["file", "--mime-encoding", os.path.join(path, i)],
                    capture_output=True
                ).stdout.lower() and
                os.path.join(path, i) not in IGNORE
            ): files.append(os.path.join(path, i))
        
        return files

    files, c, n = {}, os.getcwd(), '\n'
    for i in getFiles(folder):
        cmd = '#[[:blank:]]\+todo' if i.endswith('.py') else '//[[:blank:]]\+todo'
        if (x := subprocess.getoutput(
            f"grep -in -e \"{cmd}\" -e \"{'#todo' if i.endswith('.py') else '//todo'}\" {i}"
        )) == "": continue

        print(f"FILE: {i[len(PARENT_DIR)+1:]}\n\t{x}")

# === SHELL === #
def shell(command: str) -> typing.Any:
    global PARENT_DIR, exit_, IGNORE, WORKFLOW

    # === CHANGE DIRECTORY === #
    if command.lower().startswith("cd"):
        if len((x := command.strip().split())) == 1: return "The path has not been supplied"
        elif len(x) > 2: return "Extra arguments passed"
        elif not os.path.isdir(x[1]): return "Cannot find the path specified"

        # === TO CHECK IF THE PATH SPECIFIED GOES BEYOND PARENT_DIR === #
        if PARENT_DIR not in os.path.abspath(x[1]):
            print("Path specified goes beyond the parent directory")
            return

        os.chdir(x[1])
    
    # === GET TODOS === #
    elif command.lower().startswith("todos"):
        if len(command.strip().split()) > 1: return "Extra arguments passed"
        return todos(os.getcwd())

    # === UPDATE === #
    elif command.lower() == "update":
        try:
            os.chdir("/tmp")
            os.system("wget https://raw.githubusercontent.com/hackarmour/monoinit/main/main.py")
            if "main.py" in os.listdir(): 
                os.system(f"cp ./main.py {__file__}")
                os.system("rm main.py")
            else: raise Exception("Cannot receive file from upstream")
            sys.exit("MonoInit has been updated. Please rerun the script.")

        except Exception:
            traceback.print_exc()

    # === COMMIT MESSAGE === #
    elif command.lower().startswith("git"):
        # === KEEP THE CURRENT PATH === #
        cur_path = os.getcwd()


        # === IF THE COMMAND IS A COMMIT === #
        if command.lower().startswith("git commit -m"):
            # === COMMIT FORMATTING === #
            b = "\""
            commit = command.split(f"{b}")

            commit[1] = f'{os.path.basename(os.getcwd())}: {command.split(f"{b}")[1]}'

            # === RUN COMMAND === #
            os.system("\"".join(commit))
            
            # === EXIT FUNCTION === #
            return

        # === CHANGE PATH TO PARENT DIRECTORY === #
        os.chdir(PARENT_DIR)
        if command.lower().startswith("git log"):os.system("git log --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit")

        # === RUN THE GIT COMMAND SPECIFIED BY USER === #
        else:
            os.system(command)

        os.chdir(cur_path)

    # === HELP === #
    elif command.lower().startswith("help"):
        return str(
                "Command: todos\n"
                "\tUsage: todos\n"
                "\tUsed to get all the ToDos from the repos\n\n"
                "Command: init\n"
                "\tUsage: init\n"
                "\tUsed to create a new repo\n\n"
                "Command: newcommand\n"
                "\tUsage: newcommand\n"
                "\tUsed to add a new command to a repo\n\n"
                "Command: rmcommand\n"
                "\tUsage: rmcommand\n"
                "\tUsed to remove a command from a repo\n\n"
                "Command: update\n"
                "\tUsage: update\n"
                "\tTo update monoinit\n\n"
                "Command: exit\n"
                "\tUsage: exit\n"
                "\tTo exit the shell\n\n"
                "You can use this shell as if you are using your terminal.\n"
                "Any other command is executed by /bin/sh"
        )

    # === INITIALIZE REPO === #
    elif command.lower().startswith("init"):
        if os.getcwd() != PARENT_DIR:
            return "You need to be in the root directory of the repo to run this command."
        
        name = input("Enter the name of the new repo\n==> ")
        folder_name = input("Enter the name of the repo folder\n==> ")
        with open("workflow.json", 'w') as f:
            WORKFLOW[name] = {"folder": folder_name}
            os.mkdir(os.path.join(PARENT_DIR, folder_name))
            json.dump(WORKFLOW, f, indent=4)
        
        return f"Created a new repo named {name}"

    # === ADD COMMANDS IN A REPO === #
    elif command.lower().startswith("newcommand"):
        command = input("How do you want to invoke the custom command?\n==> ")
        func = input("What do you want the command to do? (What command does it run?)\n==> ")
        n = '\n'

        repoName = input(
            "Enter the name of the repo where you want the command to be in\n"
            f"{n.join((repos := list(WORKFLOW.keys())))}\n"
            "==> "
        )
        if repoName not in repos: return "No such repo exists"

        with open("workflow.json", 'w') as f:
            WORKFLOW[repoName][command] = func
            json.dump(WORKFLOW, f, indent=4)
    
    # === REMOVE COMMANDS === #
    elif command.lower().startswith("rmcommand"):
        n = "\n"
        repoName = input(
            "Enter the name of the repo where you want to remove the command\n"
            f"{n.join((repos := list(WORKFLOW.keys())))}\n"
            "==> "
        )
        if repoName not in repos: return "No such repo exists"
        commands = list(WORKFLOW[repoName].keys())
        commands.remove("folder")
        command = input(
            "Which command do you want to remove?\n"
            f"{n.join(commands)}\n"
            "==> "
        )
        if command not in commands: return "No such command exists"
        
        with open("workflow.json", 'w') as f:
            del WORKFLOW[repoName][command]
            json.dump(WORKFLOW, f, indent=4)

    # === TO EXIT === #     
    elif command.lower().startswith("exit"):
        exit_ = True

    else:
        commands, TERM = {}, os.environ["TERM"]
        for i in WORKFLOW:
            try:
                if (cmd := command.strip().split()[0]) in list(WORKFLOW[i].keys())[1:]:
                    commands[WORKFLOW[i]["folder"]] = WORKFLOW[i][cmd]
            except IndexError:
                return None

        if os.getcwd() != PARENT_DIR:
            os.system(command)
            return

        elif len(commands) == 1:
            os.chdir(list(commands.keys())[0])
            os.system(commands[list(commands.keys())[0]])
            os.chdir(PARENT_DIR)

        elif len(commands) > 0:
            if len((arguments := command.strip().split())) == 1:
                keys = list(commands.keys())
                for index, item in enumerate(commands.values()):
                    if TERM == "screen" and index == 0: _ = "tmux new-window"
                    elif index == 0: _ = "tmux new-session"
                    elif (index+1) % 2 == 0: _ = "split-window"
                    elif index != 0: _ = "new-window"
                    commands[keys[index]] = \
                        f"{_} \'cd {keys[index]} && {item} && sh -c read\'\\;"
                os.system(" ".join(commands.values()))
            
            else:

                arguments, tmux_cmd = arguments[1:], {}

                keys = list(WORKFLOW.keys())
                for index, item in enumerate(commands.values()):
                    if keys[index] not in arguments: continue
                    tmux_cmd[keys[index]] = \
                        f"{'tmux new-session' if len(tmux_cmd) == 0 else 'new-window'} " \
                        f"\'cd {WORKFLOW[keys[index]]['folder']} && {item} && sh -c read\'\\;"

                if len(tmux_cmd) == 0: return "Invalid repo(s)"
                os.system(" ".join(tmux_cmd.values()))
        
        else:
            os.system(command)

if __name__ == "__main__":
    # === SETUP ARGPARSE === #
    parser = argparse.ArgumentParser()
    parser.add_argument(
        'path',
        metavar='path/to/monorepo',
        type=str,
        const=None,
        nargs='?',
        help='Input repo path to monoinit'
    )

    # === GET ARGUMENTS === #
    args = parser.parse_args()

    if args.path is None:
        if "workflow.json" not in os.listdir():
            with open('workflow.json', 'w') as f:
                f.write("{}")
            print("The path specified doesn't have a workflow.json file. Creating workflow.json")

    # === IF THE PATH DOESN'T EXIST === #
    elif not os.path.isdir(args.path):
        sys.exit("Path not recognized")
    
    # === TO CHECK IF THE DIRECTORY CONTAINS REPOS === #
    elif "workflow.json" not in os.listdir(args.path):
        with open(os.path.join(args.path[0], 'workflow.json'), 'w') as f:
            f.write("{}")
        print("The path specified doesn't have a workflow.json file. Creating workflow.json")



    # === CHANGE DIRECTORY TO THE PATH === #
    if args.path is not None: os.chdir(args.path)


    # === GLOBAL VARIABLES === #
    PARENT_DIR = os.getcwd()
    IGNORE = git_ignore_scan()
    with open(os.path.join(PARENT_DIR, "workflow.json")) as f:
            WORKFLOW = json.loads(f.read())
            for i in WORKFLOW:
                if "folder" not in list(WORKFLOW[i].keys()):
                    sys.exit(f"workflow.json: There is no \"folder\" variable in \"{i}\"")
            for i in WORKFLOW:
                if(os.path.isdir(WORKFLOW[i]['folder'])):
                    pass
                else:
                    sys.exit(f"workflow.json: {WORKFLOW[i]['folder']} does not exist.")
                    
    # === IF THIS TURNS TRUE, THE SCRIPT STOPS === #
    exit_ = False

    # === COLORS === #
    LIGHT_BLUE, BLUE = "\033[36m", "\033[34m"
    GREEN = "\033[32m"
    RED = "\033[31m"
    RESET = "\033[0m"

    while not exit_:

        # === GET AND PRINT OUTPUT === #
        print(f'{GREEN}◆ {LIGHT_BLUE}{os.path.basename(os.getcwd())}{RESET}',end=" ")
        
        # ===Completer=== #
        completer = Completer([file for root,dirs,file in os.walk(PARENT_DIR)][0])
        readline.parse_and_bind('tab: complete')    
        readline.set_completer(completer.complete)

        output = shell(input(f"{RED}❯{GREEN}❯{BLUE}❯{RESET} "))
        Cmd(stdin=output)
        if output is None: continue
        else: print(output)
