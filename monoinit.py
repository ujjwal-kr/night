s="The path specified doesn't have a workflow.json file. Creating workflow.json"
r='{}'
k=False
q=enumerate
p=str
o=Exception
n=IndexError
g=' '
f='\n'
e=True
T='w'
Q=None
P=print
L='folder'
K=open
J='workflow.json'
I=input
H=list
G=len
import os as A,sys as O
if A.name=='nt':O.exit('This script is designed to run only on unix based systems.\nPlease use Windows Subsystem for Linux.')
import typing,argparse as X,readline as N,json as S,subprocess as R,traceback as l
from cmd import Cmd
class Z:
	def __init__(A,options):A.options=sorted(options)
	def complete(A,text,state):
		B=state
		if B==0:
			if text:A.matches=[B for B in A.options if B and B.startswith(text)]
			else:A.matches=A.options[:]
		try:return A.matches[B]
		except n:return Q
def a():
	F='.gitignore';C=k
	if F in A.listdir(E):C=e
	if C:
		with K(A.path.join(E,F),'r')as D:B=D.read().splitlines();B=[A.path.join(E,C)for C in B]
		return B
	return[]
def m(folder):
	H='.py'
	def F(path):
		B=path;D=[]
		for C in A.listdir(B):
			if A.path.isdir(A.path.join(B,C)):D+=F(A.path.join(B,C))
			elif b'ascii'in R.run(['file','--mime-encoding',A.path.join(B,C)],capture_output=e).stdout.lower()and A.path.join(B,C)not in d:D.append(A.path.join(B,C))
		return D
	I,J,K={},A.getcwd(),f
	for B in F(folder):
		C='#[[:blank:]]\\+todo'if B.endswith(H)else'//[[:blank:]]\\+todo'
		if(D:=R.getoutput(f'grep -in -e "{C}" -e "{"#todo"if B.endswith(H)else"//todo"}" {B}'))=='':continue
		P(f"FILE: {B[G(E)+1:]}\n\t{D}")
def b(command):
	z='new-window';y='tmux new-session';x='No such repo exists';w='"';v='Extra arguments passed';B=command;global E,Y,d,C
	if B.lower().startswith('cd'):
		if G((U:=B.strip().split()))==1:return'The path has not been supplied'
		elif G(U)>2:return v
		elif not A.path.isdir(U[1]):return'Cannot find the path specified'
		if E not in A.path.abspath(U[1]):P('Path specified goes beyond the parent directory');return
		A.chdir(U[1])
	elif B.lower().startswith('todos'):
		if G(B.strip().split())>1:return v
		return m(A.getcwd())
	elif B.lower()=='update':
		try:
			A.chdir('/tmp');A.system('wget https://raw.githubusercontent.com/hackarmour/monoinit/main/main.py')
			if'main.py'in A.listdir():A.system(f"cp ./main.py {__file__}");A.system('rm main.py')
			else:raise o('Cannot receive file from upstream')
			O.exit('MonoInit has been updated. Please rerun the script.')
		except o:l.print_exc()
	elif B.lower().startswith('git'):
		r=A.getcwd()
		if B.lower().startswith('git commit -m'):h=w;i=B.split(f"{h}");i[1]=f"{A.path.basename(A.getcwd())}: {B.split(f'{h}')[1]}";A.system(w.join(i));return
		A.chdir(E)
		if B.lower().startswith('git log'):A.system("git log --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit")
		else:A.system(B)
		A.chdir(r)
	elif B.lower().startswith('help'):return p('Command: todos\n\tUsage: todos\n\tUsed to get all the ToDos from the repos\n\nCommand: init\n\tUsage: init\n\tUsed to create a new repo\n\nCommand: newcommand\n\tUsage: newcommand\n\tUsed to add a new command to a repo\n\nCommand: rmcommand\n\tUsage: rmcommand\n\tUsed to remove a command from a repo\n\nCommand: update\n\tUsage: update\n\tTo update monoinit\n\nCommand: exit\n\tUsage: exit\n\tTo exit the shell\n\nYou can use this shell as if you are using your terminal.\nAny other command is executed by /bin/sh')
	elif B.lower().startswith('init'):
		if A.getcwd()!=E:return'You need to be in the root directory of the repo to run this command.'
		j=I('Enter the name of the new repo\n==> ');k=I('Enter the name of the repo folder\n==> ')
		with K(J,T)as R:C[j]={L:k};A.mkdir(A.path.join(E,k));S.dump(C,R,indent=4)
		return f"Created a new repo named {j}"
	elif B.lower().startswith('newcommand'):
		B=I('How do you want to invoke the custom command?\n==> ');s=I('What do you want the command to do? (What command does it run?)\n==> ');V=f;M=I(f"Enter the name of the repo where you want the command to be in\n{V.join((Z:=H(C.keys())))}\n==> ")
		if M not in Z:return x
		with K(J,T)as R:C[M][B]=s;S.dump(C,R,indent=4)
	elif B.lower().startswith('rmcommand'):
		V=f;M=I(f"Enter the name of the repo where you want to remove the command\n{V.join((Z:=H(C.keys())))}\n==> ")
		if M not in Z:return x
		D=H(C[M].keys());D.remove(L);B=I(f"Which command do you want to remove?\n{V.join(D)}\n==> ")
		if B not in D:return'No such command exists'
		with K(J,T)as R:del C[M][B];S.dump(C,R,indent=4)
	elif B.lower().startswith('exit'):Y=e
	else:
		D,t={},A.environ['TERM']
		for a in C:
			try:
				if(u:=B.strip().split()[0])in H(C[a].keys())[1:]:D[C[a][L]]=C[a][u]
			except n:return Q
		if A.getcwd()!=E:A.system(B);return
		elif G(D)==1:A.chdir(H(D.keys())[0]);A.system(D[H(D.keys())[0]]);A.chdir(E)
		elif G(D)>0:
			if G((b:=B.strip().split()))==1:
				N=H(D.keys())
				for (F,c) in q(D.values()):
					if t=='screen'and F==0:W='tmux new-window'
					elif F==0:W=y
					elif(F+1)%2==0:W='split-window'
					elif F!=0:W=z
					D[N[F]]=f"{W} 'cd {N[F]} && {c} && sh -c read'\\;"
				A.system(g.join(D.values()))
			else:
				b,X=b[1:],{};N=H(C.keys())
				for (F,c) in q(D.values()):
					if N[F]not in b:continue
					X[N[F]]=f"{y if G(X)==0 else z} 'cd {C[N[F]][L]} && {c} && sh -c read'\\;"
				if G(X)==0:return'Invalid repo(s)'
				A.system(g.join(X.values()))
		else:A.system(B)
if __name__=='__main__':
	U=X.ArgumentParser();U.add_argument('path',metavar='path/to/monorepo',type=p,const=Q,nargs='?',help='Input repo path to monoinit');B=U.parse_args()
	if B.path is Q:
		if J not in A.listdir():
			with K(J,T)as D:D.write(r)
			P(s)
	elif not A.path.isdir(B.path):O.exit('Path not recognized')
	elif J not in A.listdir(B.path):
		with K(A.path.join(B.path[0],J),T)as D:D.write(r)
		P(s)
	if B.path is not Q:A.chdir(B.path)
	E=A.getcwd();d=a()
	with K(A.path.join(E,J))as D:
		C=S.loads(D.read())
		for F in C:
			if L not in H(C[F].keys()):O.exit(f'workflow.json: There is no "folder" variable in "{F}"')
		for F in C:
			if A.path.isdir(C[F][L]):0
			else:O.exit(f"workflow.json: {C[F][L]} does not exist.")
	Y=k;c,h='\x1b[36m','\x1b[34m';V='\x1b[32m';i='\x1b[31m';W='\x1b[0m'
	while not Y:
		P(f"{V}◆ {c}{A.path.basename(A.getcwd())}{W}",end=g);j=Z([B for(C,D,B)in A.walk(E)][0]);N.parse_and_bind('tab: complete');N.set_completer(j.complete);M=b(I(f"{i}❯{V}❯{h}❯{W} "));Cmd(stdin=M)
		if M is Q:continue
		else:P(M)